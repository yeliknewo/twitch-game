#[derive(Debug)]
pub enum MainFromRender<E, ID> {
    Encoder(E, ID),
}

#[derive(Debug)]
pub enum MainToRender<E, ID> {
    Encoder(E, ID),
}
