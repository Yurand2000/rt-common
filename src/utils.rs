//! General utilities.
//! 
//! Given that tasksets are described as slices of `RTTask`s, this module
//! provides utility functions to compute properties of the given tasksets.

use crate::prelude::*;

pub mod prelude {
    pub use super::{
        RTUtils,
    };
}

/// Utility functions on tasksets.
pub struct RTUtils;

impl RTUtils {
    pub fn is_taskset_sorted_by_period(taskset: &[RTTask]) -> bool {
        taskset.windows(2).all(|w| w[0].period <= w[1].period)
    }

    pub fn is_taskset_sorted_by_deadline(taskset: &[RTTask]) -> bool {
        taskset.windows(2).all(|w| w[0].deadline <= w[1].deadline)
    }

    pub fn implicit_deadlines(taskset: &[RTTask]) -> bool {
        taskset.iter().all(RTTask::has_implicit_deadline)
    }

    pub fn constrained_deadlines(taskset: &[RTTask]) -> bool {
        taskset.iter().all(RTTask::has_constrained_deadline)
    }

    pub fn total_utilization(taskset: &[RTTask]) -> f64 {
        taskset.iter()
            .map(RTTask::utilization)
            .sum()
    }

    pub fn largest_utilization(taskset: &[RTTask]) -> f64 {
        let max = taskset.iter()
            .map(|t| ordered_float::OrderedFloat(RTTask::utilization(t)))
            .max();

        match max {
            Some(max) => *max,
            None => 0f64,
        }
    }

    pub fn total_density(taskset: &[RTTask]) -> f64 {
        taskset.iter()
            .map(RTTask::density)
            .sum()
    }

    pub fn largest_density(taskset: &[RTTask]) -> f64 {
        let max = taskset.iter()
            .map(|t| ordered_float::OrderedFloat(RTTask::density(t)))
            .max();

        match max {
            Some(max) => *max,
            None => 0f64,
        }
    }

    pub fn hyperperiod(taskset: &[RTTask]) -> Time {
        let hyperperiod =
            taskset.iter()
            .map(|task| task.period.as_nanos().floor() as i64)
            .fold(1, |lcm, period| num::integer::lcm(lcm, period));

        Time { value_ns: hyperperiod as f64 }
    }
}