use crate::math::matrix::Matrix3;
use crate::math::vector::Vector3;

#[derive(Debug, Clone)]
pub struct Camera {
    pub position: Vector3,
    pub rotation: Matrix3
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            position: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            rotation: Matrix3::identity()
        }
    }
}
