
use crate::spawns::{ Spawn, Group, };
use crate::types::{ Factory, Component };

use super::components::*;

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
            position: Position::active(),
            agenda: Agenda::active(),
            movement: Movement::from_speed(1.0),
            health: Health::from_hp(10),
            focus: Focus::new(),
            attack: Attack {
                active: true,
                weapon: Weapon::RIFLE(),
                skill: 1,
                range: 10,
            },
            damage: Damage::new(),
            defense: Defense::from_blockers(
                vec![
                    ImpactProtection {
                        against: Impact::Bashing,
                        immunity_factor: ImmunityFactor::None,
                        reduction: 1.0,
                    },
                ]
            ),
            resist: Resist::inactive(),
            afflictions: Afflictions::inactive(),
            carry: Carry::inactive(),
        }
        
    }
}

// --

pub  struct Truck;

impl Factory<GameObject> for Truck {

    fn group(&self) -> Group { Unit::Truck as usize }

    fn build(&self, _spawn: &Spawn) -> GameObject {
        GameObject {
            position: Position::active(),
            agenda: Agenda::active(),
            movement: Movement::from_speed(5.0),
            health: Health::from_hp(30),
            focus: Focus::active(),
            attack: Attack::inactive(),
            damage: Damage::active(),
            defense: Defense::from_blockers(
                vec![
                    ImpactProtection {
                        against: Impact::Bashing,
                        immunity_factor: ImmunityFactor::None,
                        reduction: 1.0,
                    },
                ]
            ),
            resist: Resist::inactive(),
            afflictions: Afflictions::inactive(),
            carry: Carry::active(),
        }
    }
}
