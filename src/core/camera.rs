use crate::math::vector::Vector2;

#[derive(Debug, Clone)]
pub struct Camera {
    pub position: Vector2,
    pub fov: f32,
    pub near: f32,
    pub far: f32
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            position: Vector2 { x: 0.0, y: 0.0 },
            fov: 90.0,
            near: 0.1,
            far: 100.0
        }
    }
}
