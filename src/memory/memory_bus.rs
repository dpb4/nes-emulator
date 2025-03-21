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
    rom: CartridgeROM,
}

impl MemoryBus {
    pub fn new(rom: CartridgeROM) -> Self {
        MemoryBus {
            cpu_vram: [0; 2048],
            rom,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            RAM_START..=RAM_END_MIRRORED => {
                let truncated_addr = addr & RAM_ADDR_MASK;
                self.cpu_vram[truncated_addr as usize]
            }
            PPU_REG_START..=PPU_REG_END_MIRRORED => {
                let _truncated_addr = addr & PPU_REG_ADDR_MASK;
                println!("{addr}");
                todo!("ppu read not implemented")
            }
            PRG_ROM_START..=PRG_ROM_END_MIRRORED => {
                let mut prg_addr = addr - PRG_ROM_START;
                if self.rom.prg_rom.len() == 0x4000 && prg_addr >= 0x4000 {
                    prg_addr %= 0x4000;
                }
                self.rom.prg_rom[prg_addr as usize]
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
            PPU_REG_START..=PPU_REG_END_MIRRORED => {
                let _truncated_addr = addr & PPU_REG_ADDR_MASK;
                println!("{addr}");
                todo!("ppu write not implemented")
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
}
