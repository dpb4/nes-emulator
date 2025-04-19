use crate::ppu::PPU;

use super::cartridge_rom::CartridgeROM;

pub const RAM_START: u16 = 0x0000;
pub const RAM_END_MIRRORED: u16 = 0x1fff;
pub const RAM_ADDR_MASK: u16 = 0b0000_0111_1111_1111;

pub const PPU_REG_START: u16 = 0x2000;
pub const PPU_REG_END_MIRRORED: u16 = 0x3fff;
pub const PPU_REG_ADDR_MASK: u16 = 0b0010_0000_0000_0111;

pub const PRG_ROM_START: u16 = 0x8000;
pub const PRG_ROM_END_MIRRORED: u16 = 0xffff;

#[derive(Debug)]
pub struct MemoryBus {
    cpu_vram: [u8; 2048],
    prg_rom: Vec<u8>,
    ppu: PPU,
}

impl MemoryBus {
    pub fn new(rom: CartridgeROM) -> Self {
        MemoryBus {
            cpu_vram: [0; 2048],
            prg_rom: rom.prg_rom,
            ppu: PPU::new(rom.chr_rom, rom.screen_mirroring),
        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        match addr {
            RAM_START..=RAM_END_MIRRORED => {
                let truncated_addr = addr & RAM_ADDR_MASK;
                self.cpu_vram[truncated_addr as usize]
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
                if self.prg_rom.len() == 0x4000 && prg_addr >= 0x4000 {
                    prg_addr %= 0x4000;
                }
                self.prg_rom[prg_addr as usize]
            }
            _ => {
                panic!("bad memory read at 0x{:x}", addr);
            }
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            RAM_START..=RAM_END_MIRRORED => {
                let truncated_addr = addr & RAM_ADDR_MASK;
                self.cpu_vram[truncated_addr as usize] = val;
            }
            PPU_REG_START..=PPU_REG_END_MIRRORED => match addr {
                0x2000 => self.ppu.write_to_ctrl(val),
                0x2001 => self.ppu.write_to_mask(val),
                0x2002 => panic!("attempting to write to PPU status (read only)"),
                0x2003 => self.ppu.write_to_oam_addr(val),
                0x2004 => self.ppu.write_to_oam_data(val),
                0x2005 => self.ppu.write_to_scrl(val),
                0x2006 => self.ppu.write_to_ppu_addr(val),
                0x2007 => self.ppu.write_to_data(val),
                _ => self.write(addr & PPU_REG_ADDR_MASK, val),
            },
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
}
