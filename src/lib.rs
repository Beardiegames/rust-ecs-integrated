
pub mod scene;
pub mod types;
pub mod spawns;

pub mod example;

pub use crate::scene::*;
pub use crate::types::*;


pub struct Ecs<E: Entity> {
    scene: Scene<E>,
    systems: Vec<Box::<dyn System<E>>>,
}

impl<E: Entity> Ecs<E> {

    pub fn update(&mut self) {
        for sys in &mut self.systems {
            for spawn in self.scene.list_spawned() {
                if sys.requirements(&self.scene.get_mut(&spawn)) {
                    sys.update(&spawn, &mut self.scene);
                }
            }
        }
    }
}

pub struct EcsBuilder<E: Entity> {
    pool_size: usize,
    systems: Vec<Box::<dyn System<E>>>,
    factories: Vec<Box::<dyn Factory<E>>>,
}

impl<E: Entity> EcsBuilder<E> {

    pub fn new(pool_size: usize) -> Self {
        EcsBuilder {
            pool_size,
            systems: Vec::new(),
            factories: Vec::new(),
        }
    }

    pub fn add_factory<F> (mut self, factory: F) -> Self
    where F: Factory<E> + 'static
    {
        self.factories.push(Box::new(factory));
        self
    }

    pub fn register_system<S>(mut self, system: S) -> Self 
    where S: System<E> + 'static
    {
        self.systems.push(Box::new(system));
        self
    }

    pub fn build(mut self) -> Ecs<E> {
        for i in 0..self.factories.len() { 
            self.factories[i].init(i); 
        }
        Ecs { 
            scene: Scene::new(self.pool_size, self.factories),
            systems: self.systems
        }
    }
}


#[cfg(test)]
mod tests {

    use super::EcsBuilder;
    use super::example::factories::*;
    use super::example::systems::*;


    #[test]
    fn spawn_and_update() {
        let soldiers = Soldier::new();
        let trucks = Truck::new();

        let mut ecs = EcsBuilder::new(1000)
            .add_factory(soldiers)
            .add_factory(trucks)
            .register_system(MoveSystem)
            .build();
        
        let soldier = ecs.scene.spawn("Private first", &0).unwrap();
        let truck = ecs.scene.spawn("Demo truck", &1).unwrap();

        assert_eq!(ecs.scene.get_ref(&soldier).position.x, 0.0);
        assert_eq!(ecs.scene.get_ref(&truck).position.x, 0.0);

        ecs.update();

        assert_eq!(ecs.scene.get_ref(&soldier).position.x, 1.0);
        assert_eq!(ecs.scene.get_ref(&truck).position.x, 2.0);

        ecs.update();

        assert_eq!(ecs.scene.get_ref(&soldier).position.x, 2.0);
        assert_eq!(ecs.scene.get_ref(&truck).position.x, 4.0);
    }


    #[test]
    fn destroy() {
        let soldiers = Soldier::new();
        let trucks = Truck::new();

        let mut ecs = EcsBuilder::new(1000)
            .add_factory(soldiers)
            .add_factory(trucks)
            .register_system(MoveSystem)
            .build();
        
        let soldier = ecs.scene.spawn("Private first", &0).unwrap();
        let truck = ecs.scene.spawn("Demo truck", &1).unwrap();

        assert_eq!(ecs.scene.exists(&soldier), true);
        assert_eq!(ecs.scene.exists(&truck), true);

        ecs.update();

        assert_eq!(ecs.scene.get_ref(&soldier).position.x, 1.0);
        assert_eq!(ecs.scene.get_ref(&truck).position.x, 2.0);

        ecs.scene.destroy(&soldier);

        assert_eq!(ecs.scene.exists(&soldier), false);
        assert_eq!(ecs.scene.exists(&truck), true);

        ecs.update();

        assert_eq!(ecs.scene.get_ref(&soldier).position.x, 1.0);
        assert_eq!(ecs.scene.get_ref(&truck).position.x, 4.0);
    }

    #[test]
    fn speed() {
        let soldiers = Soldier::new();

        let mut ecs = EcsBuilder::new(1000)
            .add_factory(soldiers)
            .register_system(MoveSystem)
            .build();
        
        let soldier = ecs.scene.spawn("Private first", &0).unwrap();

        assert_eq!(ecs.scene.get_ref(&soldier).position.x, 0.0);


        for num_tries in 0..5 {

            let updates_per_second: u128 = 36_000_000;

            let now = std::time::SystemTime::now();
            for _i in 0..updates_per_second { ecs.update(); }
            let elapsed = now.elapsed().unwrap().as_millis();

            assert!(elapsed < 1_000, "duration should be shorter than 1 second, elapsed time was '{}' seconds", elapsed as f64 / 1_000.0);
            assert_eq!(ecs.scene.get_ref(&soldier).position.x, (num_tries as f64 + 1.0) * updates_per_second as f64);
        }
    }
}