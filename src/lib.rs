
//! Classic polling based ecs system
//! 
//! 
//! # Example usage
//! 
//! ```
//! use ecs::ECS;
//! use ecs::component::ExampleComponents;
//! use ecs::system::ExampleSystem;
//! 
//! // create new ecs object
//! let mut ecs = ECS::<ExampleComponents>::new(100);
//! // register systems
//! let mut system_pointer = ecs.register_system(ExampleSystem::new());
//! // spawn entities
//! let mut entity_pointer = ecs.spawn_entity().unwrap(); 
//! 
//! ecs.update(); // update all systems registered.
//! 
//! // test if the update methode on our System has been called.
//! assert!(ecs.get_system::<ExampleSystem>(system_pointer).unwrap().was_called); 
//! // test if our Entity has been updated.
//! ecs.get_entity(&entity_pointer, |c| assert_eq!(c.value, 1));
//! ```
//! 

/// Entity object and pooling module. This is where all Entities are managed.
pub mod entities;
use entities::*;

/// The system Trait and implemention example.
pub mod system;
use system::*;

/// Contains an Components example.
pub mod component;
use component::*;

/// Module for object pooling
pub mod pool;
use pool::*;


/// SystemPointer is used to locate registered Systems within the ECS module.
type SystemPointer = usize;

/// ECS manages registered Systems and controls the Pool which manages all Entities.
///
pub struct ECS<Components: Clone + Default> {
    systems: Vec<Box<dyn System<Components>>>,
    entities: Pool<Entity<Components>>,
}

impl<Components: Clone + Default> ECS<Components> {

    /// Instantiate a new ECS manager
    /// 
    /// By setting the 'capacity' parameter, you preset the maximum amount of 
    /// Entities the new pool can hold and therefore spawn.
    ///  
    /// In order to create a preset list of unused Entities, for later use, 
    /// we need an 'empty' template components object.
    /// 
    pub fn new (entity_capacity: usize) -> Self 
    {
        ECS {
            systems: Vec::new(),
            entities: Pool::new(entity_capacity),
        }
    }

    /// Register systems for later update. These Systems will be updated in the 
    /// order they where registered.
    /// 
    /// Returns a SystemPointer that can be used for requesting a mutable reference
    /// of that System (see get_system()). This is usefull because ownership of 
    /// the System is handed over to the ECS manager.
    /// 
    pub fn register_system<S> (&mut self, system: S) -> SystemPointer
        where S: System<Components> + 'static
    {
        self.systems.push(Box::new(system));
        self.systems.len() - 1
    }

    /// Returns Some mutable reference to a systems origional type, or None if the
    /// specified type was not found.
    /// 
    /// In order to lookup a particular System, a SystemPointer is required. A Systems
    /// SystemPointer is obtained when registering a System (see register_system()).
    /// 
    /// USECASE: ecs.get_system::<ExampleSystem>(system_pointer);
    /// 
    pub fn get_system<BaseType: 'static> (&mut self, sys_pointer: SystemPointer) -> Option<&mut BaseType> {
        self.systems[sys_pointer].as_any().downcast_mut::<BaseType>()
    }

    /// Loop through all entities and pass them through a callback funtion
    /// 
    pub fn get_entities<F> (&mut self, mut action: F) 
        where F: FnMut(&Pointer, &mut Components)
    {
        self.entities.edit_all(|e| action(&e.pointer(), &mut e.components));
    }

    /// Read or write an entity by passing it through a callback function
    /// 
    pub fn get_entity<F> (&mut self, entity: &Pointer, mut action: F) 
        where F: FnMut(&mut Components)
    {
        self.entities.edit(entity, |e| action(&mut e.components));
    }

    /// Spawn a new entity
    /// 
    /// Returns an Ok(Pointer) that hold a reference to the 
    /// entity within the entity pool, or returns a Err(&str) if the max entity 
    /// capacity has been reached and no new entities could be spawned.
    /// 
    pub fn spawn_entity (&mut self) -> Result<Pointer, &str> {
        self.entities.spawn().map_err(
            |_e| "Unable to spawn new entities, maximum poolsize reached!"
        )
    }

    /// Remove an allready spawned entity
    /// 
    /// This does not remove any data the entity might hold (see wipe_entity for
    /// more information), it is only removed from the update list to maintain
    /// performance.
    /// 
    pub fn destroy_entity (&mut self, entity: &Pointer) {
        self.entities.destroy(entity)
    }

    /// Reset an entities data to default even if an entity has allready been 
    /// destroyed (see destroy_entity)
    /// 
    /// This might come in handy if entities data hold references to callback 
    /// methodes that could affect the codebase in a nagative way.
    /// 
    pub fn wipe_entity (&mut self, entity: &Pointer) {
        self.entities.wipe(entity)
    }

    /// Update all registered systems
    /// 
    /// Put this inside a loop to update your Systems every frame.
    /// 
    pub fn update(&mut self) {
        for system in &mut self.systems {
            self.entities.edit_all(|e| system.update(&e.pointer(), &mut e.components)); 
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::any::Any;

    // mockups

    struct AssertSystem {
        expected_pointer: Pointer,
        expected_value: u64,
        is_called: bool,
    }
    impl System<ExampleComponents> for AssertSystem {
        fn update (
            &mut self, 
            entity: &Pointer,
            components: &mut ExampleComponents
        ){
            assert_eq!(*entity, self.expected_pointer, "AssertSystem -- assert pointer failed!");
            assert_eq!(components.value, self.expected_value, "AssertSystem -- assert components.value failed!");
            self.is_called = true;
        }

        fn as_any(&mut self) -> &mut dyn Any {
            self
        }
    }

    struct AddSystem;
    impl System<ExampleComponents> for AddSystem {
        fn update (
            &mut self, 
            entity: &Pointer,
            components: &mut ExampleComponents
        ) {
            components.value += 1;
        }

        fn as_any(&mut self) -> &mut dyn Any { self }
    }

    struct SubtractSystem;
    impl System<ExampleComponents> for SubtractSystem {
        fn update (
            &mut self, 
            entity: &Pointer,
            components: &mut ExampleComponents
        ) {
            if components.value > 0 {
                components.value -= 1;
            }
        }

        fn as_any(&mut self) -> &mut dyn Any { self }
    }

    /// unit tests

    #[test]
    fn create_new_ecs() {
        let mut ecs = ECS::<ExampleComponents>::new(100);
    }

    #[test]
    fn register_system() {
        let add_system = AddSystem;
        let sub_system = SubtractSystem;

        let mut ecs = ECS::<ExampleComponents>::new(100);
        ecs.register_system(add_system);
        ecs.register_system(sub_system);
        
        assert_eq! (ecs.systems.len(), 2);
    }

    #[test]
    fn spawn_and_update_entity() {
        let assert_sys = AssertSystem {
            expected_pointer: 123,
            expected_value: 0,
            is_called: false,
        };

        let mut ecs = ECS::<ExampleComponents>::new(1);
        let sys_pointer = ecs.register_system(assert_sys);

        let spawn1 = ecs.spawn_entity();
        assert_eq!(spawn1.is_err(), false, "Spawn should return a pointer");
        let pointer = spawn1.unwrap();

        match ecs.get_system::<AssertSystem>(sys_pointer) {
            Some(sys) => {
                sys.expected_pointer = pointer;
                assert!(!sys.is_called);
            },
            None => assert!(false, "unable to downcast!"),
        }

        ecs.update();

        match ecs.get_system::<AssertSystem>(sys_pointer) {
            Some(sys) => assert!(sys.is_called),
            None => assert!(false, "unable to downcast!"),
        }

        assert!(ecs.spawn_entity().is_err(), "Spawn should return an overflow error");
    }

    #[test]
    fn system_update_order() {

        let mut ecs1 = ECS::<ExampleComponents>::new(100);
        let system1 = ecs1.register_system(AddSystem);
        let entity1 = ecs1.spawn_entity().unwrap();
        
        let mut ecs2 = ECS::<ExampleComponents>::new(100);
        let system2 = ecs2.register_system(AddSystem);
        let system3 = ecs2.register_system(SubtractSystem);
        let entity2 = ecs2.spawn_entity().unwrap();

        ecs1.get_entity(&entity1, |c| assert_eq!(c.value, 0));
        ecs1.update();
        ecs1.get_entity(&entity1, |c| assert_eq!(c.value, 1));

        ecs2.get_entity(&entity2, |c| assert_eq!(c.value, 0));
        ecs2.update(); 
        ecs2.get_entity(&entity2, |c| assert_eq!(c.value, 0));
    }

    #[test]
    fn test_speed() {
        let mut ecs = ECS::<ExampleComponents>::new(1);
        ecs.register_system(AddSystem);
        let entity_pointer = ecs.spawn_entity().unwrap();

        let num_calls = 7_200_000;  // <-- number of calls per frame
        let fps = 30;               // <-- number of frames per second
        let max = std::time::Duration::from_millis(1000 / fps);

        for test_run in 1..fps+1 {

            let mut _itt = 0;
            let now = std::time::SystemTime::now();

            for _itt in 0..num_calls { ecs.update(); }

            let elapsed = now.elapsed().unwrap();

            let u_str = match num_calls > 999_999 {
                true => format!("{} Milion", num_calls / 1_000_000),
                false => match num_calls > 999 {
                    true => format!("{} Thousand", num_calls / 1_000),
                    false => format!("{}", num_calls),
                },
            };

            assert!(elapsed <= max, 
                "Duration of {} system update calls was {:?}.
It shoud be under {:?} or {} frames x {} updates per second,
Failed in test run number {} out of {}
Try testing in release mode: cargo test --release", 
                u_str, elapsed, max, fps, u_str, test_run, fps
            );
        }

        ecs.get_entity(&entity_pointer, |c| assert_eq!(c.value, num_calls*fps));
    }
}