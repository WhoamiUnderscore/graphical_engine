use pixels::{SurfaceTexture, Pixels};

use crate::core::viewport::Viewport;

pub struct Engine {
    pub pixels: Pixels<'static>,
}

impl Engine {
    pub fn new(viewport: &Viewport) -> Self {
        let surface_texture = SurfaceTexture::new(viewport.width, viewport.height, viewport.window.clone().unwrap());
        let pixels: Pixels<'static> = Pixels::new(viewport.width, viewport.height, surface_texture).expect("ERROR: Durring pixels creation");

        Engine {
            pixels
        }
    }

}
