
pub mod systems;
pub mod game_objects;
pub mod ecs;

use ecs::{ ECS, System, Scene, Spawn };
use systems::{ MoveSystem, AttackSystem, DamageSystem };

#[test]
fn main() {
    let mut ecs = ECS::new(1000, 100)
        .register_system(Box::new(MoveSystem))
        .register_system(Box::new(AttackSystem))
        .register_system(Box::new(DamageSystem));
    
    loop {
        ecs.update();
    }
}