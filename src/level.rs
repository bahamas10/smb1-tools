mod block_data;
mod header;
mod sprite_data;

pub use block_data::*;
pub use header::*;
pub use sprite_data::*;

#[derive(Debug)]
pub struct Level {
    pub level_header: LevelHeader,
    pub block_data: LevelBlockData,
    pub sprite_data: LevelSpriteData,
}

impl Level {
    pub fn new(
        level_header: LevelHeader,
        block_data: LevelBlockData,
        sprite_data: LevelSpriteData,
    ) -> Self {
        Self { level_header, block_data, sprite_data }
    }
}
