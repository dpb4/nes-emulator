use crate::{make16, ppu::PPU};

pub const RAM_START: u16 = 0x0000;
pub const RAM_END_MIRRORED: u16 = 0x1fff;
pub const RAM_ADDR_MASK: u16 = 0b0000_0111_1111_1111;

pub const PPU_REG_START: u16 = 0x2000;
pub const PPU_REG_END_MIRRORED: u16 = 0x3fff;
pub const PPU_REG_ADDR_MASK: u16 = 0b0010_0000_0000_0111;

pub const PRG_ROM_START: u16 = 0x8000;
pub const PRG_ROM_END_MIRRORED: u16 = 0xffff;

#[derive(Debug)]
// pub struct MemoryBus {
//     cpu_ram: [u8; 2048],
//     prg_rom: Vec<u8>,
//     ppu: PPU,
// }

pub struct MemoryBus {
    cpu_ram: [u8; 0x800],
    ppu: PPU,
}

#[derive(Debug, Clone, Copy)]
pub enum InterruptType {
    NonMaskable, // NMI
    Request,     // IRQ
}

impl MemoryBus {
    pub fn new(ppu: PPU) -> Self {
        MemoryBus {
            cpu_ram: [0; 2048],
            ppu,
        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        match addr {
            RAM_START..=RAM_END_MIRRORED => {
                let truncated_addr = addr & RAM_ADDR_MASK;
                self.cpu_ram[truncated_addr as usize]
            }
            PPU_REG_START..=PPU_REG_END_MIRRORED => match addr {
                0x2000 | 0x2001 | 0x2003 | 0x2005 | 0x2006 | 0x4014 => {
                    panic!("cannot read from write only PPU address 0x{:x}", addr);
                }
                0x2002 => self.ppu.read_status(),
                0x2004 => self.ppu.read_oam_data(),
                0x2007 => self.ppu.read_data(),

                _ => self.read(addr & PPU_REG_ADDR_MASK),
            },
            PRG_ROM_START..=PRG_ROM_END_MIRRORED => {
                let mut prg_addr = addr - PRG_ROM_START;
                if self.ppu.cart.prg_rom.len() == 0x4000 && prg_addr >= 0x4000 {
                    prg_addr %= 0x4000;
                }
                self.ppu.cart.prg_rom[prg_addr as usize]
            }
            _ => {
                panic!("bad memory read at 0x{:x}", addr);
            }
        }
    }

    pub fn read_16bit(&mut self, addr: u16) -> u16 {
        make16!(self.read(addr + 1), self.read(addr))
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            RAM_START..=RAM_END_MIRRORED => {
                let truncated_addr = addr & RAM_ADDR_MASK;
                self.cpu_ram[truncated_addr as usize] = val;
            }
            PPU_REG_START..=PPU_REG_END_MIRRORED => match addr & PPU_REG_ADDR_MASK {
                0x2000 => self.ppu.write_to_ctrl(val),
                0x2001 => self.ppu.write_to_mask(val),
                0x2002 => panic!("attempting to write to PPU status (read only)"),
                0x2003 => self.ppu.write_to_oam_addr(val),
                0x2004 => self.ppu.write_to_oam_data(val),
                0x2005 => self.ppu.write_to_scrl(val),
                0x2006 => self.ppu.write_to_ppu_addr(val),
                0x2007 => self.ppu.write_to_data(val),
                _ => unreachable!(), // _ => self.write(addr & PPU_REG_ADDR_MASK, val),
            },
            0x4014 => {
                let mut buffer = [0; 256];
                let hi = (val as u16) << 8;
                for i in 0..256 {
                    buffer[i as usize] = self.read(hi + i);
                }

                self.ppu.write_oam_dma(&buffer);
            }
            PRG_ROM_START..=PRG_ROM_END_MIRRORED => {
                panic!(
                    "attempting to write to rom space at 0x{:x} (val {val})",
                    addr
                );
            }
            _ => {
                panic!("bad memory write at 0x{:x} (val {val})", addr);
            }
        }
    }

    pub fn get_frame_pixel_buffer(&self) -> [u8; 256 * 240] {
        self.ppu.get_frame_pixel_buffer()
    }

    pub fn tick_ppu(&mut self, cycles: usize) {
        self.ppu.tick(cycles * 3); // ppu clock cycles are 3x faster than cpu
    }

    pub fn poll_interrupt(&self) -> Option<InterruptType> {
        self.ppu.interrupt
    }

    pub fn get_ppu_cycles(&self) -> usize {
        self.ppu.cycle_count
    }

    pub fn get_ppu_scanline(&self) -> u16 {
        self.ppu.scanline
    }
}
