use anyhow::{ensure, Result};

use crate::*;

mod levels;

pub use levels::*;

const ROM_SIZE_BYTES: usize = 40976;
const ROM_MD5_BYTES: &str = "811b027eaf99c2def7b933c5208636de";

#[derive(Debug)]
pub struct Rom {
    pub rom_data: Vec<u8>,
}

impl Rom {
    pub fn new(rom_data: Vec<u8>) -> Result<Self> {
        Self::validate_rom_data(&rom_data)?;
        let rom = Self { rom_data };
        Ok(rom)
    }

    pub fn get_level(&self, level_name: &RomLevel) -> Level {
        let (header_offset, block_offset, sprite_offset) =
            level_name.get_offsets();

        let header_bytes = &self.rom_data[header_offset..];
        let block_bytes = &self.rom_data[block_offset..];
        let sprite_bytes = &self.rom_data[sprite_offset..];

        let level_header = LevelHeader::from_bytes(header_bytes);
        let block_data = LevelBlockData::from_bytes(block_bytes);
        let sprite_data = LevelSpriteData::from_bytes(sprite_bytes);

        Level { level_header, block_data, sprite_data }
    }

    fn validate_rom_data(data: &[u8]) -> Result<()> {
        // check rom length
        let len = data.len();
        ensure!(
            len == ROM_SIZE_BYTES,
            "rom size invalid: {} != {}",
            len,
            ROM_SIZE_BYTES
        );

        // check md5
        let digest = md5::compute(data);
        ensure!(
            format!("{:x}", digest) == ROM_MD5_BYTES,
            "md5 mismatch for rom file: expected {:?} got {:?}",
            ROM_MD5_BYTES,
            digest
        );

        Ok(())
    }
}
