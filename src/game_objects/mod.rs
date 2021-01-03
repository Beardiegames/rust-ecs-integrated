use crate::ecs::{ Spawn };

pub mod types;
pub use crate::game_objects::types::Types;

pub mod units;
pub use units::*;

#[derive(Default, Clone)]
pub struct GameObject {
    pub my_type: Types,
    pub faction: Factions,
    pub position: Position,
    pub target: Option<Spawn>,
    pub weapon: Weapons,
    pub health: f32,
}

#[derive(Clone, PartialEq)]
pub enum Factions {
    None,
    Red,
    Bleu,
}

impl Default for Factions {
    fn default() -> Self { Factions::None }
}

#[derive(Default, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone)]
pub enum Weapons {
    AwefulPuns(f32),
    SmartRemarks(f32),
    Fists(f32),
    Handgun(f32),
    Rifle(f32),
    Canon(f32),
}

impl Default for Weapons {
    fn default() -> Self { Weapons::AwefulPuns(2.0) }
}