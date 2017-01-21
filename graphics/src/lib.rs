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
pub type TGDevice = ::gfx_device_gl::Device;
pub type TGFactory = ::gfx_device_gl::Factory;
pub type TGResources = ::gfx_device_gl::Resources;
pub type TGCommandBuffer = ::gfx_device_gl::CommandBuffer;
pub type TGEncoder = ::gfx::Encoder<TGResources, TGCommandBuffer>;
pub type TGTexture = ::gfx::handle::ShaderResourceView<TGResources, [f32; 4]>;
pub type OutColor = ::gfx::handle::RenderTargetView<TGResources, ColorFormat>;
pub type OutDepth = ::gfx::handle::DepthStencilView<TGResources, DepthFormat>;

pub type WindowSettings<'a> = (&'a str, u32, u32);

pub struct GfxWindow<W, T> {
    out_color: OutColor,
    out_depth: OutDepth,
    device: TGDevice,
    factory: TGFactory,
    window: W,
    extras: T,
}

#[cfg(feature = "g_glutin")]
impl<T> GfxWindow<glutin::Window, T> {
    pub fn swap_buffers(&mut self) {
        self.get_mut_window().swap_buffers().unwrap_or_else(|err| panic!("{:?}", err));
    }
}

#[cfg(feature = "g_sdl2")]
impl<T> GfxWindow<sdl2::video::Window, T> {
    pub fn swap_buffers(&mut self) {
        self.get_mut_window().gl_swap_window();
    }
}

impl<W, T> GfxWindow<W, T> {
    pub fn new(out_color: OutColor, out_depth: OutDepth, device: TGDevice, factory: TGFactory, window: W, extras: T) -> GfxWindow<W, T> {
        GfxWindow {
            out_color: out_color,
            out_depth: out_depth,
            device: device,
            factory: factory,
            window: window,
            extras: extras,
        }
    }

    pub fn get_out_color(&self) -> &OutColor {
        &self.out_color
    }

    pub fn get_out_depth(&self) -> &OutDepth {
        &self.out_depth
    }

    pub fn get_device(&self) -> &TGDevice {
        &self.device
    }

    pub fn get_factory(&self) -> &TGFactory {
        &self.factory
    }

    pub fn get_window(&self) -> &W {
        &self.window
    }

    pub fn get_extras(&self) -> &T {
        &self.extras
    }

    pub fn get_mut_device(&mut self) -> &mut TGDevice {
        &mut self.device
    }

    pub fn get_mut_factory(&mut self) -> &mut TGFactory {
        &mut self.factory
    }

    pub fn get_mut_window(&mut self) -> &mut W {
        &mut self.window
    }

    pub fn get_mut_extras(&mut self) -> &mut T {
        &mut self.extras
    }
}
