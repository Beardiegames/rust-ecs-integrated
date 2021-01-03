
pub mod scene;
pub use scene::Scene;
pub use scene::Spawn;

pub trait System<Entity: Default + Clone> {
    fn update(&mut self, e: &mut Entity, s: &mut Scene<Entity>);
}

pub struct ECS<Entity: Default + Clone> {
    pool: Scene<Entity>,
    systems: Vec<Box<dyn System<Entity>>>,
}

impl<Entity: Default + Clone> ECS<Entity> {

    pub fn new(pool_size: usize, group_count: usize) -> Self {
        ECS {
            pool: Scene::new(pool_size, group_count),
            systems: Vec::new(),
        }
    }

    pub fn register_system(mut self, system: Box<dyn System<Entity>>) -> Self {
        self.systems.push(system);
        self
    }

    pub fn update(&mut self) {
        for system in &mut self.systems {
            for spawn in self.pool.spawn_list() {
                let mut clone = self.pool.clone_spawn(&spawn.pointer).unwrap();
                system.update(&mut clone, &mut self.pool);
                self.pool.override_field(&spawn.pointer, clone);
            }
        }
    }
}