//! # eva-rt-common
//! 
//! *eva-rt-common* is just a set of struct, traits and reusable functions used to develop real-time
//! analysis tools.
//! 
//! It was initially part of [eva-rt-engine](https://github.com/Yurand2000/eva-rt-engine), but has
//! now been separated to develop other real-time analysis tools.
//! 
//! 

/// Prelude module with commonly used exports.
pub mod prelude {
    pub use super::time::prelude::*;
    pub use super::rt_task::prelude::*;
}

pub mod time;
pub mod rt_task;
pub mod utils;