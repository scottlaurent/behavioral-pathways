//! Behavioral disposition state.
//!
//! Dispositions are baseline behavioral tendencies that influence how
//! an entity responds to events and interacts with others. They have
//! very slow decay rates (monthly) as they represent semi-stable traits.

use crate::state::StateValue;
use crate::types::Duration;
use serde::{Deserialize, Serialize};

/// Behavioral disposition state.
///
/// These represent stable behavioral tendencies that influence
/// decision-making, emotional regulation, and social interaction.
/// Dispositions change slowly over time and through significant events.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::state::Disposition;
///
/// let disposition = Disposition::new();
///
/// // High trust propensity means more willing to trust others
/// assert!(disposition.trust_propensity_effective() > 0.4);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Disposition {
    /// Self-regulation capacity.
    /// Range: 0 (impulsive) to 1 (highly regulated)
    /// Default decay half-life: 1 month
    impulse_control: StateValue,

    /// Concern for others' wellbeing.
    /// Range: 0 (callous) to 1 (highly empathetic)
    /// Default decay half-life: 1 month
    empathy: StateValue,

    /// Baseline hostility level.
    /// Range: 0 (peaceful) to 1 (hostile)
    /// Default decay half-life: 1 month
    aggression: StateValue,

    /// Accumulated sense of injustice.
    /// Range: 0 (no grievance) to 1 (extreme grievance)
    /// Default decay half-life: 1 week
    grievance: StateValue,

    /// Resistance to restrictions and control.
    /// Range: 0 (compliant) to 1 (highly reactant)
    /// Default decay half-life: 1 week
    reactance: StateValue,

    /// General willingness to trust others.
    /// Range: 0 (distrustful) to 1 (trusting)
    /// Default decay half-life: 1 year (stable personality trait)
    ///
    /// This is the trustor's dispositional propensity, independent of
    /// any specific relationship. Per Mayer's trust model, propensity is a
    /// stable personality trait that develops slowly through life experience,
    /// NOT a fluctuating state. It represents a general willingness to trust
    /// that a trustor carries across all relationships.
    trust_propensity: StateValue,
}

impl Disposition {
    /// Default decay half-life for impulse control (1 month).
    const IMPULSE_CONTROL_DECAY_HALF_LIFE: Duration = Duration::months(1);

    /// Default decay half-life for empathy (1 month).
    const EMPATHY_DECAY_HALF_LIFE: Duration = Duration::months(1);

    /// Default decay half-life for aggression (1 month).
    const AGGRESSION_DECAY_HALF_LIFE: Duration = Duration::months(1);

    /// Default decay half-life for grievance (1 week).
    const GRIEVANCE_DECAY_HALF_LIFE: Duration = Duration::weeks(1);

    /// Default decay half-life for reactance (1 week).
    const REACTANCE_DECAY_HALF_LIFE: Duration = Duration::weeks(1);

    /// Default decay half-life for trust propensity (1 year).
    /// This is much longer than other dispositions because propensity is a
    /// stable personality trait per Mayer's trust model, not a fluctuating state.
    const TRUST_PROPENSITY_DECAY_HALF_LIFE: Duration = Duration::years(1);

    /// Creates a new Disposition with healthy defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::state::Disposition;
    ///
    /// let disposition = Disposition::new();
    /// assert!(disposition.empathy_effective() > 0.5);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Disposition {
            impulse_control: StateValue::new(0.6)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(Self::IMPULSE_CONTROL_DECAY_HALF_LIFE),
            empathy: StateValue::new(0.7)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(Self::EMPATHY_DECAY_HALF_LIFE),
            aggression: StateValue::new(0.2)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(Self::AGGRESSION_DECAY_HALF_LIFE),
            grievance: StateValue::new(0.0)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(Self::GRIEVANCE_DECAY_HALF_LIFE),
            reactance: StateValue::new(0.0)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(Self::REACTANCE_DECAY_HALF_LIFE),
            trust_propensity: StateValue::new(0.5)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(Self::TRUST_PROPENSITY_DECAY_HALF_LIFE),
        }
    }

    // Builder methods

    /// Sets the base impulse control.
    #[must_use]
    pub fn with_impulse_control_base(mut self, value: f32) -> Self {
        self.impulse_control.set_base(value);
        self
    }

    /// Sets the base empathy.
    #[must_use]
    pub fn with_empathy_base(mut self, value: f32) -> Self {
        self.empathy.set_base(value);
        self
    }

    /// Sets the base aggression.
    #[must_use]
    pub fn with_aggression_base(mut self, value: f32) -> Self {
        self.aggression.set_base(value);
        self
    }

    /// Sets the base grievance.
    #[must_use]
    pub fn with_grievance_base(mut self, value: f32) -> Self {
        self.grievance.set_base(value);
        self
    }

    /// Sets the base reactance.
    #[must_use]
    pub fn with_reactance_base(mut self, value: f32) -> Self {
        self.reactance.set_base(value);
        self
    }

    /// Sets the base trust propensity.
    #[must_use]
    pub fn with_trust_propensity_base(mut self, value: f32) -> Self {
        self.trust_propensity.set_base(value);
        self
    }

    // Effective value accessors

    /// Returns the effective impulse control (base + delta).
    #[must_use]
    pub fn impulse_control_effective(&self) -> f32 {
        self.impulse_control.effective()
    }

    /// Returns the effective empathy (base + delta).
    #[must_use]
    pub fn empathy_effective(&self) -> f32 {
        self.empathy.effective()
    }

    /// Returns the effective aggression (base + delta).
    #[must_use]
    pub fn aggression_effective(&self) -> f32 {
        self.aggression.effective()
    }

    /// Returns the effective grievance (base + delta).
    #[must_use]
    pub fn grievance_effective(&self) -> f32 {
        self.grievance.effective()
    }

    /// Returns the effective reactance (base + delta).
    #[must_use]
    pub fn reactance_effective(&self) -> f32 {
        self.reactance.effective()
    }

    /// Returns the effective trust propensity (base + delta).
    #[must_use]
    pub fn trust_propensity_effective(&self) -> f32 {
        self.trust_propensity.effective()
    }

    // StateValue references

    /// Returns a reference to the impulse_control StateValue.
    #[must_use]
    pub fn impulse_control(&self) -> &StateValue {
        &self.impulse_control
    }

    /// Returns a reference to the empathy StateValue.
    #[must_use]
    pub fn empathy(&self) -> &StateValue {
        &self.empathy
    }

    /// Returns a reference to the aggression StateValue.
    #[must_use]
    pub fn aggression(&self) -> &StateValue {
        &self.aggression
    }

    /// Returns a reference to the grievance StateValue.
    #[must_use]
    pub fn grievance(&self) -> &StateValue {
        &self.grievance
    }

    /// Returns a reference to the reactance StateValue.
    #[must_use]
    pub fn reactance(&self) -> &StateValue {
        &self.reactance
    }

    /// Returns a reference to the trust_propensity StateValue.
    #[must_use]
    pub fn trust_propensity(&self) -> &StateValue {
        &self.trust_propensity
    }

    /// Returns a mutable reference to the impulse_control StateValue.
    pub fn impulse_control_mut(&mut self) -> &mut StateValue {
        &mut self.impulse_control
    }

    /// Returns a mutable reference to the empathy StateValue.
    pub fn empathy_mut(&mut self) -> &mut StateValue {
        &mut self.empathy
    }

    /// Returns a mutable reference to the aggression StateValue.
    pub fn aggression_mut(&mut self) -> &mut StateValue {
        &mut self.aggression
    }

    /// Returns a mutable reference to the grievance StateValue.
    pub fn grievance_mut(&mut self) -> &mut StateValue {
        &mut self.grievance
    }

    /// Returns a mutable reference to the reactance StateValue.
    pub fn reactance_mut(&mut self) -> &mut StateValue {
        &mut self.reactance
    }

    /// Returns a mutable reference to the trust_propensity StateValue.
    pub fn trust_propensity_mut(&mut self) -> &mut StateValue {
        &mut self.trust_propensity
    }

    // Delta modifiers

    /// Adds to the impulse control delta.
    pub fn add_impulse_control_delta(&mut self, amount: f32) {
        self.impulse_control.add_delta(amount);
    }

    /// Adds to the empathy delta.
    pub fn add_empathy_delta(&mut self, amount: f32) {
        self.empathy.add_delta(amount);
    }

    /// Adds to the aggression delta.
    pub fn add_aggression_delta(&mut self, amount: f32) {
        self.aggression.add_delta(amount);
    }

    /// Adds to the grievance delta.
    pub fn add_grievance_delta(&mut self, amount: f32) {
        self.grievance.add_delta(amount);
    }

    /// Adds to the reactance delta.
    pub fn add_reactance_delta(&mut self, amount: f32) {
        self.reactance.add_delta(amount);
    }

    /// Adds to the trust propensity delta.
    pub fn add_trust_propensity_delta(&mut self, amount: f32) {
        self.trust_propensity.add_delta(amount);
    }

    // Decay

    /// Applies decay to all disposition dimensions over the specified duration.
    pub fn apply_decay(&mut self, elapsed: Duration) {
        self.impulse_control.apply_decay(elapsed);
        self.empathy.apply_decay(elapsed);
        self.aggression.apply_decay(elapsed);
        self.grievance.apply_decay(elapsed);
        self.reactance.apply_decay(elapsed);
        self.trust_propensity.apply_decay(elapsed);
    }

    /// Resets all deltas to zero.
    pub fn reset_deltas(&mut self) {
        self.impulse_control.reset_delta();
        self.empathy.reset_delta();
        self.aggression.reset_delta();
        self.grievance.reset_delta();
        self.reactance.reset_delta();
        self.trust_propensity.reset_delta();
    }
}

impl Default for Disposition {
    fn default() -> Self {
        Disposition::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_healthy_defaults() {
        let disposition = Disposition::new();

        // High positive traits
        assert!(disposition.impulse_control_effective() > 0.5);
        assert!(disposition.empathy_effective() > 0.5);
        assert!(disposition.trust_propensity_effective() >= 0.5);

        // Low negative traits
        assert!(disposition.aggression_effective() < 0.5);
        assert!(disposition.grievance_effective() < 0.5);
        assert!(disposition.reactance_effective() < 0.5);
    }

    #[test]
    fn builder_methods_work() {
        let disposition = Disposition::new()
            .with_impulse_control_base(0.3)
            .with_empathy_base(0.4)
            .with_aggression_base(0.5)
            .with_grievance_base(0.6)
            .with_reactance_base(0.7)
            .with_trust_propensity_base(0.8);

        assert!((disposition.impulse_control().base() - 0.3).abs() < f32::EPSILON);
        assert!((disposition.empathy().base() - 0.4).abs() < f32::EPSILON);
        assert!((disposition.aggression().base() - 0.5).abs() < f32::EPSILON);
        assert!((disposition.grievance().base() - 0.6).abs() < f32::EPSILON);
        assert!((disposition.reactance().base() - 0.7).abs() < f32::EPSILON);
        assert!((disposition.trust_propensity().base() - 0.8).abs() < f32::EPSILON);
    }

    #[test]
    fn delta_modifiers_work() {
        let mut disposition = Disposition::new();

        disposition.add_grievance_delta(0.3);
        assert!((disposition.grievance().delta() - 0.3).abs() < f32::EPSILON);

        disposition.add_trust_propensity_delta(-0.2);
        assert!((disposition.trust_propensity().delta() - (-0.2)).abs() < f32::EPSILON);
    }

    #[test]
    fn grievance_decays_weekly() {
        let mut disposition = Disposition::new();
        disposition.add_grievance_delta(0.8);

        // After 1 week (one half-life), delta should be halved
        disposition.apply_decay(Duration::weeks(1));
        assert!((disposition.grievance().delta() - 0.4).abs() < 0.01);
    }

    #[test]
    fn trust_propensity_decays_yearly() {
        let mut disposition = Disposition::new();
        disposition.add_trust_propensity_delta(0.4);

        // After 1 year (one half-life), delta should be halved
        // This is much longer than other dispositions because propensity
        // is a stable personality trait per Mayer's trust model
        disposition.apply_decay(Duration::years(1));
        assert!((disposition.trust_propensity().delta() - 0.2).abs() < 0.01);
    }

    #[test]
    fn trust_propensity_stable_over_months() {
        let mut disposition = Disposition::new();
        disposition.add_trust_propensity_delta(0.4);

        // After 1 month, propensity should barely change (trait, not state)
        disposition.apply_decay(Duration::months(1));
        // With 1-year half-life, 1 month is 1/12 of a half-life
        // delta after = 0.4 * exp(-1/12 * ln(2)) ~= 0.38
        assert!(disposition.trust_propensity().delta() > 0.35);
    }

    #[test]
    fn reset_deltas_clears_all() {
        let mut disposition = Disposition::new();
        disposition.add_grievance_delta(0.5);
        disposition.add_reactance_delta(0.3);

        disposition.reset_deltas();

        assert!(disposition.grievance().delta().abs() < f32::EPSILON);
        assert!(disposition.reactance().delta().abs() < f32::EPSILON);
    }

    #[test]
    fn mutable_references_work() {
        let mut disposition = Disposition::new();
        disposition.aggression_mut().add_delta(0.2);
        assert!((disposition.aggression().delta() - 0.2).abs() < f32::EPSILON);
    }

    #[test]
    fn default_is_new() {
        let disposition = Disposition::default();
        assert!(disposition.empathy_effective() > 0.5);
    }

    #[test]
    fn clone_and_equality() {
        let d1 = Disposition::new().with_aggression_base(0.5);
        let d2 = d1.clone();
        assert_eq!(d1, d2);
    }

    #[test]
    fn debug_format() {
        let disposition = Disposition::new();
        let debug = format!("{:?}", disposition);
        assert!(debug.contains("Disposition"));
    }

    #[test]
    fn trust_propensity_is_dispositional() {
        // Trust propensity is the trustor's general tendency, not relationship-specific
        let disposition = Disposition::new().with_trust_propensity_base(0.8);
        assert!(disposition.trust_propensity_effective() > 0.7);
    }

    #[test]
    fn all_delta_modifiers_work() {
        let mut d = Disposition::new();

        d.add_impulse_control_delta(0.1);
        assert!((d.impulse_control().delta() - 0.1).abs() < f32::EPSILON);

        d.add_empathy_delta(0.2);
        assert!((d.empathy().delta() - 0.2).abs() < f32::EPSILON);

        d.add_aggression_delta(0.3);
        assert!((d.aggression().delta() - 0.3).abs() < f32::EPSILON);

        d.add_reactance_delta(0.4);
        assert!((d.reactance().delta() - 0.4).abs() < f32::EPSILON);
    }

    #[test]
    fn all_mutable_refs_work() {
        let mut d = Disposition::new();

        d.impulse_control_mut().add_delta(0.1);
        assert!((d.impulse_control().delta() - 0.1).abs() < f32::EPSILON);

        d.empathy_mut().add_delta(0.2);
        assert!((d.empathy().delta() - 0.2).abs() < f32::EPSILON);

        d.grievance_mut().add_delta(0.3);
        assert!((d.grievance().delta() - 0.3).abs() < f32::EPSILON);

        d.reactance_mut().add_delta(0.4);
        assert!((d.reactance().delta() - 0.4).abs() < f32::EPSILON);

        d.trust_propensity_mut().add_delta(-0.1);
        assert!((d.trust_propensity().delta() - (-0.1)).abs() < f32::EPSILON);
    }
}
