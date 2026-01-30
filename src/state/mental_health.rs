//! Mental health state based on Joiner's Interpersonal Theory of Suicide.
//!
//! This module implements the ITS model with:
//! - Thwarted Belongingness (TB): Computed from loneliness and perceived reciprocal caring
//! - Perceived Burdensomeness (PB): Computed from perceived liability and self-hate
//! - Acquired Capability (AC): Habituation to pain/fear; NEVER DECAYS
//! - Interpersonal Hopelessness: Perceived permanence of TB/PB (not general hopelessness)
//!
//! ITS Formulas:
//! - TB = (loneliness + (1 - perceived_reciprocal_caring)) / 2
//! - PB = perceived_liability * self_hate
//! - Suicidal Desire = TB * PB (when both TB and PB are above threshold AND hopelessness > threshold)
//! - Attempt Risk = Desire * Acquired Capability

use crate::state::{SocialCognition, StateValue};
use crate::types::Duration;
use serde::{Deserialize, Serialize};

/// Threshold above which Thwarted Belongingness is considered present.
/// Per ITS, TB must be significantly elevated to contribute to suicidal desire.
pub const TB_PRESENT_THRESHOLD: f32 = 0.5;

/// Threshold above which Perceived Burdensomeness is considered present.
/// Per ITS, PB must be significantly elevated to contribute to suicidal desire.
pub const PB_PRESENT_THRESHOLD: f32 = 0.5;

/// Threshold above which interpersonal hopelessness activates suicidal desire.
/// This represents the perceived permanence of TB/PB states.
pub const HOPELESSNESS_THRESHOLD: f32 = 0.5;

/// Mental health state with ITS (Interpersonal Theory of Suicide) factors.
///
/// The ITS model states that suicidal desire requires:
/// 1. Thwarted Belongingness (TB) - feeling disconnected from others
/// 2. Perceived Burdensomeness (PB) - feeling like a burden to others
/// 3. Interpersonal Hopelessness - believing these states are permanent
///
/// The capability for lethal self-harm is separate from desire and
/// represented by Acquired Capability (AC), which never decays.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::state::{MentalHealth, SocialCognition};
///
/// let mental_health = MentalHealth::new();
/// let social = SocialCognition::new()
///     .with_loneliness_base(0.3)
///     .with_perceived_reciprocal_caring_base(0.7);
///
/// // Compute TB from social cognition
/// let tb = mental_health.compute_thwarted_belongingness(&social);
/// assert!(tb < 0.5); // Low loneliness + high caring = low TB
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MentalHealth {
    /// Depression severity.
    /// Range: 0 (not depressed) to 1 (severe depression)
    /// Default decay half-life: 1 week
    depression: StateValue,

    /// Sense of personal value.
    /// Range: 0 (worthless) to 1 (high self-worth)
    /// Default decay half-life: 3 days
    self_worth: StateValue,

    /// Cognitive belief about future (general hopelessness).
    /// Range: 0 (hopeful) to 1 (hopeless)
    /// Default decay half-life: 3 days
    hopelessness: StateValue,

    /// Perceived permanence of interpersonal pain (TB/PB states).
    /// This is distinct from general hopelessness - it's specifically
    /// about whether the person believes their interpersonal situation
    /// can ever improve.
    /// Range: 0 (changeable) to 1 (permanent)
    /// Default decay half-life: 2 days
    interpersonal_hopelessness: StateValue,

    /// Habituation to pain and fear of death.
    /// This is a trait-like accumulation that NEVER DECAYS.
    /// Range: 0 (fearful of pain/death) to 1 (habituated)
    /// Decay: None (permanent)
    acquired_capability: StateValue,
}

impl MentalHealth {
    /// Default decay half-life for depression (1 week).
    const DEPRESSION_DECAY_HALF_LIFE: Duration = Duration::weeks(1);

    /// Default decay half-life for self-worth (3 days).
    const SELF_WORTH_DECAY_HALF_LIFE: Duration = Duration::days(3);

    /// Default decay half-life for hopelessness (3 days).
    const HOPELESSNESS_DECAY_HALF_LIFE: Duration = Duration::days(3);

    /// Default decay half-life for interpersonal hopelessness (2 days).
    const INTERPERSONAL_HOPELESSNESS_DECAY_HALF_LIFE: Duration = Duration::days(2);

    /// Creates a new MentalHealth with healthy defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::state::MentalHealth;
    ///
    /// let mh = MentalHealth::new();
    /// assert!(mh.depression_effective() < 0.3);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        MentalHealth {
            depression: StateValue::new(0.1)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(Self::DEPRESSION_DECAY_HALF_LIFE),
            self_worth: StateValue::new(0.6)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(Self::SELF_WORTH_DECAY_HALF_LIFE),
            hopelessness: StateValue::new(0.1)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(Self::HOPELESSNESS_DECAY_HALF_LIFE),
            interpersonal_hopelessness: StateValue::new(0.1)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(Self::INTERPERSONAL_HOPELESSNESS_DECAY_HALF_LIFE),
            // AC never decays - uses new_no_decay
            acquired_capability: StateValue::new_no_decay(0.0).with_bounds(0.0, 1.0),
        }
    }

    // ITS Compute Methods

    /// Computes Thwarted Belongingness from social cognition.
    ///
    /// Formula: TB = (loneliness + (1 - perceived_reciprocal_caring)) / 2
    ///
    /// High loneliness and/or low perceived caring results in high TB.
    /// This is an additive formula, meaning high loneliness can create TB
    /// even with high caring, and vice versa.
    ///
    /// # Arguments
    ///
    /// * `social` - The social cognition state containing loneliness and perceived_reciprocal_caring
    ///
    /// # Returns
    ///
    /// TB value from 0.0 to 1.0
    #[must_use]
    pub fn compute_thwarted_belongingness(&self, social: &SocialCognition) -> f32 {
        let loneliness = social.loneliness_effective();
        let caring = social.perceived_reciprocal_caring_effective();

        // TB = (loneliness + (1 - caring)) / 2
        let tb = (loneliness + (1.0 - caring)) / 2.0;
        tb.clamp(0.0, 1.0)
    }

    /// Computes Perceived Burdensomeness from social cognition.
    ///
    /// Formula: PB = perceived_liability * self_hate
    ///
    /// This is a multiplicative formula, meaning BOTH liability perception
    /// AND self-hate are required for PB to be present. Liability alone
    /// is insufficient without self-hate, and vice versa.
    ///
    /// # Arguments
    ///
    /// * `social` - The social cognition state containing perceived_liability and self_hate
    ///
    /// # Returns
    ///
    /// PB value from 0.0 to 1.0
    #[must_use]
    pub fn compute_perceived_burdensomeness(&self, social: &SocialCognition) -> f32 {
        let liability = social.perceived_liability_effective();
        let self_hate = social.self_hate_effective();

        // PB = liability * self_hate (multiplicative)
        let pb = liability * self_hate;
        pb.clamp(0.0, 1.0)
    }

    /// Computes suicidal desire from TB, PB, and interpersonal hopelessness.
    ///
    /// Per ITS, suicidal desire requires:
    /// 1. Thwarted Belongingness above threshold
    /// 2. Perceived Burdensomeness above threshold
    /// 3. Interpersonal hopelessness above threshold (belief that TB/PB are permanent)
    ///
    /// If any condition is not met, desire is zero.
    ///
    /// # Arguments
    ///
    /// * `social` - The social cognition state for computing TB and PB
    ///
    /// # Returns
    ///
    /// Suicidal desire from 0.0 to 1.0
    #[must_use]
    pub fn compute_suicidal_desire(&self, social: &SocialCognition) -> f32 {
        let tb = self.compute_thwarted_belongingness(social);
        let pb = self.compute_perceived_burdensomeness(social);
        let hopelessness = self.interpersonal_hopelessness.effective();

        // All three conditions must be met
        if tb < TB_PRESENT_THRESHOLD
            || pb < PB_PRESENT_THRESHOLD
            || hopelessness < HOPELESSNESS_THRESHOLD
        {
            return 0.0;
        }

        // Desire = TB * PB (both already above threshold)
        let desire = tb * pb;
        desire.clamp(0.0, 1.0)
    }

    /// Computes attempt risk from desire and acquired capability.
    ///
    /// Risk = Desire * Acquired Capability
    ///
    /// High desire with low capability means ideation without means.
    /// High capability with low desire is "dormant" risk - capability
    /// that could become dangerous if desire increases.
    ///
    /// # Arguments
    ///
    /// * `social` - The social cognition state for computing desire
    ///
    /// # Returns
    ///
    /// Attempt risk from 0.0 to 1.0
    #[must_use]
    pub fn compute_attempt_risk(&self, social: &SocialCognition) -> f32 {
        let desire = self.compute_suicidal_desire(social);
        let capability = self.acquired_capability.effective();

        let risk = desire * capability;
        risk.clamp(0.0, 1.0)
    }

    /// Checks if Thwarted Belongingness is above the present threshold.
    #[must_use]
    pub fn is_tb_present(&self, social: &SocialCognition) -> bool {
        self.compute_thwarted_belongingness(social) >= TB_PRESENT_THRESHOLD
    }

    /// Checks if Perceived Burdensomeness is above the present threshold.
    #[must_use]
    pub fn is_pb_present(&self, social: &SocialCognition) -> bool {
        self.compute_perceived_burdensomeness(social) >= PB_PRESENT_THRESHOLD
    }

    /// Checks if interpersonal hopelessness is above the threshold.
    #[must_use]
    pub fn is_interpersonal_hopelessness_present(&self) -> bool {
        self.interpersonal_hopelessness.effective() >= HOPELESSNESS_THRESHOLD
    }

    // Builder methods

    /// Sets the base depression.
    #[must_use]
    pub fn with_depression_base(mut self, value: f32) -> Self {
        self.depression.set_base(value);
        self
    }

    /// Sets the base self-worth.
    #[must_use]
    pub fn with_self_worth_base(mut self, value: f32) -> Self {
        self.self_worth.set_base(value);
        self
    }

    /// Sets the base hopelessness.
    #[must_use]
    pub fn with_hopelessness_base(mut self, value: f32) -> Self {
        self.hopelessness.set_base(value);
        self
    }

    /// Sets the base interpersonal hopelessness.
    #[must_use]
    pub fn with_interpersonal_hopelessness_base(mut self, value: f32) -> Self {
        self.interpersonal_hopelessness.set_base(value);
        self
    }

    /// Sets the base acquired capability.
    #[must_use]
    pub fn with_acquired_capability_base(mut self, value: f32) -> Self {
        self.acquired_capability.set_base(value);
        self
    }

    // Effective value accessors

    /// Returns the effective depression (base + delta).
    #[must_use]
    pub fn depression_effective(&self) -> f32 {
        self.depression.effective()
    }

    /// Returns the effective self-worth (base + delta).
    #[must_use]
    pub fn self_worth_effective(&self) -> f32 {
        self.self_worth.effective()
    }

    /// Returns the effective hopelessness (base + delta).
    #[must_use]
    pub fn hopelessness_effective(&self) -> f32 {
        self.hopelessness.effective()
    }

    /// Returns the effective interpersonal hopelessness (base + delta).
    #[must_use]
    pub fn interpersonal_hopelessness_effective(&self) -> f32 {
        self.interpersonal_hopelessness.effective()
    }

    /// Returns the effective acquired capability (base + delta).
    #[must_use]
    pub fn acquired_capability_effective(&self) -> f32 {
        self.acquired_capability.effective()
    }

    // StateValue references

    /// Returns a reference to the depression StateValue.
    #[must_use]
    pub fn depression(&self) -> &StateValue {
        &self.depression
    }

    /// Returns a reference to the self_worth StateValue.
    #[must_use]
    pub fn self_worth(&self) -> &StateValue {
        &self.self_worth
    }

    /// Returns a reference to the hopelessness StateValue.
    #[must_use]
    pub fn hopelessness(&self) -> &StateValue {
        &self.hopelessness
    }

    /// Returns a reference to the interpersonal_hopelessness StateValue.
    #[must_use]
    pub fn interpersonal_hopelessness(&self) -> &StateValue {
        &self.interpersonal_hopelessness
    }

    /// Returns a reference to the acquired_capability StateValue.
    #[must_use]
    pub fn acquired_capability(&self) -> &StateValue {
        &self.acquired_capability
    }

    /// Returns a mutable reference to the depression StateValue.
    pub fn depression_mut(&mut self) -> &mut StateValue {
        &mut self.depression
    }

    /// Returns a mutable reference to the self_worth StateValue.
    pub fn self_worth_mut(&mut self) -> &mut StateValue {
        &mut self.self_worth
    }

    /// Returns a mutable reference to the hopelessness StateValue.
    pub fn hopelessness_mut(&mut self) -> &mut StateValue {
        &mut self.hopelessness
    }

    /// Returns a mutable reference to the interpersonal_hopelessness StateValue.
    pub fn interpersonal_hopelessness_mut(&mut self) -> &mut StateValue {
        &mut self.interpersonal_hopelessness
    }

    /// Returns a mutable reference to the acquired_capability StateValue.
    pub fn acquired_capability_mut(&mut self) -> &mut StateValue {
        &mut self.acquired_capability
    }

    // Delta modifiers

    /// Adds to the depression delta.
    pub fn add_depression_delta(&mut self, amount: f32) {
        self.depression.add_delta(amount);
    }

    /// Adds to the self-worth delta.
    pub fn add_self_worth_delta(&mut self, amount: f32) {
        self.self_worth.add_delta(amount);
    }

    /// Adds to the hopelessness delta.
    pub fn add_hopelessness_delta(&mut self, amount: f32) {
        self.hopelessness.add_delta(amount);
    }

    /// Adds to the interpersonal hopelessness delta.
    pub fn add_interpersonal_hopelessness_delta(&mut self, amount: f32) {
        self.interpersonal_hopelessness.add_delta(amount);
    }

    /// Adds to the acquired capability delta.
    ///
    /// Note: AC never decays, so delta additions are permanent.
    pub fn add_acquired_capability_delta(&mut self, amount: f32) {
        self.acquired_capability.add_delta(amount);
    }

    // Decay

    /// Applies decay to mental health dimensions over the specified duration.
    ///
    /// Note: Acquired Capability NEVER decays.
    pub fn apply_decay(&mut self, elapsed: Duration) {
        self.depression.apply_decay(elapsed);
        self.self_worth.apply_decay(elapsed);
        self.hopelessness.apply_decay(elapsed);
        self.interpersonal_hopelessness.apply_decay(elapsed);
        // acquired_capability intentionally not decayed
    }

    /// Resets deltas for decaying dimensions only.
    ///
    /// Note: AC delta is not reset as it's permanent.
    pub fn reset_deltas(&mut self) {
        self.depression.reset_delta();
        self.self_worth.reset_delta();
        self.hopelessness.reset_delta();
        self.interpersonal_hopelessness.reset_delta();
        // acquired_capability intentionally not reset
    }
}

impl Default for MentalHealth {
    fn default() -> Self {
        MentalHealth::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::SocialCognition;

    #[test]
    fn thwarted_belongingness_additive_formula() {
        // TB = (loneliness + (1 - perceived_reciprocal_caring)) / 2
        let mh = MentalHealth::new();

        // High loneliness, low caring
        let social = SocialCognition::new()
            .with_loneliness_base(0.8)
            .with_perceived_reciprocal_caring_base(0.2);
        let tb = mh.compute_thwarted_belongingness(&social);
        // TB = (0.8 + (1 - 0.2)) / 2 = (0.8 + 0.8) / 2 = 0.8
        assert!((tb - 0.8).abs() < 0.01);

        // Low loneliness, high caring
        let social2 = SocialCognition::new()
            .with_loneliness_base(0.2)
            .with_perceived_reciprocal_caring_base(0.8);
        let tb2 = mh.compute_thwarted_belongingness(&social2);
        // TB = (0.2 + (1 - 0.8)) / 2 = (0.2 + 0.2) / 2 = 0.2
        assert!((tb2 - 0.2).abs() < 0.01);
    }

    #[test]
    fn perceived_burdensomeness_multiplicative_formula() {
        // PB = perceived_liability * self_hate
        let mh = MentalHealth::new();

        // High liability, high self-hate
        let social = SocialCognition::new()
            .with_perceived_liability_base(0.8)
            .with_self_hate_base(0.8);
        let pb = mh.compute_perceived_burdensomeness(&social);
        // PB = 0.8 * 0.8 = 0.64
        assert!((pb - 0.64).abs() < 0.01);

        // High liability, low self-hate - PB should be low
        let social2 = SocialCognition::new()
            .with_perceived_liability_base(0.8)
            .with_self_hate_base(0.1);
        let pb2 = mh.compute_perceived_burdensomeness(&social2);
        // PB = 0.8 * 0.1 = 0.08
        assert!((pb2 - 0.08).abs() < 0.01);
    }

    #[test]
    fn acquired_capability_never_decays() {
        let mut mh = MentalHealth::new();
        mh.add_acquired_capability_delta(0.5);

        // Apply decay over a very long time
        mh.apply_decay(Duration::years(100));

        // AC delta should remain unchanged
        assert!((mh.acquired_capability().delta() - 0.5).abs() < f32::EPSILON);
        assert!(!mh.acquired_capability().decays());
    }

    #[test]
    fn desire_requires_tb_pb_hopelessness() {
        let mut mh = MentalHealth::new().with_interpersonal_hopelessness_base(0.6); // Above threshold

        // High TB and PB inputs
        let social = SocialCognition::new()
            .with_loneliness_base(0.9)
            .with_perceived_reciprocal_caring_base(0.1)
            .with_perceived_liability_base(0.9)
            .with_self_hate_base(0.9);

        // With all conditions met, desire should be non-zero
        let desire = mh.compute_suicidal_desire(&social);
        assert!(desire > 0.0);

        // Now remove hopelessness - desire should be zero
        mh = MentalHealth::new().with_interpersonal_hopelessness_base(0.2); // Below threshold
        let desire2 = mh.compute_suicidal_desire(&social);
        assert!((desire2 - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn interpersonal_hopelessness_measures_permanence() {
        // Test that interpersonal hopelessness is about TB/PB permanence
        let mh = MentalHealth::new().with_interpersonal_hopelessness_base(0.8);

        assert!(mh.is_interpersonal_hopelessness_present());

        // General hopelessness is separate
        let mh2 = MentalHealth::new()
            .with_hopelessness_base(0.8)
            .with_interpersonal_hopelessness_base(0.2);

        assert!(!mh2.is_interpersonal_hopelessness_present());
    }

    #[test]
    fn attempt_risk_requires_desire_and_capability() {
        let mh = MentalHealth::new()
            .with_interpersonal_hopelessness_base(0.7)
            .with_acquired_capability_base(0.8);

        // High TB, PB, hopelessness, capability
        let social = SocialCognition::new()
            .with_loneliness_base(0.9)
            .with_perceived_reciprocal_caring_base(0.1)
            .with_perceived_liability_base(0.9)
            .with_self_hate_base(0.9);

        let risk = mh.compute_attempt_risk(&social);
        assert!(risk > 0.0);

        // Zero capability means zero risk even with desire
        let mh2 = MentalHealth::new()
            .with_interpersonal_hopelessness_base(0.7)
            .with_acquired_capability_base(0.0);
        let risk2 = mh2.compute_attempt_risk(&social);
        assert!((risk2 - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn tb_threshold_check() {
        let mh = MentalHealth::new();

        let low_tb_social = SocialCognition::new()
            .with_loneliness_base(0.2)
            .with_perceived_reciprocal_caring_base(0.8);
        assert!(!mh.is_tb_present(&low_tb_social));

        let high_tb_social = SocialCognition::new()
            .with_loneliness_base(0.9)
            .with_perceived_reciprocal_caring_base(0.1);
        assert!(mh.is_tb_present(&high_tb_social));
    }

    #[test]
    fn pb_threshold_check() {
        let mh = MentalHealth::new();

        let low_pb_social = SocialCognition::new()
            .with_perceived_liability_base(0.2)
            .with_self_hate_base(0.2);
        assert!(!mh.is_pb_present(&low_pb_social));

        let high_pb_social = SocialCognition::new()
            .with_perceived_liability_base(0.8)
            .with_self_hate_base(0.8);
        assert!(mh.is_pb_present(&high_pb_social));
    }

    #[test]
    fn new_creates_healthy_defaults() {
        let mh = MentalHealth::new();

        assert!(mh.depression_effective() < 0.3);
        assert!(mh.self_worth_effective() > 0.5);
        assert!(mh.hopelessness_effective() < 0.3);
        assert!(mh.interpersonal_hopelessness_effective() < 0.3);
        assert!(mh.acquired_capability_effective() < 0.1);
    }

    #[test]
    fn builder_methods_work() {
        let mh = MentalHealth::new()
            .with_depression_base(0.5)
            .with_self_worth_base(0.3)
            .with_hopelessness_base(0.4)
            .with_interpersonal_hopelessness_base(0.6)
            .with_acquired_capability_base(0.2);

        assert!((mh.depression().base() - 0.5).abs() < f32::EPSILON);
        assert!((mh.self_worth().base() - 0.3).abs() < f32::EPSILON);
        assert!((mh.hopelessness().base() - 0.4).abs() < f32::EPSILON);
        assert!((mh.interpersonal_hopelessness().base() - 0.6).abs() < f32::EPSILON);
        assert!((mh.acquired_capability().base() - 0.2).abs() < f32::EPSILON);
    }

    #[test]
    fn delta_modifiers_work() {
        let mut mh = MentalHealth::new();

        mh.add_depression_delta(0.3);
        assert!((mh.depression().delta() - 0.3).abs() < f32::EPSILON);

        mh.add_self_worth_delta(-0.2);
        assert!((mh.self_worth().delta() - (-0.2)).abs() < f32::EPSILON);
    }

    #[test]
    fn decay_does_not_affect_ac() {
        let mut mh = MentalHealth::new();
        mh.add_depression_delta(0.5);
        mh.add_acquired_capability_delta(0.5);

        mh.apply_decay(Duration::weeks(4));

        // Depression should have decayed significantly
        assert!(mh.depression().delta() < 0.3);

        // AC should not have decayed at all
        assert!((mh.acquired_capability().delta() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn reset_deltas_does_not_affect_ac() {
        let mut mh = MentalHealth::new();
        mh.add_depression_delta(0.5);
        mh.add_acquired_capability_delta(0.5);

        mh.reset_deltas();

        assert!(mh.depression().delta().abs() < f32::EPSILON);
        // AC delta should remain
        assert!((mh.acquired_capability().delta() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn default_is_new() {
        let mh = MentalHealth::default();
        assert!(mh.depression_effective() < 0.3);
    }

    #[test]
    fn clone_and_equality() {
        let mh1 = MentalHealth::new().with_depression_base(0.5);
        let mh2 = mh1.clone();
        assert_eq!(mh1, mh2);
    }

    #[test]
    fn debug_format() {
        let mh = MentalHealth::new();
        let debug = format!("{:?}", mh);
        assert!(debug.contains("MentalHealth"));
    }

    #[test]
    fn mutable_references_work() {
        let mut mh = MentalHealth::new();
        mh.depression_mut().add_delta(0.2);
        assert!((mh.depression().delta() - 0.2).abs() < f32::EPSILON);
    }

    #[test]
    fn threshold_constants_defined() {
        // Verify constants are accessible and reasonable
        assert!(TB_PRESENT_THRESHOLD > 0.0 && TB_PRESENT_THRESHOLD <= 1.0);
        assert!(PB_PRESENT_THRESHOLD > 0.0 && PB_PRESENT_THRESHOLD <= 1.0);
        assert!(HOPELESSNESS_THRESHOLD > 0.0 && HOPELESSNESS_THRESHOLD <= 1.0);
    }

    #[test]
    fn all_mutable_refs() {
        let mut mh = MentalHealth::new();

        mh.self_worth_mut().add_delta(-0.2);
        mh.hopelessness_mut().add_delta(0.3);
        mh.interpersonal_hopelessness_mut().add_delta(0.4);

        assert!((mh.self_worth().delta() - (-0.2)).abs() < f32::EPSILON);
        assert!((mh.hopelessness().delta() - 0.3).abs() < f32::EPSILON);
        assert!((mh.interpersonal_hopelessness().delta() - 0.4).abs() < f32::EPSILON);
    }

    #[test]
    fn add_interpersonal_hopelessness_delta_test() {
        let mut mh = MentalHealth::new();
        mh.add_interpersonal_hopelessness_delta(0.5);
        assert!((mh.interpersonal_hopelessness().delta() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn add_hopelessness_delta_test() {
        let mut mh = MentalHealth::new();
        mh.add_hopelessness_delta(0.4);
        assert!((mh.hopelessness().delta() - 0.4).abs() < f32::EPSILON);
    }
}
