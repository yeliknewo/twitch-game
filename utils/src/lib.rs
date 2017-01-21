#[macro_use]
extern crate log;
extern crate rustc_serialize;
extern crate cgmath;

pub mod fps_counter;
pub mod ortho_helper;

pub use self::fps_counter::FpsCounter;
pub use self::ortho_helper::OrthographicHelper;

pub type Delta = f64;
pub type Coord = f32;
pub type CoordI = i64;
