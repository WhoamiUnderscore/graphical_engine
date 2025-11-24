use crate::core::camera::Camera;

#[derive(Debug, Clone)]
pub struct View {
    pub camera: Camera
}

impl View {
    pub fn new() -> Self {
        View {
            camera: Camera::new(),
        }
    }
}
