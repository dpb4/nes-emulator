use crate::{HEIGHT, WIDTH};

use super::{NESSystemPalette, Renderer};

pub struct PixelsRenderer {
    pub palette_buffer: [u8; WIDTH * HEIGHT],
    palette: NESSystemPalette,
}

impl PixelsRenderer {
    pub fn new(palette: NESSystemPalette) -> Self {
        Self {
            palette_buffer: [0; WIDTH * HEIGHT],
            palette,
        }
    }
}

impl Renderer for PixelsRenderer {
    fn draw_to(&self, buf: &mut [u8]) {
        for (index, pixel) in buf.chunks_exact_mut(4).enumerate() {
            let col = self.palette[self.palette_buffer[index] as usize];
            pixel[0] = col.0;
            pixel[1] = col.1;
            pixel[2] = col.2;
            pixel[3] = 255; // alpha
        }
    }

    fn modify_buffer<T: FnOnce(&mut [u8; WIDTH * HEIGHT])>(&mut self, f: T) {
        f(&mut self.palette_buffer);
    }
}
