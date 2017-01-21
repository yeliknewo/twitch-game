use event_core::DuoChannel;
use events::main_x_control::{MainFromControl, MainToControl};
use events::main_x_render::{MainFromRender, MainToRender};

pub fn make_event_clumps<ID>(front_control_id: ID, back_control_id: ID, front_render_id: ID, back_render_id: ID) -> (FrontEventClump<ID>, BackEventClump<ID>)
    where ID: Send + Eq
{
    let (front_control, back_control) = DuoChannel::new_both(front_control_id, back_control_id);
    let (front_render, back_render) = DuoChannel::new_both(front_render_id, back_render_id);

    (FrontEventClump::new(front_control, front_render), BackEventClump::new(back_control, back_render))
}

pub struct BackEventClump<ID>
    where ID: Send + Eq
{
    control: Option<DuoChannel<ID, MainFromControl, MainToControl>>,
    render: Option<DuoChannel<ID, MainFromRender<ID>, MainToRender<ID>>>,
}

impl<ID, R, C> BackEventClump<ID, R, C>
    where ID: Send + Eq,
          R: Send + Resources,
          <R as Resources>::Fence: Send + Sync,
          <R as Resources>::Mapping: Send,
          C: Send + CommandBuffer<R>
{
    fn new(control: DuoChannel<ID, MainFromControl, MainToControl>,
           render: DuoChannel<ID, MainFromRender<Encoder<R, C>, ID>, MainToRender<Encoder<R, C>, ID>>)
           -> BackEventClump<ID, R, C> {
        BackEventClump {
            control: Some(control),
            render: Some(render),
        }
    }

    pub fn take_render(&mut self) -> Option<DuoChannel<ID, MainFromRender<Encoder<R, C>, ID>, MainToRender<Encoder<R, C>, ID>>> {
        self.render.take()
    }
}

pub struct FrontEventClump<ID, R, C>
    where ID: Send + Eq,
          R: Send + Resources,
          <R as Resources>::Fence: Send + Sync,
          <R as Resources>::Mapping: Send,
          C: Send + CommandBuffer<R>
{
    control: Option<DuoChannel<ID, MainToControl, MainFromControl>>,
    render: Option<DuoChannel<ID, MainToRender<Encoder<R, C>, ID>, MainFromRender<Encoder<R, C>, ID>>>,
}

impl<ID, R, C> FrontEventClump<ID, R, C>
    where ID: Send + Eq,
          R: Send + Resources,
          <R as Resources>::Fence: Send + Sync,
          <R as Resources>::Mapping: Send,
          C: Send + CommandBuffer<R>
{
    fn new(control: DuoChannel<ID, MainToControl, MainFromControl>,
           render: DuoChannel<ID, MainToRender<Encoder<R, C>, ID>, MainFromRender<Encoder<R, C>, ID>>)
           -> FrontEventClump<ID, R, C> {
        FrontEventClump {
            control: Some(control),
            render: Some(render),
        }
    }

    pub fn get_mut_control(&mut self) -> Option<&mut DuoChannel<ID, MainToControl, MainFromControl>> {
        self.control.as_mut()
    }

    pub fn get_mut_render(&mut self) -> Option<&mut DuoChannel<ID, MainToRender<Encoder<R, C>, ID>, MainFromRender<Encoder<R, C>, ID>>> {
        self.render.as_mut()
    }
}
