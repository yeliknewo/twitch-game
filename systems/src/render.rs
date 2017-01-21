use components::RenderId;
use event_core::DuoChannel;
use events::render::{FromRender, ToRender};
use gfx::{Primitive, Resources};
use gfx::handle::{DepthStencilView, RenderTargetView, ShaderResourceView};
use gfx::texture::{FilterMethod, SamplerInfo, WrapMode};
use gfx::traits::{Factory, FactoryExt};
use graphics::{ColorFormat, DepthFormat};
use graphics::pipeline::{Bundle, Packet, make_shaders, pipe};
use graphics::shaders::Shaders;
use std::sync::Arc;

pub struct RenderSystem<ID, R>
    where ID: Eq,
          R: Resources
{
    main_channel: DuoChannel<ID, FromRender, ToRender>,
    out_color: RenderTargetView<R, ColorFormat>,
    out_depth: DepthStencilView<R, DepthFormat>,
    bundles: Arc<Vec<Bundle<R>>>,
    shaders: Shaders,
}

impl<ID, R> RenderSystem<ID, R>
    where ID: Eq,
          R: Resources
{
    pub fn new(main_channel: DuoChannel<ID, FromRender, ToRender>, out_color: RenderTargetView<R, ColorFormat>, out_depth: DepthStencilView<R, DepthFormat>) -> RenderSystem<ID, R> {
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

        let program = factory.create_program(&shader_set).unwrap_or_else(|err| panic!("Create Program Error: {:?}", err));

        let pso = factory.create_pipeline_from_program(&program, Primitive::TriangleList, packet.get_rasterizer(), pipe::new()).unwrap_or_else(|err| panic!("Create Pipeline from Program Error: {:?}", err));

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
}
