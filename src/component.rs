
//! Components are data objects that live inside Entities. This data is used by 
//! Systems to perform per Object actions, based on that Entities Components.
//! 
//! # Example Components Usage
//! 
//! ```
//! #[derive(Clone)]
//! pub struct ExampleComponents { 
//!     pub value: u64, 
//! }
//! 
//! impl ExampleComponents { 
//!     pub fn new () -> Self { 
//!         ExampleComponents { value: 0, }
//!     }
//! }
//! ```

use crate::pool::Pool;


#[derive(Clone)]
pub enum ExampleEvents {
    Damage(f32),
    Heal(f32),
    Say(String),
}

pub trait Components: Clone + Default {
    type Events: Clone;

    fn event_handler(&mut self, event: Self::Events);
}

#[derive(Clone, Default)]
pub struct ExampleComponents {
    pub value: u64,
}

impl Components for ExampleComponents {
    type Events = ExampleEvents;

    fn event_handler(&mut self, event: Self::Events) {
        // handle incoming events
    }
}