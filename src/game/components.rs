
use crate::ecs::{ Spawn, Entity };


#[derive(Default, Clone)]
pub struct GameObject {
    pub position: (f32, f32),
    pub faction: Faction,

    pub movement: Option<Movement>,
    pub health: Option<Health>,
    pub assault: Option<Assault>,
    pub carry: Option<Carry>,
}

impl Entity for GameObject {}

// --component types--

#[derive(Default, Clone)]
pub struct Movement {
    pub speed: f32,
    pub move_to: Option<(f32, f32)>,
}
impl Movement {
    pub fn new(speed: f32) -> Self {
        Movement { speed, move_to: None }
    }
}

#[derive(Default, Clone)]
pub struct Health {
    pub hp: i32,
    pub damage: Vec<Attack>,
}
impl Health {
    pub fn new(hp: i32) -> Self {
        Health { hp, damage: Vec::new() }
    }
}

#[derive(Default, Clone)]
pub struct Assault {
    pub target: Option<Spawn>,
    pub attack: Attack,
}
impl Assault {
    pub fn new(attack: Attack) -> Self {
        Assault { target: None, attack }
    }
}

#[derive(Clone)]
pub enum Attack {
    SmartRemarks(Spawn, i32),
    Fist(Spawn, i32),
    Knife(Spawn, i32),
    Gun(Spawn, i32),
    Canon(Spawn, i32),
}

impl Default for Attack {
    fn default() -> Self { 
        Attack::SmartRemarks(Spawn::default(), 0)
    }
}

#[derive(Default, Clone)]
pub struct Carry {
    pub spawns: Vec<Spawn>,
}

#[derive(Clone, PartialEq)]
pub enum Faction {
    None,
    Red,
    Bleu,
}

impl Default for Faction {
    fn default() -> Self { Faction::None }
}
