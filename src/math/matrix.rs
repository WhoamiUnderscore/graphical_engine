use std::ops::Add;
use std::ops::Mul;

use crate::math::vector::Vector3;

#[derive(Debug, Clone)]
pub struct Matrix3 {
    m: [[f32; 3]; 3]
}

impl Matrix3
{
    pub fn identity() -> Self {
        Matrix3 {
            m: [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0],
            ]
        }
    }

    pub fn mul_vector(&self, vec: Vector3) -> Vector3
    {
        Vector3 {
            x: self.m[0][0]*vec.x + self.m[0][1]*vec.y + self.m[0][2]*vec.z,
            y: self.m[1][0]*vec.x + self.m[1][1]*vec.y + self.m[1][2]*vec.z,
            z: self.m[2][0]*vec.x + self.m[2][1]*vec.y + self.m[2][2]*vec.z,
        }
    }
}
