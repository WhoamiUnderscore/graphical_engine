use crate::math::vector::{Vector2, Vector4};

#[derive(Debug, Clone)]
pub struct Triangle {
    pub position: [Vector2; 3],
    pub color: Option<Vector4>
}

impl Triangle {
    pub fn draw(&self, frame: &mut [u8], width: u32, height: u32){
        self.draw_lines(self.position[0], self.position[1], frame, width, height);
        self.draw_lines(self.position[2], self.position[1], frame, width, height);
        self.draw_lines(self.position[2], self.position[0], frame, width, height);
    }

    fn draw_lines(&self, p1: Vector2, p2: Vector2, frame:  &mut [u8], width: u32, height: u32) {
        let width = width as isize;
        let height = height as isize;

        let (mut x0, mut y0) = (p1.x, p1.y);
        let (x1, y1) = (p2.x, p2.y);

        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();

        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };

        let mut e = dx + dy;

        loop {
            if x0 >= 0 && x0 < width && y0 >= 0 && y0 < height {
                // Draw pixel
                let index = (( y0 * width + x0 ) * 4) as usize;
                frame[index] = 0xff; // R
                frame[index + 1] = 0x00; // G
                frame[index + 2] = 0x00; // B
                frame[index + 3] = 0xff; // A
            }

            if x0 == x1 && y0 == y1 {
                break
            }

            let e2 = 2 * e;

            if e2 >= dy {
                e += dy;
                x0 += sx;  
            }

            if e2 <= dx {
                e += dx;
                y0 += sy;
            }
        }
    }
}
