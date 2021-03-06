use {ColorFormat, TGFactory, TGTexture};
use gfx::Factory;
use gfx::texture::{AaMode, Kind, Size};
use image;
use std::path::Path;

pub fn load_texture<P>(factory: &mut TGFactory, path: P) -> TGTexture
    where P: AsRef<Path>
{
    let image = match image::open(path) {
            Ok(image) => image,
            Err(err) => panic!("image load error: {}", err),
        }
        .to_rgba();

    let (width, height) = image.dimensions();
    let kind = Kind::D2(width as Size, height as Size, AaMode::Single);
    let (_, view) = match factory.create_texture_immutable_u8::<ColorFormat>(kind, &[&image]) {
        Ok(data) => data,
        Err(err) => panic!("factory create texture const error: {}", err),
    };
    view
}
