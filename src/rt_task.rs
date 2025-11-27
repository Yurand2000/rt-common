use crate::prelude::*;

pub mod prelude {
    pub use super::{
        RTTask,
    };
}

#[derive(Debug, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RTTask {
    pub wcet: Time,
    pub deadline: Time,
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

    /// wcet / period
    pub fn utilization(&self) -> f64 {
        self.wcet.value_ns / self.period.value_ns
    }

    /// wcet / deadline
    pub fn density(&self) -> f64 {
        self.wcet.value_ns / self.deadline.value_ns
    }

    /// deadline - wcet
    pub fn laxity(&self) -> Time {
        self.deadline - self.wcet
    }

    pub fn has_implicit_deadline(&self) -> bool {
        self.deadline == self.period
    }

    pub fn has_constrained_deadline(&self) -> bool {
        self.deadline <= self.period
    }
}