//! Entity builder for fluent construction.
//!
//! The EntityBuilder provides a fluent API for constructing entities
//! with proper validation. Species is required; other fields have defaults.

use crate::context::EcologicalContext;
use crate::enums::{LifeStage, PersonalityProfile, Species};
use crate::state::{
    Disposition, Hexaco, IndividualState, MentalHealth, Mood, Needs, PersonCharacteristics,
    SocialCognition,
};
// Note: Mood::from_personality is used below to derive baseline affect from HEXACO
use crate::types::{Duration, EntityId, Timestamp};

use super::Entity;

/// Error type for entity build failures.
///
/// This error is returned when `EntityBuilder::build()` fails validation.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::entity::{EntityBuilder, EntityBuildError};
///
/// // Building without setting species fails
/// let result = EntityBuilder::new().build();
/// assert!(matches!(result, Err(EntityBuildError::MissingSpecies)));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityBuildError {
    /// Species is required but was not set.
    MissingSpecies,

    /// The entity ID is invalid (empty string).
    InvalidId(String),
}

impl std::fmt::Display for EntityBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityBuildError::MissingSpecies => write!(f, "Species is required but was not set"),
            EntityBuildError::InvalidId(reason) => write!(f, "Invalid entity ID: {}", reason),
        }
    }
}

impl std::error::Error for EntityBuildError {}

/// Builder for constructing Entity instances.
///
/// The builder provides a fluent API for setting entity properties.
/// Species is required; all other properties have sensible defaults.
///
/// # Required Fields
///
/// - `species` - Must be set before calling `build()`
///
/// # Optional Fields with Defaults
///
/// - `id` - Auto-generated UUID if not set
/// - `age` - Zero duration if not set
/// - `life_stage` - Derived from age and species if not set
/// - `personality` - Neutral HEXACO (Balanced profile) if not set
/// - `person_characteristics` - Neutral (0.5, 0.5, 0.5) if not set
///
/// # Examples
///
/// ```
/// use behavioral_pathways::entity::EntityBuilder;
/// use behavioral_pathways::enums::{Species, LifeStage, PersonalityProfile};
/// use behavioral_pathways::types::Duration;
///
/// // Minimal entity with required species
/// let entity = EntityBuilder::new()
///     .species(Species::Human)
///     .build()
///     .unwrap();
///
/// // Full entity with all options
/// let entity = EntityBuilder::new()
///     .id("person_001")
///     .species(Species::Human)
///     .age(Duration::years(30))
///     .life_stage(LifeStage::Adult)
///     .personality(PersonalityProfile::Leader)
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone, Default)]
pub struct EntityBuilder {
    id: Option<String>,
    species: Option<Species>,
    age: Option<Duration>,
    birth_date: Option<Timestamp>,
    life_stage: Option<LifeStage>,
    personality: Option<PersonalityProfile>,
    hexaco: Option<Hexaco>,
    person_characteristics: Option<PersonCharacteristics>,
    mood: Option<Mood>,
    needs: Option<Needs>,
    mental_health: Option<MentalHealth>,
    social_cognition: Option<SocialCognition>,
    disposition: Option<Disposition>,
    context: Option<EcologicalContext>,
}

impl EntityBuilder {
    /// Creates a new entity builder with no fields set.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::entity::EntityBuilder;
    ///
    /// let builder = EntityBuilder::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        EntityBuilder::default()
    }

    /// Sets the entity ID.
    ///
    /// If not set, a UUID will be generated.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::entity::EntityBuilder;
    /// use behavioral_pathways::enums::Species;
    ///
    /// let entity = EntityBuilder::new()
    ///     .id("person_001")
    ///     .species(Species::Human)
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(entity.id().as_str(), "person_001");
    /// ```
    #[must_use]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets the species (required).
    ///
    /// Species determines time scaling, lifespan, and which subsystems
    /// are active.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::entity::EntityBuilder;
    /// use behavioral_pathways::enums::Species;
    ///
    /// let entity = EntityBuilder::new()
    ///     .species(Species::Dog)
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(entity.species(), &Species::Dog);
    /// ```
    #[must_use]
    pub fn species(mut self, species: Species) -> Self {
        self.species = Some(species);
        self
    }

    /// Sets the entity's age.
    ///
    /// Age affects life stage determination if life_stage is not
    /// explicitly set.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::entity::EntityBuilder;
    /// use behavioral_pathways::enums::Species;
    /// use behavioral_pathways::types::Duration;
    ///
    /// let entity = EntityBuilder::new()
    ///     .species(Species::Human)
    ///     .age(Duration::years(30))
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(entity.age().as_years(), 30);
    /// ```
    #[must_use]
    pub fn age(mut self, age: Duration) -> Self {
        self.age = Some(age);
        self
    }

    /// Sets the entity's birth date.
    ///
    /// When set, age at any timestamp can be computed as:
    /// `query_timestamp - birth_date`
    ///
    /// Note: If both `age()` and `birth_date()` are set, the explicit `age()`
    /// is used as the anchor age. The birth_date is stored separately for
    /// timestamp-based age computation.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::entity::EntityBuilder;
    /// use behavioral_pathways::enums::Species;
    /// use behavioral_pathways::types::Timestamp;
    ///
    /// let entity = EntityBuilder::new()
    ///     .id("person_001")
    ///     .species(Species::Human)
    ///     .birth_date(Timestamp::from_ymd_hms(1990, 6, 15, 0, 0, 0))
    ///     .build()
    ///     .unwrap();
    ///
    /// assert!(entity.birth_date().is_some());
    /// ```
    #[must_use]
    pub fn birth_date(mut self, birth_date: Timestamp) -> Self {
        self.birth_date = Some(birth_date);
        self
    }

    /// Sets the life stage explicitly.
    ///
    /// If not set, the life stage is derived from age and species.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::entity::EntityBuilder;
    /// use behavioral_pathways::enums::{Species, LifeStage};
    ///
    /// let entity = EntityBuilder::new()
    ///     .species(Species::Human)
    ///     .life_stage(LifeStage::Adult)
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(entity.life_stage(), LifeStage::Adult);
    /// ```
    #[must_use]
    pub fn life_stage(mut self, stage: LifeStage) -> Self {
        self.life_stage = Some(stage);
        self
    }

    /// Sets personality using a preset profile.
    ///
    /// This sets HEXACO values based on the profile. If both
    /// `personality()` and `hexaco()` are called, the later call wins.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::entity::EntityBuilder;
    /// use behavioral_pathways::enums::{Species, PersonalityProfile};
    ///
    /// let entity = EntityBuilder::new()
    ///     .species(Species::Human)
    ///     .personality(PersonalityProfile::Leader)
    ///     .build()
    ///     .unwrap();
    /// ```
    #[must_use]
    pub fn personality(mut self, profile: PersonalityProfile) -> Self {
        self.personality = Some(profile);
        self.hexaco = None; // Profile overrides raw HEXACO
        self
    }

    /// Sets HEXACO personality values directly.
    ///
    /// This allows fine-grained control over personality dimensions.
    /// If both `personality()` and `hexaco()` are called, the later call wins.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::entity::EntityBuilder;
    /// use behavioral_pathways::enums::Species;
    /// use behavioral_pathways::state::Hexaco;
    ///
    /// let hexaco = Hexaco::new()
    ///     .with_extraversion(0.7)
    ///     .with_conscientiousness(0.8);
    ///
    /// let entity = EntityBuilder::new()
    ///     .species(Species::Human)
    ///     .hexaco(hexaco)
    ///     .build()
    ///     .unwrap();
    /// ```
    #[must_use]
    pub fn hexaco(mut self, hexaco: Hexaco) -> Self {
        self.hexaco = Some(hexaco);
        self.personality = None; // Raw HEXACO overrides profile
        self
    }

    /// Sets person characteristics (PPCT model).
    ///
    /// Person characteristics include demand, resource, and force factors
    /// that influence proximal processes.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::entity::EntityBuilder;
    /// use behavioral_pathways::enums::Species;
    /// use behavioral_pathways::state::PersonCharacteristics;
    ///
    /// let pc = PersonCharacteristics::new()
    ///     .with_cognitive_ability_base(0.8)
    ///     .with_baseline_motivation_base(0.7);
    ///
    /// let entity = EntityBuilder::new()
    ///     .species(Species::Human)
    ///     .person_characteristics(pc)
    ///     .build()
    ///     .unwrap();
    /// ```
    #[must_use]
    pub fn person_characteristics(mut self, pc: PersonCharacteristics) -> Self {
        self.person_characteristics = Some(pc);
        self
    }

    /// Sets the initial mood state.
    ///
    /// Mood contains the PAD (Pleasure-Arousal-Dominance) dimensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::entity::EntityBuilder;
    /// use behavioral_pathways::enums::Species;
    /// use behavioral_pathways::state::Mood;
    ///
    /// let mood = Mood::new()
    ///     .with_valence_base(0.3)
    ///     .with_arousal_base(-0.2);
    ///
    /// let entity = EntityBuilder::new()
    ///     .species(Species::Human)
    ///     .mood(mood)
    ///     .build()
    ///     .unwrap();
    /// ```
    #[must_use]
    pub fn mood(mut self, mood: Mood) -> Self {
        self.mood = Some(mood);
        self
    }

    /// Sets the initial needs state.
    ///
    /// Needs includes fatigue, stress, purpose, and other physiological/psychological needs.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::entity::EntityBuilder;
    /// use behavioral_pathways::enums::Species;
    /// use behavioral_pathways::state::Needs;
    ///
    /// let needs = Needs::new()
    ///     .with_fatigue_base(0.2)
    ///     .with_purpose_base(0.7);
    ///
    /// let entity = EntityBuilder::new()
    ///     .species(Species::Human)
    ///     .needs(needs)
    ///     .build()
    ///     .unwrap();
    /// ```
    #[must_use]
    pub fn needs(mut self, needs: Needs) -> Self {
        self.needs = Some(needs);
        self
    }

    /// Sets the initial mental health state.
    ///
    /// Mental health includes ITS factors (depression, hopelessness,
    /// acquired capability, etc.).
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::entity::EntityBuilder;
    /// use behavioral_pathways::enums::Species;
    /// use behavioral_pathways::state::MentalHealth;
    ///
    /// let mh = MentalHealth::new()
    ///     .with_depression_base(0.1)
    ///     .with_hopelessness_base(0.1);
    ///
    /// let entity = EntityBuilder::new()
    ///     .species(Species::Human)
    ///     .mental_health(mh)
    ///     .build()
    ///     .unwrap();
    /// ```
    #[must_use]
    pub fn mental_health(mut self, mental_health: MentalHealth) -> Self {
        self.mental_health = Some(mental_health);
        self
    }

    /// Sets the initial social cognition state.
    ///
    /// Social cognition includes loneliness, perceived caring/liability,
    /// self-hate, and other interpersonal beliefs.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::entity::EntityBuilder;
    /// use behavioral_pathways::enums::Species;
    /// use behavioral_pathways::state::SocialCognition;
    ///
    /// let sc = SocialCognition::new()
    ///     .with_loneliness_base(0.2)
    ///     .with_perceived_reciprocal_caring_base(0.7);
    ///
    /// let entity = EntityBuilder::new()
    ///     .species(Species::Human)
    ///     .social_cognition(sc)
    ///     .build()
    ///     .unwrap();
    /// ```
    #[must_use]
    pub fn social_cognition(mut self, social_cognition: SocialCognition) -> Self {
        self.social_cognition = Some(social_cognition);
        self
    }

    /// Sets the initial disposition state.
    ///
    /// Disposition includes behavioral tendencies like empathy, aggression,
    /// grievance, and prosocial behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::entity::EntityBuilder;
    /// use behavioral_pathways::enums::Species;
    /// use behavioral_pathways::state::Disposition;
    ///
    /// let disp = Disposition::new()
    ///     .with_empathy_base(0.7)
    ///     .with_impulse_control_base(0.6);
    ///
    /// let entity = EntityBuilder::new()
    ///     .species(Species::Human)
    ///     .disposition(disp)
    ///     .build()
    ///     .unwrap();
    /// ```
    #[must_use]
    pub fn disposition(mut self, disposition: Disposition) -> Self {
        self.disposition = Some(disposition);
        self
    }

    /// Sets the initial ecological context.
    ///
    /// Allows pre-populating the entity's ecological context with
    /// microsystems, exosystem, macrosystem, and chronosystem values.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::entity::EntityBuilder;
    /// use behavioral_pathways::enums::Species;
    /// use behavioral_pathways::context::{EcologicalContext, Microsystem, WorkContext};
    /// use behavioral_pathways::types::MicrosystemId;
    ///
    /// let mut context = EcologicalContext::default();
    /// let work_id = MicrosystemId::new("work_primary").unwrap();
    /// context.add_microsystem(work_id, Microsystem::new_work(WorkContext::default()));
    ///
    /// let entity = EntityBuilder::new()
    ///     .species(Species::Human)
    ///     .with_context(context)
    ///     .build()
    ///     .unwrap();
    ///
    /// assert_eq!(entity.context().microsystem_count(), 1);
    /// ```
    #[must_use]
    pub fn with_context(mut self, context: EcologicalContext) -> Self {
        self.context = Some(context);
        self
    }

    /// Builds the entity.
    ///
    /// # Errors
    ///
    /// Returns `EntityBuildError::MissingSpecies` if species was not set.
    /// Returns `EntityBuildError::InvalidId` if the ID is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::entity::{EntityBuilder, EntityBuildError};
    /// use behavioral_pathways::enums::Species;
    ///
    /// // Success case
    /// let entity = EntityBuilder::new()
    ///     .species(Species::Human)
    ///     .build()
    ///     .unwrap();
    ///
    /// // Error case - missing species
    /// let result = EntityBuilder::new().build();
    /// assert!(matches!(result, Err(EntityBuildError::MissingSpecies)));
    /// ```
    pub fn build(self) -> Result<Entity, EntityBuildError> {
        // Validate required fields
        let species = self.species.ok_or(EntityBuildError::MissingSpecies)?;

        // Generate or validate ID
        let id_string = self.id.unwrap_or_else(generate_uuid);
        let id = match EntityId::new(id_string) {
            Ok(id) => id,
            Err(err) => return Err(EntityBuildError::InvalidId(err.reason)),
        };

        // Get age, defaulting to zero
        let age = self.age.unwrap_or(Duration::zero());

        // Get birth_date (optional)
        let birth_date = self.birth_date;

        // Determine life stage: explicit > derived from age
        let life_stage = self
            .life_stage
            .unwrap_or_else(|| LifeStage::from_age_years_for_species(&species, age.as_years_f64()));

        // Build HEXACO: explicit hexaco > profile > default
        let hexaco = if let Some(h) = self.hexaco {
            h
        } else if let Some(profile) = self.personality {
            Hexaco::from_profile(profile)
        } else {
            Hexaco::from_profile(PersonalityProfile::Balanced)
        };

        // Build person characteristics
        let person_characteristics = self.person_characteristics.unwrap_or_default();

        // Build individual state with required components
        let mut individual_state = IndividualState::new()
            .with_hexaco(hexaco.clone())
            .with_person_characteristics(person_characteristics);

        // Apply mood: explicit mood overrides, otherwise derive from personality
        if let Some(mood) = self.mood {
            individual_state = individual_state.with_mood(mood);
        } else {
            // Derive baseline affect from personality traits
            individual_state = individual_state.with_mood(Mood::from_personality(&hexaco));
        }
        if let Some(needs) = self.needs {
            individual_state = individual_state.with_needs(needs);
        }
        if let Some(mental_health) = self.mental_health {
            individual_state = individual_state.with_mental_health(mental_health);
        }
        if let Some(social_cognition) = self.social_cognition {
            individual_state = individual_state.with_social_cognition(social_cognition);
        }
        if let Some(disposition) = self.disposition {
            individual_state = individual_state.with_disposition(disposition);
        }

        // Build entity with or without custom context
        if let Some(context) = self.context {
            Ok(Entity::new_with_context(
                id,
                species,
                age,
                birth_date,
                life_stage,
                individual_state,
                context,
            ))
        } else {
            Ok(Entity::new(
                id,
                species,
                age,
                birth_date,
                life_stage,
                individual_state,
            ))
        }
    }
}

/// Generates a UUID-like unique identifier.
///
/// This uses a simple counter-based approach for determinism in tests.
/// In production, this could be replaced with actual UUID generation.
fn generate_uuid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(1);
    let count = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("entity_{:016x}", count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_sets_species() {
        let entity = EntityBuilder::new()
            .species(Species::Human)
            .build()
            .unwrap();

        assert_eq!(entity.species(), &Species::Human);
    }

    #[test]
    fn builder_sets_life_stage() {
        let entity = EntityBuilder::new()
            .species(Species::Human)
            .life_stage(LifeStage::Adult)
            .build()
            .unwrap();

        assert_eq!(entity.life_stage(), LifeStage::Adult);
    }

    #[test]
    fn builder_sets_personality_profile() {
        let entity = EntityBuilder::new()
            .species(Species::Human)
            .personality(PersonalityProfile::Leader)
            .build()
            .unwrap();

        // Leader has high extraversion
        assert!(entity.individual_state().hexaco().extraversion() > 0.5);
    }

    #[test]
    fn builder_sets_person_characteristics() {
        let pc = PersonCharacteristics::new().with_cognitive_ability_base(0.9);

        let entity = EntityBuilder::new()
            .species(Species::Human)
            .person_characteristics(pc)
            .build()
            .unwrap();

        assert!(
            (entity
                .individual_state()
                .person_characteristics()
                .cognitive_ability()
                .base()
                - 0.9)
                .abs()
                < f32::EPSILON
        );
    }

    #[test]
    fn builder_produces_entity() {
        let result = EntityBuilder::new().species(Species::Human).build();

        assert!(result.is_ok());
        let entity = result.unwrap();
        assert_eq!(entity.species(), &Species::Human);
    }

    #[test]
    fn builder_requires_species() {
        let result = EntityBuilder::new().build();

        assert_eq!(result.unwrap_err(), EntityBuildError::MissingSpecies);
    }

    #[test]
    fn builder_defaults_life_stage() {
        // Age 30 -> YoungAdult for human (18-30 range)
        let entity = EntityBuilder::new()
            .species(Species::Human)
            .age(Duration::years(30))
            .build()
            .unwrap();

        assert_eq!(entity.life_stage(), LifeStage::YoungAdult);

        // Age 40 -> Adult for human (31-55 range)
        let adult = EntityBuilder::new()
            .species(Species::Human)
            .age(Duration::years(40))
            .build()
            .unwrap();

        assert_eq!(adult.life_stage(), LifeStage::Adult);

        // Age 8 -> Child for human
        let child = EntityBuilder::new()
            .species(Species::Human)
            .age(Duration::years(8))
            .build()
            .unwrap();

        assert_eq!(child.life_stage(), LifeStage::Child);
    }

    #[test]
    fn builder_sets_id() {
        let entity = EntityBuilder::new()
            .id("test_entity")
            .species(Species::Human)
            .build()
            .unwrap();

        assert_eq!(entity.id().as_str(), "test_entity");
    }

    #[test]
    fn builder_generates_id_if_not_set() {
        let entity = EntityBuilder::new()
            .species(Species::Human)
            .build()
            .unwrap();

        assert!(!entity.id().as_str().is_empty());
        assert!(entity.id().as_str().starts_with("entity_"));
    }

    #[test]
    fn builder_sets_age() {
        let entity = EntityBuilder::new()
            .species(Species::Human)
            .age(Duration::years(25))
            .build()
            .unwrap();

        assert_eq!(entity.age().as_years(), 25);
    }

    #[test]
    fn builder_defaults_age_to_zero() {
        let entity = EntityBuilder::new()
            .species(Species::Human)
            .build()
            .unwrap();

        assert!(entity.age().is_zero());
    }

    #[test]
    fn builder_sets_hexaco_directly() {
        let hexaco = Hexaco::new().with_openness(0.9).with_neuroticism(-0.8);

        let entity = EntityBuilder::new()
            .species(Species::Human)
            .hexaco(hexaco)
            .build()
            .unwrap();

        assert!((entity.individual_state().hexaco().openness() - 0.9).abs() < f32::EPSILON);
        assert!((entity.individual_state().hexaco().neuroticism() - (-0.8)).abs() < f32::EPSILON);
    }

    #[test]
    fn hexaco_overrides_personality() {
        let hexaco = Hexaco::uniform(0.3);

        let entity = EntityBuilder::new()
            .species(Species::Human)
            .personality(PersonalityProfile::Leader) // Would set high extraversion
            .hexaco(hexaco) // Overrides to 0.3
            .build()
            .unwrap();

        assert!((entity.individual_state().hexaco().extraversion() - 0.3).abs() < f32::EPSILON);
    }

    #[test]
    fn personality_overrides_hexaco() {
        let hexaco = Hexaco::uniform(0.3);

        let entity = EntityBuilder::new()
            .species(Species::Human)
            .hexaco(hexaco)
            .personality(PersonalityProfile::Leader) // Overrides
            .build()
            .unwrap();

        // Leader has high extraversion (0.8 -> 0.6 in -1 to 1 range)
        assert!(entity.individual_state().hexaco().extraversion() > 0.5);
    }

    #[test]
    fn builder_clone() {
        let builder = EntityBuilder::new()
            .species(Species::Human)
            .age(Duration::years(30));

        let cloned = builder.clone();

        let entity1 = builder.build().unwrap();
        let entity2 = cloned.build().unwrap();

        assert_eq!(entity1.species(), entity2.species());
        assert_eq!(entity1.age(), entity2.age());
    }

    #[test]
    fn empty_id_returns_error() {
        let result = EntityBuilder::new().id("").species(Species::Human).build();

        assert_eq!(
            result.unwrap_err(),
            EntityBuildError::InvalidId("ID cannot be empty".to_string())
        );
    }

    #[test]
    fn builder_debug() {
        let builder = EntityBuilder::new().species(Species::Human);
        let debug = format!("{:?}", builder);
        assert!(debug.contains("EntityBuilder"));
    }

    #[test]
    fn error_display() {
        let err = EntityBuildError::MissingSpecies;
        let display = format!("{}", err);
        assert!(display.contains("Species"));

        let err2 = EntityBuildError::InvalidId("test reason".to_string());
        let display2 = format!("{}", err2);
        assert!(display2.contains("test reason"));
    }

    #[test]
    fn error_debug() {
        let err = EntityBuildError::MissingSpecies;
        let debug = format!("{:?}", err);
        assert!(debug.contains("MissingSpecies"));
    }

    #[test]
    fn error_is_std_error() {
        use std::error::Error;

        let err: &dyn Error = &EntityBuildError::MissingSpecies;
        // Verify it's a valid std::error::Error
        assert!(err.source().is_none());

        let err2: &dyn Error = &EntityBuildError::InvalidId("test".to_string());
        assert!(err2.source().is_none());
    }

    #[test]
    fn builder_new() {
        let builder = EntityBuilder::new();
        let debug = format!("{:?}", builder);
        assert!(debug.contains("EntityBuilder"));
    }

    #[test]
    fn dog_age_derives_life_stage() {
        // 2-year-old dog should be YoungAdult
        let dog = EntityBuilder::new()
            .species(Species::Dog)
            .age(Duration::years(2))
            .build()
            .unwrap();

        assert_eq!(dog.life_stage(), LifeStage::YoungAdult);
    }

    #[test]
    fn explicit_life_stage_overrides_age_derived() {
        let entity = EntityBuilder::new()
            .species(Species::Human)
            .age(Duration::years(30)) // Would be Adult
            .life_stage(LifeStage::Elder) // Override to Elder
            .build()
            .unwrap();

        assert_eq!(entity.life_stage(), LifeStage::Elder);
    }

    #[test]
    fn default_personality_is_balanced() {
        let entity = EntityBuilder::new()
            .species(Species::Human)
            .build()
            .unwrap();

        // Balanced profile has 0.5 for all, which maps to 0.0 in -1 to 1 range
        assert!((entity.individual_state().hexaco().openness() - 0.0).abs() < 0.01);
    }

    #[test]
    fn default_person_characteristics_are_neutral() {
        let entity = EntityBuilder::new()
            .species(Species::Human)
            .build()
            .unwrap();

        // Default PC has neutral values (around 0.5)
        let pc = entity.individual_state().person_characteristics();
        assert!(pc.resource() >= 0.3 && pc.resource() <= 0.7);
        assert!(pc.force() >= 0.3 && pc.force() <= 0.7);
    }

    #[test]
    fn builder_with_context() {
        use crate::context::{Microsystem, WorkContext};
        use crate::types::MicrosystemId;

        let mut context = EcologicalContext::default();
        let work_id = MicrosystemId::new("work_primary").unwrap();
        context.add_microsystem(work_id, Microsystem::new_work(WorkContext::default()));

        let entity = EntityBuilder::new()
            .species(Species::Human)
            .with_context(context)
            .build()
            .unwrap();

        assert_eq!(entity.context().microsystem_count(), 1);
    }

    #[test]
    fn builder_without_context_uses_default() {
        let entity = EntityBuilder::new()
            .species(Species::Human)
            .build()
            .unwrap();

        // Default context has no microsystems
        assert_eq!(entity.context().microsystem_count(), 0);
    }

    #[test]
    fn builder_sets_birth_date() {
        let birth = Timestamp::from_ymd_hms(1990, 6, 15, 0, 0, 0);
        let entity = EntityBuilder::new()
            .id("person_001")
            .species(Species::Human)
            .birth_date(birth)
            .build()
            .unwrap();

        assert_eq!(entity.birth_date(), Some(birth));
    }

    #[test]
    fn builder_without_birth_date_returns_none() {
        let entity = EntityBuilder::new()
            .species(Species::Human)
            .build()
            .unwrap();

        assert!(entity.birth_date().is_none());
    }

    #[test]
    fn builder_birth_date_can_be_used_with_age() {
        let birth = Timestamp::from_ymd_hms(1990, 6, 15, 0, 0, 0);
        let entity = EntityBuilder::new()
            .species(Species::Human)
            .birth_date(birth)
            .age(Duration::years(30))
            .build()
            .unwrap();

        // Both should be set independently
        assert_eq!(entity.birth_date(), Some(birth));
        assert_eq!(entity.age().as_years(), 30);
    }

    #[test]
    fn builder_sets_mood() {
        use crate::state::Mood;

        let mood = Mood::new().with_valence_base(0.6).with_arousal_base(-0.3);

        let entity = EntityBuilder::new()
            .species(Species::Human)
            .mood(mood.clone())
            .build()
            .unwrap();

        assert_eq!(entity.individual_state().mood(), &mood);
    }

    #[test]
    fn builder_sets_needs() {
        use crate::state::Needs;

        let needs = Needs::new().with_fatigue_base(0.4).with_stress_base(0.2);

        let entity = EntityBuilder::new()
            .species(Species::Human)
            .needs(needs.clone())
            .build()
            .unwrap();

        assert_eq!(entity.individual_state().needs(), &needs);
    }

    #[test]
    fn builder_sets_mental_health() {
        use crate::state::MentalHealth;

        let mh = MentalHealth::new()
            .with_depression_base(0.15)
            .with_hopelessness_base(0.1);

        let entity = EntityBuilder::new()
            .species(Species::Human)
            .mental_health(mh.clone())
            .build()
            .unwrap();

        assert_eq!(entity.individual_state().mental_health(), &mh);
    }

    #[test]
    fn builder_sets_social_cognition() {
        use crate::state::SocialCognition;

        let sc = SocialCognition::new()
            .with_loneliness_base(0.25)
            .with_perceived_reciprocal_caring_base(0.7);

        let entity = EntityBuilder::new()
            .species(Species::Human)
            .social_cognition(sc.clone())
            .build()
            .unwrap();

        assert_eq!(entity.individual_state().social_cognition(), &sc);
    }

    #[test]
    fn builder_sets_disposition() {
        use crate::state::Disposition;

        let disp = Disposition::new()
            .with_empathy_base(0.8)
            .with_impulse_control_base(0.7);

        let entity = EntityBuilder::new()
            .species(Species::Human)
            .disposition(disp.clone())
            .build()
            .unwrap();

        assert_eq!(entity.individual_state().disposition(), &disp);
    }

    #[test]
    fn builder_without_optional_state_uses_defaults() {
        use crate::state::{Disposition, MentalHealth, Needs, SocialCognition};

        let entity = EntityBuilder::new()
            .species(Species::Human)
            .build()
            .unwrap();

        // Mood is derived from personality (not default) - see builder_derives_mood_from_personality test
        // Other components should be default
        assert_eq!(entity.individual_state().needs(), &Needs::default());
        assert_eq!(
            entity.individual_state().mental_health(),
            &MentalHealth::default()
        );
        assert_eq!(
            entity.individual_state().social_cognition(),
            &SocialCognition::default()
        );
        assert_eq!(
            entity.individual_state().disposition(),
            &Disposition::default()
        );
    }

    #[test]
    fn builder_derives_mood_from_personality() {
        // Extraverted personality should produce positive baseline valence
        let entity = EntityBuilder::new()
            .species(Species::Human)
            .personality(PersonalityProfile::Leader) // Leaders are extraverted
            .build()
            .unwrap();

        // Leader profile has high extraversion, so baseline valence should be positive
        let valence = entity.individual_state().mood().valence_base();
        assert!(valence > 0.0);
    }

    #[test]
    fn explicit_mood_overrides_personality_derived() {
        use crate::state::Mood;

        // Set explicit mood that differs from what personality would derive
        let explicit_mood = Mood::new()
            .with_valence_base(-0.5) // Negative valence despite personality
            .with_arousal_base(0.3);

        let entity = EntityBuilder::new()
            .species(Species::Human)
            .personality(PersonalityProfile::Leader) // Would derive positive valence
            .mood(explicit_mood.clone()) // Override with explicit negative
            .build()
            .unwrap();

        // Explicit mood should override personality-derived mood
        assert_eq!(entity.individual_state().mood(), &explicit_mood);
        assert!((entity.individual_state().mood().valence_base() - (-0.5)).abs() < f32::EPSILON);
    }

    #[test]
    fn builder_sets_all_state_components_together() {
        use crate::state::{Disposition, MentalHealth, Mood, Needs, SocialCognition};

        let mood = Mood::new().with_valence_base(0.5);
        let needs = Needs::new().with_purpose_base(0.8);
        let mh = MentalHealth::new().with_depression_base(0.1);
        let sc = SocialCognition::new().with_loneliness_base(0.2);
        let disp = Disposition::new().with_empathy_base(0.9);

        let entity = EntityBuilder::new()
            .species(Species::Human)
            .mood(mood.clone())
            .needs(needs.clone())
            .mental_health(mh.clone())
            .social_cognition(sc.clone())
            .disposition(disp.clone())
            .build()
            .unwrap();

        assert_eq!(entity.individual_state().mood(), &mood);
        assert_eq!(entity.individual_state().needs(), &needs);
        assert_eq!(entity.individual_state().mental_health(), &mh);
        assert_eq!(entity.individual_state().social_cognition(), &sc);
        assert_eq!(entity.individual_state().disposition(), &disp);
    }
}
