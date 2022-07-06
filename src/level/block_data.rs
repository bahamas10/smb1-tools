#[derive(Debug)]
pub struct LevelBlockData {
    pub blocks: Vec<Block>,
}

impl LevelBlockData {
    pub fn from_bytes(block_bytes: &[u8]) -> Self {
        let mut blocks = vec![];

        // process byte-by-byte
        let mut idx = 0;
        loop {
            let byte = block_bytes[idx];
            //println!("processing byte: {:x}", byte);

            // 0xFD is the end level marker
            if byte == 0xFD {
                break;
            }

            /*
             * The 2 byte block form looks like:
             * - XXXXYYYY POOOOOOO
             */
            let block = Block::from_2_bytes(&block_bytes[idx..]);

            blocks.push(block);
            idx += 2;
        }

        Self { blocks }
    }
}

#[derive(Debug)]
pub enum BlockType {
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
pub struct Block {
    pub block_type: BlockType,
    pub x_coordinate: u8,
    pub y_coordinate: u8,
    pub new_page_flag: bool,
}

impl Block {
    /**
     * XXXXYYYY POOOOOOO
     */
    pub fn from_2_bytes(bytes: &[u8]) -> Self {
        assert!(bytes.len() >= 2);
        let x_coordinate = bytes[0] << 4;
        let y_coordinate = bytes[0] & 0b00001111;
        let block_type = BlockType::CastleAxe;
        let new_page_flag = bytes[1] & 0b10000000 != 0;

        Self { block_type, x_coordinate, y_coordinate, new_page_flag }
    }

    /*
    /**
     * XXXX1111 YYYYOOOO POOOVVVV
     */
    pub fn from_3_bytes(bytes: &[u8]) -> Self {
        assert!(bytes.len() >= 3);
               // (byte << 4, next_byte << 4)
        let x_coordinate = bytes[0] << 4;
        let y_coordinate = bytes[1] << 4;
        let block_type = BlockType::CastleAxe;
        let new_page_flag = bytes[2] & 0b10000000 != 0;

        Self {
            block_type, x_coordinate, y_coordinate, new_page_flag,
        }
    }
    */
}
