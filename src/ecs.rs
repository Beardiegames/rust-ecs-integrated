
use crate::scene;
pub use scene::Scene;
pub use scene::Spawn;
pub use scene::Group;

pub struct ECS<E: Entity> {
    scene: Scene<E>,
    systems: Vec<Box::<dyn System<E>>>,
}

impl<E: Entity> ECS<E> {

    pub fn update(&mut self) {
        for sys in &mut self.systems {
            for spawn in self.scene.spawn_list() {
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

    pub fn build(self) -> ECS<E> {
        ECS { 
            scene: Scene::new(self.pool_size, self.factories),
            systems: self.systems
        }
    }
}