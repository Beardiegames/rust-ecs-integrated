
use crate::ecs::{ System, Scene };
use crate::game_objects::*;

pub struct MoveSystem;

impl System<GameObject> for MoveSystem {
    fn update(&mut self, e: &mut GameObject, _s: &mut Scene<GameObject>) {
        match e.my_type {
            Types::Soldier => Soldier::movement(e),
            Types::Truck => Truck::movement(e),
            _ => {},
        }
    }
}

pub struct AttackSystem;

impl System<GameObject> for AttackSystem {
    fn update(&mut self, e: &mut GameObject, s: &mut Scene<GameObject>) {
        match e.my_type {
            Types::Soldier => Soldier::attack(e, s),
            _ => {},
        }
    }
}

pub struct DamageSystem;

impl System<GameObject> for DamageSystem {
    fn update(&mut self, e: &mut GameObject, s: &mut Scene<GameObject>) {
        if let Some(spawn) = &e.target {
            let target = s.get_mut(&spawn.pointer);
            match e.my_type {
                Types::Soldier => Soldier::take_damage(target, &e.weapon),
                Types::Truck => Truck::take_damage(target, &e.weapon),
                _ => {},
            }
        }
    }
}