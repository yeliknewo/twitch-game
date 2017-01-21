use components::{Camera, RenderData, RenderId, Transform};
use event_clump::BackEventClump;
use gfx::{CommandBuffer, Resources};
use gfx::handle::{DepthStencilView, RenderTargetView};
use gfx::traits::{Factory, FactoryExt};
use graphics::{ColorFormat, DepthFormat};
use specs::{Planner, World};
use systems::render::RenderSystem;
use time::precise_time_ns;
use utils::{Delta, FpsCounter, OrthographicHelper};

pub struct Game {
    last_time: u64,
    planner: Planner<Delta>,
    fps_counter: FpsCounter,
}

impl Game {
    pub fn new(factory: &mut TGFactory,
               mut back_event_clump: BackEventClump<ID, R, C>,
               ortho: OrthographicHelper,
               out_color: RenderTargetView<R, ColorFormat>,
               out_depth: DepthStencilView<R, DepthFormat>)
               -> Game {
        let mut planner = {
            let mut world = World::new();

            world.register::<Transform>();
            world.register::<RenderId>();
            world.register::<Camera>();
            world.register::<RenderData>();

            Planner::<Delta>::new(world, 8)
        };

        let mut renderer = RenderSystem::new(back_event_clump.take_render().unwrap_or_else(|| panic!("Render was none")), out_color, out_depth);

        planner.add_system(renderer, "renderer", 10);

        Game {
            last_time: precise_time_ns(),
            planner: planner,
            fps_counter: FpsCounter::new(50),
        }
    }

    pub fn frame(&mut self) -> bool {
        let new_time = precise_time_ns();
        let delta = (new_time - self.last_time) as Delta / 1e9;
        self.last_time = new_time;

        self.planner.dispatch(delta);
        self.fps_counter.frame(delta);
        true
    }
}
