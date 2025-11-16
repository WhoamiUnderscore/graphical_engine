use winit::event_loop::{ControlFlow, EventLoop};

mod engine;
mod core;
mod math;
mod utils;

use crate::core::application::App;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new();

    let _ = event_loop.run_app(&mut app);
}
