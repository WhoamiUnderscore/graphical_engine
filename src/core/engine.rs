use winit::event_loop::ActiveEventLoop;
use pixels::{SurfaceTexture, Pixels};

use crate::core::{
    viewport::Viewport,
    scene::Scene,
    error::{EngineResult, EngineError}
};

pub struct Engine {
    pub viewport: Viewport,
    pub pixels: Pixels<'static>,
}

impl Engine {
    pub  fn new(event_loop: &ActiveEventLoop) -> EngineResult<Self> {
        let mut viewport = Viewport::new(None, None);
        viewport.create_window(event_loop, None);

        let window = match viewport.window {
            Some(ref w) => w,
            None => return Err(EngineError::WindowAccessDeny)
        };

        let surface_texture = SurfaceTexture::new(viewport.width, viewport.height, window.clone());
        let pixels: Pixels<'static> = Pixels::new(viewport.width, viewport.height, surface_texture).expect("ERROR: Durring pixels creation");

        Ok(Engine {
            viewport,
            pixels
        })
    }

    fn clear(&mut self) {
        let frame = self.pixels.frame_mut();
        
        for pixel in frame.chunks_exact_mut(4) {
            pixel[0] = 0x00;
            pixel[1] = 0x00;
            pixel[2] = 0x00;
            pixel[3] = 0xff;
        }
    }

    fn render_scene(&mut self, scene: &Scene) {
        let frame = self.pixels.frame_mut();

        for triangle in scene.triangle_iter() {
            println!("there is triangle");
        }
    }

    pub fn render(&mut self, scene: &Scene) -> EngineResult<()>{
        self.clear();
        self.render_scene(scene);
        match self.pixels.render() {
            Ok(_) => return Ok(()),
            Err(err) => {
                return Err(EngineError::PixelRenderingError(err.to_string()));
            }
        }
    }
}
