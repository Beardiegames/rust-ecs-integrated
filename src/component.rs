
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

#[derive(Clone, Default)]
pub struct ExampleComponents {
    pub value: u64,
}