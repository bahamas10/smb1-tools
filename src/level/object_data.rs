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
        let kind = Self::parse_object_kind(bytes);

        Self { kind, x_coordinate, y_coordinate, new_page_flag }
    }

    fn parse_object_kind(bytes: &[u8]) -> LevelObjectType {
        let y_coordinate = bytes[0] & 0b00001111;
        let byte = bytes[1] & 0b01111111;
        LevelObjectType::new(y_coordinate, byte)
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

impl LevelObjectType {
    pub fn new(y_coordinate: u8, byte: u8) -> Self {
        let _high_nibble = byte >> 4 & 0x0f;
        let low_nibble = byte & 0x0f;

        match (y_coordinate, byte) {
            // Y offset 0xc
            (0xc, 0x00..=0x0f) => Self::Hole(low_nibble + 1),
            (0xc, 0x10..=0x1f) => Self::BalanceHorizontalRope(low_nibble + 1),
            (0xc, 0x20..=0x2f) => Self::BridgeY7(low_nibble + 1),
            (0xc, 0x30..=0x3f) => Self::BridgeY8(low_nibble + 1),
            (0xc, 0x40..=0x4f) => Self::BridgeY10(low_nibble + 1),
            (0xc, 0x50..=0x5f) => Self::FilledHole(low_nibble + 1),
            (0xc, 0x60..=0x6f) => {
                Self::HorizontalQuestionBlockY3(low_nibble + 1)
            }
            (0xc, 0x70..=0x7f) => {
                Self::HorizontalQuestionBlockY7(low_nibble + 1)
            }

            // Y offset 0xd
            (0xd, 0x00..=0x3f) => Self::PageSkip(byte),

            // Y offset 0x0 -> 0xb
            (0x0..=0xb, 0x00) => Self::QuestionBlockPowerup,
            (0x0..=0xb, 0x01) => Self::QuestionBlockCoin,
            (0x0..=0xb, 0x02) => Self::HiddenBlockCoin,
            (0x0..=0xb, 0x03) => Self::HiddenBlockExtraLife,
            (0x0..=0xb, 0x04) => Self::BrickPowerup,
            (0x0..=0xb, 0x05) => Self::BrickVine,
            (0x0..=0xb, 0x06) => Self::BrickStar,
            (0x0..=0xb, 0x07) => Self::BrickMultiCoinBlock,
            (0x0..=0xb, 0x08) => Self::BrickExtraLife,
            (0x0..=0xb, 0x09) => Self::SidewaysPipe,
            (0x0..=0xb, 0x0a) => Self::UsedBlock,
            (0x0..=0xb, 0x0b) => Self::Spring,
            (0x0..=0xb, 0x0c..=0x0f) => Self::Invalid,

            (0x0..=0xb, 0x10..=0x1f) => Self::IslandOrCannon(low_nibble + 1),
            (0x0..=0xb, 0x20..=0x2f) => Self::HorizontalBrick(low_nibble + 1),
            (0x0..=0xb, 0x30..=0x3f) => Self::HorizontalBlock(low_nibble + 1),
            (0x0..=0xb, 0x40..=0x4f) => Self::HorizontalCoin(low_nibble + 1),

            // anything above 12 is invalid (screen max)
            (0x0..=0xb, 0x50..=0x5b) => Self::VerticalBrick(low_nibble + 1),
            (0x0..=0xb, 0x5c..=0x5f) => Self::Invalid,

            (0x0..=0xb, 0x60..=0x6b) => Self::VerticalBlock(low_nibble + 1),
            (0x0..=0xb, 0x6c..=0x6f) => Self::Invalid,

            (0x0..=0xb, 0x70..=0x77) => Self::PipeNoEntry(low_nibble + 2),
            (0x0..=0xb, 0x78..=0x7f) => Self::PipeEntry(low_nibble - 6),

            _ => Self::Invalid,
            //_ => unreachable!("invalid level object byte: ({}, {})", y_coordinate, byte),
        }
    }
}
