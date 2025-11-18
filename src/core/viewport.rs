use std::sync::Arc;

use winit::{
    event_loop::ActiveEventLoop,
    dpi::LogicalSize,

    window::{Window, WindowAttributes}
};

use crate::core::error::{ViewportResult, ViewportError};

#[derive(Debug)]
pub struct Viewport {
    pub width: u32,
    pub height: u32,
    pub window: Option<Arc<Window>>
}

impl Viewport {
    pub fn new(width: Option<u32>, height: Option<u32>) -> Self {
        let ( width, height )= {
            let width = match width {
                Some(w) => w,
                None => 800
            };

            let height = match height {
                Some(h) => h,
                None => 600
            };

            ( width, height )
        };

        Viewport {
            width,
            height,
            window: None
        }
    }

    pub fn create_window(&mut self, event_loop: &ActiveEventLoop, window_attributes: Option<WindowAttributes>) -> ViewportResult<()>{
        let window_attributes = match window_attributes {
            Some(wa) => wa,
            None => Window::default_attributes()
                .with_inner_size(LogicalSize::new(self.width, self.height))
                .with_title("jeu")
        };

        match event_loop.create_window(window_attributes) {
            Ok(window) => {
                self.window = Some(Arc::new(window));
            }
            Err(e) => {
                return Err(ViewportError::WindowError(e.to_string()));
            }
        }

        return Ok(());

    }
}

