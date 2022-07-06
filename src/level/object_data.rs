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
    IslandOrCannon,
    HorizontalBrick,
    HorizontalBlock,
    HorizontalCoins,
    VerticalBrick,
    VerticalBlock,
    PipeNoEntry,
    PipeEntry,
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
    DoNotUse,
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
            (_, 0x0c) => LevelObjectType::DoNotUse,
            (_, 0x0d) => LevelObjectType::DoNotUse,
            (_, 0x0e) => LevelObjectType::DoNotUse,
            (_, 0x0f) => LevelObjectType::DoNotUse,
            _ => LevelObjectType::DoNotUse,
            //_ => unreachable!("invalid level object byte: ({}, {})", y_coordinate, byte),
        }
    }
}
