use crate::game::model::{Game, Player, DungeonFloor};
use crate::game::model::DungeonCell::{Empty, Floor, Wall};
use slotmap::SlotMap;
use glm::Vec3;

pub fn build_experimental_game() -> Game {
    Game {
        player: Player{
            pos: Vec3::new(1.0, 8.0, 0.0),
            angle: 0.0,
            direction: Vec3::new(1.0, 0.0, 0.0),
            speed: Vec3::new(0.0, 0.0, 0.0),
        },
        current_map: DungeonFloor {
            width: 10, height: 10,
            cells: vec![
                Empty, Empty, Empty, Wall,  Wall,  Wall,  Wall,  Empty, Empty, Empty,
                Empty, Empty, Empty, Wall,  Floor, Floor, Wall,  Empty, Empty, Empty,
                Wall,  Wall,  Wall,  Wall,  Floor, Floor, Wall,  Empty, Empty, Empty,
                Wall,  Floor, Floor, Floor, Floor, Floor, Wall,  Wall,  Wall,  Wall,
                Wall,  Floor, Floor, Floor, Floor, Floor, Floor, Floor, Floor, Wall,
                Wall,  Floor, Floor, Wall,  Floor, Floor, Floor, Floor, Floor, Wall,
                Wall,  Floor, Floor, Floor, Floor, Floor, Wall,  Wall,  Wall,  Wall,
                Wall,  Floor, Floor, Floor, Floor, Floor, Wall,  Empty, Empty, Empty,
                Wall,  Floor, Floor, Floor, Floor, Floor, Wall,  Empty, Empty, Empty,
                Wall,  Wall,  Wall,  Wall,  Wall,  Wall,  Wall,  Empty, Empty, Empty
            ]
        },
        entities: SlotMap::with_key()
    }
}
