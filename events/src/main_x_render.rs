use graphics::TGEncoder;

pub enum MainFromRender<ID>
    where ID: Send
{
    Encoder(TGEncoder, ID),
}

pub enum MainToRender<ID>
    where ID: Send
{
    Encoder(TGEncoder, ID),
}
