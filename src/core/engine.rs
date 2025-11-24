use winit::event_loop::ActiveEventLoop;
use pixels::{SurfaceTexture, Pixels};

use crate::core::{
    viewport::Viewport,
    view::View,
    scene::Scene,
    error::{EngineResult, EngineError}
};

pub struct Engine {
    pub viewport: Viewport,
    pub view: View,
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
            view: View::new(),
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

    fn render_scene(&mut self, scene: &mut Scene) {
        let frame = self.pixels.frame_mut();

        for triangle in scene.triangles_iter_mut() {
            triangle.draw(frame, self.viewport.width, self.viewport.height);
        }
    }

    pub fn render(&mut self, scene: &mut Scene) -> EngineResult<()>{
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
