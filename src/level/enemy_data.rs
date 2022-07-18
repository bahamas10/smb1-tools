#[derive(Debug)]
pub struct LevelEnemyData {
    pub enemies: Vec<LevelEnemy>,
    pub pipe_pointers: Vec<PipePointer>,
}

impl LevelEnemyData {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut enemies = vec![];
        let mut pipe_pointers = vec![];

        // process byte-by-byte
        let mut idx = 0;
        loop {
            let byte = bytes[idx];

            // 0xFF is the end enemy marker
            if byte == 0xFF {
                break;
            }

            if (byte & 0x0F) == 0x0E {
                // pipe pointer (3 bytes)
                pipe_pointers.push(PipePointer::from_bytes(&bytes[idx..]));
                idx += 3;
            } else {
                // enemy pointer (2 bytes)
                enemies.push(LevelEnemy::from_bytes(&bytes[idx..]));
                idx += 2;
            };
        }

        Self { enemies, pipe_pointers }
    }
}

#[derive(Debug)]
pub enum LevelEnemyKind {
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
pub struct LevelEnemy {
    pub kind: LevelEnemyKind,
    pub x_coordinate: u8,
    pub y_coordinate: u8,
    pub new_page_flag: bool,
}

impl LevelEnemy {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        assert!(bytes.len() >= 2);
        let x_coordinate = bytes[0] << 4;
        let y_coordinate = bytes[0] & 0b00001111;
        let kind = LevelEnemyKind::CastleAxe;
        let new_page_flag = bytes[1] & 0b10000000 != 0;

        Self { kind, x_coordinate, y_coordinate, new_page_flag }
    }
}

#[derive(Debug)]
pub struct PipePointer {
    pub x_coordinate: u8,
    pub y_coordinate: u8,
    pub new_page_flag: bool,
}

impl PipePointer {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        assert!(bytes.len() >= 3);
        let x_coordinate = bytes[0] << 4;
        let y_coordinate = bytes[0] & 0b00001111;
        let new_page_flag = bytes[1] & 0b10000000 != 0;

        Self { x_coordinate, y_coordinate, new_page_flag }
    }
}
