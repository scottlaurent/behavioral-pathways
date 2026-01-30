//! Chronosystem implementation for temporal dimension and life transitions.
//!
//! The chronosystem represents the temporal dimension of ecological context,
//! including:
//!
//! - Historical period effects
//! - Normative transitions (expected life events)
//! - Non-normative events (unexpected historical events)
//! - Off-time penalties for transitions occurring outside expected windows
//! - Developmental plasticity modifiers

use crate::enums::{BirthEra, ChronosystemPath};
use crate::types::{EventId, Timestamp};

/// Domain of a turning point in life.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TurningPointDomain {
    /// Job/profession change.
    Career,
    /// Major relationship change.
    Relationship,
    /// Significant health event.
    Health,
    /// Core identity shift.
    Identity,
    /// Geographic move.
    Location,
    /// Death or major loss.
    Loss,
}

/// A significant turning point in an entity's life.
#[derive(Debug, Clone, PartialEq)]
pub struct TurningPoint {
    /// Reference to triggering event.
    pub event_id: EventId,

    /// When it occurred (absolute timestamp).
    pub timestamp: Timestamp,

    /// What area of life changed.
    pub domain: TurningPointDomain,

    /// How significant the change (0-1).
    pub magnitude: f64,
}

/// A critical period where developmental effects are amplified.
#[derive(Debug, Clone, PartialEq)]
pub struct CriticalPeriod {
    /// What domain is sensitive (attachment, identity, etc.).
    pub domain: String,

    /// Age when the period begins (years).
    pub start_age: f64,

    /// Age when the period ends (years).
    pub end_age: f64,

    /// Amplification factor applied during the period.
    pub amplification: f64,
}

impl CriticalPeriod {
    /// Creates a new critical period.
    #[must_use]
    pub fn new(
        domain: impl Into<String>,
        start_age: f64,
        end_age: f64,
        amplification: f64,
    ) -> Self {
        let (start, end) = if start_age <= end_age {
            (start_age, end_age)
        } else {
            (end_age, start_age)
        };

        CriticalPeriod {
            domain: domain.into(),
            start_age: start.max(0.0),
            end_age: end.max(0.0),
            amplification: amplification.max(1.0),
        }
    }

    /// Returns true if the period applies at the given age.
    #[must_use]
    pub fn is_active(&self, age_years: f64) -> bool {
        age_years >= self.start_age && age_years <= self.end_age
    }

    /// Returns true if the period applies at the given age and domain.
    #[must_use]
    pub fn applies_to(&self, age_years: f64, domain: &str) -> bool {
        self.is_active(age_years) && self.domain.eq_ignore_ascii_case(domain)
    }
}

/// A normative transition with expected timing.
///
/// Per spec: off-time transitions (occurring before/after expected window)
/// receive penalties to developmental outcomes via early/late multipliers.
///
/// # Fields
///
/// - `expected_age` - Culturally expected age for this transition (in years)
/// - `timing_window` - Acceptable deviation from expected age (e.g., +/- 2 years)
/// - `timing_deviation` - Computed as (actual_age - expected_age) / timing_window
///   where 0 = on-time, negative = early, positive = late
/// - `early_transition_multiplier` - Stress increase for early transitions (default 1.3)
/// - `late_transition_multiplier` - Stress increase for late transitions (default 1.5)
#[derive(Debug, Clone, PartialEq)]
pub struct NormativeTransition {
    /// Name of the transition (e.g., "marriage", "parenthood", "retirement").
    pub name: String,

    /// Culturally expected age for this transition (years).
    pub expected_age: f64,

    /// Acceptable deviation from expected age (years, e.g., 2.0 means +/- 2 years).
    pub timing_window: f64,

    /// Age at which transition actually occurred (years), if it has.
    pub actual_age: Option<f64>,

    /// Computed timing deviation: (actual_age - expected_age) / timing_window.
    /// 0 = on-time, negative = early, positive = late.
    /// Only valid when actual_age is set.
    timing_deviation: f64,

    /// Stress multiplier for early transitions (default 1.3).
    pub early_transition_multiplier: f64,

    /// Stress multiplier for late transitions (default 1.5).
    pub late_transition_multiplier: f64,

    /// Whether this transition is required or optional.
    pub is_required: bool,
}

impl NormativeTransition {
    /// Default multiplier for early transitions.
    pub const DEFAULT_EARLY_MULTIPLIER: f64 = 1.3;

    /// Default multiplier for late transitions.
    pub const DEFAULT_LATE_MULTIPLIER: f64 = 1.5;

    /// Creates a new normative transition.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the transition
    /// * `expected_age` - Culturally expected age (years)
    /// * `timing_window` - Acceptable deviation (years, e.g., 2.0 for +/- 2 years)
    /// * `is_required` - Whether this transition is required or optional
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        expected_age: f64,
        timing_window: f64,
        is_required: bool,
    ) -> Self {
        NormativeTransition {
            name: name.into(),
            expected_age,
            timing_window: timing_window.max(0.1), // Avoid division by zero
            actual_age: None,
            timing_deviation: 0.0,
            early_transition_multiplier: Self::DEFAULT_EARLY_MULTIPLIER,
            late_transition_multiplier: Self::DEFAULT_LATE_MULTIPLIER,
            is_required,
        }
    }

    /// Creates a new normative transition with custom multipliers.
    #[must_use]
    pub fn with_multipliers(
        name: impl Into<String>,
        expected_age: f64,
        timing_window: f64,
        is_required: bool,
        early_multiplier: f64,
        late_multiplier: f64,
    ) -> Self {
        NormativeTransition {
            name: name.into(),
            expected_age,
            timing_window: timing_window.max(0.1),
            actual_age: None,
            timing_deviation: 0.0,
            early_transition_multiplier: early_multiplier.max(1.0),
            late_transition_multiplier: late_multiplier.max(1.0),
            is_required,
        }
    }

    /// Marks this transition as completed at the given age.
    ///
    /// Computes timing_deviation as (actual_age - expected_age) / timing_window.
    pub fn complete(&mut self, age: f64) {
        self.actual_age = Some(age);
        self.timing_deviation = (age - self.expected_age) / self.timing_window;
    }

    /// Returns whether this transition has been completed.
    #[must_use]
    pub fn is_completed(&self) -> bool {
        self.actual_age.is_some()
    }

    /// Returns the timing deviation.
    ///
    /// - 0 = on-time (within expected window)
    /// - Negative = early (before expected_age - timing_window)
    /// - Positive = late (after expected_age + timing_window)
    #[must_use]
    pub fn timing_deviation(&self) -> f64 {
        self.timing_deviation
    }

    /// Returns whether this transition occurred early (before expected window).
    #[must_use]
    pub fn is_early(&self) -> bool {
        self.actual_age.is_some() && self.timing_deviation < -1.0
    }

    /// Returns whether this transition occurred late (after expected window).
    #[must_use]
    pub fn is_late(&self) -> bool {
        self.actual_age.is_some() && self.timing_deviation > 1.0
    }

    /// Returns whether this transition occurred on-time (within expected window).
    #[must_use]
    pub fn is_on_time(&self) -> bool {
        self.actual_age.is_some() && self.timing_deviation.abs() <= 1.0
    }

    /// Returns the appropriate stress multiplier based on timing deviation.
    ///
    /// - Returns `early_transition_multiplier` for early transitions
    /// - Returns `late_transition_multiplier` for late transitions
    /// - Returns 1.0 for on-time transitions or incomplete transitions
    #[must_use]
    pub fn off_time_penalty(&self) -> f64 {
        if self.actual_age.is_none() {
            return 1.0; // Not yet completed, no penalty
        }

        if self.is_early() {
            self.early_transition_multiplier
        } else if self.is_late() {
            self.late_transition_multiplier
        } else {
            1.0 // On-time, no penalty
        }
    }

    /// Computes a stress increment based on how off-time the transition was.
    ///
    /// Returns a value in [0.0, 0.5] representing additional stress from
    /// off-time transitions. Larger deviations produce more stress.
    ///
    /// Formula: stress_increment = (|timing_deviation| - 1.0) * 0.1, capped at 0.5
    #[must_use]
    pub fn stress_increment(&self) -> f64 {
        if self.actual_age.is_none() {
            return 0.0;
        }

        let deviation_magnitude = self.timing_deviation.abs();
        if deviation_magnitude <= 1.0 {
            0.0 // Within acceptable window
        } else {
            ((deviation_magnitude - 1.0) * 0.1).min(0.5)
        }
    }
}

/// A non-normative (historical) event affecting a cohort.
#[derive(Debug, Clone, PartialEq)]
pub struct NonNormativeEvent {
    /// Name of the event (e.g., "great_recession", "pandemic").
    pub name: String,

    /// Starting year of the event.
    pub start_year: i32,

    /// Ending year of the event (or None if ongoing).
    pub end_year: Option<i32>,

    /// Impact severity (0-1).
    pub severity: f64,
}

impl NonNormativeEvent {
    /// Creates a new non-normative event.
    #[must_use]
    pub fn new(name: impl Into<String>, start_year: i32, severity: f64) -> Self {
        NonNormativeEvent {
            name: name.into(),
            start_year,
            end_year: None,
            severity: severity.clamp(0.0, 1.0),
        }
    }

    /// Creates a completed non-normative event.
    #[must_use]
    pub fn completed(
        name: impl Into<String>,
        start_year: i32,
        end_year: i32,
        severity: f64,
    ) -> Self {
        NonNormativeEvent {
            name: name.into(),
            start_year,
            end_year: Some(end_year),
            severity: severity.clamp(0.0, 1.0),
        }
    }

    /// Returns whether this event is ongoing.
    #[must_use]
    pub fn is_ongoing(&self) -> bool {
        self.end_year.is_none()
    }
}

/// Historical period context.
#[derive(Debug, Clone, PartialEq)]
pub struct HistoricalPeriod {
    /// Name of current era.
    pub era_name: String,

    /// Societal stability level (0-1).
    pub stability_level: f64,

    /// Resource scarcity (0-1).
    pub resource_scarcity: f64,

    /// Trust in institutions (0-1).
    pub institutional_trust: f64,
}

impl Default for HistoricalPeriod {
    fn default() -> Self {
        HistoricalPeriod {
            era_name: "Modern".to_string(),
            stability_level: 0.7,
            resource_scarcity: 0.3,
            institutional_trust: 0.5,
        }
    }
}

/// Cohort effects based on birth era and formative events.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CohortEffects {
    /// Era when born.
    pub birth_era: BirthEra,

    /// Major events during formative years.
    pub formative_events: Vec<String>,

    /// Values shaped by cohort.
    pub generational_values: Vec<String>,
}

/// Chronosystem context for temporal dimension.
///
/// Represents the dimension of time in ecological context, including
/// historical events, life transitions, and developmental timing.
#[derive(Debug, Clone, PartialEq)]
pub struct ChronosystemContext {
    /// Historical period context.
    historical_period: HistoricalPeriod,

    /// Critical/sensitive periods for developmental amplification.
    critical_periods: Vec<CriticalPeriod>,

    /// Normative transitions (expected life events).
    normative_transitions: Vec<NormativeTransition>,

    /// Non-normative events (historical events).
    non_normative_events: Vec<NonNormativeEvent>,

    /// Life turning points.
    turning_points: Vec<TurningPoint>,

    /// Cohort effects.
    cohort_effects: CohortEffects,

    /// Current plasticity modifier (0-1).
    /// Higher values indicate greater developmental plasticity.
    plasticity_modifier: f64,
}

impl Default for ChronosystemContext {
    fn default() -> Self {
        ChronosystemContext {
            historical_period: HistoricalPeriod::default(),
            critical_periods: Vec::new(),
            normative_transitions: Vec::new(),
            non_normative_events: Vec::new(),
            turning_points: Vec::new(),
            cohort_effects: CohortEffects::default(),
            plasticity_modifier: 0.5,
        }
    }
}

impl ChronosystemContext {
    /// Creates a new ChronosystemContext with default values.
    #[must_use]
    pub fn new() -> Self {
        ChronosystemContext::default()
    }

    /// Gets a value by chronosystem path.
    #[must_use]
    pub fn get_value(&self, path: &ChronosystemPath) -> f64 {
        match path {
            ChronosystemPath::StabilityLevel => self.historical_period.stability_level,
            ChronosystemPath::ResourceScarcity => self.historical_period.resource_scarcity,
            ChronosystemPath::InstitutionalTrust => self.historical_period.institutional_trust,
            ChronosystemPath::PlasticityModifier => self.plasticity_modifier,
        }
    }

    /// Sets a value by chronosystem path.
    pub fn set_value(&mut self, path: &ChronosystemPath, value: f64) {
        let clamped = value.clamp(0.0, 1.0);
        match path {
            ChronosystemPath::StabilityLevel => self.historical_period.stability_level = clamped,
            ChronosystemPath::ResourceScarcity => {
                self.historical_period.resource_scarcity = clamped
            }
            ChronosystemPath::InstitutionalTrust => {
                self.historical_period.institutional_trust = clamped
            }
            ChronosystemPath::PlasticityModifier => self.plasticity_modifier = clamped,
        }
    }

    // --- Historical Period ---

    /// Returns a reference to the historical period.
    #[must_use]
    pub fn historical_period(&self) -> &HistoricalPeriod {
        &self.historical_period
    }

    /// Returns a mutable reference to the historical period.
    pub fn historical_period_mut(&mut self) -> &mut HistoricalPeriod {
        &mut self.historical_period
    }

    // --- Critical Periods ---

    /// Adds a critical period definition.
    pub fn add_critical_period(&mut self, period: CriticalPeriod) {
        self.critical_periods.push(period);
    }

    /// Returns all critical period definitions.
    #[must_use]
    pub fn critical_periods(&self) -> &[CriticalPeriod] {
        &self.critical_periods
    }

    /// Returns amplification multiplier for the given age and domain.
    #[must_use]
    pub fn get_sensitive_period_multiplier(&self, age_years: f64, domain: &str) -> f64 {
        self.critical_periods
            .iter()
            .filter(|period| period.applies_to(age_years, domain))
            .map(|period| period.amplification)
            .fold(1.0, f64::max)
    }

    // --- Normative Transitions ---

    /// Adds a normative transition.
    pub fn add_normative_transition(&mut self, transition: NormativeTransition) {
        self.normative_transitions.push(transition);
    }

    /// Gets a normative transition by name.
    #[must_use]
    pub fn get_normative_transition(&self, name: &str) -> Option<&NormativeTransition> {
        self.normative_transitions.iter().find(|t| t.name == name)
    }

    /// Gets a mutable reference to a normative transition by name.
    pub fn get_normative_transition_mut(&mut self, name: &str) -> Option<&mut NormativeTransition> {
        self.normative_transitions
            .iter_mut()
            .find(|t| t.name == name)
    }

    /// Returns all normative transitions.
    #[must_use]
    pub fn normative_transitions(&self) -> &[NormativeTransition] {
        &self.normative_transitions
    }

    /// Computes total stress increment from off-time transitions.
    ///
    /// Returns an aggregate stress value in [0.0, 1.0] representing
    /// additional stress from all off-time transitions, incorporating
    /// early/late multipliers.
    ///
    /// For each transition:
    /// - Base stress is computed from timing deviation magnitude
    /// - Multiplied by the appropriate off-time penalty (1.3x for early, 1.5x for late)
    /// - On-time transitions contribute zero stress
    #[must_use]
    pub fn total_off_time_stress(&self) -> f64 {
        self.normative_transitions
            .iter()
            .map(|t| t.stress_increment() * t.off_time_penalty())
            .sum::<f64>()
            .min(1.0)
    }

    /// Computes the maximum off-time multiplier across all transitions.
    ///
    /// Returns the highest multiplier from any transition that has occurred
    /// off-time. Returns 1.0 if all transitions are on-time or incomplete.
    #[must_use]
    pub fn max_off_time_multiplier(&self) -> f64 {
        self.normative_transitions
            .iter()
            .map(|t| t.off_time_penalty())
            .fold(1.0_f64, f64::max)
    }

    // --- Non-Normative Events ---

    /// Adds a non-normative event.
    pub fn add_non_normative_event(&mut self, event: NonNormativeEvent) {
        self.non_normative_events.push(event);
    }

    /// Returns all non-normative events.
    #[must_use]
    pub fn non_normative_events(&self) -> &[NonNormativeEvent] {
        &self.non_normative_events
    }

    /// Returns ongoing non-normative events.
    #[must_use]
    pub fn ongoing_events(&self) -> Vec<&NonNormativeEvent> {
        self.non_normative_events
            .iter()
            .filter(|e| e.is_ongoing())
            .collect()
    }

    // --- Turning Points ---

    /// Adds a turning point.
    pub fn add_turning_point(&mut self, turning_point: TurningPoint) {
        self.turning_points.push(turning_point);
    }

    /// Returns all turning points.
    #[must_use]
    pub fn turning_points(&self) -> &[TurningPoint] {
        &self.turning_points
    }

    /// Returns turning points in a specific domain.
    #[must_use]
    pub fn turning_points_in_domain(&self, domain: TurningPointDomain) -> Vec<&TurningPoint> {
        self.turning_points
            .iter()
            .filter(|tp| tp.domain == domain)
            .collect()
    }

    /// Computes the plasticity boost from recent turning points.
    ///
    /// Turning points within the last two years contribute to a capped boost.
    #[must_use]
    pub fn turning_point_plasticity_boost(&self, current_timestamp: Timestamp) -> f64 {
        const RECENT_TURNING_POINT_WINDOW_DAYS: u64 = 730;

        let recent_magnitude: f64 = self
            .turning_points
            .iter()
            .filter(|tp| {
                if current_timestamp >= tp.timestamp {
                    (current_timestamp - tp.timestamp).as_days()
                        <= RECENT_TURNING_POINT_WINDOW_DAYS
                } else {
                    false
                }
            })
            .map(|tp| tp.magnitude)
            .sum();

        (recent_magnitude * 0.1).min(0.3)
    }

    // --- Cohort Effects ---

    /// Returns a reference to cohort effects.
    #[must_use]
    pub fn cohort_effects(&self) -> &CohortEffects {
        &self.cohort_effects
    }

    /// Returns a mutable reference to cohort effects.
    pub fn cohort_effects_mut(&mut self) -> &mut CohortEffects {
        &mut self.cohort_effects
    }

    /// Returns a weight for cohort effects based on the current historical era.
    ///
    /// Returns 0.0 for Unknown birth era (no cohort effects),
    /// 1.0 when the entity's birth era matches the current era,
    /// 0.5 otherwise.
    #[must_use]
    pub fn cohort_effect_weight(&self, current_era: BirthEra) -> f64 {
        if self.cohort_effects.birth_era == BirthEra::Unknown {
            return 0.0;
        }

        if self.cohort_effects.birth_era == current_era {
            1.0
        } else {
            0.5
        }
    }

    // --- Plasticity ---

    /// Returns the current plasticity modifier.
    #[must_use]
    pub fn plasticity_modifier(&self) -> f64 {
        self.plasticity_modifier
    }

    /// Sets the plasticity modifier.
    pub fn set_plasticity_modifier(&mut self, value: f64) {
        self.plasticity_modifier = value.clamp(0.0, 1.0);
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Duration;

    fn timestamp_for_days(days: u64) -> Timestamp {
        Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0) + Duration::days(days)
    }

    // --- TurningPointDomain tests ---

    #[test]
    fn turning_point_domain_all_variants() {
        let _ = TurningPointDomain::Career;
        let _ = TurningPointDomain::Relationship;
        let _ = TurningPointDomain::Health;
        let _ = TurningPointDomain::Identity;
        let _ = TurningPointDomain::Location;
        let _ = TurningPointDomain::Loss;
    }

    // --- CriticalPeriod tests ---

    #[test]
    fn critical_period_orders_bounds_and_clamps_amplification() {
        let period = CriticalPeriod::new("identity", 18.0, 12.0, 0.8);
        assert!((period.start_age - 12.0).abs() < f64::EPSILON);
        assert!((period.end_age - 18.0).abs() < f64::EPSILON);
        assert!((period.amplification - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn chronosystem_sensitive_period_multiplier_uses_max_amplification() {
        let mut chrono = ChronosystemContext::default();
        chrono.add_critical_period(CriticalPeriod::new("attachment", 0.0, 5.0, 2.0));
        chrono.add_critical_period(CriticalPeriod::new("attachment", 3.0, 7.0, 1.5));

        let match_amp = chrono.get_sensitive_period_multiplier(4.0, "Attachment");
        assert!((match_amp - 2.0).abs() < f64::EPSILON);

        let no_match = chrono.get_sensitive_period_multiplier(9.0, "attachment");
        assert!((no_match - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn turning_point_domain_eq() {
        assert_eq!(TurningPointDomain::Career, TurningPointDomain::Career);
        assert_ne!(TurningPointDomain::Career, TurningPointDomain::Health);
    }

    // --- TurningPoint tests ---

    #[test]
    fn turning_point_creation() {
        let tp = TurningPoint {
            event_id: EventId::new("event_001").unwrap(),
            timestamp: timestamp_for_days(10000),
            domain: TurningPointDomain::Career,
            magnitude: 0.8,
        };

        assert_eq!(tp.domain, TurningPointDomain::Career);
        assert!((tp.magnitude - 0.8).abs() < f64::EPSILON);
    }

    #[test]
    fn turning_point_clone_eq() {
        let tp1 = TurningPoint {
            event_id: EventId::new("event_001").unwrap(),
            timestamp: timestamp_for_days(10000),
            domain: TurningPointDomain::Career,
            magnitude: 0.8,
        };
        let tp2 = tp1.clone();
        assert_eq!(tp1, tp2);
    }

    // --- NormativeTransition tests ---

    #[test]
    fn normative_transition_creation() {
        // expected_age = 27.5 (midpoint), timing_window = 7.5 (half the 15-year range)
        let nt = NormativeTransition::new("marriage", 27.5, 7.5, false);
        assert_eq!(nt.name, "marriage");
        assert!((nt.expected_age - 27.5).abs() < f64::EPSILON);
        assert!((nt.timing_window - 7.5).abs() < f64::EPSILON);
        assert!(!nt.is_required);
        assert!(!nt.is_completed());
        assert!((nt.early_transition_multiplier - 1.3).abs() < f64::EPSILON);
        assert!((nt.late_transition_multiplier - 1.5).abs() < f64::EPSILON);
    }

    #[test]
    fn normative_transition_complete() {
        let mut nt = NormativeTransition::new("marriage", 27.5, 5.0, false);
        nt.complete(28.0);

        assert!(nt.is_completed());
        assert!((nt.actual_age.unwrap() - 28.0).abs() < f64::EPSILON);
        // timing_deviation = (28 - 27.5) / 5 = 0.1
        assert!((nt.timing_deviation() - 0.1).abs() < f64::EPSILON);
    }

    #[test]
    fn normative_transition_on_time_penalty() {
        let mut nt = NormativeTransition::new("marriage", 27.5, 5.0, false);
        nt.complete(28.0); // On-time (within window)

        // Within window, should return 1.0 (no penalty)
        let penalty = nt.off_time_penalty();
        assert!((penalty - 1.0).abs() < f64::EPSILON);
        assert!(nt.is_on_time());
        assert!(!nt.is_early());
        assert!(!nt.is_late());
    }

    #[test]
    fn normative_transition_early_penalty() {
        let mut nt = NormativeTransition::new("marriage", 27.5, 5.0, false);
        nt.complete(20.0); // Early: deviation = (20 - 27.5) / 5 = -1.5

        let penalty = nt.off_time_penalty();
        // Early returns early_transition_multiplier (1.3)
        assert!((penalty - 1.3).abs() < f64::EPSILON);
        assert!(nt.is_early());
        assert!(!nt.is_on_time());
    }

    #[test]
    fn normative_transition_late_penalty() {
        let mut nt = NormativeTransition::new("marriage", 27.5, 5.0, false);
        nt.complete(40.0); // Late: deviation = (40 - 27.5) / 5 = 2.5

        let penalty = nt.off_time_penalty();
        // Late returns late_transition_multiplier (1.5)
        assert!((penalty - 1.5).abs() < f64::EPSILON);
        assert!(nt.is_late());
        assert!(!nt.is_on_time());
    }

    #[test]
    fn normative_transition_with_custom_multipliers() {
        let nt = NormativeTransition::with_multipliers("retirement", 65.0, 3.0, false, 1.4, 1.6);
        assert!((nt.early_transition_multiplier - 1.4).abs() < f64::EPSILON);
        assert!((nt.late_transition_multiplier - 1.6).abs() < f64::EPSILON);
    }

    #[test]
    fn normative_transition_stress_increment() {
        let mut nt = NormativeTransition::new("marriage", 27.5, 5.0, false);

        // Not completed - no stress
        assert!((nt.stress_increment() - 0.0).abs() < f64::EPSILON);

        // On-time - no stress
        nt.complete(28.0);
        assert!((nt.stress_increment() - 0.0).abs() < f64::EPSILON);

        // Early (deviation = -1.5) - stress = (1.5 - 1.0) * 0.1 = 0.05
        let mut nt2 = NormativeTransition::new("marriage", 27.5, 5.0, false);
        nt2.complete(20.0);
        assert!((nt2.stress_increment() - 0.05).abs() < f64::EPSILON);
    }

    #[test]
    fn normative_transition_not_completed_penalty() {
        let nt = NormativeTransition::new("marriage", 27.5, 5.0, false);
        let penalty = nt.off_time_penalty();
        // Not completed returns 1.0 (no penalty)
        assert!((penalty - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn normative_transition_timing_window_minimum() {
        // Timing window should have a minimum to avoid division by zero
        let nt = NormativeTransition::new("test", 30.0, 0.0, false);
        assert!(nt.timing_window >= 0.1);
    }

    #[test]
    fn normative_transition_multiplier_minimum() {
        // Multipliers should be at least 1.0
        let nt = NormativeTransition::with_multipliers("test", 30.0, 5.0, false, 0.5, 0.8);
        assert!(nt.early_transition_multiplier >= 1.0);
        assert!(nt.late_transition_multiplier >= 1.0);
    }

    // --- NonNormativeEvent tests ---

    #[test]
    fn non_normative_event_creation() {
        let nne = NonNormativeEvent::new("pandemic", 2020, 0.9);
        assert_eq!(nne.name, "pandemic");
        assert_eq!(nne.start_year, 2020);
        assert!(nne.is_ongoing());
    }

    #[test]
    fn non_normative_event_completed() {
        let nne = NonNormativeEvent::completed("great_recession", 2008, 2009, 0.7);
        assert_eq!(nne.name, "great_recession");
        assert!(!nne.is_ongoing());
        assert_eq!(nne.end_year, Some(2009));
    }

    #[test]
    fn non_normative_event_severity_clamped() {
        let nne = NonNormativeEvent::new("event", 2020, 1.5);
        assert!((nne.severity - 1.0).abs() < f64::EPSILON);

        let nne2 = NonNormativeEvent::new("event", 2020, -0.5);
        assert!((nne2.severity - 0.0).abs() < f64::EPSILON);
    }

    // --- HistoricalPeriod tests ---

    #[test]
    fn historical_period_default() {
        let hp = HistoricalPeriod::default();
        assert_eq!(hp.era_name, "Modern");
        assert!((hp.stability_level - 0.7).abs() < f64::EPSILON);
        assert!((hp.resource_scarcity - 0.3).abs() < f64::EPSILON);
        assert!((hp.institutional_trust - 0.5).abs() < f64::EPSILON);
    }

    // --- CohortEffects tests ---

    #[test]
    fn cohort_effects_default() {
        let ce = CohortEffects::default();
        assert_eq!(ce.birth_era, BirthEra::Unknown);
        assert!(ce.formative_events.is_empty());
        assert!(ce.generational_values.is_empty());
    }

    // --- ChronosystemContext tests ---

    #[test]
    fn chronosystem_context_default() {
        let chrono = ChronosystemContext::default();
        assert!((chrono.plasticity_modifier - 0.5).abs() < f64::EPSILON);
        assert!(chrono.turning_points.is_empty());
        assert!(chrono.normative_transitions.is_empty());
    }

    #[test]
    fn chronosystem_context_new() {
        let chrono = ChronosystemContext::new();
        assert!((chrono.plasticity_modifier - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn chronosystem_get_value_all_paths() {
        let chrono = ChronosystemContext::default();
        for path in ChronosystemPath::all() {
            let value = chrono.get_value(&path);
            assert!(value >= 0.0 && value <= 1.0);
        }
    }

    #[test]
    fn chronosystem_set_value() {
        let mut chrono = ChronosystemContext::default();
        chrono.set_value(&ChronosystemPath::StabilityLevel, 0.3);
        assert!((chrono.historical_period.stability_level - 0.3).abs() < f64::EPSILON);
    }

    #[test]
    fn chronosystem_set_value_clamped() {
        let mut chrono = ChronosystemContext::default();
        chrono.set_value(&ChronosystemPath::ResourceScarcity, 1.5);
        assert!((chrono.historical_period.resource_scarcity - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn chronosystem_add_normative_transition() {
        let mut chrono = ChronosystemContext::default();
        let nt = NormativeTransition::new("marriage", 20.0, 35.0, false);
        chrono.add_normative_transition(nt);

        assert_eq!(chrono.normative_transitions().len(), 1);
    }

    #[test]
    fn chronosystem_get_normative_transition() {
        let mut chrono = ChronosystemContext::default();
        chrono.add_normative_transition(NormativeTransition::new("marriage", 20.0, 35.0, false));
        chrono.add_normative_transition(NormativeTransition::new("parenthood", 25.0, 40.0, false));

        let marriage = chrono.get_normative_transition("marriage");
        assert!(marriage.is_some());
        assert_eq!(marriage.unwrap().name, "marriage");

        let nonexistent = chrono.get_normative_transition("retirement");
        assert!(nonexistent.is_none());
    }

    #[test]
    fn chronosystem_get_normative_transition_mut() {
        let mut chrono = ChronosystemContext::default();
        chrono.add_normative_transition(NormativeTransition::new("marriage", 20.0, 35.0, false));

        chrono
            .get_normative_transition_mut("marriage")
            .expect("missing marriage transition")
            .complete(28.0);

        let marriage = chrono.get_normative_transition("marriage").unwrap();
        assert!(marriage.is_completed());
    }

    #[test]
    fn chronosystem_total_off_time_stress() {
        let mut chrono = ChronosystemContext::default();

        // expected_age=27.5, timing_window=7.5 for marriage
        let mut nt1 = NormativeTransition::new("marriage", 27.5, 7.5, false);
        nt1.complete(28.0); // On-time (deviation = 0.067)
        chrono.add_normative_transition(nt1);

        // expected_age=32.5, timing_window=7.5 for parenthood
        let mut nt2 = NormativeTransition::new("parenthood", 32.5, 7.5, false);
        nt2.complete(20.0); // Early: deviation = (20 - 32.5) / 7.5 = -1.67, stress = 0.067
        chrono.add_normative_transition(nt2);

        let stress = chrono.total_off_time_stress();
        // nt1: on-time, stress = 0.0 * 1.0 = 0.0
        // nt2: early, base_stress = (1.67 - 1.0) * 0.1 = 0.067
        //      multiplier = 1.3 (early_transition_multiplier)
        //      total = 0.067 * 1.3 = 0.0871
        assert!(stress > 0.08 && stress < 0.10);
    }

    #[test]
    fn chronosystem_total_off_time_stress_capped() {
        let mut chrono = ChronosystemContext::default();

        for i in 0..20 {
            // expected_age=25, timing_window=2, completing at 10 gives deviation = -7.5
            // stress_increment = (7.5 - 1.0) * 0.1 = 0.65, capped at 0.5
            let mut nt = NormativeTransition::new(format!("event_{}", i), 25.0, 2.0, false);
            nt.complete(10.0); // Very early
            chrono.add_normative_transition(nt);
        }

        let stress = chrono.total_off_time_stress();
        // 20 * 0.5 = 10.0, capped at 1.0
        assert!((stress - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn chronosystem_max_off_time_multiplier() {
        let mut chrono = ChronosystemContext::default();

        // On-time transition
        let mut nt1 = NormativeTransition::new("marriage", 27.5, 5.0, false);
        nt1.complete(28.0);
        chrono.add_normative_transition(nt1);

        // Late transition - should have 1.5 multiplier
        let mut nt2 = NormativeTransition::new("parenthood", 32.5, 5.0, false);
        nt2.complete(45.0); // Very late
        chrono.add_normative_transition(nt2);

        let max_mult = chrono.max_off_time_multiplier();
        assert!((max_mult - 1.5).abs() < f64::EPSILON);
    }

    #[test]
    fn chronosystem_add_non_normative_event() {
        let mut chrono = ChronosystemContext::default();
        chrono.add_non_normative_event(NonNormativeEvent::new("pandemic", 2020, 0.9));

        assert_eq!(chrono.non_normative_events().len(), 1);
    }

    #[test]
    fn chronosystem_ongoing_events() {
        let mut chrono = ChronosystemContext::default();
        chrono.add_non_normative_event(NonNormativeEvent::new("ongoing", 2020, 0.9));
        chrono.add_non_normative_event(NonNormativeEvent::completed("completed", 2008, 2009, 0.7));

        let ongoing = chrono.ongoing_events();
        assert_eq!(ongoing.len(), 1);
        assert_eq!(ongoing[0].name, "ongoing");
    }

    #[test]
    fn chronosystem_add_turning_point() {
        let mut chrono = ChronosystemContext::default();
        chrono.add_turning_point(TurningPoint {
            event_id: EventId::new("event_001").unwrap(),
            timestamp: timestamp_for_days(10000),
            domain: TurningPointDomain::Career,
            magnitude: 0.8,
        });

        assert_eq!(chrono.turning_points().len(), 1);
    }

    #[test]
    fn chronosystem_turning_points_in_domain() {
        let mut chrono = ChronosystemContext::default();

        chrono.add_turning_point(TurningPoint {
            event_id: EventId::new("event_001").unwrap(),
            timestamp: timestamp_for_days(10000),
            domain: TurningPointDomain::Career,
            magnitude: 0.8,
        });

        chrono.add_turning_point(TurningPoint {
            event_id: EventId::new("event_002").unwrap(),
            timestamp: timestamp_for_days(12000),
            domain: TurningPointDomain::Relationship,
            magnitude: 0.6,
        });

        chrono.add_turning_point(TurningPoint {
            event_id: EventId::new("event_003").unwrap(),
            timestamp: timestamp_for_days(15000),
            domain: TurningPointDomain::Career,
            magnitude: 0.7,
        });

        let career_tps = chrono.turning_points_in_domain(TurningPointDomain::Career);
        assert_eq!(career_tps.len(), 2);

        let relationship_tps = chrono.turning_points_in_domain(TurningPointDomain::Relationship);
        assert_eq!(relationship_tps.len(), 1);

        let health_tps = chrono.turning_points_in_domain(TurningPointDomain::Health);
        assert_eq!(health_tps.len(), 0);
    }

    #[test]
    fn chronosystem_cohort_effects() {
        let mut chrono = ChronosystemContext::default();
        chrono.cohort_effects_mut().birth_era = BirthEra::Stability;
        chrono
            .cohort_effects_mut()
            .formative_events
            .push("economic_recovery".to_string());

        assert_eq!(chrono.cohort_effects().birth_era, BirthEra::Stability);
        assert_eq!(chrono.cohort_effects().formative_events.len(), 1);
    }

    #[test]
    fn chronosystem_turning_point_plasticity_boost_recent() {
        let mut chrono = ChronosystemContext::default();
        chrono.add_turning_point(TurningPoint {
            event_id: EventId::new("tp_recent").unwrap(),
            timestamp: timestamp_for_days(1300),
            domain: TurningPointDomain::Identity,
            magnitude: 1.0,
        });
        chrono.add_turning_point(TurningPoint {
            event_id: EventId::new("tp_old").unwrap(),
            timestamp: timestamp_for_days(900),
            domain: TurningPointDomain::Identity,
            magnitude: 1.0,
        });

        let boost = chrono.turning_point_plasticity_boost(timestamp_for_days(2000));
        assert!((boost - 0.1).abs() < f64::EPSILON);
    }

    #[test]
    fn chronosystem_turning_point_plasticity_boost_future_excluded() {
        let mut chrono = ChronosystemContext::default();
        chrono.add_turning_point(TurningPoint {
            event_id: EventId::new("tp_future").unwrap(),
            timestamp: timestamp_for_days(2000), // Future turning point
            domain: TurningPointDomain::Identity,
            magnitude: 1.0,
        });

        let boost = chrono.turning_point_plasticity_boost(timestamp_for_days(1000));
        assert!((boost - 0.0).abs() < f64::EPSILON); // Future points excluded
    }

    #[test]
    fn chronosystem_cohort_effect_weight_tracks_era_alignment() {
        let mut chrono = ChronosystemContext::default();
        // Default birth_era is Unknown, which returns 0.0 (no cohort effects)
        assert!(
            (chrono.cohort_effect_weight(BirthEra::Stability) - 0.0).abs() < f64::EPSILON
        );

        chrono.cohort_effects_mut().birth_era = BirthEra::Stability;
        assert!(
            (chrono.cohort_effect_weight(BirthEra::Stability) - 1.0).abs() < f64::EPSILON
        );
        assert!(
            (chrono.cohort_effect_weight(BirthEra::Crisis) - 0.5).abs() < f64::EPSILON
        );
    }

    #[test]
    fn chronosystem_plasticity_modifier() {
        let mut chrono = ChronosystemContext::default();
        assert!((chrono.plasticity_modifier() - 0.5).abs() < f64::EPSILON);

        chrono.set_plasticity_modifier(0.8);
        assert!((chrono.plasticity_modifier() - 0.8).abs() < f64::EPSILON);

        // Test clamping
        chrono.set_plasticity_modifier(1.5);
        assert!((chrono.plasticity_modifier() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn chronosystem_historical_period_accessors() {
        let chrono = ChronosystemContext::default();
        assert_eq!(chrono.historical_period().era_name, "Modern");

        let mut chrono_mut = ChronosystemContext::default();
        chrono_mut.historical_period_mut().era_name = "Crisis".to_string();
        assert_eq!(chrono_mut.historical_period().era_name, "Crisis");
    }

    #[test]
    fn chronosystem_clone_eq() {
        let chrono1 = ChronosystemContext::default();
        let chrono2 = chrono1.clone();
        assert_eq!(chrono1, chrono2);
    }

    #[test]
    fn chronosystem_debug() {
        let chrono = ChronosystemContext::default();
        let debug = format!("{:?}", chrono);
        assert!(debug.contains("ChronosystemContext"));
    }

    #[test]
    fn chronosystem_set_value_all_paths() {
        let mut chrono = ChronosystemContext::default();

        // Test InstitutionalTrust path
        chrono.set_value(&ChronosystemPath::InstitutionalTrust, 0.7);
        assert!(
            (chrono.get_value(&ChronosystemPath::InstitutionalTrust) - 0.7).abs() < f64::EPSILON
        );

        // Test PlasticityModifier path
        chrono.set_value(&ChronosystemPath::PlasticityModifier, 0.8);
        assert!(
            (chrono.get_value(&ChronosystemPath::PlasticityModifier) - 0.8).abs() < f64::EPSILON
        );
    }

    // --- Required Phase 7 tests ---

    #[test]
    fn chronosystem_creation_default() {
        // ChronosystemContext creates with birth era defaults
        let chrono = ChronosystemContext::default();

        // Verify historical period defaults
        assert_eq!(chrono.historical_period().era_name, "Modern");
        assert!((chrono.historical_period().stability_level - 0.7).abs() < f64::EPSILON);
        assert!((chrono.historical_period().resource_scarcity - 0.3).abs() < f64::EPSILON);
        assert!((chrono.historical_period().institutional_trust - 0.5).abs() < f64::EPSILON);

        // Verify cohort effects are empty by default
        assert_eq!(chrono.cohort_effects().birth_era, BirthEra::Unknown);
        assert!(chrono.cohort_effects().formative_events.is_empty());

        // Verify plasticity modifier default
        assert!((chrono.plasticity_modifier() - 0.5).abs() < f64::EPSILON);

        // Verify no transitions/events by default
        assert!(chrono.normative_transitions().is_empty());
        assert!(chrono.non_normative_events().is_empty());
        assert!(chrono.turning_points().is_empty());
    }

    #[test]
    fn chronosystem_normative_transition_expected_timing() {
        // Graduation at expected age has timing_deviation = 0
        let mut chrono = ChronosystemContext::default();

        // Add graduation transition: expected at age 22, window +/- 2 years
        let mut graduation = NormativeTransition::new("graduation", 22.0, 2.0, true);

        // Complete at exactly expected age
        graduation.complete(22.0);

        assert!(graduation.is_completed());
        // timing_deviation = (22.0 - 22.0) / 2.0 = 0.0
        assert!((graduation.timing_deviation() - 0.0).abs() < f64::EPSILON);
        assert!(graduation.is_on_time());
        assert!(!graduation.is_early());
        assert!(!graduation.is_late());
        // No stress increment for on-time
        assert!((graduation.stress_increment() - 0.0).abs() < f64::EPSILON);
        // No penalty for on-time
        assert!((graduation.off_time_penalty() - 1.0).abs() < f64::EPSILON);

        chrono.add_normative_transition(graduation);
        // Total stress should be 0 for on-time transition
        assert!((chrono.total_off_time_stress() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn chronosystem_normative_offtime_penalty() {
        // Early/late transitions have timing_deviation != 0
        let mut early_transition = NormativeTransition::new("marriage", 27.5, 5.0, false);
        early_transition.complete(18.0); // Very early: deviation = (18 - 27.5) / 5 = -1.9

        assert!(early_transition.is_completed());
        assert!(early_transition.timing_deviation() < -1.0); // Should be -1.9
        assert!(early_transition.is_early());
        assert!(!early_transition.is_on_time());

        let mut late_transition = NormativeTransition::new("parenthood", 30.0, 5.0, false);
        late_transition.complete(42.0); // Very late: deviation = (42 - 30) / 5 = 2.4

        assert!(late_transition.is_completed());
        assert!(late_transition.timing_deviation() > 1.0); // Should be 2.4
        assert!(late_transition.is_late());
        assert!(!late_transition.is_on_time());
    }

    #[test]
    fn chronosystem_early_transition_increases_stress() {
        // Early transitions increase stress via early_transition_multiplier
        let mut chrono = ChronosystemContext::default();

        // Create transition with custom multipliers for clarity
        let mut early = NormativeTransition::with_multipliers(
            "early_event",
            25.0,
            2.0,
            false,
            1.3, // early_transition_multiplier
            1.5, // late_transition_multiplier
        );

        // Complete very early: age 20, expected 25, window 2
        // deviation = (20 - 25) / 2 = -2.5 (early)
        early.complete(20.0);

        assert!(early.is_early());
        // off_time_penalty should return early_transition_multiplier
        assert!((early.off_time_penalty() - 1.3).abs() < f64::EPSILON);

        // stress_increment = (|deviation| - 1.0) * 0.1 = (2.5 - 1.0) * 0.1 = 0.15
        let base_stress = early.stress_increment();
        assert!((base_stress - 0.15).abs() < f64::EPSILON);

        chrono.add_normative_transition(early);

        // Total stress should be base_stress * multiplier = 0.15 * 1.3 = 0.195
        let total_stress = chrono.total_off_time_stress();
        assert!((total_stress - 0.195).abs() < 0.001);
    }

    #[test]
    fn chronosystem_late_transition_increases_stress() {
        // Late transitions increase stress via late_transition_multiplier
        let mut chrono = ChronosystemContext::default();

        // Create transition with custom multipliers for clarity
        let mut late = NormativeTransition::with_multipliers(
            "late_event",
            25.0,
            2.0,
            false,
            1.3, // early_transition_multiplier
            1.5, // late_transition_multiplier
        );

        // Complete very late: age 32, expected 25, window 2
        // deviation = (32 - 25) / 2 = 3.5 (late)
        late.complete(32.0);

        assert!(late.is_late());
        // off_time_penalty should return late_transition_multiplier
        assert!((late.off_time_penalty() - 1.5).abs() < f64::EPSILON);

        // stress_increment = (|deviation| - 1.0) * 0.1 = (3.5 - 1.0) * 0.1 = 0.25
        let base_stress = late.stress_increment();
        assert!((base_stress - 0.25).abs() < f64::EPSILON);

        chrono.add_normative_transition(late);

        // Total stress should be base_stress * multiplier = 0.25 * 1.5 = 0.375
        let total_stress = chrono.total_off_time_stress();
        assert!((total_stress - 0.375).abs() < 0.001);
    }

    #[test]
    fn chronosystem_early_vs_late_stress_differs() {
        // Verify early and late transitions produce different stress levels
        let mut chrono_early = ChronosystemContext::default();
        let mut chrono_late = ChronosystemContext::default();

        // Same base parameters, same deviation magnitude (2.0)
        let mut early = NormativeTransition::new("event", 25.0, 2.0, false);
        early.complete(21.0); // deviation = -2.0 (early)

        let mut late = NormativeTransition::new("event", 25.0, 2.0, false);
        late.complete(29.0); // deviation = +2.0 (late)

        chrono_early.add_normative_transition(early.clone());
        chrono_late.add_normative_transition(late.clone());

        let early_stress = chrono_early.total_off_time_stress();
        let late_stress = chrono_late.total_off_time_stress();

        // Both should have non-zero stress
        assert!(early_stress > 0.0);
        assert!(late_stress > 0.0);

        // Late should have higher stress due to 1.5x vs 1.3x multiplier
        assert!(late_stress > early_stress);

        // Verify the ratio is approximately 1.5/1.3
        let ratio = late_stress / early_stress;
        let expected_ratio = 1.5 / 1.3;
        assert!((ratio - expected_ratio).abs() < 0.01);
    }

    #[test]
    fn critical_period_is_active_directly() {
        let period = CriticalPeriod::new("language", 0.0, 7.0, 2.5);

        assert!(period.is_active(0.0));
        assert!(period.is_active(3.5));
        assert!(period.is_active(7.0));
        assert!(!period.is_active(7.1));
        assert!(!period.is_active(10.0));
    }

    #[test]
    fn chronosystem_critical_periods_accessor() {
        let mut chrono = ChronosystemContext::default();
        assert!(chrono.critical_periods().is_empty());

        chrono.add_critical_period(CriticalPeriod::new("attachment", 0.0, 2.0, 2.0));
        chrono.add_critical_period(CriticalPeriod::new("language", 0.0, 7.0, 1.8));

        assert_eq!(chrono.critical_periods().len(), 2);
        assert_eq!(chrono.critical_periods()[0].domain, "attachment");
        assert_eq!(chrono.critical_periods()[1].domain, "language");
    }
}
