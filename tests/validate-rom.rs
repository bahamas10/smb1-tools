use anyhow::Result;

use smb1_tools::*;

const ROM_DATA: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/smb1.nes"));

/// [(RomLevel, num_blocks)]
const LEVEL_INFORMATION: &[(RomLevel, usize)] = &[
    (RomLevel::W1_1, 49),
    (RomLevel::W1_2, 80),
    (RomLevel::W1_3, 41),
    (RomLevel::W1_4, 47),
    (RomLevel::W2_1, 49),
    (RomLevel::W2_2, 60),
    (RomLevel::W2_3, 65),
    (RomLevel::W2_4, 56),
    (RomLevel::W3_1, 57),
    (RomLevel::W3_2, 24),
    (RomLevel::W3_3, 48),
    (RomLevel::W3_4, 53),
    (RomLevel::W4_1, 40),
    (RomLevel::W4_2, 79),
    (RomLevel::W4_3, 50),
    (RomLevel::W4_4, 62),
    (RomLevel::W5_1, 30),
    (RomLevel::W5_2, 56),
    (RomLevel::W5_3, 41),
    (RomLevel::W5_4, 56),
    (RomLevel::W6_1, 56),
    (RomLevel::W6_2, 70),
    (RomLevel::W6_3, 49),
    (RomLevel::W6_4, 47),
    (RomLevel::W7_1, 43),
    (RomLevel::W7_2, 60),
    (RomLevel::W7_3, 65),
    (RomLevel::W7_4, 68),
    (RomLevel::W8_1, 72),
    (RomLevel::W8_2, 59),
    (RomLevel::W8_3, 51),
    (RomLevel::W8_4, 55),
];

#[test]
fn test_rom_valid() -> Result<()> {
    let _ = Rom::new(ROM_DATA.into())?;

    Ok(())
}

#[test]
fn test_level_data_valid() -> Result<()> {
    let rom = Rom::new(ROM_DATA.into())?;

    // loop each level
    for (name, num_blocks) in LEVEL_INFORMATION {
        let level = rom.get_level(name);

        // ensure number of blocks in level data is accurate
        let blocks_len = level.block_data.blocks.len();
        assert_eq!(
            &blocks_len, num_blocks,
            "level {:?} wrong block count: found {} expected {}",
            name, blocks_len, num_blocks
        );
    }

    Ok(())
}
