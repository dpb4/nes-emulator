use crate::ppu::Mirroring;

pub const NES_TAG: [u8; 4] = [0x4e, 0x45, 0x53, 0x1a]; // string "NES<CTRL-Z>" in ascii
pub const PRG_ROM_PAGE_SIZE: usize = 0x4000;
pub const CHR_ROM_PAGE_SIZE: usize = 0x2000;

#[derive(Debug)]
pub struct CartridgeROM {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub mapper: u8,
    pub screen_mirroring: Mirroring,
}

impl CartridgeROM {
    pub fn new(raw_bytes: Vec<u8>) -> Result<CartridgeROM, String> {
        if &raw_bytes[0..4] != NES_TAG {
            return Err("file format is not iNES 1.0 (missing NES tag)".to_string());
        }

        let ines_ver = (raw_bytes[7] >> 2) & 0b11;
        if ines_ver != 0 {
            return Err("iNES 2.0 format is not supported".to_string());
        }

        let prg_rom_size = raw_bytes[4] as usize * PRG_ROM_PAGE_SIZE;
        let chr_rom_size = raw_bytes[5] as usize * CHR_ROM_PAGE_SIZE;

        let trainer_offset = if raw_bytes[6] & 0b100 != 0 { 512 } else { 0 };

        let prg_rom_start = 16 + trainer_offset;
        let chr_rom_start = prg_rom_start + prg_rom_size;

        let four_screen = raw_bytes[6] & 0b1000 != 0;
        let vertical_mirroring = raw_bytes[6] & 0b1 != 0;
        let screen_mirroring = match (four_screen, vertical_mirroring) {
            (true, _) => Mirroring::FourScreen,
            (false, true) => Mirroring::Vertical,
            (false, false) => Mirroring::Horizontal,
        };

        let mapper = (raw_bytes[7] & 0b1111_0000) | (raw_bytes[6] >> 4);

        Ok(CartridgeROM {
            prg_rom: raw_bytes[prg_rom_start..(prg_rom_start + prg_rom_size)].to_vec(),
            chr_rom: raw_bytes[chr_rom_start..(chr_rom_start + chr_rom_size)].to_vec(),
            mapper: mapper,
            screen_mirroring: screen_mirroring,
        })
    }

    pub fn dummy() -> Self {
        println!("WARNING: using dummy rom (no program can be loaded)");
        Self {
            prg_rom: vec![],
            chr_rom: vec![],
            mapper: 0,
            screen_mirroring: Mirroring::FourScreen,
        }
    }
}
