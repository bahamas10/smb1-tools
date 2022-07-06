mod enemy_data;
mod header;
mod object_data;

pub use enemy_data::*;
pub use header::*;
pub use object_data::*;

#[derive(Debug)]
pub struct Level {
    pub level_header: LevelHeader,
    pub object_data: LevelObjectData,
    pub enemy_data: LevelEnemyData,
}

impl Level {
    pub fn new(
        level_header: LevelHeader,
        object_data: LevelObjectData,
        enemy_data: LevelEnemyData,
    ) -> Self {
        Self { level_header, object_data, enemy_data }
    }
}
