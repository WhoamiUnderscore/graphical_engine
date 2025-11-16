use winit::{
    event::{KeyEvent, ElementState},
    keyboard::Key
};

use crate::engine::Engine;
use crate::core::application::App;
use crate::math::{
    triangle::Triangle,
    vector::Vector2
};

impl App {
    pub fn handle_input(&mut self, event: KeyEvent) {
        let ( key, pressed ) = {
            let key = event.logical_key;
            let pressed = event.state == ElementState::Pressed;

            ( key, pressed )
        };

        match key {
            Key::Character(c) if c == "e" =>  {
                if pressed == false {
                    return
                }

                Input::handle_e(&mut self.engine, self.viewport.width, self.viewport.height);
            }
            _ => {}
        }
    }
}


struct Input{}

impl Input {
    pub fn handle_e(engine: &mut Option<Engine>, width: u32, height: u32){
        let triangle = Triangle {
            position: [
                Vector2 { x: 400, y: 100 },
                Vector2 { x: 700, y: 300 },
                Vector2 { x: 100,   y: 300 },
            ],
            color: None
        };

        let binding = engine.as_mut().unwrap();
        let frame = binding.pixels.frame_mut();
        triangle.draw(frame, width, height);
    }
}
