//! Shared (symmetric) relationship dimensions.
//!
//! These dimensions have the same value from both entities' perspectives.

use crate::enums::SharedPath;
use crate::state::StateValue;
use crate::types::Duration;

/// Decay half-life for affinity (14 days).
const AFFINITY_DECAY_HALF_LIFE: Duration = Duration::days(14);

/// Decay half-life for respect (21 days).
const RESPECT_DECAY_HALF_LIFE: Duration = Duration::days(21);

/// Decay half-life for tension (7 days) - tension fades faster.
const TENSION_DECAY_HALF_LIFE: Duration = Duration::days(7);

/// Decay half-life for intimacy (30 days).
const INTIMACY_DECAY_HALF_LIFE: Duration = Duration::days(30);

/// Shared dimensions between two entities.
///
/// These are symmetric - both entities perceive the same value.
/// For example, "how much do they like each other" is mutual.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::relationship::SharedDimensions;
/// use behavioral_pathways::enums::SharedPath;
///
/// let mut shared = SharedDimensions::new();
/// shared.add_affinity_delta(0.2);
/// assert!(shared.affinity_effective() > 0.2);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct SharedDimensions {
    /// General liking between entities.
    /// Range: 0 (dislike) to 1 (strong liking)
    affinity: StateValue,

    /// Mutual respect and admiration.
    /// Range: 0 (no respect) to 1 (high respect)
    respect: StateValue,

    /// Unresolved conflict and tension.
    /// Range: 0 (no tension) to 1 (high tension)
    tension: StateValue,

    /// Emotional closeness and intimacy.
    /// Range: 0 (distant) to 1 (very close)
    intimacy: StateValue,

    /// Depth of shared experience (monotonically increasing).
    /// Range: 0 (no history) to 1 (extensive history)
    /// Note: History does not decay - it only accumulates.
    history: StateValue,
}

impl SharedDimensions {
    /// Creates new SharedDimensions with default values.
    ///
    /// Defaults represent a neutral starting point.
    #[must_use]
    pub fn new() -> Self {
        SharedDimensions {
            affinity: StateValue::new(0.1)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(AFFINITY_DECAY_HALF_LIFE),
            respect: StateValue::new(0.2)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(RESPECT_DECAY_HALF_LIFE),
            tension: StateValue::new(0.0)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(TENSION_DECAY_HALF_LIFE),
            intimacy: StateValue::new(0.0)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(INTIMACY_DECAY_HALF_LIFE),
            history: StateValue::new_no_decay(0.0).with_bounds(0.0, 1.0),
        }
    }

    // Effective value accessors

    /// Returns the effective affinity (base + delta).
    #[must_use]
    pub fn affinity_effective(&self) -> f32 {
        self.affinity.effective()
    }

    /// Returns the effective respect (base + delta).
    #[must_use]
    pub fn respect_effective(&self) -> f32 {
        self.respect.effective()
    }

    /// Returns the effective tension (base + delta).
    #[must_use]
    pub fn tension_effective(&self) -> f32 {
        self.tension.effective()
    }

    /// Returns the effective intimacy (base + delta).
    #[must_use]
    pub fn intimacy_effective(&self) -> f32 {
        self.intimacy.effective()
    }

    /// Returns the effective history (base + delta).
    #[must_use]
    pub fn history_effective(&self) -> f32 {
        self.history.effective()
    }

    // StateValue references

    /// Returns a reference to the affinity StateValue.
    #[must_use]
    pub fn affinity(&self) -> &StateValue {
        &self.affinity
    }

    /// Returns a reference to the respect StateValue.
    #[must_use]
    pub fn respect(&self) -> &StateValue {
        &self.respect
    }

    /// Returns a reference to the tension StateValue.
    #[must_use]
    pub fn tension(&self) -> &StateValue {
        &self.tension
    }

    /// Returns a reference to the intimacy StateValue.
    #[must_use]
    pub fn intimacy(&self) -> &StateValue {
        &self.intimacy
    }

    /// Returns a reference to the history StateValue.
    #[must_use]
    pub fn history(&self) -> &StateValue {
        &self.history
    }

    /// Returns a mutable reference to the affinity StateValue.
    pub fn affinity_mut(&mut self) -> &mut StateValue {
        &mut self.affinity
    }

    /// Returns a mutable reference to the respect StateValue.
    pub fn respect_mut(&mut self) -> &mut StateValue {
        &mut self.respect
    }

    /// Returns a mutable reference to the tension StateValue.
    pub fn tension_mut(&mut self) -> &mut StateValue {
        &mut self.tension
    }

    /// Returns a mutable reference to the intimacy StateValue.
    pub fn intimacy_mut(&mut self) -> &mut StateValue {
        &mut self.intimacy
    }

    /// Returns a mutable reference to the history StateValue.
    pub fn history_mut(&mut self) -> &mut StateValue {
        &mut self.history
    }

    /// Returns a reference to the StateValue for the given shared path.
    #[must_use]
    pub fn get(&self, path: SharedPath) -> &StateValue {
        match path {
            SharedPath::Affinity => &self.affinity,
            SharedPath::Respect => &self.respect,
            SharedPath::Tension => &self.tension,
            SharedPath::Intimacy => &self.intimacy,
            SharedPath::History => &self.history,
        }
    }

    /// Returns a mutable reference to the StateValue for the given shared path.
    pub fn get_mut(&mut self, path: SharedPath) -> &mut StateValue {
        match path {
            SharedPath::Affinity => &mut self.affinity,
            SharedPath::Respect => &mut self.respect,
            SharedPath::Tension => &mut self.tension,
            SharedPath::Intimacy => &mut self.intimacy,
            SharedPath::History => &mut self.history,
        }
    }

    // Delta modifiers

    /// Adds to the affinity delta.
    pub fn add_affinity_delta(&mut self, amount: f32) {
        self.affinity.add_delta(amount);
    }

    /// Adds to the respect delta.
    pub fn add_respect_delta(&mut self, amount: f32) {
        self.respect.add_delta(amount);
    }

    /// Adds to the tension delta.
    pub fn add_tension_delta(&mut self, amount: f32) {
        self.tension.add_delta(amount);
    }

    /// Adds to the intimacy delta.
    pub fn add_intimacy_delta(&mut self, amount: f32) {
        self.intimacy.add_delta(amount);
    }

    /// Adds to the history delta.
    ///
    /// Note: History can only increase (use positive amounts).
    pub fn add_history_delta(&mut self, amount: f32) {
        // History is monotonically increasing
        if amount > 0.0 {
            self.history.add_delta(amount);
        }
    }

    /// Adds to the delta for the specified shared path.
    pub fn add_delta(&mut self, path: SharedPath, amount: f32) {
        match path {
            SharedPath::Affinity => self.affinity.add_delta(amount),
            SharedPath::Respect => self.respect.add_delta(amount),
            SharedPath::Tension => self.tension.add_delta(amount),
            SharedPath::Intimacy => self.intimacy.add_delta(amount),
            SharedPath::History => {
                if amount > 0.0 {
                    self.history.add_delta(amount);
                }
            }
        }
    }

    // Decay

    /// Applies decay to all shared dimensions over the specified duration.
    ///
    /// Note: History does not decay.
    pub fn apply_decay(&mut self, elapsed: Duration) {
        self.affinity.apply_decay(elapsed);
        self.respect.apply_decay(elapsed);
        self.tension.apply_decay(elapsed);
        self.intimacy.apply_decay(elapsed);
        // History never decays
    }

    /// Resets all deltas to zero.
    pub fn reset_deltas(&mut self) {
        self.affinity.reset_delta();
        self.respect.reset_delta();
        self.tension.reset_delta();
        self.intimacy.reset_delta();
        // Don't reset history - it's cumulative
    }
}

impl Default for SharedDimensions {
    fn default() -> Self {
        SharedDimensions::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_default_values() {
        let shared = SharedDimensions::new();
        assert!((shared.affinity_effective() - 0.1).abs() < f32::EPSILON);
        assert!((shared.respect_effective() - 0.2).abs() < f32::EPSILON);
        assert!(shared.tension_effective().abs() < f32::EPSILON);
        assert!(shared.intimacy_effective().abs() < f32::EPSILON);
        assert!(shared.history_effective().abs() < f32::EPSILON);
    }

    #[test]
    fn add_affinity_delta() {
        let mut shared = SharedDimensions::new();
        shared.add_affinity_delta(0.3);
        assert!((shared.affinity().delta() - 0.3).abs() < f32::EPSILON);
    }

    #[test]
    fn add_respect_delta() {
        let mut shared = SharedDimensions::new();
        shared.add_respect_delta(0.2);
        assert!((shared.respect().delta() - 0.2).abs() < f32::EPSILON);
    }

    #[test]
    fn add_tension_delta() {
        let mut shared = SharedDimensions::new();
        shared.add_tension_delta(0.4);
        assert!((shared.tension().delta() - 0.4).abs() < f32::EPSILON);
    }

    #[test]
    fn add_intimacy_delta() {
        let mut shared = SharedDimensions::new();
        shared.add_intimacy_delta(0.2);
        assert!((shared.intimacy().delta() - 0.2).abs() < f32::EPSILON);
    }

    #[test]
    fn add_history_delta_positive() {
        let mut shared = SharedDimensions::new();
        shared.add_history_delta(0.1);
        assert!((shared.history().delta() - 0.1).abs() < f32::EPSILON);
    }

    #[test]
    fn add_history_delta_negative_ignored() {
        let mut shared = SharedDimensions::new();
        shared.add_history_delta(0.2);
        shared.add_history_delta(-0.1); // Should be ignored
        assert!((shared.history().delta() - 0.2).abs() < f32::EPSILON);
    }

    #[test]
    fn add_delta_by_path() {
        let mut shared = SharedDimensions::new();
        shared.add_delta(SharedPath::Affinity, 0.1);
        shared.add_delta(SharedPath::Respect, 0.2);
        shared.add_delta(SharedPath::Tension, 0.3);
        shared.add_delta(SharedPath::Intimacy, 0.4);
        shared.add_delta(SharedPath::History, 0.5);

        assert!((shared.affinity().delta() - 0.1).abs() < f32::EPSILON);
        assert!((shared.respect().delta() - 0.2).abs() < f32::EPSILON);
        assert!((shared.tension().delta() - 0.3).abs() < f32::EPSILON);
        assert!((shared.intimacy().delta() - 0.4).abs() < f32::EPSILON);
        assert!((shared.history().delta() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn add_delta_history_negative_ignored() {
        let mut shared = SharedDimensions::new();
        shared.add_delta(SharedPath::History, -0.1);
        assert!(shared.history().delta().abs() < f32::EPSILON);
    }

    #[test]
    fn get_by_path() {
        let shared = SharedDimensions::new();
        assert!((shared.get(SharedPath::Affinity).effective() - 0.1).abs() < f32::EPSILON);
        assert!((shared.get(SharedPath::Respect).effective() - 0.2).abs() < f32::EPSILON);
    }

    #[test]
    fn get_mut_by_path() {
        let mut shared = SharedDimensions::new();
        shared.get_mut(SharedPath::Affinity).add_delta(0.1);
        assert!((shared.affinity().delta() - 0.1).abs() < f32::EPSILON);
    }

    #[test]
    fn get_mut_by_path_updates_all_dimensions() {
        let mut shared = SharedDimensions::new();
        shared.get_mut(SharedPath::Respect).add_delta(0.2);
        shared.get_mut(SharedPath::Tension).add_delta(0.3);
        shared.get_mut(SharedPath::Intimacy).add_delta(0.4);
        shared.get_mut(SharedPath::History).add_delta(0.5);

        assert!((shared.respect().delta() - 0.2).abs() < f32::EPSILON);
        assert!((shared.tension().delta() - 0.3).abs() < f32::EPSILON);
        assert!((shared.intimacy().delta() - 0.4).abs() < f32::EPSILON);
        assert!((shared.history().delta() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn affinity_decays_over_14_days() {
        let mut shared = SharedDimensions::new();
        shared.add_affinity_delta(0.4);
        shared.apply_decay(Duration::days(14));
        assert!((shared.affinity().delta() - 0.2).abs() < 0.01);
    }

    #[test]
    fn tension_decays_over_7_days() {
        let mut shared = SharedDimensions::new();
        shared.add_tension_delta(0.4);
        shared.apply_decay(Duration::days(7));
        assert!((shared.tension().delta() - 0.2).abs() < 0.01);
    }

    #[test]
    fn respect_decays_over_21_days() {
        let mut shared = SharedDimensions::new();
        shared.add_respect_delta(0.4);
        shared.apply_decay(Duration::days(21));
        assert!((shared.respect().delta() - 0.2).abs() < 0.01);
    }

    #[test]
    fn intimacy_decays_over_30_days() {
        let mut shared = SharedDimensions::new();
        shared.add_intimacy_delta(0.4);
        shared.apply_decay(Duration::days(30));
        assert!((shared.intimacy().delta() - 0.2).abs() < 0.01);
    }

    #[test]
    fn history_never_decays() {
        let mut shared = SharedDimensions::new();
        shared.add_history_delta(0.4);
        shared.apply_decay(Duration::years(10));
        assert!((shared.history().delta() - 0.4).abs() < f32::EPSILON);
    }

    #[test]
    fn reset_deltas_does_not_reset_history() {
        let mut shared = SharedDimensions::new();
        shared.add_affinity_delta(0.1);
        shared.add_history_delta(0.2);

        shared.reset_deltas();

        assert!(shared.affinity().delta().abs() < f32::EPSILON);
        // History preserved
        assert!((shared.history().delta() - 0.2).abs() < f32::EPSILON);
    }

    #[test]
    fn mutable_references_work() {
        let mut shared = SharedDimensions::new();
        shared.affinity_mut().add_delta(0.1);
        shared.respect_mut().add_delta(0.2);
        shared.tension_mut().add_delta(0.3);
        shared.intimacy_mut().add_delta(0.4);
        shared.history_mut().add_delta(0.5);

        assert!((shared.affinity().delta() - 0.1).abs() < f32::EPSILON);
        assert!((shared.respect().delta() - 0.2).abs() < f32::EPSILON);
        assert!((shared.tension().delta() - 0.3).abs() < f32::EPSILON);
        assert!((shared.intimacy().delta() - 0.4).abs() < f32::EPSILON);
        assert!((shared.history().delta() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn default_equals_new() {
        let d = SharedDimensions::default();
        let n = SharedDimensions::new();
        assert_eq!(d, n);
    }

    #[test]
    fn clone_and_equality() {
        let s1 = SharedDimensions::new();
        let s2 = s1.clone();
        assert_eq!(s1, s2);
    }

    #[test]
    fn debug_format() {
        let shared = SharedDimensions::new();
        let debug = format!("{:?}", shared);
        assert!(debug.contains("SharedDimensions"));
    }

    #[test]
    fn tension_decays_faster_than_affinity() {
        let mut shared = SharedDimensions::new();
        shared.add_tension_delta(0.4);
        shared.add_affinity_delta(0.4);

        shared.apply_decay(Duration::days(7));

        // Tension should have decayed more
        assert!(shared.tension().delta() < shared.affinity().delta());
    }
}
