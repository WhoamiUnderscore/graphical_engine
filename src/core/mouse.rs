use winit::event::ElementState;

use crate::engine::Engine;
use crate::math::vector::Vector2;
use crate::math::triangle::Triangle;

pub struct Mouse {
    pub position: Vector2
}

impl Mouse {
    pub fn new() -> Self {
        Mouse {
            position: Vector2 { x: 0, y: 0 }
        }
    }

    pub fn update_position(&mut self, x: f64, y: f64) {
        self.position.x = x as isize;
        self.position.y = y as isize;
    }

    pub fn left_click(&self, state: ElementState, engine: &mut Option<Engine>, width: u32, height: u32) {
        if state.is_pressed() {
            let triangle = Triangle {
                position: [
                    Vector2 { x: self.position.x, y: self.position.y - 10 },
                    Vector2 { x: self.position.x + 10, y: self.position.y + 10 },
                    Vector2 { x: self.position.x - 10,   y: self.position.y + 10 },
                ],
                color: None
            };

            let mut binding = engine.as_mut().unwrap();
            let frame = binding.pixels.frame_mut();
            triangle.draw(frame, width, height);
        }
    }
}

