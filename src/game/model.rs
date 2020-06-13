//! Defines the game model.

use slotmap::{ SlotMap, new_key_type };
use glm::{Vec3};

pub enum DungeonCell {
    Empty,
    Wall,
    Floor,
    Door,
    OpenDoor
}

pub struct DungeonFloor {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<DungeonCell>,
}

/// The player will be dismembered in components when the 3d engine is stable
/// enough
pub struct Player {
    pub pos: Vec3,
    pub angle: f32,
    pub direction: Vec3,
    pub speed: Vec3
}

pub trait System {
    fn update(&self, game: &Game);
}

new_key_type! {
    pub struct EntityKey;
}

#[derive(Copy, Clone)]
pub struct Entity {
    pub key: EntityKey,
    // Think about ECS when needed.....
}

pub struct Game {
    pub player: Player,
    pub current_map: DungeonFloor,
    pub entities: SlotMap<EntityKey, Entity>
}
