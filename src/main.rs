use winit::event_loop::{ControlFlow, EventLoop};

mod core;
mod math;

use crate::core::application::App;
use crate::core::error::{ApplicationResult, ApplicationError};

fn main() -> ApplicationResult<()> {
    let event_loop = match EventLoop::new() {
        Ok(el) => {
            let event_loop = el;
            event_loop.set_control_flow(ControlFlow::Poll);

            event_loop
        },

        Err(err) => {
            let error = ApplicationError::EventLoop(err.to_string());
            eprintln!("{error:?}");
            return Err(error);
        }
    };

    let mut app = App::new();
    let _ = event_loop.run_app(&mut app);

    return Ok(());
}
