#[derive(Debug)]
pub struct LevelHeader {
    //pub header_data: &'a [u8],
    pub time: LevelTime,
    pub start_y_position: LevelStartYPosition,
    pub background: LevelBackground,

    pub scenery: LevelScenery,
    pub ground: LevelGround,
    pub structure: LevelStructure,
}

impl LevelHeader {
    pub fn from_bytes(header_blocks: &[u8]) -> Self {
        assert!(header_blocks.len() >= 2);

        let time = Self::parse_level_time(header_blocks[0]);
        let start_y_position = Self::parse_level_y_position(header_blocks[0]);
        let background = Self::parse_level_background(header_blocks[0]);

        let scenery = Self::parse_level_scenery(header_blocks[1]);
        let ground = Self::parse_level_ground(header_blocks[1]);
        let structure = Self::parse_level_structure(header_blocks[1]);

        Self { time, start_y_position, background, scenery, ground, structure }
    }

    fn parse_level_time(byte: u8) -> LevelTime {
        let t = (byte & 0b11000000) >> 6;
        match t {
            0b00 => LevelTime::NotSet,
            0b01 => LevelTime::T400,
            0b10 => LevelTime::T300,
            0b11 => LevelTime::T200,
            _ => unreachable!("invalid level time byte: {}", byte),
        }
    }

    fn parse_level_y_position(byte: u8) -> LevelStartYPosition {
        let t = (byte & 0b00111000) >> 3;
        match t {
            0b000 => LevelStartYPosition::FallFromSky,
            0b001 => LevelStartYPosition::WhatIsThis,
            0b010 => LevelStartYPosition::StartOnGround,
            0b011 => LevelStartYPosition::HalfwayOffGround,
            0b100 => LevelStartYPosition::FallFromSky,
            0b101 => LevelStartYPosition::FallFromSky,
            0b110 => LevelStartYPosition::AutowalkOnGround,
            0b111 => LevelStartYPosition::AutowalkOnGround,
            _ => unreachable!("invalid level start y position byte: {}", byte),
        }
    }

    fn parse_level_background(byte: u8) -> LevelBackground {
        let t = byte & 0b00000111;
        match t {
            0b000 => LevelBackground::Nothing,
            0b001 => LevelBackground::WaterTilesets,
            0b010 => LevelBackground::Unknown,
            0b011 => LevelBackground::OverWater,
            0b100 => LevelBackground::Unknown,
            0b101 => LevelBackground::Unknown,
            0b110 => LevelBackground::Unknown,
            0b111 => LevelBackground::Unknown,
            _ => unreachable!("invalid level background byte: {}", byte),
        }
    }

    fn parse_level_scenery(byte: u8) -> LevelScenery {
        let t = (byte & 0b11000000) >> 6;
        match t {
            0b00 => LevelScenery::Nothing,
            0b01 => LevelScenery::Clouds,
            0b10 => LevelScenery::Mountains,
            0b11 => LevelScenery::Fence,
            _ => unreachable!("invalid level scenery byte: {}", byte),
        }
    }

    fn parse_level_ground(byte: u8) -> LevelGround {
        let t = (byte & 0b00110000) >> 4;
        match t {
            0b00 => LevelGround::GreenAndTrees,
            0b01 => LevelGround::OrangeAndMushrooms,
            0b10 => LevelGround::BulletMachines,
            0b11 => LevelGround::Clouds,
            _ => unreachable!("invalid level ground byte: {}", byte),
        }
    }

    fn parse_level_structure(byte: u8) -> LevelStructure {
        let t = byte & 0b00001111;
        match t {
            0b0000 => LevelStructure::Nothing,
            0b0001 => LevelStructure::BasicFloor,
            0b0010 => LevelStructure::BasicFloorAndCeiling,
            0b0011 => LevelStructure::BasicFloorAndThreeLayerCeiling,
            0b0100 => LevelStructure::BasicFloorAndFourLayerCeiling,
            0b0101 => LevelStructure::BasicFloorAndEightLayerCeiling,
            0b0110 => LevelStructure::FiveLayerFloorAndCeiling,
            0b0111 => LevelStructure::FiveLayerFloorAndThreeLayerCeiling,
            0b1000 => LevelStructure::FiveLayerFloorAndFourLayerCeiling,
            0b1001 => LevelStructure::SixLayerFloorAndCeiling,
            0b1010 => LevelStructure::Ceiling,
            0b1011 => LevelStructure::SixLayerFloorAndFourLayerCeiling,
            0b1100 => LevelStructure::NineLayerFloorAndCeiling,
            0b1101 => LevelStructure::BasicFloorThreeLayerGapFiveLayerBricksTwoLayerGapAndCeiling,
            0b1110 => LevelStructure::BasicFloorThreeLayerGapFourLayerBricksThreeLayerGapAndCeiling,
            0b1111 => LevelStructure::All,
            _ => unreachable!("invalid level structure byte: {}", byte),
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
pub enum LevelStartYPosition {
    FallFromSky,
    WhatIsThis,
    StartOnGround,
    HalfwayOffGround,
    AutowalkOnGround,
}

#[derive(Debug)]
pub enum LevelBackground {
    Nothing,
    WaterTilesets,
    OverWater,
    Unknown,
}

#[derive(Debug)]
pub enum LevelScenery {
    Nothing,
    Clouds,
    Mountains,
    Fence,
}

#[derive(Debug)]
pub enum LevelGround {
    GreenAndTrees,
    OrangeAndMushrooms,
    BulletMachines,
    Clouds,
}

#[derive(Debug)]
pub enum LevelStructure {
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
