
use crate::ecs::{ Spawn, Factory, Group };
use crate::game::components::*;

pub enum Unit {
    Soldier,
    Truck,
}

// --

pub struct Soldier;

impl Factory<GameObject> for Soldier {

    fn group(&self) -> Group { Unit::Soldier as usize }

    fn build(&self, spawn: &Spawn) -> GameObject {
        GameObject {
            position: (0.0, 0.0),
            faction: Faction::None,

            movement: Some(Movement::new(5.0)),
            health: Some(Health::new(15)),
            assault: Some(Assault::new(Attack::Gun(spawn.clone(), 5))),
            carry: None,
        }
    }
}

// --

pub  struct Truck;

impl Factory<GameObject> for Truck {

    fn group(&self) -> Group { Unit::Truck as usize }

    fn build(&self, _spawn: &Spawn) -> GameObject {
        GameObject {
            position: (0.0, 0.0),
            faction: Faction::None,

            movement: Some(Movement::new(20.0)),
            health: Some(Health::new(50)),
            assault: None,
            carry: Some(Carry::default()),
        }
    }
}
