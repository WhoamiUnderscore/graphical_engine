use std::collections::{
    VecDeque,
    vec_deque::Iter
};

use crate::math::triangle::Triangle;

pub struct Scene {
    triangles: VecDeque<Triangle>
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            triangles: VecDeque::new()
        }
    }

    pub fn push(&mut self, triangle: Triangle) {
        self.triangles.push_back(triangle);
    }

    pub fn pop(&mut self) -> Option<Triangle>{
        self.triangles.pop_front()
    }

    pub fn triangle_iter(&self) -> Iter<Triangle> {
        self.triangles.iter()
    }

}
