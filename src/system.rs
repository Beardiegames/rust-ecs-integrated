
//! # Example System Implementation
//! 
//! ```
//! use ecs::EventHooks;
//! use ecs::component::{ Components, ExampleComponents };
//! use ecs::pool::Pointer;
//! use ecs::system::System;
//! use std::any::Any;
//! 
//! #[derive(Clone)]
//! pub struct ExampleSystem { 
//!     pub was_called: bool, 
//! }
//!
//! impl ExampleSystem { 
//!     pub fn new () -> Self { 
//!         ExampleSystem { was_called: false, }
//!     }
//! }
//!
//! impl System<ExampleComponents> for ExampleSystem {
//! 
//!     fn update(
//!         &mut self, 
//!         entity: &Pointer, 
//!         components: &mut ExampleComponents, 
//!         events: &mut EventHooks<<ExampleComponents as Components>::Events>
//!     ) {
//!         components.value += 1;
//!         self.was_called = true;
//!     }
//!
//!     fn as_any(&mut self) -> &mut dyn Any { self }
//! }
//! ```

use crate::*;
use crate::entities::Entity;
use crate::component::*;
use crate::pool::Pointer;
use std::any::Any;


#[derive(Clone)]
pub struct ExampleSystem { 
    pub was_called: bool, 
}

impl ExampleSystem { 
    pub fn new () -> Self { 
        ExampleSystem { was_called: false, }
    }
}

impl System<ExampleComponents> for ExampleSystem {

    fn update(
        &mut self, 
        entity: &Pointer, 
        components: &mut ExampleComponents, 
        events: &mut EventHooks<<ExampleComponents as Components>::Events>
    ) {
        components.value += 1;
        self.was_called = true;
    }

    fn as_any(&mut self) -> &mut dyn Any { self }
}


pub trait System<C: Components> {

    /// During an ECS update, if this system was registered, every Entity will be
    /// passes through a Systems update function. All System updates are called 
    /// in the order they were registered (see ECS in root)
    /// 
    /// Add your own custom system script
    /// 
    fn update(&mut self, entity: &Pointer, components: &mut C, events: &mut EventHooks<C::Events>);

    /// Used for casting Systems back down to their origional type
    /// 
    /// Allthough the implementation is allways the same (see example), it must be 
    /// implemented by hand because the compiler cannot backtrace its origional
    /// type.
    /// 
    /// An easy to use downcast methode can be found in the root ECS (see ECS.get_system())
    /// which uses this Systems implementation for that exact purpose.
    /// 
    fn as_any(&mut self) -> &mut dyn Any;
}
