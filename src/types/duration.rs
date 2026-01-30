//! Duration type for representing time spans.
//!
//! Provides a type-safe representation of time with constructors for
//! common units (seconds, minutes, hours, days, weeks, months, years)
//! and arithmetic operations.

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Sub};

/// A duration representing a span of time.
///
/// Internally stored as seconds for maximum precision. Provides constructors
/// for common time units and conversion methods.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::types::Duration;
///
/// // Creating durations
/// let one_day = Duration::days(1);
/// let one_week = Duration::weeks(1);
///
/// // Arithmetic
/// let total = one_day + one_week;
/// assert_eq!(total.as_days(), 8);
///
/// // Comparison
/// assert!(one_week > one_day);
/// ```
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Duration {
    /// Total seconds in this duration.
    seconds: u64,
}

impl Duration {
    /// Milliseconds per second.
    const MILLIS_PER_SECOND: u64 = 1000;
    /// Seconds per minute.
    const SECONDS_PER_MINUTE: u64 = 60;
    /// Seconds per hour.
    const SECONDS_PER_HOUR: u64 = 3600;
    /// Seconds per day.
    const SECONDS_PER_DAY: u64 = 86400;
    /// Seconds per week.
    const SECONDS_PER_WEEK: u64 = 604_800;
    /// Days per month (approximate - 30 days).
    const DAYS_PER_MONTH: u64 = 30;
    /// Seconds per month (approximate - 30 days).
    const SECONDS_PER_MONTH: u64 = Self::DAYS_PER_MONTH * Self::SECONDS_PER_DAY;
    /// Days per year (approximate - 365 days).
    const DAYS_PER_YEAR: u64 = 365;
    /// Seconds per year (approximate - 365 days).
    const SECONDS_PER_YEAR: u64 = Self::DAYS_PER_YEAR * Self::SECONDS_PER_DAY;

    /// Creates a new duration of zero length.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::types::Duration;
    ///
    /// let zero = Duration::zero();
    /// assert_eq!(zero.as_seconds(), 0);
    /// ```
    #[must_use]
    pub const fn zero() -> Self {
        Duration { seconds: 0 }
    }

    /// Creates a duration from the specified number of milliseconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::types::Duration;
    ///
    /// let duration = Duration::from_millis(5000);
    /// assert_eq!(duration.as_seconds(), 5);
    /// ```
    #[must_use]
    pub const fn from_millis(millis: u64) -> Self {
        Duration {
            seconds: millis / Self::MILLIS_PER_SECOND,
        }
    }

    /// Creates a duration from the specified number of seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::types::Duration;
    ///
    /// let duration = Duration::seconds(120);
    /// assert_eq!(duration.as_seconds(), 120);
    /// assert_eq!(duration.as_minutes(), 2);
    /// ```
    #[must_use]
    pub const fn seconds(seconds: u64) -> Self {
        Duration { seconds }
    }

    /// Creates a duration from the specified number of minutes.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::types::Duration;
    ///
    /// let duration = Duration::minutes(30);
    /// assert_eq!(duration.as_seconds(), 1800);
    /// ```
    #[must_use]
    pub const fn minutes(minutes: u64) -> Self {
        Duration {
            seconds: minutes * Self::SECONDS_PER_MINUTE,
        }
    }

    /// Creates a duration from the specified number of hours.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::types::Duration;
    ///
    /// let duration = Duration::hours(2);
    /// assert_eq!(duration.as_minutes(), 120);
    /// ```
    #[must_use]
    pub const fn hours(hours: u64) -> Self {
        Duration {
            seconds: hours * Self::SECONDS_PER_HOUR,
        }
    }

    /// Creates a duration from the specified number of hours as a float.
    ///
    /// This is useful when the number of hours is a calculation result
    /// that may be fractional.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::types::Duration;
    ///
    /// let duration = Duration::from_hours_f32(1.5);
    /// assert_eq!(duration.as_minutes(), 90);
    /// ```
    #[must_use]
    pub fn from_hours_f32(hours: f32) -> Self {
        let seconds = (hours * Self::SECONDS_PER_HOUR as f32) as u64;
        Duration { seconds }
    }

    /// Creates a duration from the specified number of days.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::types::Duration;
    ///
    /// let duration = Duration::days(7);
    /// assert_eq!(duration.as_weeks(), 1);
    /// ```
    #[must_use]
    pub const fn days(days: u64) -> Self {
        Duration {
            seconds: days * Self::SECONDS_PER_DAY,
        }
    }

    /// Creates a duration from the specified number of weeks.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::types::Duration;
    ///
    /// let duration = Duration::weeks(2);
    /// assert_eq!(duration.as_days(), 14);
    /// ```
    #[must_use]
    pub const fn weeks(weeks: u64) -> Self {
        Duration {
            seconds: weeks * Self::SECONDS_PER_WEEK,
        }
    }

    /// Creates a duration from the specified number of months (30 days each).
    ///
    /// Note: This uses an approximate month length of 30 days.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::types::Duration;
    ///
    /// let duration = Duration::months(6);
    /// assert_eq!(duration.as_days(), 180);
    /// ```
    #[must_use]
    pub const fn months(months: u64) -> Self {
        Duration {
            seconds: months * Self::SECONDS_PER_MONTH,
        }
    }

    /// Creates a duration from the specified number of years (365 days each).
    ///
    /// Note: This uses an approximate year length of 365 days.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::types::Duration;
    ///
    /// let duration = Duration::years(1);
    /// assert_eq!(duration.as_days(), 365);
    /// ```
    #[must_use]
    pub const fn years(years: u64) -> Self {
        Duration {
            seconds: years * Self::SECONDS_PER_YEAR,
        }
    }

    /// Returns the total number of seconds in this duration.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::types::Duration;
    ///
    /// let duration = Duration::minutes(2);
    /// assert_eq!(duration.as_seconds(), 120);
    /// ```
    #[must_use]
    pub const fn as_seconds(&self) -> u64 {
        self.seconds
    }

    /// Returns the total number of milliseconds in this duration.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::types::Duration;
    ///
    /// let duration = Duration::seconds(5);
    /// assert_eq!(duration.as_millis(), 5000);
    /// ```
    #[must_use]
    pub const fn as_millis(&self) -> u64 {
        self.seconds * Self::MILLIS_PER_SECOND
    }

    /// Returns the total number of minutes in this duration (truncated).
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::types::Duration;
    ///
    /// let duration = Duration::seconds(150);
    /// assert_eq!(duration.as_minutes(), 2); // Truncated from 2.5
    /// ```
    #[must_use]
    pub const fn as_minutes(&self) -> u64 {
        self.seconds / Self::SECONDS_PER_MINUTE
    }

    /// Returns the total number of hours in this duration (truncated).
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::types::Duration;
    ///
    /// let duration = Duration::minutes(150);
    /// assert_eq!(duration.as_hours(), 2); // Truncated from 2.5
    /// ```
    #[must_use]
    pub const fn as_hours(&self) -> u64 {
        self.seconds / Self::SECONDS_PER_HOUR
    }

    /// Returns the total number of days in this duration (truncated).
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::types::Duration;
    ///
    /// let duration = Duration::hours(36);
    /// assert_eq!(duration.as_days(), 1); // Truncated from 1.5
    /// ```
    #[must_use]
    pub const fn as_days(&self) -> u64 {
        self.seconds / Self::SECONDS_PER_DAY
    }

    /// Returns the total number of weeks in this duration (truncated).
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::types::Duration;
    ///
    /// let duration = Duration::days(10);
    /// assert_eq!(duration.as_weeks(), 1); // Truncated from ~1.43
    /// ```
    #[must_use]
    pub const fn as_weeks(&self) -> u64 {
        self.seconds / Self::SECONDS_PER_WEEK
    }

    /// Returns the total number of months in this duration (truncated, 30 days per month).
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::types::Duration;
    ///
    /// let duration = Duration::days(45);
    /// assert_eq!(duration.as_months(), 1); // Truncated from 1.5
    /// ```
    #[must_use]
    pub const fn as_months(&self) -> u64 {
        self.seconds / Self::SECONDS_PER_MONTH
    }

    /// Returns the total number of years in this duration (truncated, 365 days per year).
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::types::Duration;
    ///
    /// let duration = Duration::days(400);
    /// assert_eq!(duration.as_years(), 1); // Truncated from ~1.1
    /// ```
    #[must_use]
    pub const fn as_years(&self) -> u64 {
        self.seconds / Self::SECONDS_PER_YEAR
    }

    /// Returns the duration as a floating-point number of days.
    ///
    /// This is useful for calculations that need fractional days.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::types::Duration;
    ///
    /// let duration = Duration::hours(36);
    /// assert!((duration.as_days_f64() - 1.5).abs() < 0.001);
    /// ```
    #[must_use]
    pub fn as_days_f64(&self) -> f64 {
        self.seconds as f64 / Self::SECONDS_PER_DAY as f64
    }

    /// Returns the duration as a floating-point number of years.
    ///
    /// This is useful for age calculations.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::types::Duration;
    ///
    /// let duration = Duration::days(547);
    /// assert!((duration.as_years_f64() - 1.5).abs() < 0.01);
    /// ```
    #[must_use]
    pub fn as_years_f64(&self) -> f64 {
        self.seconds as f64 / Self::SECONDS_PER_YEAR as f64
    }

    /// Returns true if this duration is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::types::Duration;
    ///
    /// assert!(Duration::zero().is_zero());
    /// assert!(!Duration::seconds(1).is_zero());
    /// ```
    #[must_use]
    pub const fn is_zero(&self) -> bool {
        self.seconds == 0
    }

    /// Saturating addition. Returns `Duration::MAX` if overflow would occur.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::types::Duration;
    ///
    /// let d1 = Duration::days(5);
    /// let d2 = Duration::days(3);
    /// let sum = d1.saturating_add(d2);
    /// assert_eq!(sum.as_days(), 8);
    /// ```
    #[must_use]
    pub const fn saturating_add(self, other: Duration) -> Duration {
        Duration {
            seconds: self.seconds.saturating_add(other.seconds),
        }
    }

    /// Saturating subtraction. Returns `Duration::zero()` if underflow would occur.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::types::Duration;
    ///
    /// let d1 = Duration::days(3);
    /// let d2 = Duration::days(5);
    /// let diff = d1.saturating_sub(d2);
    /// assert!(diff.is_zero());
    /// ```
    #[must_use]
    pub const fn saturating_sub(self, other: Duration) -> Duration {
        Duration {
            seconds: self.seconds.saturating_sub(other.seconds),
        }
    }
}

impl Add for Duration {
    type Output = Duration;

    fn add(self, other: Duration) -> Duration {
        Duration {
            seconds: self.seconds + other.seconds,
        }
    }
}

impl Sub for Duration {
    type Output = Duration;

    fn sub(self, other: Duration) -> Duration {
        Duration {
            seconds: self.seconds.saturating_sub(other.seconds),
        }
    }
}

impl Mul<u64> for Duration {
    type Output = Duration;

    fn mul(self, scalar: u64) -> Duration {
        Duration {
            seconds: self.seconds * scalar,
        }
    }
}

impl Mul<Duration> for u64 {
    type Output = Duration;

    fn mul(self, duration: Duration) -> Duration {
        Duration {
            seconds: self * duration.seconds,
        }
    }
}

impl Div<u64> for Duration {
    type Output = Duration;

    fn div(self, divisor: u64) -> Duration {
        Duration {
            seconds: self.seconds / divisor,
        }
    }
}

impl PartialEq for Duration {
    fn eq(&self, other: &Self) -> bool {
        self.seconds == other.seconds
    }
}

impl Eq for Duration {}

impl PartialOrd for Duration {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Duration {
    fn cmp(&self, other: &Self) -> Ordering {
        self.seconds.cmp(&other.seconds)
    }
}

impl std::hash::Hash for Duration {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.seconds.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seconds_creates_duration() {
        let d = Duration::seconds(120);
        assert_eq!(d.as_seconds(), 120);
        assert_eq!(d.as_minutes(), 2);
    }

    #[test]
    fn days_plus_days_equals_sum() {
        let d1 = Duration::days(5);
        let d2 = Duration::days(3);
        let sum = d1 + d2;
        assert_eq!(sum.as_days(), 8);
    }

    #[test]
    fn weeks_minus_days() {
        let weeks = Duration::weeks(2);
        let days = Duration::days(3);
        let diff = weeks - days;
        assert_eq!(diff.as_days(), 11);
    }

    #[test]
    fn duration_multiply_scalar() {
        let d = Duration::days(5);
        let doubled = d * 2;
        assert_eq!(doubled.as_days(), 10);

        // Also test reverse order
        let tripled = 3 * d;
        assert_eq!(tripled.as_days(), 15);
    }

    #[test]
    fn years_to_days_conversion() {
        let one_year = Duration::years(1);
        assert_eq!(one_year.as_days(), 365);

        let two_years = Duration::years(2);
        assert_eq!(two_years.as_days(), 730);
    }

    #[test]
    fn hours_less_than_days() {
        let hours = Duration::hours(23);
        let day = Duration::days(1);
        assert!(hours < day);
        assert!(day > hours);
    }

    #[test]
    fn duration_equality() {
        let d1 = Duration::hours(24);
        let d2 = Duration::days(1);
        assert_eq!(d1, d2);

        let d3 = Duration::minutes(60);
        let d4 = Duration::hours(1);
        assert_eq!(d3, d4);
    }

    #[test]
    fn zero_duration() {
        let zero = Duration::zero();
        assert!(zero.is_zero());
        assert_eq!(zero.as_seconds(), 0);

        let non_zero = Duration::seconds(1);
        assert!(!non_zero.is_zero());
    }

    #[test]
    fn division() {
        let d = Duration::days(10);
        let half = d / 2;
        assert_eq!(half.as_days(), 5);
    }

    #[test]
    fn saturating_operations() {
        // Saturating sub prevents underflow
        let small = Duration::days(3);
        let large = Duration::days(10);
        let result = small.saturating_sub(large);
        assert!(result.is_zero());

        // Regular subtraction also saturates (via saturating_sub in impl)
        let result2 = small - large;
        assert!(result2.is_zero());
    }

    #[test]
    fn months_approximation() {
        let six_months = Duration::months(6);
        assert_eq!(six_months.as_days(), 180); // 6 * 30 = 180
    }

    #[test]
    fn fractional_conversions() {
        let d = Duration::hours(36);
        assert!((d.as_days_f64() - 1.5).abs() < 0.001);

        let eighteen_months = Duration::months(18);
        assert!((eighteen_months.as_years_f64() - 1.479).abs() < 0.01);
    }

    #[test]
    fn ordering() {
        let durations = [
            Duration::seconds(1),
            Duration::minutes(1),
            Duration::hours(1),
            Duration::days(1),
            Duration::weeks(1),
        ];

        for i in 0..durations.len() - 1 {
            assert!(durations[i] < durations[i + 1]);
        }
    }

    #[test]
    fn default_is_zero() {
        let d: Duration = Duration::default();
        assert!(d.is_zero());
    }

    #[test]
    fn hash_consistency() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(Duration::days(1));
        set.insert(Duration::hours(24)); // Same as 1 day

        assert_eq!(set.len(), 1); // Should be deduplicated
    }

    #[test]
    fn minutes_conversion() {
        let d = Duration::minutes(90);
        assert_eq!(d.as_minutes(), 90);
        assert_eq!(d.as_hours(), 1); // Truncated
    }

    #[test]
    fn hours_conversion() {
        let d = Duration::hours(48);
        assert_eq!(d.as_hours(), 48);
        assert_eq!(d.as_days(), 2);
    }

    #[test]
    fn weeks_conversion() {
        let d = Duration::weeks(4);
        assert_eq!(d.as_weeks(), 4);
        assert_eq!(d.as_days(), 28);
    }

    #[test]
    fn months_conversion() {
        let d = Duration::months(12);
        assert_eq!(d.as_months(), 12);
        assert_eq!(d.as_days(), 360); // 12 * 30
    }

    #[test]
    fn years_conversion() {
        let d = Duration::years(2);
        assert_eq!(d.as_years(), 2);
        assert_eq!(d.as_days(), 730); // 2 * 365
    }

    #[test]
    fn as_years_f64() {
        let d = Duration::days(365 + 182);
        assert!((d.as_years_f64() - 1.5).abs() < 0.01);
    }

    #[test]
    fn saturating_add() {
        let d1 = Duration::days(5);
        let d2 = Duration::days(3);
        let sum = d1.saturating_add(d2);
        assert_eq!(sum.as_days(), 8);
    }

    #[test]
    fn partial_ord() {
        let d1 = Duration::days(1);
        let d2 = Duration::days(2);
        assert!(d1 < d2);
        assert!(d2 > d1);
        assert!(d1 <= d2);
        assert!(d2 >= d1);
        assert!(d1 != d2);

        let d3 = Duration::days(1);
        assert!(d1 <= d3);
        assert!(d1 >= d3);
    }

    #[test]
    fn as_seconds() {
        let d = Duration::seconds(12345);
        assert_eq!(d.as_seconds(), 12345);
    }

    #[test]
    fn clone_duration() {
        let d = Duration::days(5);
        let cloned = d.clone();
        assert_eq!(d, cloned);
    }

    #[test]
    fn debug_format() {
        let d = Duration::days(5);
        let debug = format!("{:?}", d);
        assert!(debug.contains("Duration"));
    }

    #[test]
    fn copy_duration() {
        let d1 = Duration::days(5);
        let d2 = d1; // Copy
        assert_eq!(d1, d2);
    }

    #[test]
    fn from_millis() {
        let d = Duration::from_millis(5000);
        assert_eq!(d.as_seconds(), 5);
        assert_eq!(d.as_millis(), 5000);
    }

    #[test]
    fn as_millis() {
        let d = Duration::seconds(5);
        assert_eq!(d.as_millis(), 5000);

        let d2 = Duration::hours(1);
        assert_eq!(d2.as_millis(), 3600 * 1000);
    }

    #[test]
    fn from_millis_rounds_down() {
        let d = Duration::from_millis(5500);
        assert_eq!(d.as_seconds(), 5);
    }

    #[test]
    fn from_hours_f32() {
        // 1.5 hours = 90 minutes
        let d = Duration::from_hours_f32(1.5);
        assert_eq!(d.as_minutes(), 90);
        assert_eq!(d.as_seconds(), 5400);

        // Integer hours work correctly
        let d2 = Duration::from_hours_f32(2.0);
        assert_eq!(d2.as_hours(), 2);

        // Fractional hours are handled
        let d3 = Duration::from_hours_f32(0.5);
        assert_eq!(d3.as_minutes(), 30);
    }
}
