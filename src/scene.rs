
use std::cell::{ RefCell, Ref, RefMut };

use crate::types::*;

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

    pub fn spawn_list(&self) -> Vec<Spawn> {
        self.spawns.clone()
    }

    pub fn get(&self, spawn: &Spawn) -> Ref<T> { 
        self.pool[spawn.pointer].borrow()
    }

    pub fn get_mut(&self, spawn: &Spawn) -> RefMut<T> { 
        self.pool[spawn.pointer].borrow_mut()
    }

    pub fn test_all_eq<P> (&self, predicate: &mut P) -> Option<Spawn>
        where P: FnMut(&T) -> bool
    {
        for spawn in &self.in_use {
            if predicate(&self.pool[spawn.pointer].borrow()) {
                return Some(self.spawns[spawn.pointer].clone());
            }
        }
        return None;
    }

    pub fn test_all_ne<P> (&self, predicate: &mut P) -> Option<Spawn>
        where P: FnMut(&T) -> bool
    {
        for spawn in &self.in_use {
            if !predicate(&self.pool[spawn.pointer].borrow()) {
                return Some(self.spawns[spawn.pointer].clone());
            }
        }
        None
    }

    pub fn find_spawn(&self, name: &str) -> Option<Spawn> {

        for spawn in &self.in_use { 
            if self.spawns[spawn.pointer].name() == name { 
                return Some(self.spawns[spawn.pointer].clone()); 
            }
        }
        None
    }

    pub fn find_spawn_in_group(&self, name: &str, group: Group) -> Option<Spawn> {

        if group >= self.groups.len() { return None; }

        for pointer in &self.groups[group] { 
            if self.spawns[*pointer].name() == name {
                return Some(self.spawns[*pointer].clone());
            }
        }
        None
    }

    pub fn search_components<P> (&self, mut predicate: P) -> Option<Spawn>
        where P: FnMut(&T) -> bool {

        for spawn in &self.in_use { 
            if predicate(&self.pool[spawn.pointer].borrow()) {
                return Some(self.spawns[spawn.pointer].clone());
            }
        }
        None
    }

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
    
    pub fn spawn(&mut self, name: &str, group: Group) -> Result<Spawn, SceneError> {

        if group >= self.groups.len() {
            return Err(SceneError::GroupNotFound);
        } 

        match self.free.pop() {
            Some(pointer) => {
                self.spawns[pointer].pointer = pointer;
                self.spawns[pointer].group = group;
                self.spawns[pointer].new_name(name);
                
                self.in_use.push(self.spawns[pointer].clone());
                self.groups[group].push(pointer);
                self.pool[pointer].replace(self.factories[group].build(&self.spawns[pointer]));

                Ok(self.spawns[pointer].clone())
            },
            None => Err(SceneError::Overflow)
        }
    }

    /// NOTE: Destroy is slow
    pub fn destroy(&mut self, spawn: &Spawn) {
        if let Some(u_index) = self.in_use.iter().position(
            |x| x.pointer == spawn.pointer
        ) {
            if let Some(g_index) = self.groups[spawn.group].iter().position(
                |x| *x == spawn.pointer
            ) {
                self.in_use.remove(g_index);
            }

            self.in_use.remove(u_index);
            self.free.push(spawn.pointer)
        }
    }

    pub fn wipe(&mut self, pointer: &Pointer) {
        self.pool[*pointer].replace(T::default());
    }

    /// Checks if the object at the Pointer position has been spawned.
    /// 
    pub fn exists(&self, spawn: &Spawn) -> bool {
        self.in_use.contains(spawn)
    }

    /// Checks if the object with a specific group tag, and Pointer position 
    /// has been spawned.
    /// 
    /// Only check one group and, in case of many groups containing many objects,
    /// will therefore be faster than looping through all spawned objects.
    /// 
    pub fn exists_in_group(&self, spawn: &Spawn, group: Group) -> bool {
        self.groups[group].contains(spawn.pointer())
    }

    /// Returns the maximum capacity of the pool.
    /// 
    /// # Example
    /// 
    /// ```
    /// use ecs::pool::*;
    /// let mut pool = Pool::<PoolTestType>::new(42);
    /// assert_eq!(pool.size(), 42); // test if the pools has that capacity
    /// ```
    /// 
    pub fn size(&self) -> usize {
        self.pool.len()
    }
}

impl<T: Entity> Iterator for Scene<T> {
    type Item = Spawn;
    fn next(&mut self) -> Option<Self::Item> {
        self.itter_count += 1;
        if self.itter_count < self.in_use.len() {
            Some(self.in_use[self.itter_count].clone())
        } else {
            self.itter_count = 0;
            None
        }
    }
}