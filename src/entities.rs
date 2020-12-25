
use crate::pool::Pointer;

/// ECS basics: Entityties represent objects that hold component data, 
/// entities can be updated by Systems.
/// 
#[derive(Clone, Default)]
pub struct Entity<Components: Clone + Default>
{
    pointer: Pointer,
    pub components: Components,
}

impl<Components: Clone + Default> Entity<Components> {

    /// Instantiate a new Entity object.
    /// This methode should only be called by the EntityPool.
    /// 
    pub fn new (
        pointer: Pointer,
        components: Components,
    ) -> Self {

        Entity { pointer, components }
    }

    /// Returns a epointer that tells us where this enitity lives 
    /// within the EntityPool.
    /// 
    pub fn pointer(&self) -> Pointer { self.pointer }
}