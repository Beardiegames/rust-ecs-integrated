
use crate::pool::Pointer;

/// ECS basics: Entityties represent objects that hold component data, 
/// entities can be updated by Systems.
/// 
#[derive(Clone, Default)]
pub struct Entity<Components: Clone + Default>
{
    name: String,
    pointer: Pointer,
    pub components: Components,
}

impl<Components: Clone + Default> Entity<Components> {

    /// Instantiate a new Entity object.
    /// This methode should only be called by the EntityPool.
    /// 
    pub fn new (
        name: String,
        pointer: Pointer,
        components: Components,
    ) -> Self {

        Entity {name, pointer, components }
    }

    /// Returns a epointer that tells us where this enitity lives 
    /// within the EntityPool.
    /// 
    pub fn pointer(&self) -> Pointer { self.pointer }

    /// Returns the name of this Entity.
    /// 
    pub fn name(&self) -> &str { self.name.as_str() }

    /// Overrides the name of this Entity.
    /// 
    pub fn change_name(&mut self, new_name: &str) {
        self.name = new_name.to_string()
    }
}