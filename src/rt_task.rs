//! RTTask struct.
//! 
//! This module defines the `RTTask` struct, which describes a real-time task
//! following the standard Liu-Layland model. Real-Time tasks are here
//! characterized by **Worst Case Execution Time** (WCET), **Relative Deadline**
//! and **(Minimum Inter-arrival) Period**.

use crate::prelude::*;

pub mod prelude {
    pub use super::{
        RTTask,
    };
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RTTask {
    /// Worst Case Execution Time
    pub wcet: Time,
    /// Relative Deadline
    pub deadline: Time,
    /// (Minimum Inter-arrival) Period
    pub period: Time,
}

impl RTTask {
    pub fn new_ns(wcet: u64, deadline: u64, period: u64) -> Self {
        Self {
            wcet: Time::nanos(wcet as f64),
            deadline: Time::nanos(deadline as f64),
            period: Time::nanos(period as f64),
        }
    }

    /// WCET / Period
    pub fn utilization(&self) -> f64 {
        self.wcet.value_ns / self.period.value_ns
    }

    /// WCET / Deadline
    pub fn density(&self) -> f64 {
        self.wcet.value_ns / self.deadline.value_ns
    }

    /// Deadline - WCET
    pub fn laxity(&self) -> Time {
        self.deadline - self.wcet
    }

    /// Deadline == Period
    pub fn has_implicit_deadline(&self) -> bool {
        self.deadline == self.period
    }

    /// Deadline <= Period
    pub fn has_constrained_deadline(&self) -> bool {
        self.deadline <= self.period
    }
}