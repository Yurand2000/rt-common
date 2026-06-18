//! Time and Time² structs.
//!
//! The **Time** and **Time²** (Time2) structs are **f64** wrappers that
//! describe time items with nanosecond precision. The structs were built to
//! better write expressions and formulas, and catch subtle typing errors when
//! writing the formulas from academic papers into code.
//!
//! The general idea is to overload the standard unary and binary operators of
//! *f64* to better represent what a combination of different unit object is. As
//! an example, sum of `Time`s is still a `Time`, while division of `Time`s is a
//! scalar, and product of `Time`s is a `Time²`.
//!
//! Both struct additionally implement `Eq` and `Ord` for easier comparisons.
//! They use the [ordered-float](https://crates.io/crates/ordered-float/)
//! crate's functions for comparisons.

pub mod prelude {
    pub use super::{
        Time,
        Time2,
    };
}

#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct Time {
    pub value_ns: f64,
}

#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct Time2 {
    pub value_ns_2: f64,
}

// =============================================================================

impl Time {
    pub const MICRO_TO_NANO: f64 = 1000.0;
    pub const MILLI_TO_NANO: f64 = 1000_000.0;
    pub const SECS_TO_NANO: f64 = 1000_000_000.0;

    pub fn zero() -> Self {
        Self { value_ns: 0.0 }
    }

    pub fn one() -> Self {
        Self { value_ns: 1.0 }
    }

    pub fn nanos(time_ns: f64) -> Self {
        Self { value_ns: time_ns }
    }

    pub fn micros(time_us: f64) -> Self {
        Self { value_ns: time_us * Self::MICRO_TO_NANO }
    }

    pub fn millis(time_ms: f64) -> Self {
        Self { value_ns: time_ms * Self::MILLI_TO_NANO }
    }

    pub fn secs(time_s: f64) -> Self {
        Self { value_ns: time_s * Self::SECS_TO_NANO }
    }

    pub fn as_nanos(&self) -> f64 {
        self.value_ns
    }

    pub fn as_micros(&self) -> f64 {
        self.value_ns / Self::MICRO_TO_NANO
    }

    pub fn as_millis(&self) -> f64 {
        self.value_ns / Self::MILLI_TO_NANO
    }

    pub fn as_secs(&self) -> f64 {
        self.value_ns / Self::SECS_TO_NANO
    }

    pub fn floor(self) -> Self {
        Self { value_ns: f64::floor(self.value_ns) }
    }

    pub fn ceil(self) -> Self {
        Self { value_ns: f64::ceil(self.value_ns) }
    }

    pub fn round(self) -> Self {
        Self { value_ns: f64::round(self.value_ns) }
    }
}

impl PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        let error = 0.5;

        f64::abs(self.value_ns - other.value_ns) < error
    }
}

impl Eq for Time { }

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        ordered_float::OrderedFloat(self.value_ns)
            .partial_cmp(&ordered_float::OrderedFloat(other.value_ns))
    }
}

impl Ord for Time {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        ordered_float::OrderedFloat(self.value_ns)
            .cmp(&ordered_float::OrderedFloat(other.value_ns))
    }
}

impl std::ops::Neg for Time {
    type Output = Time;

    fn neg(self) -> Self::Output {
        Self::Output { value_ns: -self.value_ns }
    }
}

impl std::ops::Add for Time {
    type Output = Time;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output { value_ns: (self.value_ns + rhs.value_ns) }
    }
}

impl std::ops::Sub for Time {
    type Output = Time;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output { value_ns: (self.value_ns - rhs.value_ns) }
    }
}

impl std::ops::Mul<f64> for Time {
    type Output = Time;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output { value_ns: (self.value_ns * rhs) }
    }
}

impl std::ops::Mul<Time> for f64 {
    type Output = Time;

    fn mul(self, rhs: Time) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Div for Time {
    type Output = f64;

    fn div(self, rhs: Self) -> Self::Output {
        self.value_ns / rhs.value_ns
    }
}

impl std::ops::Div<f64> for Time {
    type Output = Time;

    fn div(self, rhs: f64) -> Self::Output {
        Time { value_ns: self.value_ns / rhs }
    }
}

impl std::ops::Rem for Time {
    type Output = Time;

    fn rem(self, rhs: Self) -> Self::Output {
        Self::Output { value_ns: self.value_ns.floor() % rhs.value_ns.floor() }
    }
}

impl std::iter::Sum for Time {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Time::zero(), |acc, val| acc + val)
    }
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let nanos = self.value_ns.floor() as u64;
        if nanos % Time::SECS_TO_NANO as u64 == 0 {
            write!(f, "{}s", nanos / Time::SECS_TO_NANO as u64)
        } else if nanos % Time::MILLI_TO_NANO as u64 == 0 {
            write!(f, "{}ms", nanos / Time::MILLI_TO_NANO as u64)
        } else if nanos % Time::MICRO_TO_NANO as u64 == 0 {
            write!(f, "{}us", nanos / Time::MICRO_TO_NANO as u64)
        } else {
            write!(f, "{}ns", nanos)
        }
    }
}

#[derive(Debug)]
pub enum TimeParseError {
    NumberParseError(std::num::ParseFloatError),
    UnitParseError(String),
    FormatError(),
}

impl std::fmt::Display for TimeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TimeParseError::*;

        match self {
            NumberParseError(err) => write!(f, "Invalid Number: {err}"),
            UnitParseError(unit) => write!(f, "Invalid Time Unit: {unit}"),
            FormatError() => write!(f, "Invalid Format"),
        }
    }
}

impl std::error::Error for TimeParseError { }

impl std::str::FromStr for Time {
    type Err = TimeParseError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let Some(split_idx) =
            str.char_indices().find_map(|(index, ch)| {
                if !(ch.is_ascii_digit() || ch == '+' || ch == '-' || ch == '.' || ch == 'e' || ch == 'E') {
                    Some(index)
                } else {
                    None
                }
            }) else {
                return Err(TimeParseError::FormatError())
            };

        let (num, unit) = str.split_at(split_idx);

        let unit = match unit {
            "s" => Time::SECS_TO_NANO,
            "ms" => Time::MILLI_TO_NANO,
            "us" => Time::MICRO_TO_NANO,
            "ns" => 1.0,
            u => { return Err(TimeParseError::UnitParseError(u.to_owned())) }
        };

        let num: f64 = num.parse()
            .map_err(|err| TimeParseError::NumberParseError(err))?;

        Ok(Time::nanos(num * unit))
    }
}

impl serde::Serialize for Time {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        format!("{}", self).serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(serde::de::Error::custom)
    }
}

impl Time2 {
    pub fn new(value: f64) -> Self {
        Self { value_ns_2: value }
    }

    pub fn value(&self) -> f64 {
        self.value_ns_2
    }

    pub fn sqrt(self) -> Time {
        Time::nanos(self.value_ns_2.sqrt())
    }
}

impl std::ops::Neg for Time2 {
    type Output = Time2;

    fn neg(self) -> Self::Output {
        Self::Output { value_ns_2: -self.value_ns_2 }
    }
}

impl std::ops::Add for Time2 {
    type Output = Time2;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output { value_ns_2: (self.value_ns_2 + rhs.value_ns_2) }
    }
}

impl std::ops::Sub for Time2 {
    type Output = Time2;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output { value_ns_2: (self.value_ns_2 - rhs.value_ns_2) }
    }
}

impl std::ops::Mul<Time> for Time {
    type Output = Time2;

    fn mul(self, rhs: Time) -> Self::Output {
        Self::Output { value_ns_2: (self.value_ns * rhs.value_ns) }
    }
}

impl std::ops::Mul<f64> for Time2 {
    type Output = Time2;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output { value_ns_2: (self.value_ns_2 * rhs) }
    }
}

impl std::ops::Mul<Time2> for f64 {
    type Output = Time2;

    fn mul(self, rhs: Time2) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Div<Time> for Time2 {
    type Output = Time;

    fn div(self, rhs: Time) -> Self::Output {
        Self::Output { value_ns: self.value_ns_2 / rhs.value_ns }
    }
}

impl std::ops::Div<f64> for Time2 {
    type Output = Time2;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output { value_ns_2: self.value_ns_2 / rhs }
    }
}