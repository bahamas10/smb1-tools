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
    pub kind: LevelObjectKind,
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

    fn parse_object_kind(bytes: &[u8]) -> LevelObjectKind {
        let y_coordinate = bytes[0] & 0b00001111;
        let byte = bytes[1] & 0b01111111;
        LevelObjectKind::new(y_coordinate, byte)
    }
}

#[derive(Debug)]
pub enum SceneryKind {
    Nothing,
    Clouds,
    Mountains,
    Fences,
}

#[derive(Debug)]
pub enum ChangeBackgroundKind {
    Nothing,
    InWater,
    CastleWall,
    OverWater,
    Night,
    Snow,
    NightAndSnow,
    NightAndCastle,
}

#[derive(Debug)]
pub enum LevelObjectKind {
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
    ScrollStopWarpZone,
    RedCheepCheep,
    ContinuousBulletBillsOrCheepCheeps,
    StopContinuation,
    LoopCommand,
    Invalid,

    LayoutEmpty(SceneryKind),
    LayoutFloor1Mddle0Ceiling0(SceneryKind),
    LayoutFloor1Mddle0Ceiling1(SceneryKind),
    LayoutFloor1Mddle0Ceiling3(SceneryKind),
    LayoutFloor1Mddle0Ceiling4(SceneryKind),
    LayoutFloor1Mddle0Ceiling8(SceneryKind),
    LayoutFloor4Mddle0Ceiling1(SceneryKind),
    LayoutFloor4Mddle0Ceiling3(SceneryKind),
    LayoutFloor4Mddle0Ceiling4(SceneryKind),
    LayoutFloor5Mddle0Ceiling1(SceneryKind),
    LayoutFloor0Mddle0Ceiling1(SceneryKind),
    LayoutFloor5Mddle0Ceiling4(SceneryKind),
    LayoutFloor8Mddle0Ceiling1(SceneryKind),
    LayoutFloor1Mddle5Ceiling1(SceneryKind),
    LayoutFloor1Mddle4Ceiling1(SceneryKind),
    LayoutFull(SceneryKind),
    ChangeBackground(ChangeBackgroundKind),

    LiftRope,
    BalanceLiftVerticalRope(u8), // length
    BigCastle,
    Staircase(u8), // width
    TallReverseLPipe(u8), // y
}

impl LevelObjectKind {
    pub fn new(y_coordinate: u8, byte: u8) -> Self {
        let _high_nibble = byte >> 4 & 0x0f;
        let low_nibble = byte & 0x0f;

        match (y_coordinate, byte) {
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
            (0xd, 0x40) => Self::ReverseLPipe,
            (0xd, 0x41) => Self::FlagPole,
            (0xd, 0x42) => Self::CastleAxe,
            (0xd, 0x43) => Self::AxeRope,
            (0xd, 0x44) => Self::CastleBridge,
            (0xd, 0x45) => Self::ScrollStopWarpZone,
            (0xd, 0x46) => Self::ScrollStop,
            (0xd, 0x47) => Self::ScrollStop,
            (0xd, 0x48) => Self::RedCheepCheep,
            (0xd, 0x49) => Self::ContinuousBulletBillsOrCheepCheeps,
            (0xd, 0x4a) => Self::StopContinuation,
            (0xd, 0x4b) => Self::LoopCommand,
            (0xd, 0x4c..=0x4f) => Self::Invalid,
            (0xd, 0x50..=0x7f) => Self::Invalid,

            // Y offset 0xe
            (0xe, 0x00) => Self::LayoutEmpty(SceneryKind::Nothing),
            (0xe, 0x01) => {
                Self::LayoutFloor1Mddle0Ceiling0(SceneryKind::Nothing)
            }
            (0xe, 0x02) => {
                Self::LayoutFloor1Mddle0Ceiling1(SceneryKind::Nothing)
            }
            (0xe, 0x03) => {
                Self::LayoutFloor1Mddle0Ceiling3(SceneryKind::Nothing)
            }
            (0xe, 0x04) => {
                Self::LayoutFloor1Mddle0Ceiling4(SceneryKind::Nothing)
            }
            (0xe, 0x05) => {
                Self::LayoutFloor1Mddle0Ceiling8(SceneryKind::Nothing)
            }
            (0xe, 0x06) => {
                Self::LayoutFloor4Mddle0Ceiling1(SceneryKind::Nothing)
            }
            (0xe, 0x07) => {
                Self::LayoutFloor4Mddle0Ceiling3(SceneryKind::Nothing)
            }
            (0xe, 0x08) => {
                Self::LayoutFloor4Mddle0Ceiling4(SceneryKind::Nothing)
            }
            (0xe, 0x09) => {
                Self::LayoutFloor5Mddle0Ceiling1(SceneryKind::Nothing)
            }
            (0xe, 0x0a) => {
                Self::LayoutFloor0Mddle0Ceiling1(SceneryKind::Nothing)
            }
            (0xe, 0x0b) => {
                Self::LayoutFloor5Mddle0Ceiling4(SceneryKind::Nothing)
            }
            (0xe, 0x0c) => {
                Self::LayoutFloor8Mddle0Ceiling1(SceneryKind::Nothing)
            }
            (0xe, 0x0d) => {
                Self::LayoutFloor1Mddle5Ceiling1(SceneryKind::Nothing)
            }
            (0xe, 0x0e) => {
                Self::LayoutFloor1Mddle4Ceiling1(SceneryKind::Nothing)
            }
            (0xe, 0x0f) => Self::LayoutFull(SceneryKind::Nothing),
            // ---
            (0xe, 0x10) => Self::LayoutEmpty(SceneryKind::Clouds),
            (0xe, 0x11) => {
                Self::LayoutFloor1Mddle0Ceiling0(SceneryKind::Clouds)
            }
            (0xe, 0x12) => {
                Self::LayoutFloor1Mddle0Ceiling1(SceneryKind::Clouds)
            }
            (0xe, 0x13) => {
                Self::LayoutFloor1Mddle0Ceiling3(SceneryKind::Clouds)
            }
            (0xe, 0x14) => {
                Self::LayoutFloor1Mddle0Ceiling4(SceneryKind::Clouds)
            }
            (0xe, 0x15) => {
                Self::LayoutFloor1Mddle0Ceiling8(SceneryKind::Clouds)
            }
            (0xe, 0x16) => {
                Self::LayoutFloor4Mddle0Ceiling1(SceneryKind::Clouds)
            }
            (0xe, 0x17) => {
                Self::LayoutFloor4Mddle0Ceiling3(SceneryKind::Clouds)
            }
            (0xe, 0x18) => {
                Self::LayoutFloor4Mddle0Ceiling4(SceneryKind::Clouds)
            }
            (0xe, 0x19) => {
                Self::LayoutFloor5Mddle0Ceiling1(SceneryKind::Clouds)
            }
            (0xe, 0x1a) => {
                Self::LayoutFloor0Mddle0Ceiling1(SceneryKind::Clouds)
            }
            (0xe, 0x1b) => {
                Self::LayoutFloor5Mddle0Ceiling4(SceneryKind::Clouds)
            }
            (0xe, 0x1c) => {
                Self::LayoutFloor8Mddle0Ceiling1(SceneryKind::Clouds)
            }
            (0xe, 0x1d) => {
                Self::LayoutFloor1Mddle5Ceiling1(SceneryKind::Clouds)
            }
            (0xe, 0x1e) => {
                Self::LayoutFloor1Mddle4Ceiling1(SceneryKind::Clouds)
            }
            (0xe, 0x1f) => Self::LayoutFull(SceneryKind::Clouds),
            // ---
            (0xe, 0x20) => Self::LayoutEmpty(SceneryKind::Mountains),
            (0xe, 0x21) => {
                Self::LayoutFloor1Mddle0Ceiling0(SceneryKind::Mountains)
            }
            (0xe, 0x22) => {
                Self::LayoutFloor1Mddle0Ceiling1(SceneryKind::Mountains)
            }
            (0xe, 0x23) => {
                Self::LayoutFloor1Mddle0Ceiling3(SceneryKind::Mountains)
            }
            (0xe, 0x24) => {
                Self::LayoutFloor1Mddle0Ceiling4(SceneryKind::Mountains)
            }
            (0xe, 0x25) => {
                Self::LayoutFloor1Mddle0Ceiling8(SceneryKind::Mountains)
            }
            (0xe, 0x26) => {
                Self::LayoutFloor4Mddle0Ceiling1(SceneryKind::Mountains)
            }
            (0xe, 0x27) => {
                Self::LayoutFloor4Mddle0Ceiling3(SceneryKind::Mountains)
            }
            (0xe, 0x28) => {
                Self::LayoutFloor4Mddle0Ceiling4(SceneryKind::Mountains)
            }
            (0xe, 0x29) => {
                Self::LayoutFloor5Mddle0Ceiling1(SceneryKind::Mountains)
            }
            (0xe, 0x2a) => {
                Self::LayoutFloor0Mddle0Ceiling1(SceneryKind::Mountains)
            }
            (0xe, 0x2b) => {
                Self::LayoutFloor5Mddle0Ceiling4(SceneryKind::Mountains)
            }
            (0xe, 0x2c) => {
                Self::LayoutFloor8Mddle0Ceiling1(SceneryKind::Mountains)
            }
            (0xe, 0x2d) => {
                Self::LayoutFloor1Mddle5Ceiling1(SceneryKind::Mountains)
            }
            (0xe, 0x2e) => {
                Self::LayoutFloor1Mddle4Ceiling1(SceneryKind::Mountains)
            }
            (0xe, 0x2f) => Self::LayoutFull(SceneryKind::Mountains),
            // ---
            (0xe, 0x30) => Self::LayoutEmpty(SceneryKind::Fences),
            (0xe, 0x31) => {
                Self::LayoutFloor1Mddle0Ceiling0(SceneryKind::Fences)
            }
            (0xe, 0x32) => {
                Self::LayoutFloor1Mddle0Ceiling1(SceneryKind::Fences)
            }
            (0xe, 0x33) => {
                Self::LayoutFloor1Mddle0Ceiling3(SceneryKind::Fences)
            }
            (0xe, 0x34) => {
                Self::LayoutFloor1Mddle0Ceiling4(SceneryKind::Fences)
            }
            (0xe, 0x35) => {
                Self::LayoutFloor1Mddle0Ceiling8(SceneryKind::Fences)
            }
            (0xe, 0x36) => {
                Self::LayoutFloor4Mddle0Ceiling1(SceneryKind::Fences)
            }
            (0xe, 0x37) => {
                Self::LayoutFloor4Mddle0Ceiling3(SceneryKind::Fences)
            }
            (0xe, 0x38) => {
                Self::LayoutFloor4Mddle0Ceiling4(SceneryKind::Fences)
            }
            (0xe, 0x39) => {
                Self::LayoutFloor5Mddle0Ceiling1(SceneryKind::Fences)
            }
            (0xe, 0x3a) => {
                Self::LayoutFloor0Mddle0Ceiling1(SceneryKind::Fences)
            }
            (0xe, 0x3b) => {
                Self::LayoutFloor5Mddle0Ceiling4(SceneryKind::Fences)
            }
            (0xe, 0x3c) => {
                Self::LayoutFloor8Mddle0Ceiling1(SceneryKind::Fences)
            }
            (0xe, 0x3d) => {
                Self::LayoutFloor1Mddle5Ceiling1(SceneryKind::Fences)
            }
            (0xe, 0x3e) => {
                Self::LayoutFloor1Mddle4Ceiling1(SceneryKind::Fences)
            }
            (0xe, 0x3f) => Self::LayoutFull(SceneryKind::Fences),
            // ---
            (0xe, 0x40) => Self::ChangeBackground(ChangeBackgroundKind::Nothing),
            (0xe, 0x41) => Self::ChangeBackground(ChangeBackgroundKind::InWater),
            (0xe, 0x42) => Self::ChangeBackground(ChangeBackgroundKind::CastleWall),
            (0xe, 0x43) => Self::ChangeBackground(ChangeBackgroundKind::OverWater),
            (0xe, 0x44) => Self::ChangeBackground(ChangeBackgroundKind::Night),
            (0xe, 0x45) => Self::ChangeBackground(ChangeBackgroundKind::Snow),
            (0xe, 0x46) => Self::ChangeBackground(ChangeBackgroundKind::NightAndSnow),
            (0xe, 0x47) => Self::ChangeBackground(ChangeBackgroundKind::NightAndCastle),
            (0xe, 0x48..=0x7f) => {
                Self::Invalid
            }

            // Y offset 0xf
            (0xf, 0x00) => Self::LiftRope,
            (0xf, 0x01..=0x0f) => Self::Invalid,
            (0xf, 0x10..=0x1f) => Self::BalanceLiftVerticalRope(low_nibble + 1),
            (0xf, 0x20) => Self::BigCastle,
            (0xf, 0x21..=0x2f) => Self::Invalid,
            (0xf, 0x30..=0x38) => Self::Staircase(low_nibble + 1),
            (0xf, 0x39..=0x3f) => Self::Invalid,
            (0xf, 0x40..=0x42) => Self::Invalid,
            (0xf, 0x43..=0x4a) => Self::TallReverseLPipe(low_nibble),
            (0xf, 0x4b..=0x4f) => Self::Invalid,
            (0xf, 0x50..=0x5f) => Self::Invalid,
            (0xf, 0x60) => Self::Nothing,
            (0xf, 0x61..=0x6f) => Self::Invalid,
            (0xf, 0x70..=0x7f) => Self::Invalid,

            _ => Self::Invalid,
            //_ => unreachable!("invalid level object byte: ({}, {})", y_coordinate, byte),
        }
    }
}
