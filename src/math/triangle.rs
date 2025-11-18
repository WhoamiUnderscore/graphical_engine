use crate::math::vector::Vector3;

#[derive(Debug)]
pub struct Triangle {
    pub position: [Vector3; 3],
}

// impl Triangle {
//     pub fn draw(&self, frame: &mut [u8], width: u32, height: u32, wireframe: bool){
//         if wireframe {
//             self.draw_lines(self.position[0], self.position[1], frame, width, height);
//             self.draw_lines(self.position[2], self.position[1], frame, width, height);
//             self.draw_lines(self.position[2], self.position[0], frame, width, height);
//         } else {
//             self.draw_triangle_fill(frame, width, height);
//         }
//     }
//
//     fn edgeFunction(&self, a: &Vector2, b: &Vector2, c: &Vector2) -> isize {
//         return (((b.x - a.x) * (c.y - a.y)) - ((b.y - a.y) * (c.x - a.x))) as isize
//     }
//
//     fn draw_triangle_fill(&self, frame: &mut [u8], width: u32, height: u32) {
//         let a = &self.position[0];
//         let b = &self.position[1];
//         let c = &self.position[2];
//
//         let min_x = a.x.min(b.x).min(c.x);
//         let min_y = a.y.min(b.y).min(c.y);
//         let max_x = a.x.max(b.x).max(c.x);
//         let max_y = a.y.max(b.y).max(c.y);
//
//         for y in min_y..max_y {
//             for x in min_x..max_x{
//                 let p = Vector2 {
//                     x,
//                     y
//                 };
//
//                 let ABP = self.edgeFunction(a, b, &p);
//                 let BCP = self.edgeFunction(b, c, &p);
//                 let CAP = self.edgeFunction(c, a, &p);
//
//                 if ABP >= 0 && BCP >= 0 && CAP >= 0 {
//                     let vec = Vector2 { x, y }; 
//                     self.draw_pixel_from_vec2(vec, frame, width);
//                 }
//             }
//         }
//     }
//
//     fn draw_lines(&self, p1: Vector2, p2: Vector2, frame:  &mut [u8], width: u32, height: u32) {
//         let (mut x0, mut y0) = (p1.x, p1.y);
//         let (x1, y1) = (p2.x, p2.y);
//
//         let dx = (x1 - x0).abs();
//         let dy = -(y1 - y0).abs();
//
//         let sx = if x0 < x1 { 1 } else { -1 };
//         let sy = if y0 < y1 { 1 } else { -1 };
//
//         let mut e = dx + dy;
//
//         loop {
//             if x0 >= 0 && x0 < width && y0 >= 0 && y0 < height {
//                 let vec = Vector2 { x: x0, y: y0 };
//                 self.draw_pixel_from_vec2(vec, frame, width)
//             }
//
//             if x0 == x1 && y0 == y1 {
//                 break
//             }
//
//             let e2 = 2 * e;
//
//             if e2 >= dy {
//                 e += dy;
//                 x0 += sx;  
//             }
//
//             if e2 <= dx {
//                 e += dx;
//                 y0 += sy;
//             }
//         }
//     }
//
//     fn draw_pixel_from_vec2(&self, vec: Vector2, frame: &mut [u8], width: u32) {
//         let index = (( vec.y as usize * width as usize + vec.x as usize ) * 4);
//         // Draw pixel
//         frame[index] = 0xff; // R
//         frame[index + 1] = 0x00; // G
//         frame[index + 2] = 0x00; // B
//         frame[index + 3] = 0xff; // A
//     }
// }
