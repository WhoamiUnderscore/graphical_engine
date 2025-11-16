use winit::{
    application::ApplicationHandler,
    event::{WindowEvent, ElementState, MouseButton},
    event_loop::ActiveEventLoop,
    dpi::LogicalSize,
    window::WindowId,
    keyboard::{Key, SmolStr}
};

use crate::core::viewport::Viewport;
use crate::core::mouse::Mouse;
use crate::engine::Engine;
use crate::math::vector::{Vector2, Vector4};
use crate::math::triangle::Triangle;
use crate::utils::time::Time;

pub struct App {
    pub viewport: Viewport,
    pub engine: Option<Engine>,
    pub mouse: Mouse,
    pub time: Time,
    pub fps: f64
}

impl App {
    pub fn new() -> Self {
        App {
            viewport: Viewport::new(None, None),
            engine: None,
            mouse: Mouse::new(),
            time: Time::new(),
            fps: 0.0
        }
    }

    fn update(&mut self) {
        self.time.update();
        self.fps = 1.0 / self.time.delta_time as f64;

        match self.engine.as_mut() {
            Some(e) => {
                e.render();
            }
            None => {}
        };
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = winit::window::Window::default_attributes()
            .with_inner_size(LogicalSize::new(self.viewport.width, self.viewport.height))
            .with_title("jeu".to_string());

        self.viewport.create_window(event_loop, Some(window_attributes));
        self.engine = Some(Engine::new(&self.viewport));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                self.handle_input(event);
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse.update_position(position.x, position.y);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                match button {
                    MouseButton::Left => {
                        self.mouse.left_click(state, &mut self.engine, self.viewport.width, self.viewport.height);
                    }
                    _ => {}
                }
            }
            WindowEvent::RedrawRequested => {
                // Draw
                self.update();

                self.viewport.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::Resized(size) => {
                println!("Window resized: {:?}", size);
            }
            WindowEvent::CloseRequested => {
                println!("Window close requested");
                event_loop.exit();
            }
            _ => {}
        }
    }
}
