use macroquad::{
    color::WHITE,
    math::Vec2,
    texture::{draw_texture_ex, DrawTextureParams, FilterMode, Texture2D},
};

use super::{Renderer, SysPal};

pub struct MacroSystem {
    pub palette: &'static SysPal,
    internal_texture: Texture2D,
}

impl MacroSystem {
    pub fn new(palette: &'static SysPal) -> Self {
        let tex = Texture2D::empty();
        tex.set_filter(FilterMode::Nearest);
        Self {
            palette,
            internal_texture: tex,
        }
    }
    pub fn draw_frame(&mut self, frame_buffer: [u8; 256 * 240], palette: &SysPal) {
        let mut pixels = Vec::with_capacity(256 * 240);

        for y in 0..240 {
            for x in 0..256 {
                let col = palette[frame_buffer[x + (y * 240)] as usize];
                pixels.push(col.0);
                pixels.push(col.1);
                pixels.push(col.2);
                pixels.push(255);
            }
        }

        self.internal_texture = Texture2D::from_rgba8(256, 240, &pixels);
        draw_texture_ex(
            &self.internal_texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(256. * 2., 240. * 2.)),
                ..Default::default()
            },
        );
    }
}

// impl Renderer for MacroSystem {
// }
