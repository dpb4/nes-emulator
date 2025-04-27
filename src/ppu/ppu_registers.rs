use bitflags::bitflags;

use crate::make_u16;

#[derive(Debug)]
pub struct Registers {
    pub ctrl: ControlFlags,
    pub mask: MaskFlags,
    pub stat: StatusFlags,
    pub oam_addr: u8,
    pub oam_data: u8,
    pub scrl: PPUScrollRegister,
    pub ppu_addr: PPUAddressRegister,
    pub ppu_data: u8,
    pub oam_dma: u8,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            ctrl: ControlFlags::empty(),
            mask: MaskFlags::empty(),
            stat: StatusFlags::empty(),
            oam_addr: 0,
            oam_data: 0,
            scrl: PPUScrollRegister::new(),
            ppu_addr: PPUAddressRegister::new(),
            ppu_data: 0,
            oam_dma: 0,
        }
    }
}

#[derive(Debug)]
pub struct PPUAddressRegister {
    value: (u8, u8),
    write_to_hi_ptr: bool,
}

impl PPUAddressRegister {
    pub fn new() -> Self {
        PPUAddressRegister {
            value: (0, 0), // high byte first, lo byte second
            write_to_hi_ptr: true,
        }
    }
    fn set_addr(&mut self, data: u16) {
        self.value.0 = (data >> 8) as u8;
        self.value.1 = (data & 0xff) as u8;
    }

    pub fn write(&mut self, data: u8) {
        if self.write_to_hi_ptr {
            self.value.0 = data;
        } else {
            self.value.1 = data;
        }

        if self.get_addr() > 0x3fff {
            self.set_addr(self.get_addr() & 0x3fff);
        }
        self.toggle_latch();
    }

    pub fn increment(&mut self, inc: u8) {
        let lo = self.value.1;
        self.value.1 = self.value.1.wrapping_add(inc);
        if lo > self.value.1 {
            self.value.0 = self.value.0.wrapping_add(1);
        }
        if self.get_addr() > 0x3fff {
            self.set_addr(self.get_addr() & 0x3fff);
        }
    }

    pub fn reset_latch(&mut self) {
        self.write_to_hi_ptr = true;
    }

    pub fn toggle_latch(&mut self) {
        self.write_to_hi_ptr = !self.write_to_hi_ptr;
    }

    pub fn get_addr(&self) -> u16 {
        make_u16!(self.value.0, self.value.1)
    }
}

#[derive(Debug)]
pub struct PPUScrollRegister {
    x_scroll: u8,
    y_scroll: u8,
    write_to_x: bool,
}

impl PPUScrollRegister {
    pub fn new() -> Self {
        Self {
            x_scroll: 0,
            y_scroll: 0,
            write_to_x: true,
        }
    }

    pub fn write(&mut self, val: u8) {
        if self.write_to_x {
            self.x_scroll = val;
        } else {
            self.y_scroll = val;
        }
        self.toggle_latch();
    }

    pub fn reset_latch(&mut self) {
        self.write_to_x = true;
    }

    pub fn toggle_latch(&mut self) {
        self.write_to_x = !self.write_to_x;
    }
}

bitflags! {
    #[derive(Debug)]
    pub struct ControlFlags: u8 {
        const BASE_NAMETABLE_ADDR_LO = 0b00000001;
        const BASE_NAMETABLE_ADDR_HI = 0b00000010;
        const VRAM_ADDR_INCREMENT = 0b00000100;
        const SPRITE_PATTERN_TABLE_ADDR = 0b00001000;
        const BACKGROUND_PATTERN_TABLE_ADDR = 0b00010000;
        const SPRITE_SIZE = 0b00100000;
        const MASTER_SLAVE_SELECT = 0b01000000;
        const VBLANK_NMI_ENABLE = 0b10000000;
    }
}

impl ControlFlags {
    pub fn get_increment_val(&self) -> u8 {
        if self.contains(ControlFlags::VRAM_ADDR_INCREMENT) {
            32
        } else {
            1
        }
    }
}

bitflags! {
    #[derive(Debug)]
    pub struct MaskFlags: u8 {
        const GREYSCALE_ENABLE = 0b00000001;
        const LEFT_EDGE_BACKGROUND = 0b00000010;
        const LEFT_EDGE_SPRITE= 0b00000100;
        const BACKGROUND_ENABLE = 0b00001000;
        const SPRITE_ENABLE = 0b00010000;
        const EMPHASIZE_RED = 0b00100000;
        const EMPHASIZE_GREEN = 0b01000000;
        const EMPHASIZE_BLUE = 0b10000000;
    }
}

bitflags! {
    #[derive(Debug)]
    pub struct StatusFlags: u8 {
        const SPRITE_OVERFLOW = 0b00100000;
        const SPRITE_0_HIT = 0b01000000;
        const VBLANK = 0b10000000;
    }
}
