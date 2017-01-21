#[macro_use]
extern crate log;
extern crate gfx;
extern crate time;

extern crate art;
extern crate event_core;
extern crate events;
extern crate graphics;
extern crate systems;
extern crate utils;

mod event_clump;
mod game;
mod handle_events;

use event_clump::{BackEventClump, make_event_clumps};
use events::main_x_control::{MainFromControl, MainToControl};
use events::main_x_render::{MainFromRender, MainToRender};
use game::Game;
use gfx::Device;
use handle_events::handle_events;
use std::thread;
use utils::OrthographicHelper;

pub fn start() {
    let (mut front_event_clump, mut back_event_clump) = make_event_clumps(0, 1, 2, 3);

    let title = "Twitch Game";

    let (width, height): (u32, u32) = (640, 480);

    use graphics::tg_glutin::build_window;

    let mut gfx_window = build_window((title, width, height));

    {
        let mut render_event_core = front_event_clump.get_mut_render().unwrap_or_else(|| panic!("Render was none"));

        render_event_core.send(MainToRender::Encoder(gfx_window.get_mut_factory().create_command_buffer().into(), 0));
        render_event_core.send(MainToRender::Encoder(gfx_window.get_mut_factory().create_command_buffer().into(), 1));
    }

    let out_color = gfx_window.get_out_color().clone();
    let out_depth = gfx_window.get_out_depth().clone();

    let aspect_ratio = width as f32 / height as f32;
    let left = -10.0;
    let right = 10.0;
    let near = 0.0;
    let far = 10.0;

    let ortho = OrthographicHelper::new(aspect_ratio, left, right, near, far);

    let game = Game::new(gfx_window.get_mut_factory(), back_event_clump, ortho.as_ref().clone(), out_color, out_depth);

    let game_handle = thread::spawn(|| {
        let mut game = game;
        while game.frame() {}
    });

    'main: loop {
        if let Some(event) = front_event_clump.get_mut_render().unwrap_or_else(|| panic!("Render was none")).try_recv() {
            match event {
                MainFromRender::Encoder(mut encoder, encoder_id) => {
                    if handle_events(&mut gfx_window, &mut front_event_clump) {
                        front_event_clump.get_mut_render().unwrap_or_else(|| panic!("Render was none")).send(MainToRender::Encoder(encoder, encoder_id));
                        break 'main;
                    }

                    encoder.flush(gfx_window.get_mut_device());
                    front_event_clump.get_mut_render().unwrap_or_else(|| panic!("Render was none")).send(MainToRender::Encoder(encoder, encoder_id));
                    gfx_window.swap_buffers();
                    gfx_window.get_mut_device().cleanup();
                }
            }
        }
    }
}
