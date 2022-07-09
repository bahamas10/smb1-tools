use crate::util::enum_mapped;

#[derive(Debug)]
pub struct LevelHeader {
    //pub header_data: &'a [u8],
    pub time: LevelTime,
    pub start_position: LevelStartPosition,
    pub start_autowalk: bool,
    pub background: LevelBackground,

    pub scenery: LevelScenery,
    pub platform: LevelPlatform,
    pub ground: LevelGround,
}

impl LevelHeader {
    /**
     *  T T A Y Y B B B   S S P P G G G G
     *  =================================|===================|
     *  |_| | |_| |_|_|   |_| |_| |_|_|_|| ground/block      |
     *   |  |  |    |      |   |     |___| type              |
     *   |  |  |    |      |   |         |===================|
     *   |  |  |    |      |   |_________| scenery           |
     *   |  |  |    |      |             | type              |
     *   |  |  |    |      |             |===================|
     *   |  |  |    |      |_____________| platform          |
     *   |  |  |    |                    |                   |
     *   |  |  |    |                    |===================|
     *   |  |  |    |____________________| background/season |
     *   |  |  |                         | type              |
     *   |  |  |                         |===================|
     *   |  |  |_________________________| starting          |
     *   |  |                            | position          |
     *   |  |                            |===================|
     *   |  |____________________________| auto walk         |
     *   |                               | on/off            |
     *   |                               |===================|
     *   |_______________________________| time              |
     *                                   |                   |
     *  =====================================================|
     */
    pub fn from_bytes(bytes: &[u8]) -> Self {
        assert!(bytes.len() >= 2);

        // first byte
        let time = Self::parse_level_time(bytes);
        let start_position = Self::parse_level_start_position(bytes);
        let start_autowalk = Self::parse_level_start_autowalk(bytes);
        let background = Self::parse_level_background(bytes);

        // second byte
        let scenery = Self::parse_level_scenery(bytes);
        let platform = Self::parse_level_platform(bytes);
        let ground = Self::parse_level_ground(bytes);

        Self {
            time,
            start_position,
            start_autowalk,
            background,
            scenery,
            platform,
            ground,
        }
    }

    /// TTxxxxxx xxxxxxxx
    fn parse_level_time(bytes: &[u8]) -> LevelTime {
        let bits = (bytes[0] & 0b11000000) >> 6;
        LevelTime::new(bits)
    }

    /// xxAxxxxx xxxxxxxx
    fn parse_level_start_autowalk(bytes: &[u8]) -> bool {
        let bit = (bytes[0] & 0b00100000) >> 5;
        bit == 1
    }

    /// xxxYYxxx xxxxxxxx
    fn parse_level_start_position(bytes: &[u8]) -> LevelStartPosition {
        let bits = (bytes[0] & 0b00011000) >> 3;
        LevelStartPosition::new(bits)
    }

    /// xxxxxBBB xxxxxxxx
    fn parse_level_background(bytes: &[u8]) -> LevelBackground {
        let bits = bytes[0] & 0b00000111;
        LevelBackground::new(bits)
    }

    /// xxxxxxxx PPxxxxxx
    fn parse_level_platform(bytes: &[u8]) -> LevelPlatform {
        let bits = (bytes[1] & 0b11000000) >> 6;
        LevelPlatform::new(bits)
    }

    /// xxxxxxxx xxSSxxxx
    fn parse_level_scenery(bytes: &[u8]) -> LevelScenery {
        let bits = (bytes[1] & 0b00110000) >> 4;
        LevelScenery::new(bits)
    }

    /// xxxxxxxx xxxxGGGG
    fn parse_level_ground(bytes: &[u8]) -> LevelGround {
        let bits = bytes[1] & 0b00001111;
        LevelGround::new(bits)
    }
}

enum_mapped!(
    LevelStartPosition (u8) {
        0b00 => FallFromSky,
        0b01 => StartOnGround,
        0b10 => FallFromSkyAlternate,
        0b11 => HalfwayOffGround,
    }
);

enum_mapped!(
    LevelTime (u8) {
        0b00 => NotSet,
        0b01 => T400,
        0b10 => T300,
        0b11 => T200,
    }
);

enum_mapped!(
    LevelBackground (u8) {
        0b000 => DayTime,
        0b001 => Underwater,
        0b010 => CastleWall,
        0b011 => Overwater,
        0b100 => NightTime,
        0b101 => DayTimeSnow,
        0b110 => NightTimeSnow,
        0b111 => BlackAndWhite,
    }
);

enum_mapped!(
    LevelScenery (u8) {
        0b00 => Nothing,
        0b01 => Clouds,
        0b10 => Mountains,
        0b11 => Fence,
    }
);

enum_mapped!(
    LevelPlatform (u8) {
        0b00 => GreenAndTrees,
        0b01 => OrangeAndMushrooms,
        0b10 => BulletBills,
        0b11 => Clouds,
    }
);

enum_mapped!(
    LevelGround (u8) {
        0b0000 => Nothing,
        0b0001 => BasicFloor,
        0b0010 => BasicFloorAndCeiling,
        0b0011 => BasicFloorAndThreeLayerCeiling,
        0b0100 => BasicFloorAndFourLayerCeiling,
        0b0101 => BasicFloorAndEightLayerCeiling,
        0b0110 => FiveLayerFloorAndCeiling,
        0b0111 => FiveLayerFloorAndThreeLayerCeiling,
        0b1000 => FiveLayerFloorAndFourLayerCeiling,
        0b1001 => SixLayerFloorAndCeiling,
        0b1010 => Ceiling,
        0b1011 => SixLayerFloorAndFourLayerCeiling,
        0b1100 => NineLayerFloorAndCeiling,
        0b1101 => BasicFloorThreeLayerGapFiveLayerBricksTwoLayerGapAndCeiling,
        0b1110 => BasicFloorThreeLayerGapFourLayerBricksThreeLayerGapAndCeiling,
        0b1111 => All,
    }
);
