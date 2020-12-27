
//! Components are data objects that live inside Entities. This data is used by 
//! Systems to perform per Object actions, based on that Entities Components.
//! 
//! Components must have a Components Trait implementation in order to be able
//! to handle events
//! 
//! # Example Components Usage
//! 
//! ```
//! use ecs::component::Components;
//! 
//! #[derive(Clone)]
//! pub enum MyEvents {
//!     Damage(f32),
//!     Heal(f32),
//!     Say(String),
//! }
//! 
//! #[derive(Clone, Default)]
//! pub struct MyComponents { 
//!     pub health: f32, 
//! }
//! 
//! impl Components for MyComponents { 
//!     type Events = MyEvents;
//!
//!     fn event_handler(&mut self, event: Self::Events) {
//!         // handle incoming events here
//!         match event {
//!             MyEvents::Damage(hp) => self.health -= hp,
//!             MyEvents::Heal(hp) => self.health += hp,
//!             MyEvents::Say(msg) => println!("message: {}", msg),
//!         }
//!     }
//! }
//! ```

use crate::pool::Pointer;
use crate::events::ExampleEvents;

pub trait Components: Clone + Default {
    type Events: Clone;

    fn event_handler(&mut self, event: Self::Events, from: Pointer);
}

#[derive(Clone, Default)]
pub struct ExampleComponents {
    pub value: u64,
}

impl Components for ExampleComponents {
    type Events = ExampleEvents;

    fn event_handler(&mut self, event: Self::Events, from: Pointer) {
        // handle incoming events here
        match event {
            ExampleEvents::Damage(hp) => self.value -= hp as u64,
            ExampleEvents::Heal(hp) => self.value += hp as u64,
            ExampleEvents::Say(msg) => println!("message: {}", msg),
        }
    }
}