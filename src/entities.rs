
use crate::pool::Pointer;

/// ECS basics: Entityties represent objects that hold component data, 
/// entities can be updated by Systems.
/// 
#[derive(Clone, Default)]
pub struct Entity<Components: Clone + Default>
{
    pub pointer: Pointer,
    pub components: Components,
}