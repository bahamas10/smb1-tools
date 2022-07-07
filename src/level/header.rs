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
        let t = (bytes[0] & 0b11000000) >> 6;
        match t {
            0b00 => LevelTime::NotSet,
            0b01 => LevelTime::T400,
            0b10 => LevelTime::T300,
            0b11 => LevelTime::T200,
            _ => unreachable!("invalid level time byte: {}", bytes[0]),
        }
    }

    /// xxAxxxxx xxxxxxxx
    fn parse_level_start_autowalk(bytes: &[u8]) -> bool {
        let t = (bytes[0] & 0b00100000) >> 5;
        match t {
            0b0 => false,
            0b1 => true,
            _ => {
                unreachable!("invalid level start autowalk byte: {}", bytes[0])
            }
        }
    }

    /// xxxYYxxx xxxxxxxx
    fn parse_level_start_position(bytes: &[u8]) -> LevelStartPosition {
        let t = (bytes[0] & 0b00011000) >> 3;
        match t {
            //XXX this probably isn't right
            0b00 => LevelStartPosition::FallFromSky,
            0b01 => LevelStartPosition::StartOnGround,
            0b10 => LevelStartPosition::FallFromSky,
            0b11 => LevelStartPosition::HalfwayOffGround,
            _ => {
                unreachable!("invalid level start position byte: {}", bytes[0])
            }
        }
    }

    /// xxxxxBBB xxxxxxxx
    fn parse_level_background(bytes: &[u8]) -> LevelBackground {
        let t = bytes[0] & 0b00000111;
        match t {
            0b000 => LevelBackground::DayTime,
            0b001 => LevelBackground::Underwater,
            0b010 => LevelBackground::CastleWall,
            0b011 => LevelBackground::Overwater,
            0b100 => LevelBackground::NightTime,
            0b101 => LevelBackground::DayTimeSnow,
            0b110 => LevelBackground::NightTimeSnow,
            0b111 => LevelBackground::BlackAndWhite,
            _ => unreachable!("invalid level background byte: {}", bytes[0]),
        }
    }

    /// xxxxxxxx PPxxxxxx
    fn parse_level_platform(bytes: &[u8]) -> LevelPlatform {
        let t = (bytes[1] & 0b11000000) >> 6;
        match t {
            0b00 => LevelPlatform::GreenAndTrees,
            0b01 => LevelPlatform::OrangeAndMushrooms,
            0b10 => LevelPlatform::BulletBills,
            0b11 => LevelPlatform::Clouds,
            _ => unreachable!("invalid level platform byte: {}", bytes[1]),
        }
    }

    /// xxxxxxxx xxSSxxxx
    fn parse_level_scenery(bytes: &[u8]) -> LevelScenery {
        let t = (bytes[1] & 0b00110000) >> 4;
        match t {
            0b00 => LevelScenery::Nothing,
            0b01 => LevelScenery::Clouds,
            0b10 => LevelScenery::Mountains,
            0b11 => LevelScenery::Fence,
            _ => unreachable!("invalid level scenery byte: {}", bytes[1]),
        }
    }

    /// xxxxxxxx xxxxGGGG
    fn parse_level_ground(bytes: &[u8]) -> LevelGround {
        let t = bytes[1] & 0b00001111;
        match t {
            0b0000 => LevelGround::Nothing,
            0b0001 => LevelGround::BasicFloor,
            0b0010 => LevelGround::BasicFloorAndCeiling,
            0b0011 => LevelGround::BasicFloorAndThreeLayerCeiling,
            0b0100 => LevelGround::BasicFloorAndFourLayerCeiling,
            0b0101 => LevelGround::BasicFloorAndEightLayerCeiling,
            0b0110 => LevelGround::FiveLayerFloorAndCeiling,
            0b0111 => LevelGround::FiveLayerFloorAndThreeLayerCeiling,
            0b1000 => LevelGround::FiveLayerFloorAndFourLayerCeiling,
            0b1001 => LevelGround::SixLayerFloorAndCeiling,
            0b1010 => LevelGround::Ceiling,
            0b1011 => LevelGround::SixLayerFloorAndFourLayerCeiling,
            0b1100 => LevelGround::NineLayerFloorAndCeiling,
            0b1101 => LevelGround::BasicFloorThreeLayerGapFiveLayerBricksTwoLayerGapAndCeiling,
            0b1110 => LevelGround::BasicFloorThreeLayerGapFourLayerBricksThreeLayerGapAndCeiling,
            0b1111 => LevelGround::All,
            _ => unreachable!("invalid level structure byte: {}", bytes[1]),
        }
    }
}

#[derive(Debug)]
pub enum LevelTime {
    NotSet,
    T400,
    T300,
    T200,
}

#[derive(Debug)]
pub enum LevelStartPosition {
    FallFromSky,
    StartOnGround,
    HalfwayOffGround,
}

#[derive(Debug)]
pub enum LevelBackground {
    DayTime,
    Underwater,
    CastleWall,
    Overwater,
    NightTime,
    DayTimeSnow,
    NightTimeSnow,
    BlackAndWhite,
}

#[derive(Debug)]
pub enum LevelScenery {
    Nothing,
    Clouds,
    Mountains,
    Fence,
}

#[derive(Debug)]
pub enum LevelPlatform {
    GreenAndTrees,
    OrangeAndMushrooms,
    BulletBills,
    Clouds,
}

#[derive(Debug)]
pub enum LevelGround {
    Nothing,
    BasicFloor,
    BasicFloorAndCeiling,
    BasicFloorAndThreeLayerCeiling,
    BasicFloorAndFourLayerCeiling,
    BasicFloorAndEightLayerCeiling,
    FiveLayerFloorAndCeiling,
    FiveLayerFloorAndThreeLayerCeiling,
    FiveLayerFloorAndFourLayerCeiling,
    SixLayerFloorAndCeiling,
    Ceiling,
    SixLayerFloorAndFourLayerCeiling,
    NineLayerFloorAndCeiling,
    BasicFloorThreeLayerGapFiveLayerBricksTwoLayerGapAndCeiling,
    BasicFloorThreeLayerGapFourLayerBricksThreeLayerGapAndCeiling,
    All,
}
