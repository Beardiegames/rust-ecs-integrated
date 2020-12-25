
/// Pointer is a reference to objects in the pool, which is used to find and update these objects.
/// A Pointer can hold a reference to an object that doesn't exist anymore,
/// the exists(pointer) methode can be used to check a pointer before using it.
/// 
pub type Pointer = usize;

#[derive(Debug, PartialEq)]
pub enum PoolError {
    Overflow, // spawned more items than the pool can hold.
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
pub struct Pool <T:Clone + Default> {
    pool: Vec<T>,
    free: Vec<usize>,
    in_use: Vec<usize>,
}

impl<T:Clone + Default> Pool<T> {

    /// Create a new Object pool manager instance.
    /// 
    /// By setting the 'size' parameter, you preset the maximum amount of 
    /// Object the new pool can hold and therefore spawn.
    /// 
    pub fn new(size: usize) -> Self {

        let mut pool: Vec<T> = Vec::with_capacity(size);
        let mut free: Vec<usize> = Vec::with_capacity(size);
        let in_use: Vec<usize> = Vec::with_capacity(size);

        for i in 0..size { 
            pool.push(T::default());
            free.push(i);
        }

        Pool { pool, free, in_use } 
    }

    /// Iterates over all spawned objects and passes a mutable reference of each
    /// object through a custom callback function.
    /// 
    /// If there are no active entities, no entities will be called and therefore
    /// nothing will happen.
    /// 
    /// # Example
    /// 
    /// ```
    /// use ecs::pool::*;
    /// // create a new pool of that type
    /// let mut pool = Pool::<PoolTestType>::new(5);
    /// // spawn new pool objects or panic an overflow error
    /// pool.spawn().unwrap_or_else(|e| { 
    ///     panic!("Overflow ERROR, spawned too many objects"); 0 });
    /// pool.spawn().unwrap_or_else(|e| { 
    ///     panic!("Overflow ERROR, spawned too many objects"); 0 });
    /// // change the value of all active objects
    /// pool.edit_all(|e| e.set("john-doe"));
    /// // test if all active object have been updated
    /// pool.edit_all(|e| assert_eq!(e.get(), "john-doe")); 
    /// ```
    /// 
    pub fn edit_all<F> (&mut self, mut action: F) 
        where F: FnMut(&mut T)
    {
        for i in &self.in_use {
            action(&mut self.pool[*i]);
        }
    }

    /// Passes a mutable reference of a single Object through a custom callback function.
    /// 
    /// In order to specify the target Object a Pointer object is required.
    /// A pointer to an Object's location within the pool can be obtained by using
    /// the pointer that was returned when the Object was spawned (see spawn methode), 
    /// or by using the find methode (see find methode).
    /// 
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
    /// // change the value of target active object
    /// pool.edit(&pointer, |e| e.set("john-doe"));
    /// // test if target active object has been updated
    /// pool.edit(&pointer, |e| assert_eq!(e.get(), "john-doe"));
    /// ```
    /// 
    pub fn edit<F> (&mut self, pointer: &Pointer, mut action: F)
        where F: FnMut(&mut T) {
            
        if pointer < &self.pool.len() {
            action(&mut self.pool[*pointer]);
        }
    }

    /// Search for a Some(Pointer) to the first Object within the pool with the 
    /// specified name. Returns 'None' if no Entities whith that name where found.
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
    pub fn find<P> (&mut self, predicate: P) -> Option<Pointer>
        where P: FnMut(&T) -> bool {

        self.pool.iter().position(predicate)
    }
    
    /// Activates a new Object within the pool. 
    /// 
    /// Returns Ok(Pointer) to the position of the Object in the pool or returns 
    /// Err(PoolError::Overflow) if the maximum capacity is reached and no new 
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
    pub fn spawn(&mut self) -> Result<Pointer, PoolError> {

        match self.free.pop() {
            Some(index) => {
                self.in_use.push(index);
                self.wipe(&index);
                Ok(index)
            },
            None => Err(PoolError::Overflow)
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
    pub fn destroy(&mut self, pointer: &Pointer) {
        if let Some(index) = self.in_use.iter().position(|x| x == pointer) {
            self.in_use.remove(index);
            self.free.push(*pointer)
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
        self.pool[*pointer] = T::default();
    }

    /// Checks if the object at the Pointer position has been spawned.
    /// 
    pub fn exists(&self, pointer: &Pointer) -> bool {
        if self.in_use.len() < 1 { return false; }
        self.in_use.contains(&pointer)
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