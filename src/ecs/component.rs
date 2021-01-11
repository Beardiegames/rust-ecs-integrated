
use crate::ecs::Factory;
use crate::ecs::Entity;
use crate::ecs::Component;

/// Pointer is a reference to objects in the pool, which is used to find and update these objects.
/// A Pointer can hold a reference to an object that doesn't exist anymore,
/// the exists(pointer) methode can be used to check a pointer before using it.
/// 
pub type Pointer = usize;
pub type Group = usize;

#[derive(Clone, Default)]
pub struct Spawn {
    pub pointer: Pointer,
    pub group: Group,
}

#[derive(Debug, PartialEq)]
pub enum SceneError {
    Overflow, // spawned more items than the pool can hold.
    OutOfBounds, // Pointer not within boundaries as where preset during new().
    GroupNotFound, // Group not within boundaries as where preset during new().
    FactoryNotFound, // There is no factory for this Group available
}

/// TestObject for unit testing the object pooling system, and usecase example.
/// 
#[derive(Clone, Default)]
pub struct PoolTestType (String);

impl PoolTestType {
    pub fn set(&mut self, val: &str) { self.0 = val.to_string(); }
    pub fn get(&self) -> &str { self.0.as_str() }
}

/// Object pooling system for instantating large number of data types at lower CPU cost.
/// 
pub struct Scene {
    factories: Vec<Box::<dyn Factory>>,
    pool: Vec<Entity>,
    free: Vec<Pointer>,
    in_use: Vec<Spawn>,
    groups: Vec<Vec<Pointer>>,
    itter_count: usize,
    entity_base: Vec<Box::<dyn Component>>,
}

impl Scene {

    /// Create a new Object pool manager instance.
    /// 
    /// By setting the 'size' parameter, you preset the maximum amount of 
    /// Object the new pool can hold and therefore spawn.
    /// 
    /// Setting 'num_groups' presets the maximum number of different 'group' tags 
    /// that can be passed when spawning. This tag is used for sorting through
    /// spawned objects faster.
    /// 
    pub fn new(size: usize, entity_base: Vec<Box::<dyn Component>>, factories: Vec<Box::<dyn Factory>>) -> Self {

        let mut pool: Vec<Entity> = Vec::with_capacity(size);
        let mut free: Vec<Pointer> = Vec::with_capacity(size);
        let in_use: Vec<Spawn> = Vec::with_capacity(size);
        let mut groups: Vec<Vec<Pointer>> = Vec::with_capacity(factories.len());

        for i in 0..size { 
            pool.push(Entity{ components:entity_base });
            free.push(i);
        }

        for _j in 0..factories.len() {
            groups.push(Vec::with_capacity(size));
        }

        Scene { factories, pool, free, in_use, groups, itter_count: 0, entity_base } 
    }

    pub fn update_component(&mut self, component_index: usize, spawn: &Spawn) {
        self.pool[spawn.pointer].components[component_index].update(&spawn, &mut self)
    }

    pub fn override_field(&mut self, pointer: &Pointer, value: Entity) {
            
        if pointer < &self.pool.len() {
            self.pool[*pointer] = value;
        }
    }

    pub fn get(&self, spawn: &Spawn) -> &Entity { 
        &self.pool[spawn.pointer] 
    }

    pub fn get_mut(&mut self, spawn: &Spawn) -> &mut Entity { 
        &mut self.pool[spawn.pointer] 
    }

    pub fn test_object<P> (&self, predicate: &mut P) -> Option<Spawn>
        where P: FnMut(&Entity) -> bool
    {
        for spawn in &self.in_use {
            if predicate(&self.pool[spawn.pointer]) {
                return Some(spawn.clone());
            }
        }
        return None;
    }

    pub fn test_object_ne<P> (&self, predicate: &mut P) -> Option<Spawn>
        where P: FnMut(&Entity) -> bool
    {
        for spawn in &self.in_use {
            if !predicate(&self.pool[spawn.pointer]) {
                return Some(spawn.clone());
            }
        }
        return None;
    }

    // pub fn clone_object(&self, spawn: &Spawn) -> Result<Entity, SceneError> {

    //     if spawn.pointer >= self.pool.len() { 
    //         Err(SceneError::OutOfBounds)
    //     } else {
    //         Ok(self.pool[spawn.pointer].clone())
    //     }
    // }

    pub fn list_spawns(&self) -> Vec<Spawn> {
        self.in_use.clone()
    }
    

    // pub fn clone_into_object_list(&self) -> Vec<Entity> {

    //     let mut clones = Vec::<Entity>::new();
    //     for spawn in &self.in_use {
    //         clones.push(self.pool[spawn.pointer].clone());
    //     }
    //     clones
    // }

    

    // pub fn mut_compare_all<F> (&mut self, mut on_compare: F)
    //     where F: FnMut(&mut T, &mut T) 
    // {
    //     let count = self.in_use.len();
    //     for i in 0..count {
    //         for j in 0..count {
    //             on_compare(&mut self.pool[self.in_use[i].0], &mut self.pool[self.in_use[j].0]);
    //         }
    //     }
    // }


    /// Search for Some(Pointer) to the first Object within the pool that succeeds
    /// the specified predicate. Returns 'None' if all objects fail the predicate.
    /// 
    /// # Example
    /// 
    /// ```
    /// use ecs::pool::*;
    /// // create a new pool
    /// let mut pool = Pool::<PoolTestType>::new(5);
    /// // spawn new pool object or panic an overflow error
    /// let pointer = pool.spawn().unwrap_or_else(|e| { 
    ///     panic!("Overflow ERROR, spawned too many objects"); 0 });
    /// // test a failing find
    /// assert!(pool.find(|x| x.get() == "john-doe").is_none(), "Expected to find None!");
    /// // set the object value
    /// pool.edit(&pointer, |e| e.set("john-doe"));
    /// // find pointer to Object with the previously set value or panic a not found error
    /// let findings = pool.find(|x| x.get() == "john-doe").unwrap_or_else(|| { 
    ///     panic!("Object at pointer position not found!"); 0 });
    /// // test if pointer points to correct Object
    /// pool.edit(&findings, |e| assert_eq!(e.get(), "john-doe"));
    /// ```
    /// 
    pub fn find_spawn<P> (&self, mut predicate: P) -> Option<Spawn>
        where P: FnMut(&Entity) -> bool {

        for spawn in &self.in_use { 
            if predicate(&self.pool[spawn.pointer]) {
                return Some(
                    Spawn { 
                        pointer: spawn.pointer.clone(), 
                        group: spawn.group.clone()
                    }
                );
            }
        }
        None
    }

    /// Search for Some(Pointer) to the first Object within the pool, that has a
    /// specified group tag and succeeds the specified predicate. Returns 'None' 
    /// if all objects fail the predicate.
    /// 
    /// Only check one group and, in case of many groups containing many objects,
    /// will therefore be faster than looping through all spawned objects.
    /// 
    pub fn find_in_group<P> (&self, group: Group, mut predicate: P) -> Option<Spawn>
        where P: FnMut(&Entity) -> bool {

        if group >= self.groups.len() { return None; }

        for i in &self.groups[group] { 
            if predicate(&self.pool[*i]) {
                return Some(Spawn { pointer: i.clone(), group });
            }
        }
        None
    }

    pub fn compare_against<F> (&self, against: Spawn, mut on_compare: F) -> Option<Spawn>
        where F: FnMut(&Entity, &Entity) -> bool
    {
        for inner_spawn in &self.in_use {
            if on_compare(
                &self.pool[against.pointer], 
                &self.pool[inner_spawn.pointer]
            ){
                return Some(inner_spawn.clone());
            }
        }
        None
    }

    pub fn compare_all<F> (&self, mut on_compare: F) -> Option<(Spawn, Spawn)>
        where F: FnMut(&Entity, &Entity) -> bool
    {
        for outer_spawn in &self.in_use {
            for inner_spawn in &self.in_use {
                if on_compare(
                    &self.pool[outer_spawn.pointer], 
                    &self.pool[inner_spawn.pointer]
                ){
                    return Some((outer_spawn.clone(), inner_spawn.clone()));
                }
            }
        }
        None
    }
    
    /// Activates a new Object within the pool. 
    /// 
    /// Returns Ok(Pointer) to the position of the Object in the pool or returns 
    /// Err(SceneError::Overflow) if the maximum capacity is reached and no new 
    /// Object could be spawned.
    /// 
    /// # Example
    /// 
    /// ```
    /// use ecs::pool::*;
    /// // create a new pool with a maximum of one object
    /// let mut pool = Pool::<PoolTestType>::new(1);
    /// // spawn new pool object or panic an overflow error
    /// let pointer = pool.spawn().unwrap_or_else(|e| { 
    ///     panic!("Overflow ERROR, spawned too many objects"); 0 });
    /// // test if pool obejct was activated
    /// assert!(pool.exists(&pointer));
    /// // spawn new pool object and expect an overflow error
    /// assert!(pool.spawn().is_err());
    /// ```
    /// 
    pub fn spawn(&mut self, group: Group) -> Result<Spawn, SceneError> {

        if group >= self.groups.len() {
            return Err(SceneError::GroupNotFound);
        } 

        match self.free.pop() {
            Some(pointer) => {
                self.in_use.push(Spawn { pointer, group });
                self.groups[group].push(pointer);
                self.pool[pointer] = self.factories[group].build();
                Ok(self.in_use[pointer].clone())
            },
            None => Err(SceneError::Overflow)
        }
    }

    /// Deactivate an allready active Object within the pool.
    /// 
    /// Its actual data will not be removed. Only after a respawn or a wipe() call
    /// will this data be reset to its default value.
    /// 
    /// # Example
    /// 
    /// ```
    /// use ecs::pool::*;
    /// // create a new pool of that type
    /// let mut pool = Pool::<PoolTestType>::new(5);
    /// // spawn a new object
    /// let pointer = pool.spawn().unwrap_or_else(|e| { 
    ///     panic!("Overflow ERROR, spawned too many objects"); 0 });
    /// assert!(pool.exists(&pointer)); // test if it exists
    /// pool.destroy(&pointer); // remove the object from existance
    /// assert!(!pool.exists(&pointer)); // test if object doesn't exist
    /// ```
    /// 
    pub fn destroy(&mut self, spawn: &Spawn) {
        if let Some(index) = self.in_use.iter().position(
            |x| x.pointer == spawn.pointer
        ) {
            self.in_use.remove(index);
            self.free.push(spawn.pointer)
        }
    }

    /// Reset a pool object to its default value, this can be used for all pool
    /// objects even if they are not active. This can be useful is a pool object 
    /// holds references that should be removed from the pool. Or for resetting 
    /// an active object to its start state.
    /// 
    /// This methode can be called before or after destroying an object.
    /// 
    pub fn wipe(&mut self, pointer: &Pointer) {
        self.pool[*pointer] = Entity{ components: self.entity_base }
    }

    /// Checks if the object at the Pointer position has been spawned.
    /// 
    pub fn exists(&self, spawn: &Spawn) -> bool {
        if self.in_use.len() < 1 { return false; }
        self.in_use.iter().position(|x| x.pointer == spawn.pointer).is_some()
    }

    /// Checks if the object with a specific group tag, and Pointer position 
    /// has been spawned.
    /// 
    /// Only check one group and, in case of many groups containing many objects,
    /// will therefore be faster than looping through all spawned objects.
    /// 
    pub fn exists_in_group(&self, spawn: &Spawn, group: Group) -> bool {
        if group >= self.groups.len() { return false; }
        self.groups[group].contains(&spawn.pointer)
    }

    /// Returns the maximum capacity of the pool.
    /// 
    /// # Example
    /// 
    /// ```
    /// use ecs::pool::*;
    /// // create a new pool of a specific capacity
    /// let mut pool = Pool::<PoolTestType>::new(42);
    /// // test if the pools has that capacity
    /// assert_eq!(pool.size(), 42);
    /// ```
    /// 
    pub fn size(&self) -> usize {
        self.pool.len()
    }
}

impl Iterator for Scene {
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