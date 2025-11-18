use winit::{
    application::ApplicationHandler,
    event::{WindowEvent, MouseButton, ElementState},
    event_loop::ActiveEventLoop,
    keyboard::{PhysicalKey, KeyCode},
    dpi::LogicalSize,
    window::WindowId,
};

use crate::core::{
    scene::Scene,
    error::*,
    engine::Engine,
    input::{InputEvent, InputQueue},
    view::View
};

pub struct App {
    pub engine: Option<Engine>,
    pub scene: Scene,
    pub view: View,
    pub input: InputQueue,
    pub last_error: Option<GlobalError>
}

impl App {
    pub fn new() -> Self {
        App { 
            engine: None,
            scene: Scene::new(),
            view: View::new(),
            input: InputQueue::new(),
            last_error: None
        }

    }

    pub fn update(&mut self) {
        // Game loop
        let engine: &mut Engine = match &mut self.engine {
            Some(e) => e,
            None => {
                self.last_error = Some(GlobalError::Engine(EngineError::EngineAccessDeny));
                return;
            }
        };
        engine.render(&self.scene);

        match engine.viewport.window.as_ref() {
            Some(window) => {
                window.request_redraw();
            }
            None => {
                self.last_error = Some(GlobalError::Application(ApplicationError::WindowMissing));
            }
        }
    }

    pub fn handle_key_event(&mut self) {
        if let Some(event) = self.input.pop() {
            match event {
                InputEvent::KeyUp(kc) => {
                    match kc {
                        KeyCode::KeyE => {
                            println!("{:?}", self.view);
                            println!("{:?}", self.view.camera);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        } 
    }

    pub fn handle_error(&mut self, event_loop: &ActiveEventLoop) {
        // Check if there is an error
        if let Some(err) = &self.last_error {
            match err {
                GlobalError::Input(err) => {
                    eprintln!("{err}");
                    return;
                }
                _ => {
                    eprintln!("FATAL ERROR");
                    event_loop.exit();
                    return;
                }
            }
        }
    }
}


impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.engine = match Engine::new(event_loop) {
            Ok(e) => Some(e),
            Err(e) => {
                self.last_error = Some(GlobalError::Engine(e.into()));
                None
            }
        };
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                let key_code = match event.physical_key {
                    PhysicalKey::Code(key) => key,
                    PhysicalKey::Unidentified(key) => {
                        self.last_error = Some(GlobalError::Input(InputError::UnknownKey(key)));
                        return;
                    }
                };

                if event.state == ElementState::Pressed {
                    self.input.push(InputEvent::KeyDown(key_code));
                } else {
                    self.input.push(InputEvent::KeyUp(key_code));
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
            }
            WindowEvent::MouseInput { state, button, .. } => {
            }
            WindowEvent::RedrawRequested => {
                self.handle_key_event();
                self.handle_error(event_loop);
                self.update();

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
