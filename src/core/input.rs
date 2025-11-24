use std::collections::VecDeque;
use winit::keyboard::KeyCode;
use winit::event::MouseButton;

use crate::math::vector::Vector2;

pub enum InputEvent {
    KeyDown(KeyCode),
    KeyUp(KeyCode),
    MouseMove { x: f32, y: f32 },
    MouseDown(MouseButton),
    MouseUp(MouseButton)
}

pub struct Input {
    events: VecDeque<InputEvent>,
    pub mouse: Vector2
}

impl Input {
    pub fn new() -> Self {
        Input {
            events: VecDeque::new(),
            mouse: Vector2::new()
        }
    }

    pub fn push(&mut self, event: InputEvent) {
        self.events.push_back(event);
    }

    pub fn pop(&mut self) -> Option<InputEvent> {
        self.events.pop_front()
    }
}
