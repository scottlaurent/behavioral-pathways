//! Person characteristics from Bronfenbrenner's PPCT model.
//!
//! The PPCT (Process-Person-Context-Time) model identifies three types
//! of person characteristics that influence development:
//!
//! - **Demand** characteristics: Immediate social signals that invite or
//!   discourage reactions from others (appearance, temperament markers)
//!
//! - **Resource** characteristics: Assets and liabilities that influence
//!   capacity for effective proximal processes (cognitive ability, skills,
//!   social capital, material resources)
//!
//! - **Force** characteristics: Differences in motivation, persistence, and
//!   temperament that set proximal processes in motion (curiosity, drive,
//!   self-efficacy)

use crate::state::StateValue;
use crate::types::Duration;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Person characteristics per the PPCT model.
///
/// These characteristics influence how the person engages with their
/// environment and how the environment responds to them. Demand
/// characteristics (observable social signals) live in
/// [`DemandCharacteristics`].
///
/// # Examples
///
/// ```
/// use behavioral_pathways::state::PersonCharacteristics;
///
/// let pc = PersonCharacteristics::new();
///
/// // Resource characteristics affect capability
/// assert!(pc.cognitive_ability_effective() > 0.4);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersonCharacteristics {
    // --- Resource Characteristics ---
    /// Reasoning and problem-solving ability.
    /// Range: 0 (low) to 1 (high)
    /// Default decay half-life: 6 months (slow to change)
    cognitive_ability: StateValue,

    /// Learned coping strategies and emotional regulation skills.
    /// Range: 0 (few resources) to 1 (many resources)
    /// Default decay half-life: 3 months
    emotional_regulation_assets: StateValue,

    /// Count and quality of supportive relationships.
    /// Range: 0 (isolated) to 1 (well-connected)
    /// Default decay half-life: 1 month
    social_capital: StateValue,

    /// Access to food, shelter, savings, stability.
    /// Range: 0 (insecure) to 1 (secure)
    /// Default decay half-life: 1 month
    material_security: StateValue,

    /// Variety of life domains encountered.
    /// Range: 0 (narrow) to 1 (diverse)
    /// Default decay half-life: 6 months (accumulative)
    experience_diversity: StateValue,

    // --- Force Characteristics ---
    /// Drive to initiate action.
    /// Range: 0 (passive) to 1 (highly driven)
    /// Default decay half-life: 1 month
    baseline_motivation: StateValue,

    /// Tendency to persist despite difficulty.
    /// Range: 0 (gives up easily) to 1 (highly persistent)
    /// Default decay half-life: 1 month
    persistence_tendency: StateValue,

    /// Tendency to seek information and novelty.
    /// Range: 0 (incurious) to 1 (highly curious)
    /// Default decay half-life: 1 month
    curiosity_tendency: StateValue,

    /// Domain-specific self-efficacy beliefs (0-1).
    self_efficacy_by_domain: HashMap<String, f32>,
}

impl PersonCharacteristics {
    /// Default decay half-life for cognitive ability (6 months).
    const COGNITIVE_ABILITY_DECAY: Duration = Duration::months(6);

    /// Default decay half-life for emotional regulation (3 months).
    const EMOTIONAL_REGULATION_DECAY: Duration = Duration::months(3);

    /// Default decay half-life for social capital (1 month).
    const SOCIAL_CAPITAL_DECAY: Duration = Duration::months(1);

    /// Default decay half-life for material security (1 month).
    const MATERIAL_SECURITY_DECAY: Duration = Duration::months(1);

    /// Default decay half-life for experience diversity (6 months).
    const EXPERIENCE_DIVERSITY_DECAY: Duration = Duration::months(6);

    /// Default decay half-life for motivation (1 month).
    const MOTIVATION_DECAY: Duration = Duration::months(1);

    /// Default decay half-life for persistence (1 month).
    const PERSISTENCE_DECAY: Duration = Duration::months(1);

    /// Default decay half-life for curiosity (1 month).
    const CURIOSITY_DECAY: Duration = Duration::months(1);

    /// Creates a new PersonCharacteristics with healthy defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::state::PersonCharacteristics;
    ///
    /// let pc = PersonCharacteristics::new();
    /// assert!(pc.baseline_motivation_effective() > 0.4);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        PersonCharacteristics {
            // Resource
            cognitive_ability: StateValue::new(0.5)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(Self::COGNITIVE_ABILITY_DECAY),
            emotional_regulation_assets: StateValue::new(0.5)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(Self::EMOTIONAL_REGULATION_DECAY),
            social_capital: StateValue::new(0.5)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(Self::SOCIAL_CAPITAL_DECAY),
            material_security: StateValue::new(0.5)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(Self::MATERIAL_SECURITY_DECAY),
            experience_diversity: StateValue::new(0.3)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(Self::EXPERIENCE_DIVERSITY_DECAY),
            // Force
            baseline_motivation: StateValue::new(0.5)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(Self::MOTIVATION_DECAY),
            persistence_tendency: StateValue::new(0.5)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(Self::PERSISTENCE_DECAY),
            curiosity_tendency: StateValue::new(0.5)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(Self::CURIOSITY_DECAY),
            self_efficacy_by_domain: HashMap::new(),
        }
    }

    // --- Composite Accessors ---

    /// Returns the overall resource characteristic level.
    ///
    /// This is an average of all resource dimensions.
    #[must_use]
    pub fn resource(&self) -> f32 {
        let sum = self.cognitive_ability.effective()
            + self.emotional_regulation_assets.effective()
            + self.social_capital.effective()
            + self.material_security.effective()
            + self.experience_diversity.effective();
        sum / 5.0
    }

    /// Returns the overall force characteristic level.
    ///
    /// This is an average of all force dimensions.
    #[must_use]
    pub fn force(&self) -> f32 {
        let sum = self.baseline_motivation.effective()
            + self.persistence_tendency.effective()
            + self.curiosity_tendency.effective();
        sum / 3.0
    }

    // --- Builder Methods ---

    /// Sets the base cognitive ability.
    #[must_use]
    pub fn with_cognitive_ability_base(mut self, value: f32) -> Self {
        self.cognitive_ability.set_base(value);
        self
    }

    /// Sets the base emotional regulation assets.
    #[must_use]
    pub fn with_emotional_regulation_assets_base(mut self, value: f32) -> Self {
        self.emotional_regulation_assets.set_base(value);
        self
    }

    /// Sets the base social capital.
    #[must_use]
    pub fn with_social_capital_base(mut self, value: f32) -> Self {
        self.social_capital.set_base(value);
        self
    }

    /// Sets the base material security.
    #[must_use]
    pub fn with_material_security_base(mut self, value: f32) -> Self {
        self.material_security.set_base(value);
        self
    }

    /// Sets the base experience diversity.
    #[must_use]
    pub fn with_experience_diversity_base(mut self, value: f32) -> Self {
        self.experience_diversity.set_base(value);
        self
    }

    /// Sets the base motivation.
    #[must_use]
    pub fn with_baseline_motivation_base(mut self, value: f32) -> Self {
        self.baseline_motivation.set_base(value);
        self
    }

    /// Sets the base persistence tendency.
    #[must_use]
    pub fn with_persistence_tendency_base(mut self, value: f32) -> Self {
        self.persistence_tendency.set_base(value);
        self
    }

    /// Sets the base curiosity tendency.
    #[must_use]
    pub fn with_curiosity_tendency_base(mut self, value: f32) -> Self {
        self.curiosity_tendency.set_base(value);
        self
    }

    /// Sets a domain-specific self-efficacy value.
    #[must_use]
    pub fn with_self_efficacy(mut self, domain: impl Into<String>, value: f32) -> Self {
        let clamped = value.clamp(0.0, 1.0);
        self.self_efficacy_by_domain.insert(domain.into(), clamped);
        self
    }

    // --- Effective Value Accessors ---

    /// Returns the effective cognitive ability.
    #[must_use]
    pub fn cognitive_ability_effective(&self) -> f32 {
        self.cognitive_ability.effective()
    }

    /// Returns the effective emotional regulation assets.
    #[must_use]
    pub fn emotional_regulation_assets_effective(&self) -> f32 {
        self.emotional_regulation_assets.effective()
    }

    /// Returns the effective social capital.
    #[must_use]
    pub fn social_capital_effective(&self) -> f32 {
        self.social_capital.effective()
    }

    /// Returns the effective material security.
    #[must_use]
    pub fn material_security_effective(&self) -> f32 {
        self.material_security.effective()
    }

    /// Returns the effective experience diversity.
    #[must_use]
    pub fn experience_diversity_effective(&self) -> f32 {
        self.experience_diversity.effective()
    }

    /// Returns the effective baseline motivation.
    #[must_use]
    pub fn baseline_motivation_effective(&self) -> f32 {
        self.baseline_motivation.effective()
    }

    /// Returns the effective persistence tendency.
    #[must_use]
    pub fn persistence_tendency_effective(&self) -> f32 {
        self.persistence_tendency.effective()
    }

    /// Returns the effective curiosity tendency.
    #[must_use]
    pub fn curiosity_tendency_effective(&self) -> f32 {
        self.curiosity_tendency.effective()
    }

    /// Returns the self-efficacy value for a domain, if present.
    #[must_use]
    pub fn self_efficacy(&self, domain: &str) -> Option<f32> {
        self.self_efficacy_by_domain.get(domain).copied()
    }

    // --- StateValue References ---

    /// Returns a reference to the cognitive_ability StateValue.
    #[must_use]
    pub fn cognitive_ability(&self) -> &StateValue {
        &self.cognitive_ability
    }

    /// Returns a reference to the emotional_regulation_assets StateValue.
    #[must_use]
    pub fn emotional_regulation_assets(&self) -> &StateValue {
        &self.emotional_regulation_assets
    }

    /// Returns a reference to the social_capital StateValue.
    #[must_use]
    pub fn social_capital(&self) -> &StateValue {
        &self.social_capital
    }

    /// Returns a reference to the material_security StateValue.
    #[must_use]
    pub fn material_security(&self) -> &StateValue {
        &self.material_security
    }

    /// Returns a reference to the experience_diversity StateValue.
    #[must_use]
    pub fn experience_diversity(&self) -> &StateValue {
        &self.experience_diversity
    }

    /// Returns a reference to the baseline_motivation StateValue.
    #[must_use]
    pub fn baseline_motivation(&self) -> &StateValue {
        &self.baseline_motivation
    }

    /// Returns a reference to the persistence_tendency StateValue.
    #[must_use]
    pub fn persistence_tendency(&self) -> &StateValue {
        &self.persistence_tendency
    }

    /// Returns a reference to the curiosity_tendency StateValue.
    #[must_use]
    pub fn curiosity_tendency(&self) -> &StateValue {
        &self.curiosity_tendency
    }

    /// Returns a reference to the self-efficacy map.
    #[must_use]
    pub fn self_efficacy_by_domain(&self) -> &HashMap<String, f32> {
        &self.self_efficacy_by_domain
    }

    // --- Mutable References ---

    /// Returns a mutable reference to the cognitive_ability StateValue.
    pub fn cognitive_ability_mut(&mut self) -> &mut StateValue {
        &mut self.cognitive_ability
    }

    /// Returns a mutable reference to the emotional_regulation_assets StateValue.
    pub fn emotional_regulation_assets_mut(&mut self) -> &mut StateValue {
        &mut self.emotional_regulation_assets
    }

    /// Returns a mutable reference to the social_capital StateValue.
    pub fn social_capital_mut(&mut self) -> &mut StateValue {
        &mut self.social_capital
    }

    /// Returns a mutable reference to the material_security StateValue.
    pub fn material_security_mut(&mut self) -> &mut StateValue {
        &mut self.material_security
    }

    /// Returns a mutable reference to the experience_diversity StateValue.
    pub fn experience_diversity_mut(&mut self) -> &mut StateValue {
        &mut self.experience_diversity
    }

    /// Returns a mutable reference to the baseline_motivation StateValue.
    pub fn baseline_motivation_mut(&mut self) -> &mut StateValue {
        &mut self.baseline_motivation
    }

    /// Returns a mutable reference to the persistence_tendency StateValue.
    pub fn persistence_tendency_mut(&mut self) -> &mut StateValue {
        &mut self.persistence_tendency
    }

    /// Returns a mutable reference to the curiosity_tendency StateValue.
    pub fn curiosity_tendency_mut(&mut self) -> &mut StateValue {
        &mut self.curiosity_tendency
    }

    /// Returns a mutable reference to the self-efficacy map.
    pub fn self_efficacy_by_domain_mut(&mut self) -> &mut HashMap<String, f32> {
        &mut self.self_efficacy_by_domain
    }

    /// Sets a domain-specific self-efficacy value.
    pub fn set_self_efficacy(&mut self, domain: impl Into<String>, value: f32) {
        let clamped = value.clamp(0.0, 1.0);
        self.self_efficacy_by_domain.insert(domain.into(), clamped);
    }

    // --- Decay ---

    /// Applies decay to all person characteristics over the specified duration.
    pub fn apply_decay(&mut self, elapsed: Duration) {
        self.cognitive_ability.apply_decay(elapsed);
        self.emotional_regulation_assets.apply_decay(elapsed);
        self.social_capital.apply_decay(elapsed);
        self.material_security.apply_decay(elapsed);
        self.experience_diversity.apply_decay(elapsed);
        self.baseline_motivation.apply_decay(elapsed);
        self.persistence_tendency.apply_decay(elapsed);
        self.curiosity_tendency.apply_decay(elapsed);
    }

    /// Resets all deltas to zero.
    pub fn reset_deltas(&mut self) {
        self.cognitive_ability.reset_delta();
        self.emotional_regulation_assets.reset_delta();
        self.social_capital.reset_delta();
        self.material_security.reset_delta();
        self.experience_diversity.reset_delta();
        self.baseline_motivation.reset_delta();
        self.persistence_tendency.reset_delta();
        self.curiosity_tendency.reset_delta();
    }
}

impl Default for PersonCharacteristics {
    fn default() -> Self {
        PersonCharacteristics::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn person_characteristics_has_resource_force() {
        let pc = PersonCharacteristics::new();

        // Resource and force characteristics should be accessible
        let _ = pc.resource();
        let _ = pc.force();
    }

    #[test]
    fn resource_averages_all_resources() {
        let pc = PersonCharacteristics::new()
            .with_cognitive_ability_base(0.6)
            .with_emotional_regulation_assets_base(0.6)
            .with_social_capital_base(0.6)
            .with_material_security_base(0.6)
            .with_experience_diversity_base(0.6);

        // All at 0.6, average should be 0.6
        assert!((pc.resource() - 0.6).abs() < f32::EPSILON);
    }

    #[test]
    fn force_averages_all_force_dimensions() {
        let pc = PersonCharacteristics::new()
            .with_baseline_motivation_base(0.9)
            .with_persistence_tendency_base(0.9)
            .with_curiosity_tendency_base(0.9);

        // All at 0.9, average should be 0.9
        assert!((pc.force() - 0.9).abs() < f32::EPSILON);
    }

    #[test]
    fn new_creates_moderate_defaults() {
        let pc = PersonCharacteristics::new();

        // All values should be around 0.3-0.5 for healthy defaults
        assert!(pc.resource() >= 0.3 && pc.resource() <= 0.7);
        assert!(pc.force() >= 0.3 && pc.force() <= 0.7);
    }

    #[test]
    fn builder_methods_work() {
        let pc = PersonCharacteristics::new()
            .with_cognitive_ability_base(0.8)
            .with_baseline_motivation_base(0.9)
            .with_self_efficacy("work", 0.6);

        assert!((pc.cognitive_ability().base() - 0.8).abs() < f32::EPSILON);
        assert!((pc.baseline_motivation().base() - 0.9).abs() < f32::EPSILON);
        assert!((pc.self_efficacy("work").unwrap() - 0.6).abs() < f32::EPSILON);
    }

    #[test]
    fn mutable_references_work() {
        let mut pc = PersonCharacteristics::new();
        pc.social_capital_mut().add_delta(0.2);
        assert!((pc.social_capital().delta() - 0.2).abs() < f32::EPSILON);
    }

    #[test]
    fn self_efficacy_defaults_empty() {
        let pc = PersonCharacteristics::new();
        assert!(pc.self_efficacy_by_domain().is_empty());
    }

    #[test]
    fn self_efficacy_setter_clamps() {
        let mut pc = PersonCharacteristics::new();
        pc.set_self_efficacy("relationships", 1.5);
        assert!((pc.self_efficacy("relationships").unwrap() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn decay_affects_all_dimensions() {
        let mut pc = PersonCharacteristics::new();
        pc.social_capital_mut().add_delta(0.4);
        pc.baseline_motivation_mut().add_delta(0.4);

        pc.apply_decay(Duration::months(1));

        // Both should have decayed (1 month is their half-life)
        assert!(pc.social_capital().delta() < 0.3);
        assert!(pc.baseline_motivation().delta() < 0.3);
    }

    #[test]
    fn reset_deltas_clears_all() {
        let mut pc = PersonCharacteristics::new();
        pc.cognitive_ability_mut().add_delta(0.3);
        pc.persistence_tendency_mut().add_delta(0.2);

        pc.reset_deltas();

        assert!(pc.cognitive_ability().delta().abs() < f32::EPSILON);
        assert!(pc.persistence_tendency().delta().abs() < f32::EPSILON);
    }

    #[test]
    fn default_is_new() {
        let pc = PersonCharacteristics::default();
        assert!(pc.resource() >= 0.3);
    }

    #[test]
    fn clone_and_equality() {
        let pc1 = PersonCharacteristics::new().with_cognitive_ability_base(0.8);
        let pc2 = pc1.clone();
        assert_eq!(pc1, pc2);
    }

    #[test]
    fn debug_format() {
        let pc = PersonCharacteristics::new();
        let debug = format!("{:?}", pc);
        assert!(debug.contains("PersonCharacteristics"));
    }

    #[test]
    fn all_effective_accessors_work() {
        let pc = PersonCharacteristics::new();

        let _ = pc.cognitive_ability_effective();
        let _ = pc.emotional_regulation_assets_effective();
        let _ = pc.social_capital_effective();
        let _ = pc.material_security_effective();
        let _ = pc.experience_diversity_effective();
        let _ = pc.baseline_motivation_effective();
        let _ = pc.persistence_tendency_effective();
        let _ = pc.curiosity_tendency_effective();
    }

    #[test]
    fn all_immutable_refs_work() {
        let pc = PersonCharacteristics::new();

        let _ = pc.cognitive_ability();
        let _ = pc.emotional_regulation_assets();
        let _ = pc.social_capital();
        let _ = pc.material_security();
        let _ = pc.experience_diversity();
        let _ = pc.baseline_motivation();
        let _ = pc.persistence_tendency();
        let _ = pc.curiosity_tendency();
        let _ = pc.self_efficacy_by_domain();
    }

    #[test]
    fn all_mutable_refs_work() {
        let mut pc = PersonCharacteristics::new();

        pc.cognitive_ability_mut().add_delta(0.1);
        pc.emotional_regulation_assets_mut().add_delta(0.1);
        pc.material_security_mut().add_delta(0.1);
        pc.experience_diversity_mut().add_delta(0.1);
        pc.curiosity_tendency_mut().add_delta(0.1);
        pc.self_efficacy_by_domain_mut()
            .insert("school".to_string(), 0.4);

        // Verify changes took effect
        assert!((pc.cognitive_ability().delta() - 0.1).abs() < f32::EPSILON);
        assert!((pc.emotional_regulation_assets().delta() - 0.1).abs() < f32::EPSILON);
        assert!((pc.material_security().delta() - 0.1).abs() < f32::EPSILON);
        assert!((pc.experience_diversity().delta() - 0.1).abs() < f32::EPSILON);
        assert!((pc.curiosity_tendency().delta() - 0.1).abs() < f32::EPSILON);
        assert!((pc.self_efficacy("school").unwrap() - 0.4).abs() < f32::EPSILON);
    }

    #[test]
    fn all_builder_methods_work() {
        let pc = PersonCharacteristics::new()
            .with_cognitive_ability_base(0.7)
            .with_emotional_regulation_assets_base(0.8)
            .with_social_capital_base(0.5)
            .with_material_security_base(0.4)
            .with_experience_diversity_base(0.3)
            .with_baseline_motivation_base(0.9)
            .with_persistence_tendency_base(0.85)
            .with_curiosity_tendency_base(0.75)
            .with_self_efficacy("health", 0.55);

        assert!((pc.cognitive_ability().base() - 0.7).abs() < f32::EPSILON);
        assert!((pc.emotional_regulation_assets().base() - 0.8).abs() < f32::EPSILON);
        assert!((pc.social_capital().base() - 0.5).abs() < f32::EPSILON);
        assert!((pc.material_security().base() - 0.4).abs() < f32::EPSILON);
        assert!((pc.experience_diversity().base() - 0.3).abs() < f32::EPSILON);
        assert!((pc.baseline_motivation().base() - 0.9).abs() < f32::EPSILON);
        assert!((pc.persistence_tendency().base() - 0.85).abs() < f32::EPSILON);
        assert!((pc.curiosity_tendency().base() - 0.75).abs() < f32::EPSILON);
        assert!((pc.self_efficacy("health").unwrap() - 0.55).abs() < f32::EPSILON);
    }
}
