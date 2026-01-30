//! Timestamp type for absolute time representation.
//!
//! Provides a type-safe wrapper around chrono::NaiveDateTime for representing
//! absolute timestamps in the simulation. All timestamps are timezone-naive,
//! representing a specific calendar date and time.
//!
//! # Usage
//!
//! Timestamps are the primary time type for the consumer API. Entities are
//! anchored at a timestamp, events occur at timestamps, and state queries
//! are made at timestamps.
//!
//! ```
//! use behavioral_pathways::types::Timestamp;
//!
//! // Create from components
//! let ts = Timestamp::from_ymd_hms(2024, 1, 15, 14, 30, 0);
//!
//! // Create from string
//! let ts = Timestamp::from_str("2024-01-15 14:30:00").unwrap();
//! ```

use crate::types::Duration;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Sub};

/// Error type for timestamp parsing failures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimestampParseError {
    /// The string format was not recognized.
    InvalidFormat(String),
    /// The date components were invalid.
    InvalidDate { year: i32, month: u32, day: u32 },
    /// The time components were invalid.
    InvalidTime { hour: u32, min: u32, sec: u32 },
}

impl fmt::Display for TimestampParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimestampParseError::InvalidFormat(s) => {
                write!(f, "Invalid timestamp format: {}", s)
            }
            TimestampParseError::InvalidDate { year, month, day } => {
                write!(f, "Invalid date: {}-{:02}-{:02}", year, month, day)
            }
            TimestampParseError::InvalidTime { hour, min, sec } => {
                write!(f, "Invalid time: {:02}:{:02}:{:02}", hour, min, sec)
            }
        }
    }
}

impl std::error::Error for TimestampParseError {}

/// An absolute timestamp representing a specific point in time.
///
/// Timestamps are timezone-naive and use the format "YYYY-MM-DD HH:mm:ss".
/// They are used for anchoring entity state, scheduling events, and
/// querying state at specific points in time.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::types::{Timestamp, Duration};
///
/// // Create a timestamp
/// let birth = Timestamp::from_ymd_hms(1990, 1, 15, 0, 0, 0);
/// let now = Timestamp::from_ymd_hms(2024, 6, 15, 12, 0, 0);
///
/// // Compute duration between timestamps
/// let age = now - birth;
/// assert!(age.as_years() >= 34);
///
/// // Add duration to timestamp
/// let future = now + Duration::years(10);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Timestamp {
    inner: NaiveDateTime,
}

impl Timestamp {
    /// Creates a timestamp from year, month, day, hour, minute, second.
    ///
    /// # Arguments
    ///
    /// * `year` - The year (e.g., 2024)
    /// * `month` - The month (1-12)
    /// * `day` - The day of month (1-31)
    /// * `hour` - The hour (0-23)
    /// * `min` - The minute (0-59)
    /// * `sec` - The second (0-59)
    ///
    /// # Panics
    ///
    /// Panics if the date or time components are invalid. For fallible
    /// construction, use `try_from_ymd_hms`.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::types::Timestamp;
    ///
    /// let ts = Timestamp::from_ymd_hms(2024, 1, 15, 14, 30, 0);
    /// ```
    #[must_use]
    pub fn from_ymd_hms(year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32) -> Self {
        Self::try_from_ymd_hms(year, month, day, hour, min, sec)
            .expect("Invalid date/time components")
    }

    /// Attempts to create a timestamp from year, month, day, hour, minute, second.
    ///
    /// Returns an error if the date or time components are invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::types::Timestamp;
    ///
    /// // Valid timestamp
    /// let ts = Timestamp::try_from_ymd_hms(2024, 1, 15, 14, 30, 0);
    /// assert!(ts.is_ok());
    ///
    /// // Invalid month
    /// let invalid = Timestamp::try_from_ymd_hms(2024, 13, 15, 14, 30, 0);
    /// assert!(invalid.is_err());
    /// ```
    pub fn try_from_ymd_hms(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        min: u32,
        sec: u32,
    ) -> Result<Self, TimestampParseError> {
        use chrono::NaiveDate;

        let date = NaiveDate::from_ymd_opt(year, month, day)
            .ok_or(TimestampParseError::InvalidDate { year, month, day })?;

        let datetime = date
            .and_hms_opt(hour, min, sec)
            .ok_or(TimestampParseError::InvalidTime { hour, min, sec })?;

        Ok(Timestamp { inner: datetime })
    }

    /// Parses a timestamp from a string in "YYYY-MM-DD HH:mm:ss" format.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to parse
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::types::Timestamp;
    ///
    /// let ts = Timestamp::from_str("2024-01-15 14:30:00").unwrap();
    /// ```
    #[allow(clippy::should_implement_trait)] // Returns domain-specific error type
    pub fn from_str(s: &str) -> Result<Self, TimestampParseError> {
        NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
            .map(|inner| Timestamp { inner })
            .map_err(|_| TimestampParseError::InvalidFormat(s.to_string()))
    }

    /// Returns the year component.
    #[must_use]
    pub fn year(&self) -> i32 {
        self.inner.date().year()
    }

    /// Returns the month component (1-12).
    #[must_use]
    pub fn month(&self) -> u32 {
        self.inner.date().month()
    }

    /// Returns the day component (1-31).
    #[must_use]
    pub fn day(&self) -> u32 {
        self.inner.date().day()
    }

    /// Returns the hour component (0-23).
    #[must_use]
    pub fn hour(&self) -> u32 {
        self.inner.time().hour()
    }

    /// Returns the minute component (0-59).
    #[must_use]
    pub fn minute(&self) -> u32 {
        self.inner.time().minute()
    }

    /// Returns the second component (0-59).
    #[must_use]
    pub fn second(&self) -> u32 {
        self.inner.time().second()
    }

    /// Returns the underlying NaiveDateTime.
    ///
    /// This is primarily for internal use or interoperability.
    #[must_use]
    pub fn as_naive_datetime(&self) -> NaiveDateTime {
        self.inner
    }

    /// Creates a timestamp from a NaiveDateTime.
    ///
    /// This is primarily for internal use or interoperability.
    #[must_use]
    pub fn from_naive_datetime(dt: NaiveDateTime) -> Self {
        Timestamp { inner: dt }
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner.format("%Y-%m-%d %H:%M:%S"))
    }
}

impl PartialOrd for Timestamp {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Timestamp {
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl Add<Duration> for Timestamp {
    type Output = Timestamp;

    fn add(self, duration: Duration) -> Timestamp {
        let chrono_duration = chrono::Duration::seconds(duration.as_seconds() as i64);
        Timestamp {
            inner: self.inner + chrono_duration,
        }
    }
}

impl Sub<Timestamp> for Timestamp {
    type Output = Duration;

    fn sub(self, other: Timestamp) -> Duration {
        let diff = self.inner.signed_duration_since(other.inner);
        let seconds = diff.num_seconds();
        if seconds < 0 {
            Duration::zero()
        } else {
            Duration::seconds(seconds as u64)
        }
    }
}

impl Sub<Duration> for Timestamp {
    type Output = Timestamp;

    fn sub(self, duration: Duration) -> Timestamp {
        let chrono_duration = chrono::Duration::seconds(duration.as_seconds() as i64);
        Timestamp {
            inner: self.inner - chrono_duration,
        }
    }
}

/// Computes the duration from a reference timestamp to a target timestamp.
///
/// If target is before reference, returns Duration::zero().
///
/// # Arguments
///
/// * `reference` - The starting timestamp
/// * `target` - The ending timestamp
///
/// # Examples
///
/// ```
/// use behavioral_pathways::types::{Timestamp, timestamp_to_duration};
///
/// let start = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
/// let end = Timestamp::from_ymd_hms(2024, 1, 8, 0, 0, 0);
///
/// let duration = timestamp_to_duration(&start, &end);
/// assert_eq!(duration.as_days(), 7);
/// ```
#[must_use]
pub fn timestamp_to_duration(reference: &Timestamp, target: &Timestamp) -> Duration {
    *target - *reference
}

/// Computes a timestamp by adding a duration to a reference timestamp.
///
/// # Arguments
///
/// * `reference` - The starting timestamp
/// * `duration` - The duration to add
///
/// # Examples
///
/// ```
/// use behavioral_pathways::types::{Timestamp, Duration, duration_to_timestamp};
///
/// let start = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
/// let offset = Duration::days(7);
///
/// let result = duration_to_timestamp(&start, offset);
/// assert_eq!(result.day(), 8);
/// ```
#[must_use]
pub fn duration_to_timestamp(reference: &Timestamp, duration: Duration) -> Timestamp {
    *reference + duration
}

use chrono::Datelike;
use chrono::Timelike;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timestamp_from_ymd_hms_creates_correctly() {
        let ts = Timestamp::from_ymd_hms(2024, 1, 15, 14, 30, 45);

        assert_eq!(ts.year(), 2024);
        assert_eq!(ts.month(), 1);
        assert_eq!(ts.day(), 15);
        assert_eq!(ts.hour(), 14);
        assert_eq!(ts.minute(), 30);
        assert_eq!(ts.second(), 45);
    }

    #[test]
    fn timestamp_from_str_parses_standard_format() {
        let ts = Timestamp::from_str("2024-01-15 14:30:00").unwrap();

        assert_eq!(ts.year(), 2024);
        assert_eq!(ts.month(), 1);
        assert_eq!(ts.day(), 15);
        assert_eq!(ts.hour(), 14);
        assert_eq!(ts.minute(), 30);
        assert_eq!(ts.second(), 0);
    }

    #[test]
    fn timestamp_from_str_rejects_invalid() {
        // Invalid format
        let result = Timestamp::from_str("not a date");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.to_string(), "Invalid timestamp format: not a date");

        // Wrong format
        let result2 = Timestamp::from_str("01-15-2024 14:30:00");
        assert!(result2.is_err());

        // Missing time
        let result3 = Timestamp::from_str("2024-01-15");
        assert!(result3.is_err());
    }

    #[test]
    fn timestamp_to_duration_positive_difference() {
        let start = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
        let end = Timestamp::from_ymd_hms(2024, 1, 8, 0, 0, 0);

        let duration = timestamp_to_duration(&start, &end);
        assert_eq!(duration.as_days(), 7);
    }

    #[test]
    fn timestamp_to_duration_negative_difference() {
        let start = Timestamp::from_ymd_hms(2024, 1, 8, 0, 0, 0);
        let end = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);

        // When target is before reference, returns zero
        let duration = timestamp_to_duration(&start, &end);
        assert!(duration.is_zero());
    }

    #[test]
    fn duration_to_timestamp_adds_correctly() {
        let start = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
        let duration = Duration::days(7);

        let result = duration_to_timestamp(&start, duration);
        assert_eq!(result.day(), 8);
        assert_eq!(result.month(), 1);
    }

    #[test]
    fn timestamp_add_duration_operator() {
        let start = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
        let duration = Duration::days(10);

        let result = start + duration;
        assert_eq!(result.day(), 11);
    }

    #[test]
    fn timestamp_sub_timestamp_operator() {
        let start = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
        let end = Timestamp::from_ymd_hms(2024, 1, 15, 12, 0, 0);

        let duration = end - start;
        assert_eq!(duration.as_days(), 14);
        assert_eq!(duration.as_hours(), 14 * 24 + 12);
    }

    #[test]
    fn timestamp_sub_duration_operator() {
        let ts = Timestamp::from_ymd_hms(2024, 1, 15, 0, 0, 0);
        let duration = Duration::days(5);

        let result = ts - duration;
        assert_eq!(result.day(), 10);
    }

    #[test]
    fn timestamp_equality() {
        let ts1 = Timestamp::from_ymd_hms(2024, 1, 15, 14, 30, 0);
        let ts2 = Timestamp::from_ymd_hms(2024, 1, 15, 14, 30, 0);
        let ts3 = Timestamp::from_ymd_hms(2024, 1, 15, 14, 30, 1);

        assert_eq!(ts1, ts2);
        assert_ne!(ts1, ts3);
    }

    #[test]
    fn timestamp_ordering() {
        let early = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
        let late = Timestamp::from_ymd_hms(2024, 12, 31, 23, 59, 59);

        assert!(early < late);
        assert!(late > early);
        assert!(early <= early);
        assert!(late >= late);
    }

    #[test]
    fn timestamp_display() {
        let ts = Timestamp::from_ymd_hms(2024, 1, 15, 14, 30, 45);
        let display = format!("{}", ts);

        assert_eq!(display, "2024-01-15 14:30:45");
    }

    #[test]
    fn timestamp_debug() {
        let ts = Timestamp::from_ymd_hms(2024, 1, 15, 14, 30, 0);
        let debug = format!("{:?}", ts);
        assert!(debug.contains("Timestamp"));
    }

    #[test]
    fn timestamp_clone_and_copy() {
        let ts1 = Timestamp::from_ymd_hms(2024, 1, 15, 14, 30, 0);
        let ts2 = ts1; // Copy
        let ts3 = ts1.clone();

        assert_eq!(ts1, ts2);
        assert_eq!(ts1, ts3);
    }

    #[test]
    fn timestamp_hash() {
        use std::collections::HashSet;

        let ts1 = Timestamp::from_ymd_hms(2024, 1, 15, 14, 30, 0);
        let ts2 = Timestamp::from_ymd_hms(2024, 1, 15, 14, 30, 0);
        let ts3 = Timestamp::from_ymd_hms(2024, 1, 16, 14, 30, 0);

        let mut set = HashSet::new();
        set.insert(ts1);
        set.insert(ts2); // Duplicate

        assert_eq!(set.len(), 1);

        set.insert(ts3);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn try_from_ymd_hms_valid() {
        let result = Timestamp::try_from_ymd_hms(2024, 6, 15, 12, 30, 45);
        assert!(result.is_ok());
    }

    #[test]
    fn try_from_ymd_hms_invalid_date() {
        // Invalid month
        let result = Timestamp::try_from_ymd_hms(2024, 13, 15, 12, 30, 45);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.to_string(), "Invalid date: 2024-13-15");

        // Invalid day
        let result2 = Timestamp::try_from_ymd_hms(2024, 2, 30, 12, 30, 45);
        assert!(result2.is_err());
        let err2 = result2.unwrap_err();
        assert_eq!(err2.to_string(), "Invalid date: 2024-02-30");
    }

    #[test]
    fn try_from_ymd_hms_invalid_time() {
        // Invalid hour
        let result = Timestamp::try_from_ymd_hms(2024, 6, 15, 25, 30, 45);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.to_string(), "Invalid time: 25:30:45");

        // Invalid minute
        let result2 = Timestamp::try_from_ymd_hms(2024, 6, 15, 12, 60, 45);
        assert!(result2.is_err());
        let err2 = result2.unwrap_err();
        assert_eq!(err2.to_string(), "Invalid time: 12:60:45");

        // Invalid second
        let result3 = Timestamp::try_from_ymd_hms(2024, 6, 15, 12, 30, 60);
        assert!(result3.is_err());
        let err3 = result3.unwrap_err();
        assert_eq!(err3.to_string(), "Invalid time: 12:30:60");
    }

    #[test]
    fn timestamp_parse_error_display() {
        let err1 = TimestampParseError::InvalidFormat("bad".to_string());
        let display1 = format!("{}", err1);
        assert!(display1.contains("Invalid timestamp format"));
        assert!(display1.contains("bad"));

        let err2 = TimestampParseError::InvalidDate {
            year: 2024,
            month: 13,
            day: 1,
        };
        let display2 = format!("{}", err2);
        assert!(display2.contains("Invalid date"));

        let err3 = TimestampParseError::InvalidTime {
            hour: 25,
            min: 0,
            sec: 0,
        };
        let display3 = format!("{}", err3);
        assert!(display3.contains("Invalid time"));
    }

    #[test]
    fn timestamp_parse_error_debug() {
        let err = TimestampParseError::InvalidFormat("test".to_string());
        let debug = format!("{:?}", err);
        assert!(debug.contains("InvalidFormat"));
    }

    #[test]
    fn as_naive_datetime() {
        let ts = Timestamp::from_ymd_hms(2024, 1, 15, 14, 30, 0);
        let ndt = ts.as_naive_datetime();
        assert_eq!(ndt.year(), 2024);
    }

    #[test]
    fn from_naive_datetime() {
        use chrono::NaiveDate;
        let ndt = NaiveDate::from_ymd_opt(2024, 6, 15)
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap();
        let ts = Timestamp::from_naive_datetime(ndt);
        assert_eq!(ts.year(), 2024);
        assert_eq!(ts.month(), 6);
        assert_eq!(ts.day(), 15);
    }

    #[test]
    fn timestamp_years_arithmetic() {
        let birth = Timestamp::from_ymd_hms(1990, 1, 15, 0, 0, 0);
        let now = Timestamp::from_ymd_hms(2024, 6, 15, 12, 0, 0);

        let age = now - birth;
        assert!(age.as_years() >= 34);
    }

    #[test]
    fn timestamp_sub_larger_returns_zero() {
        let small = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
        let large = Timestamp::from_ymd_hms(2024, 12, 31, 0, 0, 0);

        let result = small - large;
        assert!(result.is_zero());
    }

    #[test]
    fn timestamp_add_large_duration() {
        let start = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
        let duration = Duration::years(50);

        let result = start + duration;
        // Duration::years uses 365 days per year, so 50 years = 18250 days
        // From 2024-01-01 + 18250 days = ~2073-12-16 (due to leap years)
        // The year should be either 2073 or 2074 depending on exact calculation
        assert!(result.year() >= 2073 && result.year() <= 2074);
    }

    #[test]
    fn timestamp_partial_ord() {
        let ts1 = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
        let ts2 = Timestamp::from_ymd_hms(2024, 1, 2, 0, 0, 0);

        assert!(ts1.partial_cmp(&ts2) == Some(Ordering::Less));
        assert!(ts2.partial_cmp(&ts1) == Some(Ordering::Greater));
        assert!(ts1.partial_cmp(&ts1) == Some(Ordering::Equal));
    }
}
