use components::{Camera, RenderData, RenderId, Transform};
use event_core::DuoChannel;
use events::main_x_render::{MainFromRender, MainToRender};
use gfx::{CommandBuffer, Encoder, Primitive, Resources};
use gfx::handle::{DepthStencilView, RenderTargetView, ShaderResourceView};
use gfx::texture::{FilterMethod, SamplerInfo, WrapMode};
use gfx::traits::{Factory, FactoryExt};
use graphics::{ColorFormat, DepthFormat};
use graphics::pipeline::{Bundle, Packet, ProjectionData, TextureData, make_shaders, pipe};
use graphics::shaders::Shaders;
use specs::{RunArg, System};
use std::sync::Arc;
use utils::Delta;

pub struct RenderSystem<ID, R, C>
    where ID: Send + Eq,
          R: Send + Resources,
          C: Send + CommandBuffer<R>
{
    main_channel: DuoChannel<ID, MainFromRender<Encoder<R, C>, ID>, MainToRender<Encoder<R, C>, ID>>,
    out_color: RenderTargetView<R, ColorFormat>,
    out_depth: DepthStencilView<R, DepthFormat>,
    bundles: Arc<Vec<Bundle<R>>>,
    shaders: Shaders,
}

impl<ID, R, C> RenderSystem<ID, R, C>
    where ID: Send + Eq,
          R: Send + Resources,
          C: Send + CommandBuffer<R>
{
    pub fn new(main_channel: DuoChannel<ID, MainFromRender<Encoder<R, C>, ID>, MainToRender<Encoder<R, C>, ID>>,
               out_color: RenderTargetView<R, ColorFormat>,
               out_depth: DepthStencilView<R, DepthFormat>)
               -> RenderSystem<ID, R, C> {
        RenderSystem {
            main_channel: main_channel,
            out_color: out_color,
            out_depth: out_depth,
            bundles: Arc::new(vec![]),
            shaders: make_shaders(),
        }
    }

    pub fn add_render<F>(&mut self, factory: &mut F, packet: &Packet, texture: ShaderResourceView<R, [f32; 4]>) -> RenderId
        where F: Factory<R> + FactoryExt<R>
    {
        let shader_set = factory.create_shader_set(self.shaders.get_vertex_shader(), self.shaders.get_fragment_shader())
            .unwrap_or_else(|err| panic!("Create Shader Set Error: {:?}", err));

        let program = factory.create_program(&shader_set)
            .unwrap_or_else(|err| panic!("Create Program Error: {:?}", err));

        let pso = factory.create_pipeline_from_program(&program, Primitive::TriangleList, packet.get_rasterizer(), pipe::new())
            .unwrap_or_else(|err| panic!("Create Pipeline from Program Error: {:?}", err));

        let sampler_info = SamplerInfo::new(FilterMethod::Scale, WrapMode::Mirror);

        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(packet.get_vertices(), packet.get_indices());

        let data = pipe::Data {
            vbuf: vbuf,
            spritesheet: (texture, factory.create_sampler(sampler_info)),
            texture_data: factory.create_constant_buffer(1),
            projection_data: factory.create_constant_buffer(1),
            out_color: self.out_color.clone(),
            out_depth: self.out_depth.clone(),
        };

        let mut bundles = Arc::get_mut(&mut self.bundles).unwrap_or_else(|| panic!("Arc Shit"));

        let id = bundles.len();

        bundles.push(Bundle::new(slice, pso, data));

        RenderId::new(id)
    }

    fn render(&mut self, arg: &RunArg, mut encoder: Encoder<R, C>, encoder_id: ID) {
        use specs::Join;

        let (render_ids, mut transforms, mut cameras, mut render_datas) =
            arg.fetch(|w| (w.read::<RenderId>(), w.write::<Transform>(), w.write::<Camera>(), w.write::<RenderData>()));

        encoder.clear(&self.out_color, [0.0, 0.0, 0.0, 1.0]);
        encoder.clear_depth(&self.out_depth, 1.0);
        let (dirty_cam, view, proj) = {
            let camera = {
                let mut camera_opt = None;

                for camera in (&mut cameras).iter() {
                    if camera.is_main() {
                        camera_opt = Some(camera);
                    }
                }

                camera_opt.unwrap_or_else(|| panic!("No Main Camera Entity"))
            };

            (camera.take_dirty(), camera.get_view(), camera.get_proj())
        };

        let mut datas = vec![];

        for (render_id, transform, render_data) in (&render_ids, &mut transforms, &mut render_datas).iter() {
            let mut projection_data = None;

            if dirty_cam || transform.take_dirty() {
                projection_data = Some(ProjectionData {
                    model: transform.get_model().into(),
                    view: view.into(),
                    proj: proj.into(),
                });
            }

            let mut texture_data = None;

            if render_data.take_dirty() {
                texture_data = Some(TextureData {
                    tint: render_data.get_tint(),
                    spritesheet_rect: render_data.get_spritesheet_rect(),
                    spritesheet_size: render_data.get_spritesheet_size(),
                });
            }

            datas.push((render_id.get_render_id_num(), render_data.get_layer(), texture_data, projection_data));
        }

        datas.sort_by_key(|k| k.1);

        for data in datas {
            let b = self.bundles
                .get(data.0)
                .unwrap_or_else(|| panic!("No Bundle found"));

            if let Some(texture_data) = data.2 {
                encoder.update_constant_buffer(&b.get_data().texture_data, &texture_data);
            }

            if let Some(projection_data) = data.3 {
                encoder.update_constant_buffer(&b.get_data().projection_data, &projection_data);
            }

            b.encode(&mut encoder);
        }

        self.main_channel.send(MainFromRender::Encoder(encoder, encoder_id));
    }

    fn process_event(&mut self, arg: &RunArg, event: MainToRender<Encoder<R, C>, ID>) -> bool {
        match event {
            MainToRender::Encoder(encoder, encoder_id) => {
                self.render(arg, encoder, encoder_id);
                false
            }
        }
    }
}

impl<ID, R, C> System<f64> for RenderSystem<ID, R, C>
    where ID: Send + Eq,
          R: Send + Resources,
          <R as Resources>::Fence: Send + Sync,
          <R as Resources>::Mapping: Send,
          C: Send + CommandBuffer<R>
{
    fn run(&mut self, arg: RunArg, delta_time: Delta) {
        let mut event = self.main_channel.try_recv();
        while self.process_event(&arg,
                                 match event {
                                     Some(event) => event,
                                     None => {
                                         arg.fetch(|_| {});
                                         return;
                                     }
                                 }) {
            event = self.main_channel.try_recv();
        }
    }
}
