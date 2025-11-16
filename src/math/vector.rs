#[derive(Debug, Copy, Clone)]
pub struct Vector2 {
    pub x: isize,
    pub y: isize
}

impl Vector2 {
    pub fn new(x: isize, y: isize) -> Self {
        Vector2 {
            x,
            y
        } 
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vector4 {
    pub r: isize,
    pub g: isize,
    pub b: isize,
    pub a: isize,
}

impl Vector4 {
    pub fn new(r: isize, g: isize, b: isize, a: isize) -> Self {
        Vector4 {
            r, 
            g,
            b,
            a
        } 
    }
}
