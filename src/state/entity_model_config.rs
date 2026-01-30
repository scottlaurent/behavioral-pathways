//! Entity model configuration.
//!
//! This module defines which subsystems are active for an entity model
//! and other entity-type-specific configuration. Different entity types
//! (Human, Animal) have different subsystem requirements.

use crate::enums::{Species, SubsystemId};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Configuration for an entity model.
///
/// This determines which subsystems are active for processing,
/// enabling or disabling features based on entity type complexity.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::state::EntityModelConfig;
/// use behavioral_pathways::enums::SubsystemId;
///
/// // Create config with all subsystems active (human)
/// let human_config = EntityModelConfig::human_default();
/// assert!(human_config.is_active(SubsystemId::Developmental));
///
/// // Create config for simpler entity
/// let simple_config = EntityModelConfig::new()
///     .with_subsystem(SubsystemId::State)
///     .with_subsystem(SubsystemId::Memory);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityModelConfig {
    /// Active subsystems for this entity.
    active_subsystems: HashSet<SubsystemId>,

    /// Whether personality modeling is enabled.
    personality_enabled: bool,

    /// Whether mental health (ITS) tracking is enabled.
    mental_health_enabled: bool,

    /// Time scale for psychological processing.
    /// 1.0 = human baseline, higher = faster subjective time.
    time_scale: f32,

    /// Minimum interaction frequency required for proximal process effects.
    /// Effects are blocked when frequency is below this threshold.
    /// Default: 0.3
    proximal_process_frequency_threshold: f64,

    /// Minimum interaction complexity required for proximal process effects.
    /// Effects are blocked when complexity is below this threshold.
    /// Default: 0.3
    proximal_process_complexity_threshold: f64,
}

/// Default proximal process frequency threshold.
pub const DEFAULT_PROXIMAL_FREQUENCY_THRESHOLD: f64 = 0.3;

/// Default proximal process complexity threshold.
pub const DEFAULT_PROXIMAL_COMPLEXITY_THRESHOLD: f64 = 0.3;

impl EntityModelConfig {
    /// Creates a new EntityModelConfig with no active subsystems.
    ///
    /// Use builder methods to add subsystems.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::state::EntityModelConfig;
    /// use behavioral_pathways::enums::SubsystemId;
    ///
    /// let config = EntityModelConfig::new()
    ///     .with_subsystem(SubsystemId::State);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        EntityModelConfig {
            active_subsystems: HashSet::new(),
            personality_enabled: false,
            mental_health_enabled: false,
            time_scale: 1.0,
            proximal_process_frequency_threshold: DEFAULT_PROXIMAL_FREQUENCY_THRESHOLD,
            proximal_process_complexity_threshold: DEFAULT_PROXIMAL_COMPLEXITY_THRESHOLD,
        }
    }

    /// Creates a configuration appropriate for a human entity.
    ///
    /// All subsystems are active, personality and mental health are enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::state::EntityModelConfig;
    /// use behavioral_pathways::enums::SubsystemId;
    ///
    /// let config = EntityModelConfig::human_default();
    /// assert!(config.is_active(SubsystemId::State));
    /// assert!(config.is_active(SubsystemId::Developmental));
    /// assert!(config.mental_health_enabled());
    /// ```
    #[must_use]
    pub fn human_default() -> Self {
        let mut active = HashSet::new();
        for subsystem in SubsystemId::all() {
            active.insert(subsystem);
        }

        EntityModelConfig {
            active_subsystems: active,
            personality_enabled: true,
            mental_health_enabled: true,
            time_scale: 1.0,
            proximal_process_frequency_threshold: DEFAULT_PROXIMAL_FREQUENCY_THRESHOLD,
            proximal_process_complexity_threshold: DEFAULT_PROXIMAL_COMPLEXITY_THRESHOLD,
        }
    }

    /// Creates a configuration appropriate for the given species.
    ///
    /// # Arguments
    ///
    /// * `species` - The species to create configuration for
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::state::EntityModelConfig;
    /// use behavioral_pathways::enums::{Species, SubsystemId};
    ///
    /// let config = EntityModelConfig::for_species(&Species::Human);
    /// assert!(config.is_active(SubsystemId::Developmental));
    /// assert!(config.mental_health_enabled());
    ///
    /// let dog_config = EntityModelConfig::for_species(&Species::Dog);
    /// assert!(!dog_config.mental_health_enabled());
    /// ```
    #[must_use]
    pub fn for_species(species: &Species) -> Self {
        match species {
            Species::Human => EntityModelConfig::human_default(),
            // All non-human species use the animal simple config
            Species::Dog
            | Species::Cat
            | Species::Dolphin
            | Species::Horse
            | Species::Elephant
            | Species::Chimpanzee
            | Species::Crow
            | Species::Mouse
            | Species::Custom { .. } => EntityModelConfig::animal_simple(),
        }
    }

    /// Creates a configuration appropriate for a simple animal entity.
    ///
    /// Core subsystems are active, but developmental and mental health
    /// tracking are disabled.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::state::EntityModelConfig;
    /// use behavioral_pathways::enums::SubsystemId;
    ///
    /// let config = EntityModelConfig::animal_simple();
    /// assert!(config.is_active(SubsystemId::State));
    /// assert!(!config.is_active(SubsystemId::Developmental));
    /// assert!(!config.mental_health_enabled());
    /// ```
    #[must_use]
    pub fn animal_simple() -> Self {
        let mut active = HashSet::new();
        active.insert(SubsystemId::State);
        active.insert(SubsystemId::Memory);
        active.insert(SubsystemId::Relationship);
        active.insert(SubsystemId::Event);
        active.insert(SubsystemId::BehavioralDecision);

        EntityModelConfig {
            active_subsystems: active,
            personality_enabled: true,
            mental_health_enabled: false,
            time_scale: 1.0,
            proximal_process_frequency_threshold: DEFAULT_PROXIMAL_FREQUENCY_THRESHOLD,
            proximal_process_complexity_threshold: DEFAULT_PROXIMAL_COMPLEXITY_THRESHOLD,
        }
    }

    /// Creates a configuration appropriate for a high-complexity animal.
    ///
    /// Similar to human but with mental health disabled.
    #[must_use]
    pub fn animal_complex() -> Self {
        let mut active = HashSet::new();
        for subsystem in SubsystemId::all() {
            active.insert(subsystem);
        }

        EntityModelConfig {
            active_subsystems: active,
            personality_enabled: true,
            mental_health_enabled: false,
            time_scale: 1.0,
            proximal_process_frequency_threshold: DEFAULT_PROXIMAL_FREQUENCY_THRESHOLD,
            proximal_process_complexity_threshold: DEFAULT_PROXIMAL_COMPLEXITY_THRESHOLD,
        }
    }

    // Builder methods

    /// Adds a subsystem to the active set.
    #[must_use]
    pub fn with_subsystem(mut self, subsystem: SubsystemId) -> Self {
        self.active_subsystems.insert(subsystem);
        self
    }

    /// Removes a subsystem from the active set.
    #[must_use]
    pub fn without_subsystem(mut self, subsystem: SubsystemId) -> Self {
        self.active_subsystems.remove(&subsystem);
        self
    }

    /// Enables or disables personality modeling.
    #[must_use]
    pub fn with_personality_enabled(mut self, enabled: bool) -> Self {
        self.personality_enabled = enabled;
        self
    }

    /// Enables or disables mental health tracking.
    #[must_use]
    pub fn with_mental_health_enabled(mut self, enabled: bool) -> Self {
        self.mental_health_enabled = enabled;
        self
    }

    /// Sets the time scale for psychological processing.
    #[must_use]
    pub fn with_time_scale(mut self, scale: f32) -> Self {
        self.time_scale = scale.max(0.01); // Minimum 0.01 to avoid division by zero
        self
    }

    /// Sets the proximal process frequency threshold.
    #[must_use]
    pub fn with_proximal_frequency_threshold(mut self, threshold: f64) -> Self {
        self.proximal_process_frequency_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// Sets the proximal process complexity threshold.
    #[must_use]
    pub fn with_proximal_complexity_threshold(mut self, threshold: f64) -> Self {
        self.proximal_process_complexity_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    // Accessors

    /// Checks if a subsystem is active.
    ///
    /// # Arguments
    ///
    /// * `subsystem` - The subsystem to check
    ///
    /// # Returns
    ///
    /// True if the subsystem is active for this entity model.
    #[must_use]
    pub fn is_active(&self, subsystem: SubsystemId) -> bool {
        self.active_subsystems.contains(&subsystem)
    }

    /// Returns the set of active subsystems.
    #[must_use]
    pub fn active_subsystems(&self) -> &HashSet<SubsystemId> {
        &self.active_subsystems
    }

    /// Returns true if personality modeling is enabled.
    #[must_use]
    pub fn personality_enabled(&self) -> bool {
        self.personality_enabled
    }

    /// Returns true if mental health (ITS) tracking is enabled.
    #[must_use]
    pub fn mental_health_enabled(&self) -> bool {
        self.mental_health_enabled
    }

    /// Returns the time scale for psychological processing.
    ///
    /// Higher values mean faster subjective time (more psychological
    /// change per unit of real time).
    #[must_use]
    pub fn time_scale(&self) -> f32 {
        self.time_scale
    }

    /// Returns the proximal process frequency threshold.
    ///
    /// Context effects are blocked when interaction frequency is below this.
    #[must_use]
    pub fn proximal_frequency_threshold(&self) -> f64 {
        self.proximal_process_frequency_threshold
    }

    /// Returns the proximal process complexity threshold.
    ///
    /// Context effects are blocked when interaction complexity is below this.
    #[must_use]
    pub fn proximal_complexity_threshold(&self) -> f64 {
        self.proximal_process_complexity_threshold
    }

    /// Checks whether proximal process criteria are met.
    ///
    /// Returns true if both frequency and complexity meet or exceed thresholds.
    #[must_use]
    pub fn check_proximal_process_gate(&self, frequency: f64, complexity: f64) -> bool {
        frequency >= self.proximal_process_frequency_threshold
            && complexity >= self.proximal_process_complexity_threshold
    }

    // Mutators

    /// Activates a subsystem.
    pub fn activate_subsystem(&mut self, subsystem: SubsystemId) {
        self.active_subsystems.insert(subsystem);
    }

    /// Deactivates a subsystem.
    pub fn deactivate_subsystem(&mut self, subsystem: SubsystemId) {
        self.active_subsystems.remove(&subsystem);
    }

    /// Sets whether personality is enabled.
    pub fn set_personality_enabled(&mut self, enabled: bool) {
        self.personality_enabled = enabled;
    }

    /// Sets whether mental health is enabled.
    pub fn set_mental_health_enabled(&mut self, enabled: bool) {
        self.mental_health_enabled = enabled;
    }

    /// Sets the time scale.
    pub fn set_time_scale(&mut self, scale: f32) {
        self.time_scale = scale.max(0.01);
    }

    /// Sets the proximal process frequency threshold.
    pub fn set_proximal_frequency_threshold(&mut self, threshold: f64) {
        self.proximal_process_frequency_threshold = threshold.clamp(0.0, 1.0);
    }

    /// Sets the proximal process complexity threshold.
    pub fn set_proximal_complexity_threshold(&mut self, threshold: f64) {
        self.proximal_process_complexity_threshold = threshold.clamp(0.0, 1.0);
    }
}

impl Default for EntityModelConfig {
    fn default() -> Self {
        EntityModelConfig::human_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_empty_config() {
        let config = EntityModelConfig::new();
        assert!(config.active_subsystems().is_empty());
        assert!(!config.personality_enabled());
        assert!(!config.mental_health_enabled());
    }

    #[test]
    fn human_default_has_all_subsystems() {
        let config = EntityModelConfig::human_default();

        for subsystem in SubsystemId::all() {
            assert!(config.is_active(subsystem));
        }
    }

    #[test]
    fn human_default_has_personality_and_mental_health() {
        let config = EntityModelConfig::human_default();
        assert!(config.personality_enabled());
        assert!(config.mental_health_enabled());
    }

    #[test]
    fn human_default_has_time_scale_one() {
        let config = EntityModelConfig::human_default();
        assert!((config.time_scale() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn animal_simple_lacks_developmental() {
        let config = EntityModelConfig::animal_simple();

        assert!(config.is_active(SubsystemId::State));
        assert!(!config.is_active(SubsystemId::Developmental));
        assert!(!config.is_active(SubsystemId::EcologicalContext));
    }

    #[test]
    fn animal_simple_lacks_mental_health() {
        let config = EntityModelConfig::animal_simple();
        assert!(!config.mental_health_enabled());
        assert!(config.personality_enabled());
    }

    #[test]
    fn animal_complex_has_all_subsystems() {
        let config = EntityModelConfig::animal_complex();

        for subsystem in SubsystemId::all() {
            assert!(config.is_active(subsystem));
        }
    }

    #[test]
    fn animal_complex_lacks_mental_health() {
        let config = EntityModelConfig::animal_complex();
        assert!(!config.mental_health_enabled());
    }

    #[test]
    fn with_subsystem_adds_to_set() {
        let config = EntityModelConfig::new()
            .with_subsystem(SubsystemId::State)
            .with_subsystem(SubsystemId::Memory);

        assert!(config.is_active(SubsystemId::State));
        assert!(config.is_active(SubsystemId::Memory));
        assert!(!config.is_active(SubsystemId::Relationship));
    }

    #[test]
    fn without_subsystem_removes_from_set() {
        let config =
            EntityModelConfig::human_default().without_subsystem(SubsystemId::Developmental);

        assert!(!config.is_active(SubsystemId::Developmental));
        assert!(config.is_active(SubsystemId::State));
    }

    #[test]
    fn with_personality_enabled_works() {
        let config = EntityModelConfig::new().with_personality_enabled(true);
        assert!(config.personality_enabled());

        let config2 = EntityModelConfig::human_default().with_personality_enabled(false);
        assert!(!config2.personality_enabled());
    }

    #[test]
    fn with_mental_health_enabled_works() {
        let config = EntityModelConfig::new().with_mental_health_enabled(true);
        assert!(config.mental_health_enabled());
    }

    #[test]
    fn with_time_scale_works() {
        let config = EntityModelConfig::new().with_time_scale(6.7);
        assert!((config.time_scale() - 6.7).abs() < f32::EPSILON);
    }

    #[test]
    fn time_scale_has_minimum() {
        let config = EntityModelConfig::new().with_time_scale(-5.0);
        assert!(config.time_scale() >= 0.01);
    }

    #[test]
    fn mutators_work() {
        let mut config = EntityModelConfig::new();

        config.activate_subsystem(SubsystemId::State);
        assert!(config.is_active(SubsystemId::State));

        config.deactivate_subsystem(SubsystemId::State);
        assert!(!config.is_active(SubsystemId::State));

        config.set_personality_enabled(true);
        assert!(config.personality_enabled());

        config.set_mental_health_enabled(true);
        assert!(config.mental_health_enabled());

        config.set_time_scale(2.0);
        assert!((config.time_scale() - 2.0).abs() < f32::EPSILON);
    }

    #[test]
    fn default_is_human() {
        let config = EntityModelConfig::default();
        assert!(config.mental_health_enabled());
        assert!(config.is_active(SubsystemId::Developmental));
    }

    #[test]
    fn clone_and_equality() {
        let config1 = EntityModelConfig::human_default();
        let config2 = config1.clone();
        assert_eq!(config1, config2);
    }

    #[test]
    fn debug_format() {
        let config = EntityModelConfig::new();
        let debug = format!("{:?}", config);
        assert!(debug.contains("EntityModelConfig"));
    }

    #[test]
    fn active_subsystems_accessor() {
        let config = EntityModelConfig::new()
            .with_subsystem(SubsystemId::State)
            .with_subsystem(SubsystemId::Memory);

        let active = config.active_subsystems();
        assert_eq!(active.len(), 2);
        assert!(active.contains(&SubsystemId::State));
        assert!(active.contains(&SubsystemId::Memory));
    }

    // --- Proximal process threshold tests ---

    #[test]
    fn default_proximal_process_thresholds() {
        let config = EntityModelConfig::human_default();
        assert!((config.proximal_frequency_threshold() - 0.3).abs() < f64::EPSILON);
        assert!((config.proximal_complexity_threshold() - 0.3).abs() < f64::EPSILON);
    }

    #[test]
    fn with_proximal_frequency_threshold() {
        let config = EntityModelConfig::new().with_proximal_frequency_threshold(0.5);
        assert!((config.proximal_frequency_threshold() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn with_proximal_complexity_threshold() {
        let config = EntityModelConfig::new().with_proximal_complexity_threshold(0.6);
        assert!((config.proximal_complexity_threshold() - 0.6).abs() < f64::EPSILON);
    }

    #[test]
    fn proximal_thresholds_clamped() {
        let config = EntityModelConfig::new()
            .with_proximal_frequency_threshold(1.5)
            .with_proximal_complexity_threshold(-0.5);

        assert!((config.proximal_frequency_threshold() - 1.0).abs() < f64::EPSILON);
        assert!((config.proximal_complexity_threshold() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn check_proximal_process_gate_both_pass() {
        let config = EntityModelConfig::new()
            .with_proximal_frequency_threshold(0.3)
            .with_proximal_complexity_threshold(0.3);

        assert!(config.check_proximal_process_gate(0.5, 0.5));
        assert!(config.check_proximal_process_gate(0.3, 0.3)); // Exactly at threshold
    }

    #[test]
    fn check_proximal_process_gate_frequency_fails() {
        let config = EntityModelConfig::new()
            .with_proximal_frequency_threshold(0.3)
            .with_proximal_complexity_threshold(0.3);

        assert!(!config.check_proximal_process_gate(0.2, 0.5));
    }

    #[test]
    fn check_proximal_process_gate_complexity_fails() {
        let config = EntityModelConfig::new()
            .with_proximal_frequency_threshold(0.3)
            .with_proximal_complexity_threshold(0.3);

        assert!(!config.check_proximal_process_gate(0.5, 0.2));
    }

    #[test]
    fn check_proximal_process_gate_both_fail() {
        let config = EntityModelConfig::new()
            .with_proximal_frequency_threshold(0.3)
            .with_proximal_complexity_threshold(0.3);

        assert!(!config.check_proximal_process_gate(0.1, 0.1));
    }

    #[test]
    fn set_proximal_thresholds() {
        let mut config = EntityModelConfig::new();

        config.set_proximal_frequency_threshold(0.4);
        config.set_proximal_complexity_threshold(0.5);

        assert!((config.proximal_frequency_threshold() - 0.4).abs() < f64::EPSILON);
        assert!((config.proximal_complexity_threshold() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn set_proximal_thresholds_clamped() {
        let mut config = EntityModelConfig::new();

        config.set_proximal_frequency_threshold(2.0);
        config.set_proximal_complexity_threshold(-1.0);

        assert!((config.proximal_frequency_threshold() - 1.0).abs() < f64::EPSILON);
        assert!((config.proximal_complexity_threshold() - 0.0).abs() < f64::EPSILON);
    }

    // --- for_species tests ---

    #[test]
    fn for_species_human_returns_human_default() {
        let config = EntityModelConfig::for_species(&Species::Human);
        assert!(config.mental_health_enabled());
        assert!(config.is_active(SubsystemId::Developmental));
    }

    #[test]
    fn for_species_dog_returns_animal_simple() {
        let config = EntityModelConfig::for_species(&Species::Dog);
        assert!(!config.mental_health_enabled());
        assert!(!config.is_active(SubsystemId::Developmental));
    }

    #[test]
    fn for_species_cat_returns_animal_simple() {
        let config = EntityModelConfig::for_species(&Species::Cat);
        assert!(!config.mental_health_enabled());
        assert!(!config.is_active(SubsystemId::Developmental));
    }

    #[test]
    fn for_species_dolphin_returns_animal_simple() {
        let config = EntityModelConfig::for_species(&Species::Dolphin);
        assert!(!config.mental_health_enabled());
    }

    #[test]
    fn for_species_chimpanzee_returns_animal_simple() {
        let config = EntityModelConfig::for_species(&Species::Chimpanzee);
        assert!(!config.mental_health_enabled());
    }

    #[test]
    fn for_species_mouse_returns_animal_simple() {
        let config = EntityModelConfig::for_species(&Species::Mouse);
        assert!(!config.mental_health_enabled());
    }

    #[test]
    fn for_species_custom_returns_animal_simple() {
        let custom = Species::custom("Parrot", 60, 5, 0.6);
        let config = EntityModelConfig::for_species(&custom);
        assert!(!config.mental_health_enabled());
    }
}
