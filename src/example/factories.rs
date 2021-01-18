
#![allow(unused_variables)]

use crate::spawns::{ Spawn, Group, };
use crate::types::{ Factory, Component };

use super::components::*;

// --

pub struct Soldier {
    group: Group,
}

impl Soldier {
    pub fn new() -> Self { Soldier { group: 0 } }

    pub fn group(&self) -> &Group { &self.group } 
}

impl Factory<GameObject> for Soldier {

    fn init(&mut self, group: Group) {
        self.group = group;
    }

    fn build(&self, spawn: &Spawn) -> GameObject {
        GameObject {
            position: Position::active(),
            agenda: Agenda::active(),
            movement: Movement::from_speed(1.0),
            health: Health::from_hp(10),
            focus: Focus::new(),
            attack: Attack {
                active: true,
                weapon: Weapon::rifle(),
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

pub struct Truck {
    group: Group,
}

impl Truck {
    pub fn new() -> Self { Truck { group: 0 } }

    pub fn group(&self) -> &Group { &self.group } 
}

impl Factory<GameObject> for Truck {

    fn init(&mut self, group: Group) {
        self.group = group;
    }

    fn build(&self, _spawn: &Spawn) -> GameObject {
        GameObject {
            position: Position::active(),
            agenda: Agenda::active(),
            movement: Movement::from_speed(2.0),
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
