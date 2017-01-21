use event_clump::FrontEventClump;
use gfx::{CommandBuffer, Resources};
use graphics::GfxWindow;

pub fn handle_events<ID, R, C, D, F, W, T>(gfx_window: &mut GfxWindow<W, T, D, F, R>, front_event_clump: &mut FrontEventClump<ID, R, C>) -> bool
    where ID: Eq,
          R: Resources,
          C: CommandBuffer<R>
{
    false
}
