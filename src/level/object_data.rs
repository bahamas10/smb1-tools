#[derive(Debug)]
pub struct LevelObjectData {
    pub objects: Vec<Object>,
}

impl LevelObjectData {
    pub fn from_bytes(object_bytes: &[u8]) -> Self {
        let mut objects = vec![];

        // process byte-by-byte
        let mut idx = 0;
        loop {
            let byte = object_bytes[idx];
            //println!("processing byte: {:x}", byte);

            // 0xFD is the end level marker
            if byte == 0xFD {
                break;
            }

            /*
             * The 2 byte block form looks like:
             * - XXXXYYYY POOOOOOO
             */
            let object = Object::from_bytes(&object_bytes[idx..]);

            objects.push(object);
            idx += 2;
        }

        Self { objects }
    }
}

#[derive(Debug)]
pub enum ObjectType {
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
pub struct Object {
    pub object_type: ObjectType,
    pub x_coordinate: u8,
    pub y_coordinate: u8,
    pub new_page_flag: bool,
}

impl Object {
    /**
     * XXXXYYYY POOOOOOO
     */
    pub fn from_bytes(bytes: &[u8]) -> Self {
        assert!(bytes.len() >= 2);
        let x_coordinate = bytes[0] << 4;
        let y_coordinate = bytes[0] & 0b00001111;
        let object_type = ObjectType::CastleAxe;
        let new_page_flag = bytes[1] & 0b10000000 != 0;

        Self { object_type, x_coordinate, y_coordinate, new_page_flag }
    }
}
