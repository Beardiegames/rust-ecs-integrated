
pub mod game;
pub mod ecs;

#[cfg(test)]
mod tests {

    use super::ecs::EcsBuilder;
    use super::game::factories::*;
    use super::game::systems::*;

    #[test]
    fn update() {
        let mut ecs = EcsBuilder::new(1000)
            .add_factory(Soldier)
            .add_factory(Truck)
            .register_system(MoveSystem)
            .register_system(AttackSystem)
            .register_system(DamageSystem)
            .build();
        
        //loop {
            ecs.update();
        //}
    }


    #[test]
    fn components() {

    }
}