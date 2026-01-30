//! Individual state aggregate container.
//!
//! IndividualState composes all the individual psychological state components:
//! - Hexaco (personality)
//! - Mood (PAD dimensions)
//! - Recent moral violation flag (disgust gating)
//! - Needs (physiological and psychological)
//! - SocialCognition (interpersonal beliefs)
//! - MentalHealth (ITS factors)
//! - Disposition (behavioral tendencies)
//! - PersonCharacteristics (PPCT factors)
//! - Demographical (name, DOB, age, gender, ethnicity)
//! - DemandCharacteristics (observable social signals)
//!
//! This is the primary container for an entity's internal state.

use crate::state::{
    DemandCharacteristics, Demographical, Disposition, EntityModelConfig, Hexaco, MentalHealth,
    Mood, Needs, PersonCharacteristics, SocialCognition, StateValue,
};
use crate::types::Duration;
use serde::{Deserialize, Serialize};

/// Aggregate container for all individual psychological state.
///
/// This struct composes all state components that define who an entity
/// is and how they currently feel. It provides unified access to all
/// state dimensions.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::state::{IndividualState, Mood, Needs, SocialCognition};
/// use behavioral_pathways::types::Duration;
///
/// let mut state = IndividualState::new();
///
/// // Access components
/// state.mood_mut().add_valence_delta(0.3);
/// state.needs_mut().add_stress_delta(0.2);
///
/// // Apply decay to all components
/// state.apply_decay(Duration::hours(6));
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndividualState {
    /// HEXACO personality factors.
    hexaco: Hexaco,

    /// PAD mood dimensions.
    mood: Mood,

    /// Recent moral violation flag (gates disgust derivation).
    recent_moral_violation_flag: StateValue,

    /// Physiological and psychological needs.
    needs: Needs,

    /// Social cognition (interpersonal beliefs).
    social_cognition: SocialCognition,

    /// Mental health / ITS factors.
    mental_health: MentalHealth,

    /// Behavioral dispositions.
    disposition: Disposition,

    /// PPCT person characteristics.
    person_characteristics: PersonCharacteristics,

    /// Demographical metadata.
    demographical: Demographical,

    /// Demand characteristics (observable signals).
    demand_characteristics: DemandCharacteristics,

    /// Entity model configuration.
    config: EntityModelConfig,
}

impl IndividualState {
    /// Creates a new IndividualState with default components.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::state::IndividualState;
    ///
    /// let state = IndividualState::new();
    /// assert!(state.mood().valence_effective().abs() < 0.1);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        IndividualState {
            hexaco: Hexaco::default(),
            mood: Mood::default(),
            recent_moral_violation_flag: StateValue::new(0.0)
                .with_decay_half_life(Duration::hours(24)),
            needs: Needs::default(),
            social_cognition: SocialCognition::default(),
            mental_health: MentalHealth::default(),
            disposition: Disposition::default(),
            person_characteristics: PersonCharacteristics::default(),
            demographical: Demographical::default(),
            demand_characteristics: DemandCharacteristics::default(),
            config: EntityModelConfig::default(),
        }
    }

    // Builder methods

    /// Sets the Hexaco personality.
    #[must_use]
    pub fn with_hexaco(mut self, hexaco: Hexaco) -> Self {
        self.hexaco = hexaco;
        self
    }

    /// Sets the Mood state.
    #[must_use]
    pub fn with_mood(mut self, mood: Mood) -> Self {
        self.mood = mood;
        self
    }

    /// Sets the Needs state.
    #[must_use]
    pub fn with_needs(mut self, needs: Needs) -> Self {
        self.needs = needs;
        self
    }

    /// Sets the SocialCognition state.
    #[must_use]
    pub fn with_social_cognition(mut self, social_cognition: SocialCognition) -> Self {
        self.social_cognition = social_cognition;
        self
    }

    /// Sets the MentalHealth state.
    #[must_use]
    pub fn with_mental_health(mut self, mental_health: MentalHealth) -> Self {
        self.mental_health = mental_health;
        self
    }

    /// Sets the Disposition state.
    #[must_use]
    pub fn with_disposition(mut self, disposition: Disposition) -> Self {
        self.disposition = disposition;
        self
    }

    /// Sets the PersonCharacteristics.
    #[must_use]
    pub fn with_person_characteristics(mut self, pc: PersonCharacteristics) -> Self {
        self.person_characteristics = pc;
        self
    }

    /// Sets the Demographical component.
    #[must_use]
    pub fn with_demographical(mut self, demographical: Demographical) -> Self {
        self.demographical = demographical;
        self
    }

    /// Sets the DemandCharacteristics component.
    #[must_use]
    pub fn with_demand_characteristics(
        mut self,
        demand_characteristics: DemandCharacteristics,
    ) -> Self {
        self.demand_characteristics = demand_characteristics;
        self
    }

    /// Sets the EntityModelConfig.
    #[must_use]
    pub fn with_config(mut self, config: EntityModelConfig) -> Self {
        self.config = config;
        self
    }

    // Accessors (immutable)

    /// Returns a reference to the Hexaco personality.
    #[must_use]
    pub fn hexaco(&self) -> &Hexaco {
        &self.hexaco
    }

    /// Returns a reference to the Mood state.
    #[must_use]
    pub fn mood(&self) -> &Mood {
        &self.mood
    }

    /// Returns the recent moral violation flag (0.0 to 1.0).
    #[must_use]
    pub fn recent_moral_violation_flag(&self) -> f32 {
        self.recent_moral_violation_flag.effective()
    }

    /// Returns a reference to the Needs state.
    #[must_use]
    pub fn needs(&self) -> &Needs {
        &self.needs
    }

    /// Returns a reference to the SocialCognition state.
    #[must_use]
    pub fn social_cognition(&self) -> &SocialCognition {
        &self.social_cognition
    }

    /// Returns a reference to the MentalHealth state.
    #[must_use]
    pub fn mental_health(&self) -> &MentalHealth {
        &self.mental_health
    }

    /// Returns a reference to the Disposition state.
    #[must_use]
    pub fn disposition(&self) -> &Disposition {
        &self.disposition
    }

    /// Returns a reference to the PersonCharacteristics.
    #[must_use]
    pub fn person_characteristics(&self) -> &PersonCharacteristics {
        &self.person_characteristics
    }

    /// Returns a reference to the Demographical component.
    #[must_use]
    pub fn demographical(&self) -> &Demographical {
        &self.demographical
    }

    /// Returns a reference to the DemandCharacteristics component.
    #[must_use]
    pub fn demand_characteristics(&self) -> &DemandCharacteristics {
        &self.demand_characteristics
    }

    /// Returns a reference to the EntityModelConfig.
    #[must_use]
    pub fn config(&self) -> &EntityModelConfig {
        &self.config
    }

    // Accessors (mutable)

    /// Returns a mutable reference to the Hexaco personality.
    pub fn hexaco_mut(&mut self) -> &mut Hexaco {
        &mut self.hexaco
    }

    /// Returns a mutable reference to the Mood state.
    pub fn mood_mut(&mut self) -> &mut Mood {
        &mut self.mood
    }

    /// Sets the recent moral violation flag (0.0 to 1.0).
    pub fn set_recent_moral_violation_flag(&mut self, value: f32) {
        self.recent_moral_violation_flag
            .set_delta(value.clamp(0.0, 1.0));
    }

    /// Returns a mutable reference to the Needs state.
    pub fn needs_mut(&mut self) -> &mut Needs {
        &mut self.needs
    }

    /// Returns a mutable reference to the SocialCognition state.
    pub fn social_cognition_mut(&mut self) -> &mut SocialCognition {
        &mut self.social_cognition
    }

    /// Returns a mutable reference to the MentalHealth state.
    pub fn mental_health_mut(&mut self) -> &mut MentalHealth {
        &mut self.mental_health
    }

    /// Returns a mutable reference to the Disposition state.
    pub fn disposition_mut(&mut self) -> &mut Disposition {
        &mut self.disposition
    }

    /// Returns a mutable reference to the PersonCharacteristics.
    pub fn person_characteristics_mut(&mut self) -> &mut PersonCharacteristics {
        &mut self.person_characteristics
    }

    /// Returns a mutable reference to the Demographical component.
    pub fn demographical_mut(&mut self) -> &mut Demographical {
        &mut self.demographical
    }

    /// Returns a mutable reference to the DemandCharacteristics component.
    pub fn demand_characteristics_mut(&mut self) -> &mut DemandCharacteristics {
        &mut self.demand_characteristics
    }

    /// Returns a mutable reference to the EntityModelConfig.
    pub fn config_mut(&mut self) -> &mut EntityModelConfig {
        &mut self.config
    }

    // Unified operations

    /// Applies decay to all state components over the specified duration.
    ///
    /// Note: Hexaco (personality) is stable and does not decay.
    /// Note: Acquired Capability in MentalHealth does not decay.
    pub fn apply_decay(&mut self, elapsed: Duration) {
        // Hexaco is stable - no decay
        self.mood.apply_decay(elapsed);
        self.recent_moral_violation_flag.apply_decay(elapsed);
        self.needs.apply_decay(elapsed);
        self.social_cognition.apply_decay(elapsed);
        self.mental_health.apply_decay(elapsed);
        self.disposition.apply_decay(elapsed);
        self.person_characteristics.apply_decay(elapsed);
    }

    /// Resets all deltas across all components.
    ///
    /// Note: Acquired Capability delta is not reset (permanent accumulation).
    pub fn reset_all_deltas(&mut self) {
        self.mood.reset_deltas();
        self.recent_moral_violation_flag.reset_delta();
        self.needs.reset_deltas();
        self.social_cognition.reset_deltas();
        self.mental_health.reset_deltas();
        self.disposition.reset_deltas();
        self.person_characteristics.reset_deltas();
    }

    // ITS computation convenience methods

    /// Computes Thwarted Belongingness from current social cognition.
    #[must_use]
    pub fn compute_thwarted_belongingness(&self) -> f32 {
        self.mental_health
            .compute_thwarted_belongingness(&self.social_cognition)
    }

    /// Computes Perceived Burdensomeness from current social cognition.
    #[must_use]
    pub fn compute_perceived_burdensomeness(&self) -> f32 {
        self.mental_health
            .compute_perceived_burdensomeness(&self.social_cognition)
    }

    /// Computes suicidal desire from current state.
    #[must_use]
    pub fn compute_suicidal_desire(&self) -> f32 {
        self.mental_health
            .compute_suicidal_desire(&self.social_cognition)
    }

    /// Computes attempt risk from current state.
    #[must_use]
    pub fn compute_attempt_risk(&self) -> f32 {
        self.mental_health.compute_attempt_risk(&self.social_cognition)
    }
}

impl Default for IndividualState {
    fn default() -> Self {
        IndividualState::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::PersonalityProfile;

    #[test]
    fn individual_state_composes_all_components() {
        let state = IndividualState::new();

        // All components should be accessible
        let _ = state.hexaco();
        let _ = state.mood();
        let _ = state.recent_moral_violation_flag();
        let _ = state.needs();
        let _ = state.mental_health();
        let _ = state.disposition();
        let _ = state.person_characteristics();
        let _ = state.demographical();
        let _ = state.demand_characteristics();
        let _ = state.config();
    }

    #[test]
    fn new_creates_default_components() {
        let state = IndividualState::new();

        // Mood should be neutral
        assert!(state.mood().valence_effective().abs() < 0.1);

        // Moral violation flag should be clear
        assert!(state.recent_moral_violation_flag().abs() < f32::EPSILON);

        // Social cognition should be healthy
        assert!(state.social_cognition().loneliness_effective() < 0.5);

        // Mental health should be healthy
        assert!(state.mental_health().depression_effective() < 0.3);

        // Disposition should be healthy
        assert!(state.disposition().empathy_effective() > 0.5);
    }

    #[test]
    fn builder_methods_set_components() {
        let hexaco = Hexaco::from_profile(PersonalityProfile::Leader);
        let mood = Mood::new().with_valence_base(0.5);

        let state = IndividualState::new()
            .with_hexaco(hexaco.clone())
            .with_mood(mood.clone());

        assert_eq!(state.hexaco(), &hexaco);
        assert_eq!(state.mood(), &mood);
    }

    #[test]
    fn builder_sets_demographical_and_demand_characteristics() {
        let demographical = Demographical::new().with_name("Alyx");
        let demand = DemandCharacteristics::new().with_appearance("casual");

        let state = IndividualState::new()
            .with_demographical(demographical.clone())
            .with_demand_characteristics(demand.clone());

        assert_eq!(state.demographical(), &demographical);
        assert_eq!(state.demand_characteristics(), &demand);
    }

    #[test]
    fn builder_sets_social_cognition() {
        let social = SocialCognition::default();
        let state = IndividualState::new().with_social_cognition(social.clone());

        assert_eq!(state.social_cognition(), &social);
    }

    #[test]
    fn mutable_references_work() {
        let mut state = IndividualState::new();

        state.mood_mut().add_valence_delta(0.3);
        assert!((state.mood().valence_delta() - 0.3).abs() < f32::EPSILON);

        state.needs_mut().add_stress_delta(0.2);
        assert!((state.needs().stress().delta() - 0.2).abs() < f32::EPSILON);
    }

    #[test]
    fn apply_decay_affects_all_decaying_components() {
        let mut state = IndividualState::new();

        state.mood_mut().add_valence_delta(0.8);
        state.needs_mut().add_stress_delta(0.8);
        state.disposition_mut().add_grievance_delta(0.8);

        // Apply 1 week of decay
        state.apply_decay(Duration::weeks(1));

        // Mood should have decayed significantly (6 hour half-life)
        assert!(state.mood().valence_delta() < 0.1);

        // Stress should have decayed (12 hour half-life)
        assert!(state.needs().stress().delta() < 0.1);

        // Grievance should be halved (1 week half-life)
        assert!(state.disposition().grievance().delta() < 0.5);
    }

    #[test]
    fn apply_decay_does_not_affect_acquired_capability() {
        let mut state = IndividualState::new();

        state.mental_health_mut().add_acquired_capability_delta(0.5);

        state.apply_decay(Duration::years(10));

        // AC should not have decayed
        assert!((state.mental_health().acquired_capability().delta() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn recent_moral_violation_flag_decays_over_a_day() {
        let mut state = IndividualState::new();
        state.set_recent_moral_violation_flag(1.0);

        state.apply_decay(Duration::hours(24));

        let flag = state.recent_moral_violation_flag();
        assert!((flag - 0.5).abs() < 0.01);
    }

    #[test]
    fn reset_all_deltas_clears_components() {
        let mut state = IndividualState::new();

        state.mood_mut().add_valence_delta(0.5);
        state.social_cognition_mut().add_loneliness_delta(0.3);
        state.disposition_mut().add_aggression_delta(0.2);

        state.reset_all_deltas();

        assert!(state.mood().valence_delta().abs() < f32::EPSILON);
        assert!(state.social_cognition().loneliness().delta().abs() < f32::EPSILON);
        assert!(state.disposition().aggression().delta().abs() < f32::EPSILON);
    }

    #[test]
    fn its_convenience_methods_work() {
        let mut state = IndividualState::new();

        // Set up high risk state
        state.social_cognition_mut().loneliness_mut().set_base(0.9);
        state
            .social_cognition_mut()
            .perceived_reciprocal_caring_mut()
            .set_base(0.1);
        state
            .social_cognition_mut()
            .perceived_liability_mut()
            .set_base(0.9);
        state.social_cognition_mut().self_hate_mut().set_base(0.9);
        state
            .mental_health_mut()
            .interpersonal_hopelessness_mut()
            .set_base(0.7);
        state
            .mental_health_mut()
            .acquired_capability_mut()
            .set_base(0.8);

        // Compute TB and PB
        let tb = state.compute_thwarted_belongingness();
        let pb = state.compute_perceived_burdensomeness();

        assert!(tb > 0.7);
        assert!(pb > 0.7);

        // Compute desire and risk
        let desire = state.compute_suicidal_desire();
        let risk = state.compute_attempt_risk();

        assert!(desire > 0.0);
        assert!(risk > 0.0);
    }

    #[test]
    fn default_is_new() {
        let state = IndividualState::default();
        assert!(state.social_cognition().loneliness_effective() < 0.5);
    }

    #[test]
    fn clone_and_equality() {
        let state1 = IndividualState::new();
        let state2 = state1.clone();
        assert_eq!(state1, state2);
    }

    #[test]
    fn debug_format() {
        let state = IndividualState::new();
        let debug = format!("{:?}", state);
        assert!(debug.contains("IndividualState"));
    }

    #[test]
    fn config_builder_and_accessor() {
        let config = EntityModelConfig::animal_simple();
        let state = IndividualState::new().with_config(config.clone());

        assert_eq!(state.config(), &config);
    }

    #[test]
    fn config_mutable() {
        let mut state = IndividualState::new();
        state.config_mut().set_time_scale(2.0);
        assert!((state.config().time_scale() - 2.0).abs() < f32::EPSILON);
    }

    #[test]
    fn all_builder_methods() {
        let hexaco = Hexaco::default();
        let mood = Mood::default();
        let needs = Needs::default();
        let mental_health = MentalHealth::default();
        let disposition = Disposition::default();
        let pc = PersonCharacteristics::default();
        let config = EntityModelConfig::default();

        let state = IndividualState::new()
            .with_hexaco(hexaco.clone())
            .with_mood(mood.clone())
            .with_needs(needs.clone())
            .with_mental_health(mental_health.clone())
            .with_disposition(disposition.clone())
            .with_person_characteristics(pc.clone())
            .with_config(config.clone());

        assert_eq!(state.hexaco(), &hexaco);
        assert_eq!(state.mood(), &mood);
        assert_eq!(state.needs(), &needs);
        assert_eq!(state.mental_health(), &mental_health);
        assert_eq!(state.disposition(), &disposition);
        assert_eq!(state.person_characteristics(), &pc);
        assert_eq!(state.config(), &config);
    }

    #[test]
    fn all_mutable_refs() {
        let mut state = IndividualState::new();

        state.hexaco_mut().set_openness(0.5);
        state.disposition_mut().add_empathy_delta(0.1);
        state
            .person_characteristics_mut()
            .social_capital_mut()
            .add_delta(0.2);
        state.demographical_mut();
        state.demand_characteristics_mut();

        assert!((state.hexaco().openness() - 0.5).abs() < f32::EPSILON);
        assert!((state.disposition().empathy().delta() - 0.1).abs() < f32::EPSILON);
        assert!(
            (state.person_characteristics().social_capital().delta() - 0.2).abs() < f32::EPSILON
        );
    }
}
