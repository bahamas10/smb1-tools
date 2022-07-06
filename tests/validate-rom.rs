use anyhow::Result;

use smb1_tools::*;

const ROM_DATA: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/smb1.nes"));

/// [(RomLevel, num_objects, num_enemies, num_pipe_pointers)]
const LEVEL_INFORMATION: &[(RomLevel, usize, usize, usize)] = &[
    (RomLevel::W1_1, 49, 13, 1),
    (RomLevel::W1_2, 80, 19, 2),
    (RomLevel::W1_3, 41, 14, 0),
    (RomLevel::W1_4, 47, 19, 0),
    (RomLevel::W2_1, 49, 18, 2),
    (RomLevel::W2_2, 60, 16, 3),
    (RomLevel::W2_3, 65, 10, 0),
    (RomLevel::W2_4, 56, 23, 0),
    (RomLevel::W3_1, 57, 21, 2),
    (RomLevel::W3_2, 24, 18, 0),
    (RomLevel::W3_3, 48, 18, 0),
    (RomLevel::W3_4, 53, 21, 0),
    (RomLevel::W4_1, 40, 5, 1),
    (RomLevel::W4_2, 79, 18, 3),
    (RomLevel::W4_3, 50, 18, 0),
    (RomLevel::W4_4, 62, 12, 0),
    (RomLevel::W5_1, 30, 16, 1),
    (RomLevel::W5_2, 56, 18, 2),
    (RomLevel::W5_3, 41, 14, 0),
    (RomLevel::W5_4, 56, 23, 0),
    (RomLevel::W6_1, 56, 4, 0),
    (RomLevel::W6_2, 70, 13, 4),
    (RomLevel::W6_3, 49, 17, 0),
    (RomLevel::W6_4, 47, 19, 0),
    (RomLevel::W7_1, 43, 12, 1),
    (RomLevel::W7_2, 60, 16, 3),
    (RomLevel::W7_3, 65, 10, 0),
    (RomLevel::W7_4, 68, 10, 0),
    (RomLevel::W8_1, 72, 27, 1),
    (RomLevel::W8_2, 59, 21, 1),
    (RomLevel::W8_3, 51, 14, 0),
    (RomLevel::W8_4, 55, 18, 7),
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
    for (name, num_objects, num_enemies, num_pipe_pointers) in LEVEL_INFORMATION
    {
        let level = rom.get_level(name);

        // ensure number of objects in level data is accurate
        let objects_len = level.object_data.objects.len();
        assert_eq!(
            &objects_len, num_objects,
            "level {:?} wrong object count: found {} expected {}",
            name, objects_len, num_objects
        );

        // ensure number of enemies in level data is accurate
        let enemies_len = level.enemy_data.enemies.len();
        assert_eq!(
            &enemies_len, num_enemies,
            "level {:?} wrong enemies count: found {} expected {}",
            name, enemies_len, num_enemies
        );

        // ensure number of pipe pointers in level data is accurate
        let pipe_pointers_len = level.enemy_data.pipe_pointers.len();
        assert_eq!(
            &pipe_pointers_len, num_pipe_pointers,
            "level {:?} wrong pipe pointers count: found {} expected {}",
            name, pipe_pointers_len, num_pipe_pointers
        );
    }

    Ok(())
}
