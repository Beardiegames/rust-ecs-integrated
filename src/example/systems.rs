
#![allow(unused_variables)]

use crate::scene::Scene;
use crate::spawns::Spawn;
use crate::types::System;

use super::components::*;


pub struct MoveSystem;

impl System<GameObject> for MoveSystem {

    fn requirements(&self, target: &GameObject) -> bool {
        target.has_position()
        && target.has_movement()
    }

    fn update(&mut self, spawn: &Spawn, scene: &mut Scene<GameObject>) {
        let target = &mut scene.get_mut(spawn);
        target.position.x += *target.movement.speed() as f64;
    }
}


pub struct AttackSystem;

impl System<GameObject> for AttackSystem {

    fn requirements(&self, target: &GameObject) -> bool {
        target.has_position()
        && target.has_focus()
        && target.has_attack()
        && target.has_agenda()
    }

    fn update(&mut self, spawn: &Spawn, scene: &mut Scene<GameObject>) {
        let target = &mut scene.get_mut(spawn);

        // if target has a focus, than attack the first focus
        if let Some(other_spawn) = target.focus.prime() {
            
            let opponent = &mut scene.get_mut(other_spawn);

            if opponent.has_health() {
                opponent.damage.take_damage(target.attack.clone());
            }
        
        // if target doesn't have a focus find and add a new one
        } else {

            if let Some(spawn) = scene.search_components(|other| {
                other.has_damage() 
                && target.agenda.faction.opposing(&other.agenda.faction)
                && target.position.distance(&other.position) < 10.0
            }) {
                target.focus.add(&spawn);
            }
        }
    }
}


pub struct DamageSystem;

impl System<GameObject> for DamageSystem {

    fn requirements(&self, target: &GameObject) -> bool {
        target.has_health()
        && target.has_damage()
    }

    fn update(&mut self, spawn: &Spawn, scene: &mut Scene<GameObject>) {
        let target = &mut scene.get_mut(spawn);

        for attack in target.damage.clone() {
            let power = match target.has_defense() {
                true => target.defense.resolve_attack(&attack),
                false => attack.power(),
            };
            target.health.damage(power)
        }
    }
}
