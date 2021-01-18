
use crate::scene::Scene;
use crate::spawn::*;

pub trait Factory<E: Entity> {
    fn group(&self) -> Group; 
    fn build(&self, spawn: &Spawn) -> E;
}

pub trait System<E: Entity> {
    fn requirements(&self, target: &E) -> bool;
    fn update(&mut self, spawn: &Spawn, scene: &mut Scene<E>);
}

pub trait Entity: Default + Clone {}

pub trait Component: Default { 
    fn set_active(&mut self, activate: bool);
    fn is_active(&self) -> &bool; 

    fn active() -> Self { 
        let mut instance = Self::default();
        instance.set_active(true); 
        instance
    }
    fn inactive() -> Self { 
        let mut instance = Self::default();
        instance.set_active(false); 
        instance
    }
}

