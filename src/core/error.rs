use thiserror::Error;

use winit::keyboard::NativeKeyCode;

#[derive(Error, Debug)]
pub enum GlobalError {
    #[error("APPLICATION ERROR: {0}")]
    Application(#[from] ApplicationError),

    #[error("ENGINE ERROR: {0}")]
    Engine(#[from] EngineError),

    #[error("VIEWPORT ERROR: {0}")]
    Viewport(#[from] ViewportError),

    #[error("INPUT ERROR: {0}")]
    Input(#[from] InputError),

    #[error("Unknown error")]
    Unknown,
}

pub type ApplicationResult<T> = Result<T, ApplicationError>;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Failed to initalize the event_loop: {0}")]
    EventLoop(String),

    #[error("No window initialize")]
    WindowMissing,

    #[error("Unknown error")]
    Unknown
}

pub type EngineResult<T> = Result<T, EngineError>;

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("Try accessing the window before it initializes")]
    WindowAccessDeny,

    #[error("Try accessing the engine before it initializes")]
    EngineAccessDeny,

    #[error("Error durring pixels rendering: {0}")]
    PixelRenderingError(String),

    #[error("Unknown error")]
    Unknown
}

pub type ViewportResult<T> = Result<T, ViewportError>;

#[derive(Error, Debug)]
pub enum ViewportError {
    #[error("Failed to initialize window: {0}")]
    WindowError(String),

    #[error("Unknown error")]
    Unknown
}

pub type InputResult<T> = Result<T, InputError>;

#[derive(Error, Debug)]
pub enum InputError {
    #[error("Key unknown: {0:?}")]
    UnknownKey(NativeKeyCode),

    #[error("Unknown error")]
    Unknown
}
