use std::collections::VecDeque;
use winit::keyboard::KeyCode;

pub enum InputEvent {
    KeyDown(KeyCode),
    KeyUp(KeyCode),
    MouseMove { x: f32, y: f32 }
}

pub struct InputQueue {
    events: VecDeque<InputEvent>
}

impl InputQueue {
    pub fn new() -> Self {
        InputQueue {
            events: VecDeque::new()
        }
    }

    pub fn push(&mut self, event: InputEvent) {
        self.events.push_back(event);
    }

    pub fn pop(&mut self) -> Option<InputEvent> {
        self.events.pop_front()
    }
}
