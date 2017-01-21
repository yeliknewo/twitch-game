pub mod glutin {
    use event_clump::FrontEventClump;
    use gfx::{CommandBuffer, Resources};
    use glutin::{Event, VirtualKeyCode, Window};
    use graphics::GfxWindow;

    pub fn handle_events<ID, R, C, D, F, T>(gfx_window: &mut GfxWindow<Window, T, D, F, R>, front_event_clump: &mut FrontEventClump<ID, R, C>) -> bool
        where ID: Send + Eq,
              R: Send + Resources,
              <R as Resources>::Fence: Send + Sync,
              <R as Resources>::Mapping: Send,
              C: Send + CommandBuffer<R>
    {
        while let Some(event) = gfx_window.get_mut_window().poll_events().next() {
            match event {
                Event::KeyboardInput(_, _, Some(key_code)) => {
                    match key_code {
                        VirtualKeyCode::Escape => return true,
                        _ => (),
                    }
                }
                Event::Closed => return true,
                _ => (),
            }
        }

        false
    }
}
