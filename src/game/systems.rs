
use crate::ecs::{ Scene, Spawn, System };
use crate::game::components::*;


pub struct MoveSystem;

impl System<GameObject> for MoveSystem {

    fn update(&mut self, spawn: &Spawn, scene: &mut Scene<GameObject>) {

        if let Some(_movement) = &mut scene.get_mut(spawn).movement {
            //
        }
    }
}


pub struct AttackSystem;

impl System<GameObject> for AttackSystem {

    fn update(&mut self, spawn: &Spawn, scene: &mut Scene<GameObject>) {

        let mut object = scene.get_mut(spawn).clone();

        if object.assault.is_some() {
            
            let mut assault = object.assault.unwrap();
            let faction = &object.faction;
            let position = &object.position;
            let target = assault.target.clone();

            if assault.target.is_none() {

                assault.target = scene.find_spawn(|x| { 
                    x.faction != Faction::None
                    && x.faction != *faction
                    && position.0 - x.position.0 < 10.0
                });

            } else {

                let opponent = &mut scene.get_mut(&target.unwrap());
                let attack = assault.attack.clone();

                if let Some(health) = &mut opponent.health {
                    health.damage.push(attack)
                }
            }
            object.assault = Some(assault);
            scene.set(spawn, object);
        }
    }
}


pub struct DamageSystem;

impl System<GameObject> for DamageSystem {
    fn update(&mut self, _spawn: &Spawn, _scene: &mut Scene<GameObject>) {

    }
}
