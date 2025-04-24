const WIDTH: usize = 256;
const HEIGHT: usize = 240;

#[derive(Debug, Clone, Copy)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl From<(u8, u8, u8)> for Pixel {
    fn from(value: (u8, u8, u8)) -> Self {
        Self {
            r: value.0,
            g: value.1,
            b: value.2,
        }
    }
}

impl From<&[u8; 3]> for Pixel {
    fn from(value: &[u8; 3]) -> Self {
        Self {
            r: value[0],
            g: value[1],
            b: value[2],
        }
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }
}
pub struct Frame {
    data: [Pixel; WIDTH * HEIGHT],
}

impl Default for Frame {
    fn default() -> Self {
        Self {
            data: [Pixel::default(); WIDTH * HEIGHT],
        }
    }
}
impl Frame {}
