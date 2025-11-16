use std::sync::Arc;

use winit::event_loop::ActiveEventLoop;

#[derive(Debug)]
pub struct Viewport {
    pub width: u32,
    pub height: u32,
    pub window: Option<Arc<winit::window::Window>>
}

impl Viewport {
    pub fn new(width: Option<u32>, height: Option<u32>) -> Self {
        let ( width, height )= {
            let width = match width {
                Some(w) => w,
                None => 800 as u32
            };

            let height = match height {
                Some(h) => h,
                None => 600 as u32
            };

            ( width, height )
        };

        Viewport {
            width,
            height,
            window: None
        }
    }

    pub fn create_window(&mut self, event_loop: &ActiveEventLoop, window_attributes: Option<winit::window::WindowAttributes>) {
        let window_attributes = match window_attributes {
            Some(wa) => wa,
            None => winit::window::Window::default_attributes().with_title("jeu")
        };

        self.window = Some(Arc::new(
            event_loop.create_window(window_attributes).unwrap()
        ));
    }
}
