use crate::engine::Engine;

impl Engine {
    fn clean(&mut self) {
        let frame = self.pixels.frame_mut();
        for pixel in frame.chunks_exact_mut(4) {
            pixel[0] = 0x00; // R
            pixel[1] = 0x00; // G
            pixel[2] = 0x00; // B
            pixel[3] = 0xff; // A
        }
    }

    pub fn render(&mut self) {
        let _ = self.pixels.render();
    }
}
