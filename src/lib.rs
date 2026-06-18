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

/// Custom literals helper macros to express time values. Use in conjunction with the
/// [culit](https://crates.io/crates/culit) crate.
///
/// Example:
///
/// ```
/// #[culit(eva_rt_common::literals)]
/// fn foo() {
///     let time: Time = 10ms;
/// }
/// ```
pub mod literals {
    pub mod integer {
        #[macro_export]
        macro_rules! s { ($value:literal) => { $crate::time::Time::secs($value as f64) } }

        #[macro_export]
        macro_rules! ms { ($value:literal) => { $crate::time::Time::millis($value as f64) } }

        #[macro_export]
        macro_rules! us { ($value:literal) => { $crate::time::Time::micros($value as f64) } }

        #[macro_export]
        macro_rules! ns { ($value:literal) => { $crate::time::Time::nanos($value as f64) } }

        pub use s;
        pub use ms;
        pub use us;
        pub use ns;
    }
}

