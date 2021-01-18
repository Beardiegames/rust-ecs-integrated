
use std::cell::{ RefCell, Ref, RefMut };

use crate::types::*;
use crate::spawns::*;

/// Pointer is a reference to objects in the scene, which is used to find and update these objects.
/// A Pointer can hold a reference to an object that doesn't exist anymore,
/// the exists(pointer) methode can be used to check a pointer before using it.
/// 
pub type Pointer = usize;


#[derive(Debug, PartialEq)]
pub enum SceneError {
    Overflow, // spawned more items than the pool can hold.
    OutOfBounds, // Pointer not within boundaries as where preset during new().
    GroupNotFound, // Group not within boundaries as where preset during new().
    FactoryNotFound, // There is no factory for this Group available
}

/// Scene is basically a manager for all entities and where to find them.
/// It uses object pooling by instantiating a fixed number of entities at startup,
/// in order to maintain a decent render speeds when creating and destoying entities
/// during the game.
/// Scene also provides tools for searching and borrowing spawned entities.
/// 
pub struct Scene<T: Entity> {
    factories: Vec<Box::<dyn Factory<T>>>,
    pool: Vec<RefCell<T>>,
    spawns: Vec<Spawn>,
    free: Vec<Pointer>,
    in_use: Vec<Spawn>,
    groups: Vec<Vec<Pointer>>,
    itter_count: usize,
}

impl<T: Entity> Scene<T>  {

    /// Create a new Scene entity manager instance.
    /// 
    /// By setting the 'size' parameter, you preset the maximum amount of 
    /// Object the new pool can hold and therefore spawn.
    /// 
    /// Factories are custom object factories that implement the Factory trait
    /// which can be called upon, by the Scene, when spawning a new object. 
    /// Factories give the user the freedom to create different types of 
    /// objects by customizing the output of these factories.
    /// 
    pub fn new(size: usize, factories: Vec<Box::<dyn Factory<T>>>) -> Self {

        let mut pool: Vec<RefCell<T>> = Vec::new();
        pool.resize_with(size, || { RefCell::new(T::default()) });

        let mut spawns: Vec<Spawn> = Vec::new();
        spawns.resize_with(size, Spawn::default);
        
        let mut free: Vec<Pointer> = Vec::with_capacity(size);
        let in_use: Vec<Spawn> = Vec::with_capacity(size);

        let mut groups: Vec<Vec<Pointer>> = Vec::new();
        groups.resize_with(factories.len(), Vec::new);

        for i in 0..size { 
            spawns[i].pointer = i; 
            free.push(i);
        }

        for group in &mut groups {
            group.resize_with(size, Pointer::default);
        }

        Scene { factories, pool, spawns, free, in_use, groups, itter_count: 0, } 
    }

    pub fn get_factory(&self, group: &Group) -> &Box::<dyn Factory<T>> {
        &self.factories[*group]
    }

    pub fn mut_factory(&mut self, group: &Group) -> &mut Box::<dyn Factory<T>> {
        &mut self.factories[*group]
    }

    /// Returns a cloned list of spawn currently in use.
    /// 
    pub fn list_spawned(&self) -> Vec<Spawn> {
        self.in_use.clone()
    }

    /// Returns a reference to a RefCell box containing the requested object.
    /// If the spawned object has been destroyed the inactive object will still be returned.
    /// You can use the methodes exists and exists_in_group to find out if objects are currently active.
    /// 
    pub fn get_ref(&self, spawn: &Spawn) -> Ref<T> { 
        self.pool[spawn.pointer].borrow()
    }

    /// Same as the get_ref methode but returns a mutable reference.
    /// 
    pub fn get_mut(&self, spawn: &Spawn) -> RefMut<T> { 
        self.pool[spawn.pointer].borrow_mut()
    }

    /// Run a custom test that tells if all active (spawned) objects comply to the predicate specified.
    /// 
    pub fn test_all<P> (&self, predicate: &mut P) -> bool
        where P: FnMut(&T) -> bool
    {
        for spawn in &self.in_use {
            if !predicate(&self.pool[spawn.pointer].borrow()) {
                return false;
            }
        }
        true
    }

    /// Find an active (spawned) object by its spawn name.
    /// 
    pub fn find_spawn(&self, name: &str) -> Option<Spawn> {

        for spawn in &self.in_use { 
            if self.spawns[spawn.pointer].name() == name { 
                return Some(self.spawns[spawn.pointer].clone()); 
            }
        }
        None
    }

    /// Find an active (spawned) object by its spawn name and factory group.
    /// This methode can be faster as find_spawn, when there are multiple groups, sinds it does not need to itterate over all objects.
    /// Find_in_group can also come in handy when using the same name in different groups (sinds spawn names do not need to be unique).
    /// 
    pub fn find_spawn_in_group(&self, name: &str, group: Group) -> Option<Spawn> {

        if group >= self.groups.len() { return None; }

        for pointer in &self.groups[group] { 
            if self.spawns[*pointer].name() == name {
                return Some(self.spawns[*pointer].clone());
            }
        }
        None
    }

    /// As find_spawn, but lets you write a custom predicate using object values.
    /// 
    pub fn search_components<P> (&self, mut predicate: P) -> Option<Spawn>
        where P: FnMut(&T) -> bool {

        for spawn in &self.in_use { 
            if predicate(&self.pool[spawn.pointer].borrow()) {
                return Some(self.spawns[spawn.pointer].clone());
            }
        }
        None
    }

    /// As find_spawn_in_group, but lets you write a custom predicate using object values.
    /// 
    pub fn search_components_in_group<P> (&self, group: Group, mut predicate: P) -> Option<Spawn>
        where P: FnMut(&T) -> bool {

        if group >= self.groups.len() { return None; }

        for pointer in &self.groups[group] { 
            if predicate(&self.pool[*pointer].borrow()) {
                return Some(self.spawns[*pointer].clone());
            }
        }
        None
    }

    /// Compare an objects values, with the values of all other objects. 
    /// Returns an Option of the Spawn on which the predicate succeeded first, or None is all comparisons failed.
    /// 
    pub fn compare_against<F> (&self, against: Spawn, mut on_compare: F) -> Option<Spawn>
        where F: FnMut(&T, &T) -> bool
    {
        for spawn in &self.in_use {
            if on_compare(
                &self.pool[against.pointer].borrow(), 
                &self.pool[spawn.pointer].borrow()
            ){
                return Some(spawn.clone());
            }
        }
        None
    }

    /// Compares all values of all objects to eachother.
    /// Returns an Option of the two Spawn on which the predicate succeeded first, or None is all comparisons failed.
    /// 
    pub fn compare_all<F> (&self, mut on_compare: F) -> Option<(Spawn, Spawn)>
        where F: FnMut(&T, &T) -> bool
    {
        for spawn_a in &self.in_use {
            for spawn_b in &self.in_use {
                if spawn_a == spawn_b { continue; }

                else if on_compare(
                    &self.pool[spawn_a.pointer].borrow(), 
                    &self.pool[spawn_b.pointer].borrow()
                ){
                    return Some( (spawn_a.clone(), spawn_b.clone()) );
                }
            }
        }
        None
    }
    
    /// Spawn a new object. Spawned objects are updated every frame by the core ECS system.
    /// The spawn methode activates a new object that will inherit all the settings of the factory of the corresponding group. 
    /// A name must be added to the spawn, this can be used to find the spawn if necessary.
    /// 
    pub fn spawn(&mut self, name: &str, group: &Group) -> Result<Spawn, SceneError> {

        if *group >= self.groups.len() {
            return Err(SceneError::GroupNotFound);
        } 

        match self.free.pop() {
            Some(pointer) => {
                self.spawns[pointer].pointer = pointer;
                self.spawns[pointer].group = group.clone();
                self.spawns[pointer].new_name(name);
                
                self.in_use.push(self.spawns[pointer].clone());
                self.groups[*group].push(pointer);
                self.pool[pointer].replace(self.factories[*group].build(&self.spawns[pointer]));

                Ok(self.spawns[pointer].clone())
            },
            None => Err(SceneError::Overflow)
        }
    }

    /// Destroy an object. Destroy deactivates an object and therefore stops it from being updated by the core ECS system.
    /// 
    /// NOTE: Destroy is slow
    pub fn destroy(&mut self, spawn: &Spawn) {
        if let Some(u_index) = self.in_use.iter().position(
            |x| x.pointer == spawn.pointer
        ) {
            if let Some(g_index) = self.groups[spawn.group].iter().position(
                |x| *x == spawn.pointer
            ) {
                self.groups[spawn.group].remove(g_index);
            }

            self.in_use.remove(u_index);
            self.free.push(spawn.pointer)
        }
    }

    pub fn wipe(&mut self, pointer: &Pointer) {
        self.pool[*pointer].replace(T::default());
    }

    /// Checks if the object at the Pointer position has been spawned (is active).
    /// 
    pub fn exists(&self, spawn: &Spawn) -> bool {
        self.in_use.contains(spawn)
    }

    /// Checks if the object with a specific group tag, and Pointer position 
    /// has been spawned (is active).
    /// 
    /// Only check one group and, in case of many groups containing many objects,
    /// will therefore be faster than looping through all spawned objects.
    /// 
    pub fn exists_in_group(&self, spawn: &Spawn, group: Group) -> bool {
        self.groups[group].contains(spawn.pointer())
    }

    /// Returns the maximum capacity of the pool.
    /// 
    pub fn size(&self) -> usize {
        self.pool.len()
    }
}