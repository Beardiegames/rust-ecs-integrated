
use crate::game_objects::*;

#[derive(Clone)]
pub enum Types {
    // terrain
    Grass,
    Water,
    Bush,
    Hill,
    Pit,

    // structures
    Wall,
    CentryGun,
    WeaponsFactory,

    // units
    Soldier,
    Truck,
}

impl Default for Types {
    fn default() -> Self { Types::Grass }
}
