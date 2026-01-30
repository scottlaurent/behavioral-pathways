//! Core Relationship struct for entity-to-entity connections.
//!
//! A Relationship represents the connection between two entities, including
//! shared dimensions, directional feelings, trustworthiness perceptions,
//! and perceived risk.

use crate::enums::{BondType, Direction, DirectionalPath, RelPath, RelationshipSchema};
use crate::relationship::{
    AntecedentDirection, DirectionalDimensions, InteractionPattern, PerceivedRisk,
    RelationshipStage, SharedDimensions, StakesLevel, TrustAntecedent, TrustDecision,
    TrustworthinessFactors,
};
use crate::state::StateValue;
use crate::types::{Duration, EntityId, RelationshipId, Timestamp};

/// Error type for relationship operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelationshipError {
    /// Cannot create relationship between an entity and itself.
    SelfRelationship,
    /// Invalid stage transition.
    InvalidStageTransition {
        from: RelationshipStage,
        to: RelationshipStage,
    },
}

impl std::fmt::Display for RelationshipError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RelationshipError::SelfRelationship => {
                write!(f, "Cannot create relationship between an entity and itself")
            }
            RelationshipError::InvalidStageTransition { from, to } => {
                write!(f, "Invalid stage transition from {} to {}", from, to)
            }
        }
    }
}

impl std::error::Error for RelationshipError {}

/// Error type for invalid stage transitions.
///
/// This is returned by `Relationship::set_stage()` when the transition
/// is not allowed. Currently all transitions are allowed for flexibility,
/// but Phase 8 may add restrictions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StageTransitionError {
    /// The stage being transitioned from.
    pub from: RelationshipStage,
    /// The stage being transitioned to.
    pub to: RelationshipStage,
}

impl StageTransitionError {
    /// Creates a new stage transition error.
    #[must_use]
    pub fn new(from: RelationshipStage, to: RelationshipStage) -> Self {
        StageTransitionError { from, to }
    }
}

impl std::fmt::Display for StageTransitionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid stage transition from {} to {}",
            self.from, self.to
        )
    }
}

impl std::error::Error for StageTransitionError {}

/// Maximum number of antecedents retained per direction.
/// Increased from 20 to 100 to avoid cliff truncation (Issue #12).
/// With temporal decay applied during recomputation, older antecedents
/// have minimal impact anyway, so this cap is primarily for memory safety.
const MAX_ANTECEDENT_HISTORY: usize = 100;

/// A relationship between two entities.
///
/// Relationships contain:
/// - Identification (pair of entities, relationship ID)
/// - Bond types (Friend, Colleague, Family, etc.)
/// - Relationship stage (Stranger -> Acquaintance -> Established -> Intimate)
/// - Shared dimensions (symmetric - same from both perspectives)
/// - Directional data (asymmetric - A's view of B may differ from B's view of A)
///
/// # Direction Semantics
///
/// In `Relationship::try_between(entity_a, entity_b)`:
/// - `Direction::AToB` = entity_a's perspective on entity_b
/// - `Direction::BToA` = entity_b's perspective on entity_a
///
/// The first entity passed to `try_between()` is always "A".
///
/// # Examples
///
/// ```
/// use behavioral_pathways::relationship::Relationship;
/// use behavioral_pathways::types::EntityId;
/// use behavioral_pathways::enums::{BondType, RelPath, SharedPath, Direction, DirectionalPath, TrustPath};
///
/// let alice = EntityId::new("alice").unwrap();
/// let bob = EntityId::new("bob").unwrap();
///
/// let mut rel = Relationship::try_between(alice.clone(), bob.clone()).unwrap();
///
/// // Add bond type
/// rel.add_bond(BondType::Colleague);
///
/// // Access shared dimensions
/// let affinity = rel.get(RelPath::Shared(SharedPath::Affinity));
///
/// // Access directional trust (Alice's perception of Bob)
/// let competence = rel.get(RelPath::Directional(
///     Direction::AToB,
///     DirectionalPath::Trust(TrustPath::Competence)
/// ));
/// ```
#[derive(Debug, Clone)]
pub struct Relationship {
    /// Unique identifier for this relationship.
    id: RelationshipId,

    /// The first entity (defines "A" in AToB).
    entity_a: EntityId,

    /// The second entity (defines "B" in AToB).
    entity_b: EntityId,

    /// Bond types (can have multiple).
    bonds: Vec<BondType>,

    /// Relationship schema (structural type).
    schema: RelationshipSchema,

    /// Current relationship stage.
    stage: RelationshipStage,

    /// Shared (symmetric) dimensions.
    shared: SharedDimensions,

    /// A's trustworthiness perceptions of B.
    trustworthiness_a_to_b: TrustworthinessFactors,

    /// B's trustworthiness perceptions of A.
    trustworthiness_b_to_a: TrustworthinessFactors,

    /// A's perceived risk of trusting B.
    perceived_risk_a_to_b: PerceivedRisk,

    /// B's perceived risk of trusting A.
    perceived_risk_b_to_a: PerceivedRisk,

    /// A's directional feelings toward B.
    directional_a_to_b: DirectionalDimensions,

    /// B's directional feelings toward A.
    directional_b_to_a: DirectionalDimensions,

    /// Interaction pattern (frequency/consistency).
    pattern: InteractionPattern,

    /// Trust antecedent history for A's perception of B.
    antecedent_history_a_to_b: Vec<TrustAntecedent>,

    /// Trust antecedent history for B's perception of A.
    antecedent_history_b_to_a: Vec<TrustAntecedent>,

    /// Most recent negative antecedent timestamp (A's perspective).
    last_negative_antecedent_a_to_b: Option<Timestamp>,

    /// Most recent negative antecedent timestamp (B's perspective).
    last_negative_antecedent_b_to_a: Option<Timestamp>,
}

impl Relationship {
    /// Creates a new relationship between two entities.
    ///
    /// Returns an error if entity_a equals entity_b.
    ///
    /// # Arguments
    ///
    /// * `entity_a` - The first entity (defines "A")
    /// * `entity_b` - The second entity (defines "B")
    ///
    /// # Errors
    ///
    /// Returns `RelationshipError::SelfRelationship` if both entities are the same.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::relationship::Relationship;
    /// use behavioral_pathways::types::EntityId;
    ///
    /// let alice = EntityId::new("alice").unwrap();
    /// let bob = EntityId::new("bob").unwrap();
    ///
    /// let rel = Relationship::try_between(alice, bob).unwrap();
    /// ```
    pub fn try_between(entity_a: EntityId, entity_b: EntityId) -> Result<Self, RelationshipError> {
        if entity_a == entity_b {
            return Err(RelationshipError::SelfRelationship);
        }

        // Generate relationship ID from entity IDs.
        // The format "rel_{a}_{b}" is always non-empty since EntityId requires
        // non-empty strings, so RelationshipId::new() cannot fail.
        let rel_id = format!("rel_{}_{}", entity_a.as_str(), entity_b.as_str());
        let id = RelationshipId::new(rel_id).unwrap();

        Ok(Relationship {
            id,
            entity_a,
            entity_b,
            bonds: Vec::new(),
            schema: RelationshipSchema::default(),
            stage: RelationshipStage::default(),
            shared: SharedDimensions::new(),
            trustworthiness_a_to_b: TrustworthinessFactors::new(),
            trustworthiness_b_to_a: TrustworthinessFactors::new(),
            perceived_risk_a_to_b: PerceivedRisk::new(),
            perceived_risk_b_to_a: PerceivedRisk::new(),
            directional_a_to_b: DirectionalDimensions::new(),
            directional_b_to_a: DirectionalDimensions::new(),
            pattern: InteractionPattern::default(),
            antecedent_history_a_to_b: Vec::new(),
            antecedent_history_b_to_a: Vec::new(),
            last_negative_antecedent_a_to_b: None,
            last_negative_antecedent_b_to_a: None,
        })
    }

    /// Sets the bond type, replacing any existing bonds.
    ///
    /// Use `add_bond` to add multiple bond types.
    #[must_use]
    pub fn with_bond(mut self, bond: BondType) -> Self {
        self.bonds = vec![bond];
        self
    }

    /// Sets multiple bond types.
    #[must_use]
    pub fn with_bonds(mut self, bonds: Vec<BondType>) -> Self {
        self.bonds = bonds;
        self
    }

    /// Sets the relationship schema.
    #[must_use]
    pub fn with_schema(mut self, schema: RelationshipSchema) -> Self {
        self.schema = schema;
        self
    }

    /// Sets the initial relationship stage.
    #[must_use]
    pub fn with_stage(mut self, stage: RelationshipStage) -> Self {
        self.stage = stage;
        self
    }

    // Accessors

    /// Returns the relationship ID.
    #[must_use]
    pub fn id(&self) -> &RelationshipId {
        &self.id
    }

    /// Returns the first entity (A).
    #[must_use]
    pub fn entity_a(&self) -> &EntityId {
        &self.entity_a
    }

    /// Returns the second entity (B).
    #[must_use]
    pub fn entity_b(&self) -> &EntityId {
        &self.entity_b
    }

    /// Returns the entity IDs as a tuple (A, B).
    #[must_use]
    pub fn entities(&self) -> (&EntityId, &EntityId) {
        (&self.entity_a, &self.entity_b)
    }

    /// Returns the bond types.
    #[must_use]
    pub fn bonds(&self) -> &[BondType] {
        &self.bonds
    }

    /// Returns the relationship schema.
    #[must_use]
    pub fn schema(&self) -> RelationshipSchema {
        self.schema
    }

    /// Returns the current relationship stage.
    #[must_use]
    pub fn stage(&self) -> RelationshipStage {
        self.stage
    }

    /// Returns a reference to the shared dimensions.
    #[must_use]
    pub fn shared(&self) -> &SharedDimensions {
        &self.shared
    }

    /// Returns a mutable reference to the shared dimensions.
    pub fn shared_mut(&mut self) -> &mut SharedDimensions {
        &mut self.shared
    }

    /// Returns a reference to the interaction pattern.
    #[must_use]
    pub fn pattern(&self) -> &InteractionPattern {
        &self.pattern
    }

    /// Returns a mutable reference to the interaction pattern.
    pub fn pattern_mut(&mut self) -> &mut InteractionPattern {
        &mut self.pattern
    }

    // Bond management

    /// Adds a bond type to this relationship.
    pub fn add_bond(&mut self, bond: BondType) {
        if self.bonds.contains(&bond) {
            return;
        }
        self.bonds.push(bond);
    }

    /// Removes a bond type from this relationship.
    pub fn remove_bond(&mut self, bond: BondType) {
        // Remove all instances by iterating backwards
        let mut index = self.bonds.len();
        while index > 0 {
            index -= 1;
            // Skip if not matching
            if self.bonds[index] != bond {
                continue;
            }
            self.bonds.swap_remove(index);
        }
    }

    /// Returns true if this relationship has the specified bond.
    #[must_use]
    pub fn has_bond(&self, bond: BondType) -> bool {
        self.bonds.contains(&bond)
    }

    /// Sets the relationship schema.
    pub fn set_schema(&mut self, schema: RelationshipSchema) {
        self.schema = schema;
    }

    // Stage management

    /// Sets the relationship stage.
    ///
    /// This is primarily for testing. In production, stages should
    /// transition based on events.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid transitions (currently all transitions
    /// are allowed for flexibility, but Phase 8 may add restrictions).
    pub fn set_stage(&mut self, stage: RelationshipStage) -> Result<(), StageTransitionError> {
        // For now, allow all transitions. Phase 8 may add restrictions.
        self.stage = stage;
        Ok(())
    }

    // Trustworthiness access

    /// Returns a reference to the trustworthiness factors for a direction.
    #[must_use]
    pub fn trustworthiness(&self, direction: Direction) -> &TrustworthinessFactors {
        match direction {
            Direction::AToB => &self.trustworthiness_a_to_b,
            Direction::BToA => &self.trustworthiness_b_to_a,
        }
    }

    /// Returns a mutable reference to the trustworthiness factors for a direction.
    pub fn trustworthiness_mut(&mut self, direction: Direction) -> &mut TrustworthinessFactors {
        match direction {
            Direction::AToB => &mut self.trustworthiness_a_to_b,
            Direction::BToA => &mut self.trustworthiness_b_to_a,
        }
    }

    // Perceived risk access

    /// Returns a reference to the perceived risk for a direction.
    #[must_use]
    pub fn perceived_risk(&self, direction: Direction) -> &PerceivedRisk {
        match direction {
            Direction::AToB => &self.perceived_risk_a_to_b,
            Direction::BToA => &self.perceived_risk_b_to_a,
        }
    }

    /// Returns a mutable reference to the perceived risk for a direction.
    pub fn perceived_risk_mut(&mut self, direction: Direction) -> &mut PerceivedRisk {
        match direction {
            Direction::AToB => &mut self.perceived_risk_a_to_b,
            Direction::BToA => &mut self.perceived_risk_b_to_a,
        }
    }

    // Directional dimensions access

    /// Returns a reference to the directional dimensions for a direction.
    #[must_use]
    pub fn directional(&self, direction: Direction) -> &DirectionalDimensions {
        match direction {
            Direction::AToB => &self.directional_a_to_b,
            Direction::BToA => &self.directional_b_to_a,
        }
    }

    /// Returns a mutable reference to the directional dimensions for a direction.
    pub fn directional_mut(&mut self, direction: Direction) -> &mut DirectionalDimensions {
        match direction {
            Direction::AToB => &mut self.directional_a_to_b,
            Direction::BToA => &mut self.directional_b_to_a,
        }
    }

    // Trust antecedent history

    /// Appends a trust antecedent to the history for the given direction.
    pub fn append_antecedent(&mut self, direction: Direction, antecedent: TrustAntecedent) {
        match direction {
            Direction::AToB => {
                Self::push_antecedent(
                    &mut self.antecedent_history_a_to_b,
                    antecedent,
                    &mut self.last_negative_antecedent_a_to_b,
                );
            }
            Direction::BToA => {
                Self::push_antecedent(
                    &mut self.antecedent_history_b_to_a,
                    antecedent,
                    &mut self.last_negative_antecedent_b_to_a,
                );
            }
        }
    }

    /// Returns the antecedent history for the given direction.
    #[must_use]
    pub fn antecedent_history(&self, direction: Direction) -> &[TrustAntecedent] {
        match direction {
            Direction::AToB => &self.antecedent_history_a_to_b,
            Direction::BToA => &self.antecedent_history_b_to_a,
        }
    }

    /// Returns the last negative antecedent timestamp for the given direction.
    #[must_use]
    pub fn last_negative_antecedent(&self, direction: Direction) -> Option<Timestamp> {
        match direction {
            Direction::AToB => self.last_negative_antecedent_a_to_b,
            Direction::BToA => self.last_negative_antecedent_b_to_a,
        }
    }

    fn push_antecedent(
        history: &mut Vec<TrustAntecedent>,
        antecedent: TrustAntecedent,
        last_negative: &mut Option<Timestamp>,
    ) {
        if antecedent.direction() == AntecedentDirection::Negative {
            *last_negative = Some(antecedent.timestamp());
        }

        history.push(antecedent);
        if history.len() > MAX_ANTECEDENT_HISTORY {
            history.sort_by_key(|entry| entry.timestamp());
            let overflow = history.len().saturating_sub(MAX_ANTECEDENT_HISTORY);
            history.drain(0..overflow);
        }
    }

    // Path-based access

    /// Gets a StateValue reference by path.
    ///
    /// Returns None for RelPath::Stage (not a StateValue).
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::relationship::Relationship;
    /// use behavioral_pathways::types::EntityId;
    /// use behavioral_pathways::enums::{RelPath, SharedPath, Direction, DirectionalPath, TrustPath};
    ///
    /// let alice = EntityId::new("alice").unwrap();
    /// let bob = EntityId::new("bob").unwrap();
    /// let rel = Relationship::try_between(alice, bob).unwrap();
    ///
    /// // Get shared affinity
    /// let affinity = rel.get(RelPath::Shared(SharedPath::Affinity));
    /// assert!(affinity.is_some());
    ///
    /// // Get directional trust
    /// let competence = rel.get(RelPath::Directional(
    ///     Direction::AToB,
    ///     DirectionalPath::Trust(TrustPath::Competence)
    /// ));
    /// assert!(competence.is_some());
    /// ```
    #[must_use]
    pub fn get(&self, path: RelPath) -> Option<&StateValue> {
        match path {
            RelPath::Shared(sp) => Some(self.shared.get(sp)),
            RelPath::Directional(dir, dp) => self.get_directional(dir, dp),
            RelPath::Stage => None, // Stage is not a StateValue
        }
    }

    /// Gets a mutable StateValue reference by path.
    ///
    /// Returns None for RelPath::Stage (not a StateValue).
    pub fn get_mut(&mut self, path: RelPath) -> Option<&mut StateValue> {
        match path {
            RelPath::Shared(sp) => Some(self.shared.get_mut(sp)),
            RelPath::Directional(dir, dp) => self.get_directional_mut(dir, dp),
            RelPath::Stage => None,
        }
    }

    /// Gets a directional StateValue reference.
    ///
    /// Returns None for computed paths like TrustPath::SupportWillingness.
    fn get_directional(&self, direction: Direction, path: DirectionalPath) -> Option<&StateValue> {
        let dims = self.directional(direction);
        let trust = self.trustworthiness(direction);
        let risk = self.perceived_risk(direction);

        match path {
            DirectionalPath::Trust(tp) => trust.get(tp),
            DirectionalPath::Warmth => Some(dims.warmth()),
            DirectionalPath::Resentment => Some(dims.resentment()),
            DirectionalPath::Dependence => Some(dims.dependence()),
            DirectionalPath::Attraction => Some(dims.attraction()),
            DirectionalPath::Attachment => Some(dims.attachment()),
            DirectionalPath::Jealousy => Some(dims.jealousy()),
            DirectionalPath::Fear => Some(dims.fear()),
            DirectionalPath::Obligation => Some(dims.obligation()),
            DirectionalPath::PerceivedRisk => Some(risk.state_value()),
        }
    }

    /// Gets a mutable directional StateValue reference.
    ///
    /// Returns None for computed paths like TrustPath::SupportWillingness.
    fn get_directional_mut(
        &mut self,
        direction: Direction,
        path: DirectionalPath,
    ) -> Option<&mut StateValue> {
        match path {
            DirectionalPath::Trust(tp) => self.trustworthiness_mut(direction).get_mut(tp),
            DirectionalPath::Warmth => Some(self.directional_mut(direction).warmth_mut()),
            DirectionalPath::Resentment => Some(self.directional_mut(direction).resentment_mut()),
            DirectionalPath::Dependence => Some(self.directional_mut(direction).dependence_mut()),
            DirectionalPath::Attraction => Some(self.directional_mut(direction).attraction_mut()),
            DirectionalPath::Attachment => Some(self.directional_mut(direction).attachment_mut()),
            DirectionalPath::Jealousy => Some(self.directional_mut(direction).jealousy_mut()),
            DirectionalPath::Fear => Some(self.directional_mut(direction).fear_mut()),
            DirectionalPath::Obligation => Some(self.directional_mut(direction).obligation_mut()),
            DirectionalPath::PerceivedRisk => {
                Some(self.perceived_risk_mut(direction).state_value_mut())
            }
        }
    }

    // Trust computation

    /// Computes a trust decision for the specified direction.
    ///
    /// # Arguments
    ///
    /// * `direction` - Which direction (AToB or BToA)
    /// * `trustor_propensity` - The trustor's dispositional trust propensity (0-1)
    /// * `stakes` - The stakes level for the action
    ///
    /// # Trust Computation Formula
    ///
    /// ```text
    /// willingness = context_multiplier * (
    ///                 propensity_weight * propensity
    ///               + trustworthiness_weight * perceived_trustworthiness
    ///             )
    ///             - risk_weight * perceived_risk
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::relationship::{Relationship, StakesLevel};
    /// use behavioral_pathways::types::EntityId;
    /// use behavioral_pathways::enums::Direction;
    ///
    /// let alice = EntityId::new("alice").unwrap();
    /// let bob = EntityId::new("bob").unwrap();
    /// let rel = Relationship::try_between(alice, bob).unwrap();
    ///
    /// let decision = rel.compute_trust_decision(
    ///     Direction::AToB,
    ///     0.5, // Alice's trust propensity
    ///     StakesLevel::Medium
    /// );
    /// ```
    #[must_use]
    pub fn compute_trust_decision(
        &self,
        direction: Direction,
        trustor_propensity: f32,
        stakes: StakesLevel,
    ) -> TrustDecision {
        self.compute_trust_decision_with_context(direction, trustor_propensity, stakes, 1.0)
    }

    /// Computes a TrustDecision for the given stakes level with a context multiplier.
    #[must_use]
    pub fn compute_trust_decision_with_context(
        &self,
        direction: Direction,
        trustor_propensity: f32,
        stakes: StakesLevel,
        context_multiplier: f32,
    ) -> TrustDecision {
        let trustworthiness = self.trustworthiness(direction);
        let risk = self.perceived_risk(direction);
        let context_multiplier = context_multiplier.clamp(0.0, 2.0);

        // Get weights based on relationship stage
        let prop_weight = self.stage.propensity_weight();
        let trust_weight = self.stage.trustworthiness_weight();
        let stage_risk_mod = self.stage.risk_modifier();

        // Compute risk with stage modifier
        let perceived_risk = risk.compute_with_stage_modifier(stakes, stage_risk_mod);

        // Risk weight is a constant factor
        let risk_weight = 0.5;

        // Compute each willingness domain
        // Task willingness uses competence
        let task_base =
            prop_weight * trustor_propensity + trust_weight * trustworthiness.competence_effective();
        let task_willingness = (task_base * context_multiplier) - risk_weight * perceived_risk;

        // Support willingness uses benevolence
        let support_base = prop_weight * trustor_propensity
            + trust_weight * trustworthiness.benevolence_effective();
        let support_willingness =
            (support_base * context_multiplier) - risk_weight * perceived_risk;

        // Disclosure willingness uses integrity
        let disclosure_base = prop_weight * trustor_propensity
            + trust_weight * trustworthiness.integrity_effective();
        let disclosure_willingness =
            (disclosure_base * context_multiplier) - risk_weight * perceived_risk;

        // Decision certainty: how confident we are in our willingness assessment
        // Lower for estranged because our willingness is conflicted
        let history = self.shared.history_effective();
        let stage_decision_certainty = match self.stage {
            RelationshipStage::Stranger => 0.1,
            RelationshipStage::Acquaintance => 0.3,
            RelationshipStage::Established => 0.6,
            RelationshipStage::Intimate => 0.9,
            // Know them, but trust is broken - uncertain about willingness
            RelationshipStage::Estranged => 0.5,
        };
        let decision_certainty = (history * 0.3 + stage_decision_certainty * 0.7).clamp(0.0, 1.0);

        // Trustee confidence: certainty about the trustee's attributes
        // Higher for estranged because we know them well (even if negatively)
        let stage_trustee_confidence = match self.stage {
            RelationshipStage::Stranger => 0.1,
            RelationshipStage::Acquaintance => 0.4,
            RelationshipStage::Established => 0.7,
            RelationshipStage::Intimate => 0.9,
            // Know them well, even if relationship is broken
            RelationshipStage::Estranged => 0.8,
        };
        let trustee_confidence = (history * 0.4 + stage_trustee_confidence * 0.6).clamp(0.0, 1.0);

        TrustDecision::new(
            task_willingness.clamp(0.0, 1.0),
            support_willingness.clamp(0.0, 1.0),
            disclosure_willingness.clamp(0.0, 1.0),
            decision_certainty,
            trustee_confidence,
        )
    }

    // Decay

    /// Applies decay to all relationship dimensions over the specified duration.
    pub fn apply_decay(&mut self, elapsed: Duration) {
        self.shared.apply_decay(elapsed);
        self.trustworthiness_a_to_b.apply_decay(elapsed);
        self.trustworthiness_b_to_a.apply_decay(elapsed);
        self.perceived_risk_a_to_b.apply_decay(elapsed);
        self.perceived_risk_b_to_a.apply_decay(elapsed);
        self.directional_a_to_b.apply_decay(elapsed);
        self.directional_b_to_a.apply_decay(elapsed);
    }
}

impl PartialEq for Relationship {
    fn eq(&self, other: &Self) -> bool {
        // Note: entity_a and entity_b are not compared separately because
        // the id is derived from them (format!("rel_{}_{}", entity_a, entity_b)).
        // If id matches, entity_a and entity_b are guaranteed to match.
        self.id == other.id
            && self.bonds == other.bonds
            && self.schema == other.schema
            && self.stage == other.stage
            && self.shared == other.shared
            && self.trustworthiness_a_to_b == other.trustworthiness_a_to_b
            && self.trustworthiness_b_to_a == other.trustworthiness_b_to_a
            && self.perceived_risk_a_to_b == other.perceived_risk_a_to_b
            && self.perceived_risk_b_to_a == other.perceived_risk_b_to_a
            && self.directional_a_to_b == other.directional_a_to_b
            && self.directional_b_to_a == other.directional_b_to_a
            && self.pattern == other.pattern
            && self.antecedent_history_a_to_b == other.antecedent_history_a_to_b
            && self.antecedent_history_b_to_a == other.antecedent_history_b_to_a
            && self.last_negative_antecedent_a_to_b == other.last_negative_antecedent_a_to_b
            && self.last_negative_antecedent_b_to_a == other.last_negative_antecedent_b_to_a
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::{DirectionalPath, SharedPath, TrustPath};
    use crate::relationship::{AntecedentDirection, AntecedentType};
    use crate::types::Duration;

    fn alice() -> EntityId {
        EntityId::new("alice").unwrap()
    }

    fn bob() -> EntityId {
        EntityId::new("bob").unwrap()
    }

    #[test]
    fn append_antecedent_tracks_history_and_last_negative() {
        let mut rel = Relationship::try_between(alice(), bob()).unwrap();
        let ts_positive = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
        let ts_negative = Timestamp::from_ymd_hms(2024, 2, 1, 0, 0, 0);

        let positive = TrustAntecedent::new(
            ts_positive,
            AntecedentType::Benevolence,
            AntecedentDirection::Positive,
            0.4,
            "support",
        );
        let negative = TrustAntecedent::new(
            ts_negative,
            AntecedentType::Integrity,
            AntecedentDirection::Negative,
            0.5,
            "betrayal",
        );

        rel.append_antecedent(Direction::AToB, positive);
        rel.append_antecedent(Direction::AToB, negative);

        assert_eq!(rel.antecedent_history(Direction::AToB).len(), 2);
        assert_eq!(rel.antecedent_history(Direction::BToA).len(), 0);
        assert_eq!(
            rel.last_negative_antecedent(Direction::AToB),
            Some(ts_negative)
        );
    }

    #[test]
    fn last_negative_antecedent_tracks_b_to_a() {
        let mut rel = Relationship::try_between(alice(), bob()).unwrap();
        let ts_negative = Timestamp::from_ymd_hms(2024, 3, 1, 0, 0, 0);
        let negative = TrustAntecedent::new(
            ts_negative,
            AntecedentType::Integrity,
            AntecedentDirection::Negative,
            0.4,
            "betrayal",
        );

        rel.append_antecedent(Direction::BToA, negative);

        assert_eq!(
            rel.last_negative_antecedent(Direction::BToA),
            Some(ts_negative)
        );
    }

    #[test]
    fn antecedent_history_caps_at_max_entries() {
        let mut rel = Relationship::try_between(alice(), bob()).unwrap();
        let base_time = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);

        for i in 0..(MAX_ANTECEDENT_HISTORY + 2) {
            let ts = base_time + Duration::days(i as u64);
            let antecedent = TrustAntecedent::new(
                ts,
                AntecedentType::Ability,
                AntecedentDirection::Positive,
                0.1,
                "interaction",
            );
            rel.append_antecedent(Direction::AToB, antecedent);
        }

        assert_eq!(
            rel.antecedent_history(Direction::AToB).len(),
            MAX_ANTECEDENT_HISTORY
        );
    }

    #[test]
    fn try_between_creates_relationship() {
        let rel = Relationship::try_between(alice(), bob()).unwrap();
        assert_eq!(rel.entity_a(), &alice());
        assert_eq!(rel.entity_b(), &bob());
    }

    #[test]
    fn relationship_creation_with_ids() {
        let rel = Relationship::try_between(alice(), bob()).unwrap();
        assert!(!rel.id().as_str().is_empty());
        assert!(rel.id().as_str().contains("alice"));
        assert!(rel.id().as_str().contains("bob"));
    }

    #[test]
    fn relationship_id_accessible() {
        let rel = Relationship::try_between(alice(), bob()).unwrap();
        let id = rel.id();
        assert_eq!(id.as_str(), "rel_alice_bob");
    }

    #[test]
    fn try_between_returns_error_on_self() {
        let result = Relationship::try_between(alice(), alice());
        assert_eq!(result, Err(RelationshipError::SelfRelationship));
    }

    #[test]
    fn relationship_with_bond_type() {
        let rel = Relationship::try_between(alice(), bob())
            .unwrap()
            .with_bond(BondType::Friend);
        assert!(rel.has_bond(BondType::Friend));
    }

    #[test]
    fn with_bonds() {
        let rel = Relationship::try_between(alice(), bob())
            .unwrap()
            .with_bonds(vec![BondType::Friend, BondType::Colleague]);
        assert!(rel.has_bond(BondType::Friend));
        assert!(rel.has_bond(BondType::Colleague));
    }

    #[test]
    fn with_schema() {
        let rel = Relationship::try_between(alice(), bob())
            .unwrap()
            .with_schema(RelationshipSchema::Mentor);
        assert_eq!(rel.schema(), RelationshipSchema::Mentor);
    }

    #[test]
    fn with_stage() {
        let rel = Relationship::try_between(alice(), bob())
            .unwrap()
            .with_stage(RelationshipStage::Established);
        assert_eq!(rel.stage(), RelationshipStage::Established);
    }

    #[test]
    fn entities_accessor() {
        let rel = Relationship::try_between(alice(), bob()).unwrap();
        let (a, b) = rel.entities();
        assert_eq!(a, &alice());
        assert_eq!(b, &bob());
    }

    #[test]
    fn add_and_remove_bond() {
        let mut rel = Relationship::try_between(alice(), bob()).unwrap();
        rel.add_bond(BondType::Friend);
        assert!(rel.has_bond(BondType::Friend));

        rel.add_bond(BondType::Friend); // Duplicate
        assert_eq!(rel.bonds().len(), 1);

        rel.remove_bond(BondType::Friend);
        assert!(!rel.has_bond(BondType::Friend));
    }

    #[test]
    fn remove_bond_skips_non_matching_entries() {
        let mut rel = Relationship::try_between(alice(), bob()).unwrap();
        rel.add_bond(BondType::Friend);
        rel.add_bond(BondType::Colleague);

        rel.remove_bond(BondType::Family);

        assert!(rel.has_bond(BondType::Friend));
        assert!(rel.has_bond(BondType::Colleague));
    }

    #[test]
    fn set_schema() {
        let mut rel = Relationship::try_between(alice(), bob()).unwrap();
        rel.set_schema(RelationshipSchema::Romantic);
        assert_eq!(rel.schema(), RelationshipSchema::Romantic);
    }

    #[test]
    fn relationship_stage_manual_transition() {
        let mut rel = Relationship::try_between(alice(), bob()).unwrap();
        assert_eq!(rel.stage(), RelationshipStage::Stranger);

        rel.set_stage(RelationshipStage::Acquaintance).unwrap();
        assert_eq!(rel.stage(), RelationshipStage::Acquaintance);

        rel.set_stage(RelationshipStage::Established).unwrap();
        assert_eq!(rel.stage(), RelationshipStage::Established);
    }

    #[test]
    fn directional_trust_asymmetry() {
        let mut rel = Relationship::try_between(alice(), bob()).unwrap();

        // A's perception of B
        rel.trustworthiness_mut(Direction::AToB)
            .add_competence_delta(0.3);

        // B's perception of A
        rel.trustworthiness_mut(Direction::BToA)
            .add_competence_delta(0.1);

        let a_of_b = rel.trustworthiness(Direction::AToB).competence_effective();
        let b_of_a = rel.trustworthiness(Direction::BToA).competence_effective();

        assert!(a_of_b > b_of_a);
    }

    #[test]
    fn relationship_direction_semantics() {
        let mut rel = Relationship::try_between(alice(), bob()).unwrap();

        // AToB is first argument's (alice) view of second argument (bob)
        rel.directional_mut(Direction::AToB).add_warmth_delta(0.5);

        // BToA is second argument's (bob) view of first argument (alice)
        rel.directional_mut(Direction::BToA).add_warmth_delta(0.1);

        let alice_warmth_to_bob = rel.directional(Direction::AToB).warmth_effective();
        let bob_warmth_to_alice = rel.directional(Direction::BToA).warmth_effective();

        assert!(alice_warmth_to_bob > bob_warmth_to_alice);
    }

    #[test]
    fn rel_path_shared_affinity() {
        let rel = Relationship::try_between(alice(), bob()).unwrap();
        let affinity = rel.get(RelPath::Shared(SharedPath::Affinity));
        assert!(affinity.is_some());
    }

    #[test]
    fn rel_path_directional_trust() {
        let rel = Relationship::try_between(alice(), bob()).unwrap();
        let competence = rel.get(RelPath::Directional(
            Direction::AToB,
            DirectionalPath::Trust(TrustPath::Competence),
        ));
        assert!(competence.is_some());
    }

    #[test]
    fn direction_a_to_b_semantics() {
        let mut rel = Relationship::try_between(alice(), bob()).unwrap();

        // Modify via path
        let sv = rel
            .get_mut(RelPath::Directional(
                Direction::AToB,
                DirectionalPath::Trust(TrustPath::Benevolence),
            ))
            .expect("directional trust path should be mutable");
        sv.add_delta(0.2);

        // Verify via direct access
        let benevolence = rel.trustworthiness(Direction::AToB).benevolence_effective();
        assert!((benevolence - 0.5).abs() < f32::EPSILON); // base 0.3 + delta 0.2
    }

    #[test]
    fn get_returns_none_for_stage() {
        let rel = Relationship::try_between(alice(), bob()).unwrap();
        assert!(rel.get(RelPath::Stage).is_none());
    }

    #[test]
    fn get_mut_returns_none_for_stage() {
        let mut rel = Relationship::try_between(alice(), bob()).unwrap();
        assert!(rel.get_mut(RelPath::Stage).is_none());
    }

    #[test]
    fn get_mut_shared_paths_updates_values() {
        let mut rel = Relationship::try_between(alice(), bob()).unwrap();
        rel.get_mut(RelPath::Shared(SharedPath::Respect))
            .unwrap()
            .add_delta(0.2);
        rel.get_mut(RelPath::Shared(SharedPath::Tension))
            .unwrap()
            .add_delta(0.3);
        rel.get_mut(RelPath::Shared(SharedPath::Intimacy))
            .unwrap()
            .add_delta(0.4);
        rel.get_mut(RelPath::Shared(SharedPath::History))
            .unwrap()
            .add_delta(0.5);

        assert!((rel.shared().respect().delta() - 0.2).abs() < f32::EPSILON);
        assert!((rel.shared().tension().delta() - 0.3).abs() < f32::EPSILON);
        assert!((rel.shared().intimacy().delta() - 0.4).abs() < f32::EPSILON);
        assert!((rel.shared().history().delta() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn get_mut_directional_non_trust_paths() {
        let mut rel = Relationship::try_between(alice(), bob()).unwrap();
        let paths = [
            (DirectionalPath::Warmth, 0.1),
            (DirectionalPath::Resentment, 0.2),
            (DirectionalPath::Dependence, 0.3),
            (DirectionalPath::Attraction, 0.4),
            (DirectionalPath::Attachment, 0.5),
            (DirectionalPath::Jealousy, 0.6),
            (DirectionalPath::Fear, 0.7),
            (DirectionalPath::Obligation, 0.8),
        ];

        for (path, delta) in paths {
            rel.get_mut(RelPath::Directional(Direction::AToB, path))
                .unwrap()
                .add_delta(delta);
        }

        rel.get_mut(RelPath::Directional(
            Direction::BToA,
            DirectionalPath::PerceivedRisk,
        ))
        .unwrap()
        .add_delta(0.2);

        let directional = rel.directional(Direction::AToB);
        assert!((directional.warmth().delta() - 0.1).abs() < f32::EPSILON);
        assert!((directional.resentment().delta() - 0.2).abs() < f32::EPSILON);
        assert!((directional.dependence().delta() - 0.3).abs() < f32::EPSILON);
        assert!((directional.attraction().delta() - 0.4).abs() < f32::EPSILON);
        assert!((directional.attachment().delta() - 0.5).abs() < f32::EPSILON);
        assert!((directional.jealousy().delta() - 0.6).abs() < f32::EPSILON);
        assert!((directional.fear().delta() - 0.7).abs() < f32::EPSILON);
        assert!((directional.obligation().delta() - 0.8).abs() < f32::EPSILON);
        assert!((rel.perceived_risk(Direction::BToA).delta() - 0.2).abs() < f32::EPSILON);
    }

    #[test]
    fn compute_trust_decision_basic() {
        let rel = Relationship::try_between(alice(), bob()).unwrap();
        let decision = rel.compute_trust_decision(Direction::AToB, 0.5, StakesLevel::Low);

        assert!(decision.task_willingness() > 0.0);
        assert!(decision.task_willingness() <= 1.0);
    }

    #[test]
    fn context_multiplier_scales_trust_decision() {
        let rel = Relationship::try_between(alice(), bob()).unwrap();
        let baseline =
            rel.compute_trust_decision_with_context(Direction::AToB, 0.5, StakesLevel::Low, 1.0);
        let constrained =
            rel.compute_trust_decision_with_context(Direction::AToB, 0.5, StakesLevel::Low, 0.5);

        assert!(constrained.task_willingness() < baseline.task_willingness());
    }

    #[test]
    fn propensity_weight_diminishes_with_stage() {
        let stranger = Relationship::try_between(alice(), bob()).unwrap();
        let mut intimate = Relationship::try_between(alice(), bob()).unwrap();
        intimate.set_stage(RelationshipStage::Intimate).unwrap();

        // Same propensity and trustworthiness
        let propensity = 0.8;

        let stranger_decision =
            stranger.compute_trust_decision(Direction::AToB, propensity, StakesLevel::Low);
        let intimate_decision =
            intimate.compute_trust_decision(Direction::AToB, propensity, StakesLevel::Low);

        // In intimate relationships, propensity matters less (lower weight)
        // but trustworthiness matters more. With same values, the difference
        // comes from the weighting.

        // Stranger: 0.6 * 0.8 + 0.4 * 0.3 = 0.48 + 0.12 = 0.60 (minus risk)
        // Intimate: 0.1 * 0.8 + 0.9 * 0.3 = 0.08 + 0.27 = 0.35 (minus risk)

        // Due to stage risk modifier, intimate has lower risk
        // Overall, the trust decision should differ
        assert!(stranger_decision.task_willingness() != intimate_decision.task_willingness());
    }

    #[test]
    fn perceived_risk_affects_willingness() {
        let low_risk = Relationship::try_between(alice(), bob()).unwrap();
        let mut high_risk = Relationship::try_between(alice(), bob()).unwrap();

        high_risk.perceived_risk_mut(Direction::AToB).add_delta(0.5);

        let low_decision =
            low_risk.compute_trust_decision(Direction::AToB, 0.5, StakesLevel::Medium);
        let high_decision =
            high_risk.compute_trust_decision(Direction::AToB, 0.5, StakesLevel::Medium);

        assert!(low_decision.task_willingness() > high_decision.task_willingness());
    }

    #[test]
    fn trust_competence_update() {
        let mut rel = Relationship::try_between(alice(), bob()).unwrap();
        rel.trustworthiness_mut(Direction::AToB)
            .add_competence_delta(0.3);

        let competence = rel.trustworthiness(Direction::AToB).competence_effective();
        assert!((competence - 0.6).abs() < f32::EPSILON); // base 0.3 + delta 0.3
    }

    #[test]
    fn trust_benevolence_update() {
        let mut rel = Relationship::try_between(alice(), bob()).unwrap();
        rel.trustworthiness_mut(Direction::AToB)
            .add_benevolence_delta(0.2);

        let benevolence = rel.trustworthiness(Direction::AToB).benevolence_effective();
        assert!((benevolence - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn trust_integrity_update() {
        let mut rel = Relationship::try_between(alice(), bob()).unwrap();
        rel.trustworthiness_mut(Direction::AToB)
            .add_integrity_delta(0.4);

        let integrity = rel.trustworthiness(Direction::AToB).integrity_effective();
        assert!((integrity - 0.7).abs() < f32::EPSILON);
    }

    #[test]
    fn trust_decision_integrates_risk() {
        let rel = Relationship::try_between(alice(), bob()).unwrap();

        let low_stakes = rel.compute_trust_decision(Direction::AToB, 0.5, StakesLevel::Low);
        let high_stakes = rel.compute_trust_decision(Direction::AToB, 0.5, StakesLevel::High);

        assert!(low_stakes.task_willingness() > high_stakes.task_willingness());
    }

    #[test]
    fn apply_decay() {
        let mut rel = Relationship::try_between(alice(), bob()).unwrap();
        rel.shared_mut().add_affinity_delta(0.4);
        rel.trustworthiness_mut(Direction::AToB)
            .add_competence_delta(0.4);

        rel.apply_decay(Duration::days(14));

        // Affinity should have decayed (14-day half-life)
        assert!((rel.shared().affinity().delta() - 0.2).abs() < 0.01);
    }

    #[test]
    fn shared_accessor() {
        let rel = Relationship::try_between(alice(), bob()).unwrap();
        assert!(rel.shared().affinity_effective() > 0.0);
    }

    #[test]
    fn shared_mut_accessor() {
        let mut rel = Relationship::try_between(alice(), bob()).unwrap();
        rel.shared_mut().add_affinity_delta(0.2);
        assert!((rel.shared().affinity().delta() - 0.2).abs() < f32::EPSILON);
    }

    #[test]
    fn relationship_error_display() {
        let err = RelationshipError::SelfRelationship;
        let display = format!("{}", err);
        assert!(display.contains("self"));

        let err2 = RelationshipError::InvalidStageTransition {
            from: RelationshipStage::Stranger,
            to: RelationshipStage::Intimate,
        };
        let display2 = format!("{}", err2);
        assert!(display2.contains("Stranger"));
        assert!(display2.contains("Intimate"));
    }

    #[test]
    fn relationship_error_equality() {
        let err1 = RelationshipError::SelfRelationship;
        let err2 = RelationshipError::SelfRelationship;
        assert_eq!(err1, err2);

        let err3 = RelationshipError::InvalidStageTransition {
            from: RelationshipStage::Stranger,
            to: RelationshipStage::Intimate,
        };
        let err4 = RelationshipError::InvalidStageTransition {
            from: RelationshipStage::Stranger,
            to: RelationshipStage::Intimate,
        };
        assert_eq!(err3, err4);
    }

    #[test]
    fn relationship_error_inequality() {
        let err1 = RelationshipError::SelfRelationship;
        let err2 = RelationshipError::InvalidStageTransition {
            from: RelationshipStage::Stranger,
            to: RelationshipStage::Intimate,
        };
        assert_ne!(err1, err2);
        // Also test the reverse direction to cover both orderings in the match
        assert_ne!(err2, err1);
    }

    #[test]
    fn relationship_error_inequality_different_fields() {
        let err1 = RelationshipError::InvalidStageTransition {
            from: RelationshipStage::Stranger,
            to: RelationshipStage::Intimate,
        };
        let err2 = RelationshipError::InvalidStageTransition {
            from: RelationshipStage::Acquaintance,
            to: RelationshipStage::Intimate,
        };
        assert_ne!(err1, err2);

        let err3 = RelationshipError::InvalidStageTransition {
            from: RelationshipStage::Stranger,
            to: RelationshipStage::Established,
        };
        assert_ne!(err1, err3);
    }

    #[test]
    fn clone_and_equality() {
        let rel1 = Relationship::try_between(alice(), bob()).unwrap();
        let rel2 = rel1.clone();
        assert_eq!(rel1, rel2);
    }

    #[test]
    fn inequality_different_entities() {
        let rel1 = Relationship::try_between(alice(), bob()).unwrap();
        let carol = EntityId::new("carol").unwrap();
        let rel2 = Relationship::try_between(alice(), carol).unwrap();
        assert_ne!(rel1, rel2);
    }

    #[test]
    fn inequality_modified_shared_dimension() {
        let rel1 = Relationship::try_between(alice(), bob()).unwrap();
        let mut rel2 = rel1.clone();
        rel2.shared_mut().add_affinity_delta(0.5);
        assert_ne!(rel1, rel2);
    }

    #[test]
    fn inequality_different_bonds() {
        let rel1 = Relationship::try_between(alice(), bob()).unwrap();
        let mut rel2 = rel1.clone();
        rel2.add_bond(BondType::Friend);
        assert_ne!(rel1, rel2);
    }

    #[test]
    fn inequality_different_schema() {
        let rel1 = Relationship::try_between(alice(), bob()).unwrap();
        let mut rel2 = rel1.clone();
        rel2.set_schema(RelationshipSchema::Mentor);
        assert_ne!(rel1, rel2);
    }

    #[test]
    fn inequality_different_stage() {
        let rel1 = Relationship::try_between(alice(), bob()).unwrap();
        let mut rel2 = rel1.clone();
        rel2.set_stage(RelationshipStage::Established).unwrap();
        assert_ne!(rel1, rel2);
    }

    #[test]
    fn inequality_different_trustworthiness() {
        let rel1 = Relationship::try_between(alice(), bob()).unwrap();
        let mut rel2 = rel1.clone();
        rel2.trustworthiness_mut(Direction::AToB)
            .add_competence_delta(0.5);
        assert_ne!(rel1, rel2);
    }

    #[test]
    fn inequality_different_perceived_risk() {
        let rel1 = Relationship::try_between(alice(), bob()).unwrap();
        let mut rel2 = rel1.clone();
        rel2.perceived_risk_mut(Direction::BToA).add_delta(0.5);
        assert_ne!(rel1, rel2);
    }

    #[test]
    fn inequality_different_directional() {
        let rel1 = Relationship::try_between(alice(), bob()).unwrap();
        let mut rel2 = rel1.clone();
        rel2.directional_mut(Direction::AToB).add_warmth_delta(0.5);
        assert_ne!(rel1, rel2);
    }

    #[test]
    fn inequality_different_trustworthiness_b_to_a() {
        let rel1 = Relationship::try_between(alice(), bob()).unwrap();
        let mut rel2 = rel1.clone();
        rel2.trustworthiness_mut(Direction::BToA)
            .add_benevolence_delta(0.5);
        assert_ne!(rel1, rel2);
    }

    #[test]
    fn inequality_different_perceived_risk_a_to_b() {
        let rel1 = Relationship::try_between(alice(), bob()).unwrap();
        let mut rel2 = rel1.clone();
        rel2.perceived_risk_mut(Direction::AToB).add_delta(0.5);
        assert_ne!(rel1, rel2);
    }

    #[test]
    fn inequality_different_directional_b_to_a() {
        let rel1 = Relationship::try_between(alice(), bob()).unwrap();
        let mut rel2 = rel1.clone();
        rel2.directional_mut(Direction::BToA)
            .add_resentment_delta(0.5);
        assert_ne!(rel1, rel2);
    }

    #[test]
    fn debug_format() {
        let rel = Relationship::try_between(alice(), bob()).unwrap();
        let debug = format!("{:?}", rel);
        assert!(debug.contains("Relationship"));
    }

    #[test]
    fn default_stage_is_stranger() {
        let rel = Relationship::try_between(alice(), bob()).unwrap();
        assert_eq!(rel.stage(), RelationshipStage::Stranger);
    }

    #[test]
    fn default_schema_is_peer() {
        let rel = Relationship::try_between(alice(), bob()).unwrap();
        assert_eq!(rel.schema(), RelationshipSchema::Peer);
    }

    #[test]
    fn get_all_directional_paths() {
        let rel = Relationship::try_between(alice(), bob()).unwrap();

        // Test all directional paths return Some
        assert!(rel
            .get(RelPath::Directional(
                Direction::AToB,
                DirectionalPath::Warmth
            ))
            .is_some());
        assert!(rel
            .get(RelPath::Directional(
                Direction::AToB,
                DirectionalPath::Resentment
            ))
            .is_some());
        assert!(rel
            .get(RelPath::Directional(
                Direction::AToB,
                DirectionalPath::Dependence
            ))
            .is_some());
        assert!(rel
            .get(RelPath::Directional(
                Direction::AToB,
                DirectionalPath::Attraction
            ))
            .is_some());
        assert!(rel
            .get(RelPath::Directional(
                Direction::AToB,
                DirectionalPath::Attachment
            ))
            .is_some());
        assert!(rel
            .get(RelPath::Directional(
                Direction::AToB,
                DirectionalPath::Jealousy
            ))
            .is_some());
        assert!(rel
            .get(RelPath::Directional(Direction::AToB, DirectionalPath::Fear))
            .is_some());
        assert!(rel
            .get(RelPath::Directional(
                Direction::AToB,
                DirectionalPath::Obligation
            ))
            .is_some());
        assert!(rel
            .get(RelPath::Directional(
                Direction::AToB,
                DirectionalPath::PerceivedRisk
            ))
            .is_some());
    }

    #[test]
    fn get_all_shared_paths() {
        let rel = Relationship::try_between(alice(), bob()).unwrap();

        assert!(rel.get(RelPath::Shared(SharedPath::Affinity)).is_some());
        assert!(rel.get(RelPath::Shared(SharedPath::Respect)).is_some());
        assert!(rel.get(RelPath::Shared(SharedPath::Tension)).is_some());
        assert!(rel.get(RelPath::Shared(SharedPath::Intimacy)).is_some());
        assert!(rel.get(RelPath::Shared(SharedPath::History)).is_some());
    }

    #[test]
    fn get_all_trust_paths() {
        let rel = Relationship::try_between(alice(), bob()).unwrap();

        assert!(rel
            .get(RelPath::Directional(
                Direction::AToB,
                DirectionalPath::Trust(TrustPath::Competence)
            ))
            .is_some());
        assert!(rel
            .get(RelPath::Directional(
                Direction::AToB,
                DirectionalPath::Trust(TrustPath::Benevolence)
            ))
            .is_some());
        assert!(rel
            .get(RelPath::Directional(
                Direction::AToB,
                DirectionalPath::Trust(TrustPath::Integrity)
            ))
            .is_some());
    }

    #[test]
    fn get_support_willingness_returns_none() {
        // SupportWillingness is a computed value from TrustDecision,
        // not a stored StateValue
        let rel = Relationship::try_between(alice(), bob()).unwrap();
        assert!(rel
            .get(RelPath::Directional(
                Direction::AToB,
                DirectionalPath::Trust(TrustPath::SupportWillingness)
            ))
            .is_none());
    }

    #[test]
    fn get_mut_support_willingness_returns_none() {
        let mut rel = Relationship::try_between(alice(), bob()).unwrap();
        assert!(rel
            .get_mut(RelPath::Directional(
                Direction::AToB,
                DirectionalPath::Trust(TrustPath::SupportWillingness)
            ))
            .is_none());
    }

    #[test]
    fn stage_transition_error_new() {
        let err =
            StageTransitionError::new(RelationshipStage::Stranger, RelationshipStage::Intimate);
        assert_eq!(err.from, RelationshipStage::Stranger);
        assert_eq!(err.to, RelationshipStage::Intimate);
    }

    #[test]
    fn stage_transition_error_display() {
        let err =
            StageTransitionError::new(RelationshipStage::Stranger, RelationshipStage::Intimate);
        let display = format!("{}", err);
        assert!(display.contains("Stranger"));
        assert!(display.contains("Intimate"));
        assert!(display.contains("Invalid stage transition"));
    }

    #[test]
    fn stage_transition_error_debug() {
        let err =
            StageTransitionError::new(RelationshipStage::Stranger, RelationshipStage::Intimate);
        let debug = format!("{:?}", err);
        assert!(debug.contains("StageTransitionError"));
    }

    #[test]
    fn stage_transition_error_clone_and_equality() {
        let err1 = StageTransitionError::new(
            RelationshipStage::Acquaintance,
            RelationshipStage::Estranged,
        );
        let err2 = err1.clone();
        assert_eq!(err1, err2);
    }

    #[test]
    fn stage_transition_error_implements_error_trait() {
        let err =
            StageTransitionError::new(RelationshipStage::Stranger, RelationshipStage::Intimate);
        let error: &dyn std::error::Error = &err;
        assert!(error.to_string().contains("Invalid"));
    }

    #[test]
    fn compute_trust_decision_estranged_stage() {
        let mut rel = Relationship::try_between(alice(), bob()).unwrap();
        rel.set_stage(RelationshipStage::Estranged).unwrap();

        let decision = rel.compute_trust_decision(Direction::AToB, 0.5, StakesLevel::Medium);

        // Estranged relationships have 0.5 stage confidence
        // history = 0, so confidence = 0.0 * 0.3 + 0.5 * 0.7 = 0.35
        assert!(decision.confidence() > 0.0);
        assert!(decision.confidence() < 1.0);
        assert!(decision.task_willingness() >= 0.0);
        assert!(decision.task_willingness() <= 1.0);
    }

    #[test]
    fn compute_trust_decision_acquaintance_stage() {
        let mut rel = Relationship::try_between(alice(), bob()).unwrap();
        rel.set_stage(RelationshipStage::Acquaintance).unwrap();

        let decision = rel.compute_trust_decision(Direction::AToB, 0.5, StakesLevel::Medium);

        // Acquaintance stage has 0.3 stage confidence
        assert!(decision.confidence() > 0.0);
        assert!(decision.confidence() < 1.0);
        assert!(decision.task_willingness() >= 0.0);
        assert!(decision.task_willingness() <= 1.0);
    }

    #[test]
    fn compute_trust_decision_established_stage() {
        let mut rel = Relationship::try_between(alice(), bob()).unwrap();
        rel.set_stage(RelationshipStage::Established).unwrap();

        let decision = rel.compute_trust_decision(Direction::AToB, 0.5, StakesLevel::Medium);

        // Established stage has 0.6 stage confidence
        assert!(decision.confidence() > 0.0);
        assert!(decision.confidence() < 1.0);
        assert!(decision.task_willingness() >= 0.0);
        assert!(decision.task_willingness() <= 1.0);
    }
}
