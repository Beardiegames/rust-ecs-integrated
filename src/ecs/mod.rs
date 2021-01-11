
pub mod scene;
pub use scene::Scene;
pub use scene::Spawn;
pub use scene::Group;

pub trait Factory<T: Entity> {
    fn group(&self) -> Group; 
    fn build(&self, spawn: &Spawn) -> T;
}

pub trait System<T: Entity> {
    fn update(&mut self, spawn: &Spawn, scene: &mut Scene<T>);
}

pub trait Entity: Default + Clone {}

pub struct ECS<T: Entity> {
    scene: Scene<T>,
    systems: Vec<Box::<dyn System<T>>>,
}

impl<T: Entity> ECS<T> {

    pub fn update(&mut self) {
        for sys in &mut self.systems {
            for spawn in self.scene.list_spawns() {
                sys.update(&spawn, &mut self.scene);
            }
        }
    }
}

pub struct EcsBuilder<T: Entity> {
    pool_size: usize,
    systems: Vec<Box::<dyn System<T>>>,
    factories: Vec<Box::<dyn Factory<T>>>,
}

impl<T: Entity> EcsBuilder<T> {

    pub fn new(pool_size: usize) -> Self {
        EcsBuilder {
            pool_size,
            systems: Vec::new(),
            factories: Vec::new(),
        }
    }

    pub fn add_factory<F> (mut self, factory: F) -> Self
    where F: Factory<T> + 'static
    {
        self.factories.push(Box::new(factory));
        self
    }

    pub fn register_system<S>(mut self, system: S) -> Self 
    where S: System<T> + 'static
    {
        self.systems.push(Box::new(system));
        self
    }

    pub fn build(self) -> ECS<T> {
        ECS { 
            scene: Scene::new(self.pool_size, self.factories),
            systems: self.systems
        }
    }
}




// pub trait System<T: Entity> {
//     fn update(&mut self, spawn: &Spawn, _scene: &mut Scene<T>);
// }

// pub type Systems<T> = Vec<Box<dyn System<T>>>;


// pub struct ECS<T: Entity> {
//     pool: Scene<T>,
//     systems: Systems<T>,
// }

// impl<T: Entity> ECS<T> {

//     pub fn new(pool_size: usize, group_count: usize) -> Self {
//         ECS {
//             pool: Scene::new(pool_size, group_count),
//             systems: Systems::new(),
//         }
//     }

//     pub fn register_system(mut self, system: Box<dyn System<T>>) -> Self {
//         self.systems.push(system);
//         self
//     }

//     pub fn update(&mut self) {

//         for system in &mut self.systems {
//             for spawn in self.pool.list_spawns() {
//                 system.update(&spawn, &mut self.pool);
//             }
//         }

//         //for system in &mut self.systems {
//             // for spawn in self.pool.spawn_list() {
//             //     let mut clone: T = self.pool.clone_spawn(&spawn.pointer).unwrap();
//             //     MoveSystem::update(&mut clone, &mut self.pool);
                
//             //     //spawn.update(&mut self.pool);
//             //     //self.pool.override_field(&spawn.pointer, clone);
//             // }
//         //}
//     }
// }