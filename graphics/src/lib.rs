#[macro_use]
extern crate gfx;
#[macro_use]
extern crate gfx_device_gl;
extern crate image;
#[macro_use]
extern crate log;
extern crate find_folder;
#[cfg(feature = "g_glutin")]
extern crate glutin;
#[cfg(feature = "g_glutin")]
extern crate gfx_window_glutin;
#[cfg(feature = "g_sdl2")]
extern crate sdl2;
#[cfg(feature = "g_sdl2")]
extern crate gfx_window_sdl;

extern crate utils;

pub mod pipeline;
#[cfg(feature = "g_glutin")]
pub mod tg_glutin;
#[cfg(feature = "g_sdl2")]
pub mod tg_sdl2;

pub mod shaders;
pub mod texture;

pub type ColorFormat = ::gfx::format::Srgba8;
pub type DepthFormat = ::gfx::format::DepthStencil;

pub type WindowSettings<'a> = (&'a str, u32, u32);

use gfx::Resources;
use gfx::handle::{DepthStencilView, RenderTargetView};

pub struct GfxWindow<W, T, D, F, R>
    where R: Resources
{
    out_color: RenderTargetView<R, ColorFormat>,
    out_depth: DepthStencilView<R, DepthFormat>,
    device: D,
    factory: F,
    window: W,
    extras: T,
}

#[cfg(feature = "g_glutin")]
impl<T, D, F, R> GfxWindow<glutin::Window, T, D, F, R>
    where R: Resources
{
    pub fn swap_buffers(&mut self) {
        self.get_mut_window().swap_buffers().unwrap_or_else(|err| panic!("{:?}", err));
    }
}

#[cfg(feature = "g_sdl2")]
impl<T, D, F, R> GfxWindow<sdl2::video::Window, T, D, F, R>
    where R: Resources
{
    pub fn swap_buffers(&mut self) {
        self.get_mut_window().gl_swap_window();
    }
}

impl<W, T, D, F, R> GfxWindow<W, T, D, F, R>
    where R: Resources
{
    pub fn new(out_color: RenderTargetView<R, ColorFormat>, out_depth: DepthStencilView<R, DepthFormat>, device: D, factory: F, window: W, extras: T) -> GfxWindow<W, T, D, F, R> {
        GfxWindow {
            out_color: out_color,
            out_depth: out_depth,
            device: device,
            factory: factory,
            window: window,
            extras: extras,
        }
    }

    pub fn get_out_color(&self) -> &RenderTargetView<R, ColorFormat> {
        &self.out_color
    }

    pub fn get_out_depth(&self) -> &DepthStencilView<R, DepthFormat> {
        &self.out_depth
    }

    pub fn get_device(&self) -> &D {
        &self.device
    }

    pub fn get_factory(&self) -> &F {
        &self.factory
    }

    pub fn get_window(&self) -> &W {
        &self.window
    }

    pub fn get_extras(&self) -> &T {
        &self.extras
    }

    pub fn get_mut_device(&mut self) -> &mut D {
        &mut self.device
    }

    pub fn get_mut_factory(&mut self) -> &mut F {
        &mut self.factory
    }

    pub fn get_mut_window(&mut self) -> &mut W {
        &mut self.window
    }

    pub fn get_mut_extras(&mut self) -> &mut T {
        &mut self.extras
    }
}
