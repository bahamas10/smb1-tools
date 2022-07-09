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

        let _hi_nibble = byte >> 4 & 0x0f;
        let lo_nibble = byte & 0x0f;
        match (y_coordinate, byte) {
            // Y offset 0xc
            (0xc, 0x00..=0x0f) => LevelObjectType::Hole(lo_nibble + 1),
            (0xc, 0x10..=0x1f) => LevelObjectType::BalanceHorizontalRope(lo_nibble + 1),
            (0xc, 0x20..=0x2f) => LevelObjectType::BridgeY7(lo_nibble + 1),
            (0xc, 0x30..=0x3f) => LevelObjectType::BridgeY8(lo_nibble + 1),
            (0xc, 0x40..=0x4f) => LevelObjectType::BridgeY10(lo_nibble + 1),
            (0xc, 0x50..=0x5f) => LevelObjectType::FilledHole(lo_nibble + 1),
            (0xc, 0x60..=0x6f) => LevelObjectType::HorizontalQuestionBlockY3(lo_nibble + 1),
            (0xc, 0x70..=0x7f) => LevelObjectType::HorizontalQuestionBlockY7(lo_nibble + 1),

            // Y offset 0xd
            (0xd, 0x00..=0x3f) => LevelObjectType::PageSkip(byte),

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
            (_, 0x0c..=0x0f) => LevelObjectType::Invalid,

            (_, 0x10..=0x1f) => LevelObjectType::IslandOrCannon(lo_nibble + 1),
            (_, 0x20..=0x2f) => LevelObjectType::HorizontalBrick(lo_nibble + 1),
            (_, 0x30..=0x3f) => LevelObjectType::HorizontalBlock(lo_nibble + 1),
            (_, 0x40..=0x4f) => LevelObjectType::HorizontalCoin(lo_nibble + 1),

            // anything above 12 is invalid (screen max)
            (_, 0x50..=0x5b) => LevelObjectType::VerticalBrick(lo_nibble + 1),
            (_, 0x5c..=0x5f) => LevelObjectType::Invalid,

            (_, 0x60..=0x6b) => LevelObjectType::VerticalBlock(lo_nibble + 1),
            (_, 0x6c..=0x6f) => LevelObjectType::Invalid,

            (_, 0x70..=0x77) => LevelObjectType::PipeNoEntry(lo_nibble + 2),
            (_, 0x78..=0x7f) => LevelObjectType::PipeEntry(lo_nibble - 6),


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
    Hole(u8),
    BalanceHorizontalRope(u8),
    BridgeY7(u8),
    BridgeY8(u8),
    BridgeY10(u8),
    FilledHole(u8),
    HorizontalQuestionBlockY3(u8),
    HorizontalQuestionBlockY7(u8),
    PageSkip(u8),
    CastleAxe,
    AxeRope,
    ScrollStop,
    RedCheepCheep,
    ContinuousBulletBillsOrCheepCheeps,
    StopContinuation,
    LoopCommand,
    Invalid,
}
