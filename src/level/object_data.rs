#[derive(Debug)]
pub struct LevelObjectData {
    pub objects: Vec<LevelObject>,
}

impl LevelObjectData {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut objects = vec![];

        // process byte-by-byte
        let mut idx = 0;
        loop {
            let byte = bytes[idx];

            // 0xFD is the end level marker
            if byte == 0xFD {
                break;
            }

            let object = LevelObject::from_bytes(&bytes[idx..]);

            objects.push(object);
            idx += 2;
        }

        Self { objects }
    }
}

#[derive(Debug)]
pub struct LevelObject {
    pub kind: LevelObjectType,
    pub x_coordinate: u8,
    pub y_coordinate: u8,
    pub new_page_flag: bool,
}

impl LevelObject {
    /**
     * XXXXYYYY POOOOOOO
     */
    pub fn from_bytes(bytes: &[u8]) -> Self {
        assert!(bytes.len() >= 2);
        let x_coordinate = bytes[0] << 4;
        let y_coordinate = bytes[0] & 0b00001111;
        let new_page_flag = bytes[1] & 0b10000000 != 0;
        let kind = Self::parse_object_kind(y_coordinate, bytes[1]);

        Self { kind, x_coordinate, y_coordinate, new_page_flag }
    }

    fn parse_object_kind(y_coordinate: u8, byte: u8) -> LevelObjectType {
        // turn off new_page_flag
        let byte = byte & 0b01111111;

        match (y_coordinate, byte) {
            (_, 0x00) => LevelObjectType::QuestionBlockPowerup,
            (_, 0x01) => LevelObjectType::QuestionBlockCoin,
            (_, 0x02) => LevelObjectType::HiddenBlockCoin,
            (_, 0x03) => LevelObjectType::HiddenBlockExtraLife,
            (_, 0x04) => LevelObjectType::BrickPowerup,
            (_, 0x05) => LevelObjectType::BrickVine,
            (_, 0x06) => LevelObjectType::BrickStar,
            (_, 0x07) => LevelObjectType::BrickMultiCoinBlock,
            (_, 0x08) => LevelObjectType::BrickExtraLife,
            (_, 0x09) => LevelObjectType::SidewaysPipe,
            (_, 0x0a) => LevelObjectType::UsedBlock,
            (_, 0x0b) => LevelObjectType::Spring,
            (_, 0x0c) => LevelObjectType::Invalid,
            (_, 0x0d) => LevelObjectType::Invalid,
            (_, 0x0e) => LevelObjectType::Invalid,
            (_, 0x0f) => LevelObjectType::Invalid,
            (_, 0x10) => LevelObjectType::IslandOrCannon(1),
            (_, 0x11) => LevelObjectType::IslandOrCannon(2),
            (_, 0x12) => LevelObjectType::IslandOrCannon(3),
            (_, 0x13) => LevelObjectType::IslandOrCannon(4),
            (_, 0x14) => LevelObjectType::IslandOrCannon(5),
            (_, 0x15) => LevelObjectType::IslandOrCannon(6),
            (_, 0x16) => LevelObjectType::IslandOrCannon(7),
            (_, 0x17) => LevelObjectType::IslandOrCannon(8),
            (_, 0x18) => LevelObjectType::IslandOrCannon(9),
            (_, 0x19) => LevelObjectType::IslandOrCannon(10),
            (_, 0x1a) => LevelObjectType::IslandOrCannon(11),
            (_, 0x1b) => LevelObjectType::IslandOrCannon(12),
            (_, 0x1c) => LevelObjectType::IslandOrCannon(13),
            (_, 0x1d) => LevelObjectType::IslandOrCannon(14),
            (_, 0x1e) => LevelObjectType::IslandOrCannon(15),
            (_, 0x1f) => LevelObjectType::IslandOrCannon(16),
            (_, 0x20) => LevelObjectType::HorizontalBrick(1),
            (_, 0x21) => LevelObjectType::HorizontalBrick(2),
            (_, 0x22) => LevelObjectType::HorizontalBrick(3),
            (_, 0x23) => LevelObjectType::HorizontalBrick(4),
            (_, 0x24) => LevelObjectType::HorizontalBrick(5),
            (_, 0x25) => LevelObjectType::HorizontalBrick(6),
            (_, 0x26) => LevelObjectType::HorizontalBrick(7),
            (_, 0x27) => LevelObjectType::HorizontalBrick(8),
            (_, 0x28) => LevelObjectType::HorizontalBrick(9),
            (_, 0x29) => LevelObjectType::HorizontalBrick(10),
            (_, 0x2a) => LevelObjectType::HorizontalBrick(11),
            (_, 0x2b) => LevelObjectType::HorizontalBrick(12),
            (_, 0x2c) => LevelObjectType::HorizontalBrick(13),
            (_, 0x2d) => LevelObjectType::HorizontalBrick(14),
            (_, 0x2e) => LevelObjectType::HorizontalBrick(15),
            (_, 0x2f) => LevelObjectType::HorizontalBrick(16),
            (_, 0x30) => LevelObjectType::HorizontalBlock(1),
            (_, 0x31) => LevelObjectType::HorizontalBlock(2),
            (_, 0x32) => LevelObjectType::HorizontalBlock(3),
            (_, 0x33) => LevelObjectType::HorizontalBlock(4),
            (_, 0x34) => LevelObjectType::HorizontalBlock(5),
            (_, 0x35) => LevelObjectType::HorizontalBlock(6),
            (_, 0x36) => LevelObjectType::HorizontalBlock(7),
            (_, 0x37) => LevelObjectType::HorizontalBlock(8),
            (_, 0x38) => LevelObjectType::HorizontalBlock(9),
            (_, 0x39) => LevelObjectType::HorizontalBlock(10),
            (_, 0x3a) => LevelObjectType::HorizontalBlock(11),
            (_, 0x3b) => LevelObjectType::HorizontalBlock(12),
            (_, 0x3c) => LevelObjectType::HorizontalBlock(13),
            (_, 0x3d) => LevelObjectType::HorizontalBlock(14),
            (_, 0x3e) => LevelObjectType::HorizontalBlock(15),
            (_, 0x3f) => LevelObjectType::HorizontalBlock(16),
            (_, 0x40) => LevelObjectType::HorizontalCoin(1),
            (_, 0x41) => LevelObjectType::HorizontalCoin(2),
            (_, 0x42) => LevelObjectType::HorizontalCoin(3),
            (_, 0x43) => LevelObjectType::HorizontalCoin(4),
            (_, 0x44) => LevelObjectType::HorizontalCoin(5),
            (_, 0x45) => LevelObjectType::HorizontalCoin(6),
            (_, 0x46) => LevelObjectType::HorizontalCoin(7),
            (_, 0x47) => LevelObjectType::HorizontalCoin(8),
            (_, 0x48) => LevelObjectType::HorizontalCoin(9),
            (_, 0x49) => LevelObjectType::HorizontalCoin(10),
            (_, 0x4a) => LevelObjectType::HorizontalCoin(11),
            (_, 0x4b) => LevelObjectType::HorizontalCoin(12),
            (_, 0x4c) => LevelObjectType::HorizontalCoin(13),
            (_, 0x4d) => LevelObjectType::HorizontalCoin(14),
            (_, 0x4e) => LevelObjectType::HorizontalCoin(15),
            (_, 0x4f) => LevelObjectType::HorizontalCoin(16),
            (_, 0x50) => LevelObjectType::VerticalBrick(1),
            (_, 0x51) => LevelObjectType::VerticalBrick(2),
            (_, 0x52) => LevelObjectType::VerticalBrick(3),
            (_, 0x53) => LevelObjectType::VerticalBrick(4),
            (_, 0x54) => LevelObjectType::VerticalBrick(5),
            (_, 0x55) => LevelObjectType::VerticalBrick(6),
            (_, 0x56) => LevelObjectType::VerticalBrick(7),
            (_, 0x57) => LevelObjectType::VerticalBrick(8),
            (_, 0x58) => LevelObjectType::VerticalBrick(9),
            (_, 0x59) => LevelObjectType::VerticalBrick(10),
            (_, 0x5a) => LevelObjectType::VerticalBrick(11),
            (_, 0x5b) => LevelObjectType::VerticalBrick(12),
            (_, 0x5c) => LevelObjectType::Invalid,
            (_, 0x5d) => LevelObjectType::Invalid,
            (_, 0x5e) => LevelObjectType::Invalid,
            (_, 0x5f) => LevelObjectType::Invalid,
            (_, 0x60) => LevelObjectType::VerticalBlock(1),
            (_, 0x61) => LevelObjectType::VerticalBlock(2),
            (_, 0x62) => LevelObjectType::VerticalBlock(3),
            (_, 0x63) => LevelObjectType::VerticalBlock(4),
            (_, 0x64) => LevelObjectType::VerticalBlock(5),
            (_, 0x65) => LevelObjectType::VerticalBlock(6),
            (_, 0x66) => LevelObjectType::VerticalBlock(7),
            (_, 0x67) => LevelObjectType::VerticalBlock(8),
            (_, 0x68) => LevelObjectType::VerticalBlock(9),
            (_, 0x69) => LevelObjectType::VerticalBlock(10),
            (_, 0x6a) => LevelObjectType::VerticalBlock(11),
            (_, 0x6b) => LevelObjectType::VerticalBlock(12),
            (_, 0x6c) => LevelObjectType::Invalid,
            (_, 0x6d) => LevelObjectType::Invalid,
            (_, 0x6e) => LevelObjectType::Invalid,
            (_, 0x6f) => LevelObjectType::Invalid,
            (_, 0x70) => LevelObjectType::PipeNoEntry(2),
            (_, 0x71) => LevelObjectType::PipeNoEntry(3),
            (_, 0x72) => LevelObjectType::PipeNoEntry(4),
            (_, 0x73) => LevelObjectType::PipeNoEntry(5),
            (_, 0x74) => LevelObjectType::PipeNoEntry(6),
            (_, 0x75) => LevelObjectType::PipeNoEntry(7),
            (_, 0x76) => LevelObjectType::PipeNoEntry(8),
            (_, 0x77) => LevelObjectType::PipeNoEntry(9),
            (_, 0x78) => LevelObjectType::PipeEntry(2),
            (_, 0x79) => LevelObjectType::PipeEntry(3),
            (_, 0x7a) => LevelObjectType::PipeEntry(4),
            (_, 0x7b) => LevelObjectType::PipeEntry(5),
            (_, 0x7c) => LevelObjectType::PipeEntry(6),
            (_, 0x7d) => LevelObjectType::PipeEntry(7),
            (_, 0x7e) => LevelObjectType::PipeEntry(8),
            (_, 0x7f) => LevelObjectType::PipeEntry(9),
            _ => LevelObjectType::Invalid,
            //_ => unreachable!("invalid level object byte: ({}, {})", y_coordinate, byte),
        }
    }
}

#[derive(Debug)]
pub enum LevelObjectType {
    QuestionBlockPowerup,
    QuestionBlockCoin,
    HiddenBlockCoin,
    HiddenBlockExtraLife,
    BrickPowerup,
    BrickVine,
    BrickStar,
    BrickMultiCoinBlock,
    BrickExtraLife,
    SidewaysPipe,
    UsedBlock,
    Spring,
    ReverseLPipe,
    FlagPole,
    CastleBridge,
    Nothing,
    IslandOrCannon(u8),
    HorizontalBrick(u8),
    HorizontalBlock(u8),
    HorizontalCoin(u8),
    VerticalBrick(u8),
    VerticalBlock(u8),
    PipeNoEntry(u8),
    PipeEntry(u8),
    Hole,
    BalanceHorizontalRope,
    BridgeV7,
    BridgeV8,
    BridgeV10,
    FilledHole,
    HorizontalQuestionBlockV3,
    HorizontalQuestionBlockV7,
    PageSkip,
    CastleAxe,
    AxeRope,
    ScrollStop,
    RedCheepCheep,
    ContinuousBulletBillsOrCheepCheeps,
    StopContinuation,
    LoopCommand,
    Invalid,
}
