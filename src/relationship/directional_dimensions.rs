//! Directional (asymmetric) relationship dimensions.
//!
//! These dimensions may differ from one entity's perspective to another.
//! A's warmth toward B may differ from B's warmth toward A.

use crate::state::StateValue;
use crate::types::Duration;

/// Default decay half-life for directional dimensions (14 days).
const DEFAULT_DECAY_HALF_LIFE: Duration = Duration::days(14);

/// Directional dimensions from one entity toward another.
///
/// These are asymmetric - A's feelings toward B may differ from
/// B's feelings toward A.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::relationship::DirectionalDimensions;
///
/// let mut dims = DirectionalDimensions::new();
/// dims.add_warmth_delta(0.3);
/// assert!(dims.warmth_effective() > 0.3);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct DirectionalDimensions {
    /// Positive feeling toward the other entity.
    /// Range: 0 (cold) to 1 (warm)
    warmth: StateValue,

    /// Negative feeling toward the other entity.
    /// Range: 0 (no resentment) to 1 (high resentment)
    resentment: StateValue,

    /// Reliance on the other entity.
    /// Range: 0 (independent) to 1 (highly dependent)
    dependence: StateValue,

    /// Romantic or sexual interest.
    /// Range: 0 (none) to 1 (high attraction)
    attraction: StateValue,

    /// Emotional bonding and fear of loss.
    /// Range: 0 (detached) to 1 (strongly attached)
    attachment: StateValue,

    /// Possessiveness and rivalry.
    /// Range: 0 (none) to 1 (high jealousy)
    jealousy: StateValue,

    /// Threat perception.
    /// Range: 0 (no fear) to 1 (high fear)
    fear: StateValue,

    /// Sense of duty or authority pressure.
    /// Range: 0 (no obligation) to 1 (strong obligation)
    obligation: StateValue,
}

impl DirectionalDimensions {
    /// Creates new DirectionalDimensions with default values.
    ///
    /// Defaults represent a neutral starting point with slight warmth.
    #[must_use]
    pub fn new() -> Self {
        DirectionalDimensions {
            warmth: StateValue::new(0.2)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(DEFAULT_DECAY_HALF_LIFE),
            resentment: StateValue::new(0.0)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(DEFAULT_DECAY_HALF_LIFE),
            dependence: StateValue::new(0.0)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(DEFAULT_DECAY_HALF_LIFE),
            attraction: StateValue::new(0.0)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(DEFAULT_DECAY_HALF_LIFE),
            attachment: StateValue::new(0.0)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(Duration::days(30)), // Attachment decays slower
            jealousy: StateValue::new(0.0)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(Duration::days(7)), // Jealousy decays faster
            fear: StateValue::new(0.0)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(Duration::days(7)), // Fear decays faster
            obligation: StateValue::new(0.0)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(Duration::days(30)), // Obligation is more stable
        }
    }

    // Effective value accessors

    /// Returns the effective warmth (base + delta).
    #[must_use]
    pub fn warmth_effective(&self) -> f32 {
        self.warmth.effective()
    }

    /// Returns the effective resentment (base + delta).
    #[must_use]
    pub fn resentment_effective(&self) -> f32 {
        self.resentment.effective()
    }

    /// Returns the effective dependence (base + delta).
    #[must_use]
    pub fn dependence_effective(&self) -> f32 {
        self.dependence.effective()
    }

    /// Returns the effective attraction (base + delta).
    #[must_use]
    pub fn attraction_effective(&self) -> f32 {
        self.attraction.effective()
    }

    /// Returns the effective attachment (base + delta).
    #[must_use]
    pub fn attachment_effective(&self) -> f32 {
        self.attachment.effective()
    }

    /// Returns the effective jealousy (base + delta).
    #[must_use]
    pub fn jealousy_effective(&self) -> f32 {
        self.jealousy.effective()
    }

    /// Returns the effective fear (base + delta).
    #[must_use]
    pub fn fear_effective(&self) -> f32 {
        self.fear.effective()
    }

    /// Returns the effective obligation (base + delta).
    #[must_use]
    pub fn obligation_effective(&self) -> f32 {
        self.obligation.effective()
    }

    // StateValue references

    /// Returns a reference to the warmth StateValue.
    #[must_use]
    pub fn warmth(&self) -> &StateValue {
        &self.warmth
    }

    /// Returns a reference to the resentment StateValue.
    #[must_use]
    pub fn resentment(&self) -> &StateValue {
        &self.resentment
    }

    /// Returns a reference to the dependence StateValue.
    #[must_use]
    pub fn dependence(&self) -> &StateValue {
        &self.dependence
    }

    /// Returns a reference to the attraction StateValue.
    #[must_use]
    pub fn attraction(&self) -> &StateValue {
        &self.attraction
    }

    /// Returns a reference to the attachment StateValue.
    #[must_use]
    pub fn attachment(&self) -> &StateValue {
        &self.attachment
    }

    /// Returns a reference to the jealousy StateValue.
    #[must_use]
    pub fn jealousy(&self) -> &StateValue {
        &self.jealousy
    }

    /// Returns a reference to the fear StateValue.
    #[must_use]
    pub fn fear(&self) -> &StateValue {
        &self.fear
    }

    /// Returns a reference to the obligation StateValue.
    #[must_use]
    pub fn obligation(&self) -> &StateValue {
        &self.obligation
    }

    // Mutable references

    /// Returns a mutable reference to the warmth StateValue.
    pub fn warmth_mut(&mut self) -> &mut StateValue {
        &mut self.warmth
    }

    /// Returns a mutable reference to the resentment StateValue.
    pub fn resentment_mut(&mut self) -> &mut StateValue {
        &mut self.resentment
    }

    /// Returns a mutable reference to the dependence StateValue.
    pub fn dependence_mut(&mut self) -> &mut StateValue {
        &mut self.dependence
    }

    /// Returns a mutable reference to the attraction StateValue.
    pub fn attraction_mut(&mut self) -> &mut StateValue {
        &mut self.attraction
    }

    /// Returns a mutable reference to the attachment StateValue.
    pub fn attachment_mut(&mut self) -> &mut StateValue {
        &mut self.attachment
    }

    /// Returns a mutable reference to the jealousy StateValue.
    pub fn jealousy_mut(&mut self) -> &mut StateValue {
        &mut self.jealousy
    }

    /// Returns a mutable reference to the fear StateValue.
    pub fn fear_mut(&mut self) -> &mut StateValue {
        &mut self.fear
    }

    /// Returns a mutable reference to the obligation StateValue.
    pub fn obligation_mut(&mut self) -> &mut StateValue {
        &mut self.obligation
    }

    // Delta modifiers

    /// Adds to the warmth delta.
    pub fn add_warmth_delta(&mut self, amount: f32) {
        self.warmth.add_delta(amount);
    }

    /// Adds to the resentment delta.
    pub fn add_resentment_delta(&mut self, amount: f32) {
        self.resentment.add_delta(amount);
    }

    /// Adds to the dependence delta.
    pub fn add_dependence_delta(&mut self, amount: f32) {
        self.dependence.add_delta(amount);
    }

    /// Adds to the attraction delta.
    pub fn add_attraction_delta(&mut self, amount: f32) {
        self.attraction.add_delta(amount);
    }

    /// Adds to the attachment delta.
    pub fn add_attachment_delta(&mut self, amount: f32) {
        self.attachment.add_delta(amount);
    }

    /// Adds to the jealousy delta.
    pub fn add_jealousy_delta(&mut self, amount: f32) {
        self.jealousy.add_delta(amount);
    }

    /// Adds to the fear delta.
    pub fn add_fear_delta(&mut self, amount: f32) {
        self.fear.add_delta(amount);
    }

    /// Adds to the obligation delta.
    pub fn add_obligation_delta(&mut self, amount: f32) {
        self.obligation.add_delta(amount);
    }

    // Decay

    /// Applies decay to all directional dimensions over the specified duration.
    pub fn apply_decay(&mut self, elapsed: Duration) {
        self.warmth.apply_decay(elapsed);
        self.resentment.apply_decay(elapsed);
        self.dependence.apply_decay(elapsed);
        self.attraction.apply_decay(elapsed);
        self.attachment.apply_decay(elapsed);
        self.jealousy.apply_decay(elapsed);
        self.fear.apply_decay(elapsed);
        self.obligation.apply_decay(elapsed);
    }

    /// Resets all deltas to zero.
    pub fn reset_deltas(&mut self) {
        self.warmth.reset_delta();
        self.resentment.reset_delta();
        self.dependence.reset_delta();
        self.attraction.reset_delta();
        self.attachment.reset_delta();
        self.jealousy.reset_delta();
        self.fear.reset_delta();
        self.obligation.reset_delta();
    }
}

impl Default for DirectionalDimensions {
    fn default() -> Self {
        DirectionalDimensions::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_default_values() {
        let dims = DirectionalDimensions::new();
        assert!((dims.warmth_effective() - 0.2).abs() < f32::EPSILON);
        assert!(dims.resentment_effective().abs() < f32::EPSILON);
        assert!(dims.dependence_effective().abs() < f32::EPSILON);
        assert!(dims.attraction_effective().abs() < f32::EPSILON);
        assert!(dims.attachment_effective().abs() < f32::EPSILON);
        assert!(dims.jealousy_effective().abs() < f32::EPSILON);
        assert!(dims.fear_effective().abs() < f32::EPSILON);
        assert!(dims.obligation_effective().abs() < f32::EPSILON);
    }

    #[test]
    fn add_warmth_delta() {
        let mut dims = DirectionalDimensions::new();
        dims.add_warmth_delta(0.3);
        assert!((dims.warmth().delta() - 0.3).abs() < f32::EPSILON);
    }

    #[test]
    fn add_resentment_delta() {
        let mut dims = DirectionalDimensions::new();
        dims.add_resentment_delta(0.4);
        assert!((dims.resentment().delta() - 0.4).abs() < f32::EPSILON);
    }

    #[test]
    fn add_dependence_delta() {
        let mut dims = DirectionalDimensions::new();
        dims.add_dependence_delta(0.2);
        assert!((dims.dependence().delta() - 0.2).abs() < f32::EPSILON);
    }

    #[test]
    fn add_attraction_delta() {
        let mut dims = DirectionalDimensions::new();
        dims.add_attraction_delta(0.5);
        assert!((dims.attraction().delta() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn add_attachment_delta() {
        let mut dims = DirectionalDimensions::new();
        dims.add_attachment_delta(0.3);
        assert!((dims.attachment().delta() - 0.3).abs() < f32::EPSILON);
    }

    #[test]
    fn add_jealousy_delta() {
        let mut dims = DirectionalDimensions::new();
        dims.add_jealousy_delta(0.2);
        assert!((dims.jealousy().delta() - 0.2).abs() < f32::EPSILON);
    }

    #[test]
    fn add_fear_delta() {
        let mut dims = DirectionalDimensions::new();
        dims.add_fear_delta(0.4);
        assert!((dims.fear().delta() - 0.4).abs() < f32::EPSILON);
    }

    #[test]
    fn add_obligation_delta() {
        let mut dims = DirectionalDimensions::new();
        dims.add_obligation_delta(0.3);
        assert!((dims.obligation().delta() - 0.3).abs() < f32::EPSILON);
    }

    #[test]
    fn warmth_decays_over_14_days() {
        let mut dims = DirectionalDimensions::new();
        dims.add_warmth_delta(0.4);
        dims.apply_decay(Duration::days(14));
        assert!((dims.warmth().delta() - 0.2).abs() < 0.01);
    }

    #[test]
    fn fear_decays_over_7_days() {
        let mut dims = DirectionalDimensions::new();
        dims.add_fear_delta(0.4);
        dims.apply_decay(Duration::days(7));
        assert!((dims.fear().delta() - 0.2).abs() < 0.01);
    }

    #[test]
    fn jealousy_decays_over_7_days() {
        let mut dims = DirectionalDimensions::new();
        dims.add_jealousy_delta(0.4);
        dims.apply_decay(Duration::days(7));
        assert!((dims.jealousy().delta() - 0.2).abs() < 0.01);
    }

    #[test]
    fn attachment_decays_over_30_days() {
        let mut dims = DirectionalDimensions::new();
        dims.add_attachment_delta(0.4);
        dims.apply_decay(Duration::days(30));
        assert!((dims.attachment().delta() - 0.2).abs() < 0.01);
    }

    #[test]
    fn obligation_decays_over_30_days() {
        let mut dims = DirectionalDimensions::new();
        dims.add_obligation_delta(0.4);
        dims.apply_decay(Duration::days(30));
        assert!((dims.obligation().delta() - 0.2).abs() < 0.01);
    }

    #[test]
    fn fear_decays_faster_than_warmth() {
        let mut dims = DirectionalDimensions::new();
        dims.add_fear_delta(0.4);
        dims.add_warmth_delta(0.4);

        dims.apply_decay(Duration::days(7));

        // Fear should have decayed more (half-life reached)
        assert!(dims.fear().delta() < dims.warmth().delta());
    }

    #[test]
    fn reset_deltas() {
        let mut dims = DirectionalDimensions::new();
        dims.add_warmth_delta(0.3);
        dims.add_fear_delta(0.2);

        dims.reset_deltas();

        assert!(dims.warmth().delta().abs() < f32::EPSILON);
        assert!(dims.fear().delta().abs() < f32::EPSILON);
    }

    #[test]
    fn mutable_references_work() {
        let mut dims = DirectionalDimensions::new();
        dims.warmth_mut().add_delta(0.1);
        dims.resentment_mut().add_delta(0.2);
        dims.dependence_mut().add_delta(0.3);
        dims.attraction_mut().add_delta(0.4);
        dims.attachment_mut().add_delta(0.5);
        dims.jealousy_mut().add_delta(0.1);
        dims.fear_mut().add_delta(0.2);
        dims.obligation_mut().add_delta(0.3);

        assert!((dims.warmth().delta() - 0.1).abs() < f32::EPSILON);
        assert!((dims.resentment().delta() - 0.2).abs() < f32::EPSILON);
        assert!((dims.dependence().delta() - 0.3).abs() < f32::EPSILON);
        assert!((dims.attraction().delta() - 0.4).abs() < f32::EPSILON);
        assert!((dims.attachment().delta() - 0.5).abs() < f32::EPSILON);
        assert!((dims.jealousy().delta() - 0.1).abs() < f32::EPSILON);
        assert!((dims.fear().delta() - 0.2).abs() < f32::EPSILON);
        assert!((dims.obligation().delta() - 0.3).abs() < f32::EPSILON);
    }

    #[test]
    fn default_equals_new() {
        let d = DirectionalDimensions::default();
        let n = DirectionalDimensions::new();
        assert_eq!(d, n);
    }

    #[test]
    fn clone_and_equality() {
        let d1 = DirectionalDimensions::new();
        let d2 = d1.clone();
        assert_eq!(d1, d2);
    }

    #[test]
    fn debug_format() {
        let dims = DirectionalDimensions::new();
        let debug = format!("{:?}", dims);
        assert!(debug.contains("DirectionalDimensions"));
    }
}
