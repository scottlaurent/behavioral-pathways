//! Macrosystem implementation for cultural and societal patterns.
//!
//! The macrosystem represents overarching cultural, societal, and ideological
//! patterns that influence all other ecological layers. These include:
//!
//! - Cultural values (individualism/collectivism, power distance)
//! - Institutional structures (rule of law, social mobility)
//! - Society-wide stressors (cultural stress, collective trauma)

use crate::enums::{MacrosystemPath, RelationshipSchema};
use std::collections::HashMap;

/// Cultural orientation dimensions (Hofstede-inspired).
#[derive(Debug, Clone, PartialEq)]
pub struct CulturalOrientation {
    /// Individualism vs collectivism (-1 collectivist to +1 individualist).
    pub individualism_collectivism: f64,

    /// Acceptance of hierarchy (0-1).
    pub power_distance: f64,

    /// Discomfort with ambiguity (0-1).
    pub uncertainty_avoidance: f64,
}

impl Default for CulturalOrientation {
    fn default() -> Self {
        CulturalOrientation {
            individualism_collectivism: 0.0, // Neutral
            power_distance: 0.5,
            uncertainty_avoidance: 0.5,
        }
    }
}

/// Institutional structure dimensions.
#[derive(Debug, Clone, PartialEq)]
pub struct InstitutionalStructure {
    /// Legal system reliability (0-1).
    pub rule_of_law: f64,

    /// Ability to change social status (0-1).
    pub social_mobility: f64,

    /// Institutional corruption level (0-1).
    pub corruption_level: f64,
}

impl Default for InstitutionalStructure {
    fn default() -> Self {
        InstitutionalStructure {
            rule_of_law: 0.6,
            social_mobility: 0.5,
            corruption_level: 0.3,
        }
    }
}

/// Cultural constraints derived from macrosystem context.
#[derive(Debug, Clone, PartialEq)]
pub struct MacrosystemConstraintSet {
    /// Relationship schemas permitted in this culture.
    pub allowed_relationship_schemas: Vec<RelationshipSchema>,

    /// Penalty applied when hierarchical norms are violated (0-1).
    pub hierarchy_violation_penalty: f64,
}

impl MacrosystemConstraintSet {
    /// Returns true if the schema is culturally permitted.
    #[must_use]
    pub fn allows_schema(&self, schema: RelationshipSchema) -> bool {
        self.allowed_relationship_schemas.contains(&schema)
    }
}

impl Default for MacrosystemConstraintSet {
    fn default() -> Self {
        MacrosystemConstraintSet {
            allowed_relationship_schemas: RelationshipSchema::all().to_vec(),
            hierarchy_violation_penalty: 0.0,
        }
    }
}

/// Overrides applied for subcultures or distinct entity groups.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MacrosystemModifier {
    /// Individualism vs collectivism (-1 to 1).
    pub individualism_collectivism: Option<f64>,

    /// Acceptance of hierarchy (0-1).
    pub power_distance: Option<f64>,

    /// Discomfort with ambiguity (0-1).
    pub uncertainty_avoidance: Option<f64>,

    /// Legal system reliability (0-1).
    pub rule_of_law: Option<f64>,

    /// Ability to change social status (0-1).
    pub social_mobility: Option<f64>,

    /// Institutional corruption level (0-1).
    pub corruption_level: Option<f64>,

    /// Society-level stress (0-1).
    pub cultural_stress: Option<f64>,

    /// Shared historical trauma (0-1).
    pub collective_trauma: Option<f64>,

    /// Wealth disparity level (0-1).
    pub economic_inequality: Option<f64>,
}

impl MacrosystemModifier {
    /// Applies this modifier to a base macrosystem context.
    #[must_use]
    pub fn apply_to(&self, base: &MacrosystemContext) -> MacrosystemContext {
        let mut modified = base.clone();

        if let Some(value) = self.individualism_collectivism {
            modified.cultural_orientation.individualism_collectivism = value.clamp(-1.0, 1.0);
        }
        if let Some(value) = self.power_distance {
            modified.cultural_orientation.power_distance = value.clamp(0.0, 1.0);
        }
        if let Some(value) = self.uncertainty_avoidance {
            modified.cultural_orientation.uncertainty_avoidance = value.clamp(0.0, 1.0);
        }
        if let Some(value) = self.rule_of_law {
            modified.institutional_structure.rule_of_law = value.clamp(0.0, 1.0);
        }
        if let Some(value) = self.social_mobility {
            modified.institutional_structure.social_mobility = value.clamp(0.0, 1.0);
        }
        if let Some(value) = self.corruption_level {
            modified.institutional_structure.corruption_level = value.clamp(0.0, 1.0);
        }
        if let Some(value) = self.cultural_stress {
            modified.cultural_stress = value.clamp(0.0, 1.0);
        }
        if let Some(value) = self.collective_trauma {
            modified.collective_trauma = value.clamp(0.0, 1.0);
        }
        if let Some(value) = self.economic_inequality {
            modified.economic_inequality = value.clamp(0.0, 1.0);
        }

        modified
    }
}

/// Macrosystem context for cultural and societal patterns.
///
/// These are overarching patterns that influence all other ecological layers.
/// Macrosystem values typically remain stable over long periods but can shift
/// due to major societal events.
#[derive(Debug, Clone, PartialEq)]
pub struct MacrosystemContext {
    /// Cultural orientation (Hofstede dimensions).
    pub cultural_orientation: CulturalOrientation,

    /// Institutional structures.
    pub institutional_structure: InstitutionalStructure,

    /// Society-level stress (0-1).
    pub cultural_stress: f64,

    /// Shared historical trauma (0-1).
    pub collective_trauma: f64,

    /// Wealth disparity level (0-1).
    pub economic_inequality: f64,

    /// Subculture-specific overrides keyed by group label.
    pub subculture_overrides: HashMap<String, MacrosystemModifier>,
}

impl Default for MacrosystemContext {
    fn default() -> Self {
        MacrosystemContext {
            cultural_orientation: CulturalOrientation::default(),
            institutional_structure: InstitutionalStructure::default(),
            cultural_stress: 0.2,
            collective_trauma: 0.1,
            economic_inequality: 0.4,
            subculture_overrides: HashMap::new(),
        }
    }
}

impl MacrosystemContext {
    /// Creates a new MacrosystemContext with default values.
    #[must_use]
    pub fn new() -> Self {
        MacrosystemContext::default()
    }

    /// Returns the macrosystem context adjusted for a subculture key.
    #[must_use]
    pub fn for_subculture(&self, key: &str) -> MacrosystemContext {
        self.subculture_overrides
            .get(key)
            .map(|modifier| modifier.apply_to(self))
            .unwrap_or_else(|| self.clone())
    }

    /// Gets a value by macrosystem path.
    #[must_use]
    pub fn get_value(&self, path: &MacrosystemPath) -> f64 {
        match path {
            MacrosystemPath::IndividualismCollectivism => {
                self.cultural_orientation.individualism_collectivism
            }
            MacrosystemPath::PowerDistance => self.cultural_orientation.power_distance,
            MacrosystemPath::UncertaintyAvoidance => {
                self.cultural_orientation.uncertainty_avoidance
            }
            MacrosystemPath::CulturalStress => self.cultural_stress,
            MacrosystemPath::CollectiveTrauma => self.collective_trauma,
            MacrosystemPath::EconomicInequality => self.economic_inequality,
            MacrosystemPath::RuleOfLaw => self.institutional_structure.rule_of_law,
            MacrosystemPath::SocialMobility => self.institutional_structure.social_mobility,
            MacrosystemPath::CorruptionLevel => self.institutional_structure.corruption_level,
        }
    }

    /// Sets a value by macrosystem path.
    pub fn set_value(&mut self, path: &MacrosystemPath, value: f64) {
        // Most values are clamped 0-1, except individualism_collectivism which is -1 to 1
        match path {
            MacrosystemPath::IndividualismCollectivism => {
                self.cultural_orientation.individualism_collectivism = value.clamp(-1.0, 1.0);
            }
            MacrosystemPath::PowerDistance => {
                self.cultural_orientation.power_distance = value.clamp(0.0, 1.0);
            }
            MacrosystemPath::UncertaintyAvoidance => {
                self.cultural_orientation.uncertainty_avoidance = value.clamp(0.0, 1.0);
            }
            MacrosystemPath::CulturalStress => {
                self.cultural_stress = value.clamp(0.0, 1.0);
            }
            MacrosystemPath::CollectiveTrauma => {
                self.collective_trauma = value.clamp(0.0, 1.0);
            }
            MacrosystemPath::EconomicInequality => {
                self.economic_inequality = value.clamp(0.0, 1.0);
            }
            MacrosystemPath::RuleOfLaw => {
                self.institutional_structure.rule_of_law = value.clamp(0.0, 1.0);
            }
            MacrosystemPath::SocialMobility => {
                self.institutional_structure.social_mobility = value.clamp(0.0, 1.0);
            }
            MacrosystemPath::CorruptionLevel => {
                self.institutional_structure.corruption_level = value.clamp(0.0, 1.0);
            }
        }
    }

    /// Returns whether this is an individualist culture.
    ///
    /// Per spec: individualism > 0.3 is considered individualist.
    #[must_use]
    pub fn is_individualist(&self) -> bool {
        self.cultural_orientation.individualism_collectivism > 0.3
    }

    /// Returns whether this is a collectivist culture.
    ///
    /// Per spec: individualism < -0.3 is considered collectivist.
    #[must_use]
    pub fn is_collectivist(&self) -> bool {
        self.cultural_orientation.individualism_collectivism < -0.3
    }

    /// Returns whether this is a high power distance culture.
    ///
    /// Per spec: power_distance > 0.6 is considered high.
    #[must_use]
    pub fn is_high_power_distance(&self) -> bool {
        self.cultural_orientation.power_distance > 0.6
    }

    /// Computes cultural constraints on allowable relationship schemas.
    #[must_use]
    pub fn constraint_set(&self) -> MacrosystemConstraintSet {
        let power_distance = self.cultural_orientation.power_distance;
        if power_distance > 0.6 {
            MacrosystemConstraintSet {
                allowed_relationship_schemas: vec![
                    RelationshipSchema::Mentor,
                    RelationshipSchema::Subordinate,
                    RelationshipSchema::Family,
                    RelationshipSchema::Nuclear,
                    RelationshipSchema::Extended,
                ],
                hierarchy_violation_penalty: power_distance * 0.2,
            }
        } else {
            MacrosystemConstraintSet {
                allowed_relationship_schemas: vec![
                    RelationshipSchema::Peer,
                    RelationshipSchema::Mentor,
                    RelationshipSchema::Romantic,
                    RelationshipSchema::Family,
                    RelationshipSchema::Nuclear,
                    RelationshipSchema::Extended,
                ],
                hierarchy_violation_penalty: 0.0,
            }
        }
    }

    /// Computes autonomy need weight modifier based on cultural orientation.
    ///
    /// Per spec: In individualist cultures, autonomy needs are weighted higher.
    #[must_use]
    pub fn autonomy_need_weight(&self) -> f64 {
        let ic = self.cultural_orientation.individualism_collectivism;
        if ic > 0.3 {
            1.0 + ic * 0.3
        } else {
            1.0 + ic * 0.2
        }
    }

    /// Computes belonging need weight modifier based on cultural orientation.
    ///
    /// Per spec: In collectivist cultures, belonging needs are weighted higher.
    #[must_use]
    pub fn belonging_need_weight(&self) -> f64 {
        let ic = self.cultural_orientation.individualism_collectivism;
        if ic > 0.3 {
            1.0 - ic * 0.2
        } else {
            1.0 - ic * 0.3
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- CulturalOrientation tests ---

    #[test]
    fn cultural_orientation_default() {
        let co = CulturalOrientation::default();
        assert!((co.individualism_collectivism - 0.0).abs() < f64::EPSILON);
        assert!((co.power_distance - 0.5).abs() < f64::EPSILON);
        assert!((co.uncertainty_avoidance - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn cultural_orientation_clone_eq() {
        let co1 = CulturalOrientation::default();
        let co2 = co1.clone();
        assert_eq!(co1, co2);
    }

    // --- InstitutionalStructure tests ---

    #[test]
    fn institutional_structure_default() {
        let is = InstitutionalStructure::default();
        assert!((is.rule_of_law - 0.6).abs() < f64::EPSILON);
        assert!((is.social_mobility - 0.5).abs() < f64::EPSILON);
        assert!((is.corruption_level - 0.3).abs() < f64::EPSILON);
    }

    #[test]
    fn institutional_structure_clone_eq() {
        let is1 = InstitutionalStructure::default();
        let is2 = is1.clone();
        assert_eq!(is1, is2);
    }

    // --- MacrosystemContext tests ---

    #[test]
    fn macrosystem_context_default() {
        let macro_ctx = MacrosystemContext::default();
        assert!((macro_ctx.cultural_stress - 0.2).abs() < f64::EPSILON);
        assert!((macro_ctx.collective_trauma - 0.1).abs() < f64::EPSILON);
        assert!((macro_ctx.economic_inequality - 0.4).abs() < f64::EPSILON);
    }

    #[test]
    fn macrosystem_context_new() {
        let macro_ctx = MacrosystemContext::new();
        assert!((macro_ctx.cultural_stress - 0.2).abs() < f64::EPSILON);
    }

    #[test]
    fn macrosystem_subculture_override_applies() {
        let mut macro_ctx = MacrosystemContext::default();
        let mut modifier = MacrosystemModifier::default();
        modifier.power_distance = Some(0.9);
        modifier.cultural_stress = Some(0.75);

        macro_ctx
            .subculture_overrides
            .insert("subculture_a".to_string(), modifier);

        let adjusted = macro_ctx.for_subculture("subculture_a");
        assert!(
            (adjusted.cultural_orientation.power_distance - 0.9).abs() < f64::EPSILON
        );
        assert!((adjusted.cultural_stress - 0.75).abs() < f64::EPSILON);

        let baseline = macro_ctx.for_subculture("missing");
        assert_eq!(baseline, macro_ctx);
    }

    #[test]
    fn macrosystem_get_value_all_paths() {
        let macro_ctx = MacrosystemContext::default();
        for path in MacrosystemPath::all() {
            let value = macro_ctx.get_value(&path);
            // Most values are 0-1, except individualism_collectivism which is -1 to 1
            if matches!(path, MacrosystemPath::IndividualismCollectivism) {
                assert!(value >= -1.0 && value <= 1.0);
            } else {
                assert!(value >= 0.0 && value <= 1.0);
            }
        }
    }

    #[test]
    fn macrosystem_set_value() {
        let mut macro_ctx = MacrosystemContext::default();
        macro_ctx.set_value(&MacrosystemPath::CulturalStress, 0.8);
        assert!((macro_ctx.cultural_stress - 0.8).abs() < f64::EPSILON);
    }

    #[test]
    fn macrosystem_set_value_clamped() {
        let mut macro_ctx = MacrosystemContext::default();

        // Test 0-1 clamping
        macro_ctx.set_value(&MacrosystemPath::CulturalStress, 1.5);
        assert!((macro_ctx.cultural_stress - 1.0).abs() < f64::EPSILON);

        macro_ctx.set_value(&MacrosystemPath::CulturalStress, -0.5);
        assert!((macro_ctx.cultural_stress - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn macrosystem_set_individualism_clamped() {
        let mut macro_ctx = MacrosystemContext::default();

        // Test -1 to 1 clamping
        macro_ctx.set_value(&MacrosystemPath::IndividualismCollectivism, 1.5);
        assert!(
            (macro_ctx.cultural_orientation.individualism_collectivism - 1.0).abs() < f64::EPSILON
        );

        macro_ctx.set_value(&MacrosystemPath::IndividualismCollectivism, -1.5);
        assert!(
            (macro_ctx.cultural_orientation.individualism_collectivism - (-1.0)).abs()
                < f64::EPSILON
        );
    }

    #[test]
    fn macrosystem_set_all_paths() {
        let mut macro_ctx = MacrosystemContext::default();

        for path in MacrosystemPath::all() {
            macro_ctx.set_value(&path, 0.75);
        }

        // Verify all values changed (except individualism_collectivism which clamps to 0.75)
        assert!((macro_ctx.cultural_stress - 0.75).abs() < f64::EPSILON);
        assert!((macro_ctx.collective_trauma - 0.75).abs() < f64::EPSILON);
        assert!(
            (macro_ctx.cultural_orientation.individualism_collectivism - 0.75).abs() < f64::EPSILON
        );
        assert!((macro_ctx.cultural_orientation.power_distance - 0.75).abs() < f64::EPSILON);
    }

    #[test]
    fn macrosystem_is_individualist() {
        let mut macro_ctx = MacrosystemContext::default();

        // Default is neutral (0.0)
        assert!(!macro_ctx.is_individualist());

        // Set to individualist
        macro_ctx.cultural_orientation.individualism_collectivism = 0.5;
        assert!(macro_ctx.is_individualist());

        // Set to collectivist
        macro_ctx.cultural_orientation.individualism_collectivism = -0.5;
        assert!(!macro_ctx.is_individualist());
    }

    #[test]
    fn macrosystem_is_collectivist() {
        let mut macro_ctx = MacrosystemContext::default();

        // Default is neutral (0.0)
        assert!(!macro_ctx.is_collectivist());

        // Set to collectivist
        macro_ctx.cultural_orientation.individualism_collectivism = -0.5;
        assert!(macro_ctx.is_collectivist());

        // Set to individualist
        macro_ctx.cultural_orientation.individualism_collectivism = 0.5;
        assert!(!macro_ctx.is_collectivist());
    }

    #[test]
    fn macrosystem_is_high_power_distance() {
        let mut macro_ctx = MacrosystemContext::default();

        // Default power distance is 0.5
        assert!(!macro_ctx.is_high_power_distance());

        // Set to high power distance
        macro_ctx.cultural_orientation.power_distance = 0.8;
        assert!(macro_ctx.is_high_power_distance());
    }

    #[test]
    fn macrosystem_autonomy_need_weight_individualist() {
        let mut macro_ctx = MacrosystemContext::default();
        macro_ctx.cultural_orientation.individualism_collectivism = 0.7;

        let weight = macro_ctx.autonomy_need_weight();
        // 1.0 + 0.7 * 0.3 = 1.21
        assert!(weight > 1.2);
    }

    #[test]
    fn macrosystem_autonomy_need_weight_collectivist() {
        let mut macro_ctx = MacrosystemContext::default();
        macro_ctx.cultural_orientation.individualism_collectivism = -0.7;

        let weight = macro_ctx.autonomy_need_weight();
        // 1.0 + (-0.7) * 0.2 = 0.86
        assert!(weight < 0.9);
    }

    #[test]
    fn macrosystem_belonging_need_weight_individualist() {
        let mut macro_ctx = MacrosystemContext::default();
        macro_ctx.cultural_orientation.individualism_collectivism = 0.7;

        let weight = macro_ctx.belonging_need_weight();
        // 1.0 - 0.7 * 0.2 = 0.86
        assert!(weight < 0.9);
    }

    #[test]
    fn macrosystem_belonging_need_weight_collectivist() {
        let mut macro_ctx = MacrosystemContext::default();
        macro_ctx.cultural_orientation.individualism_collectivism = -0.7;

        let weight = macro_ctx.belonging_need_weight();
        // 1.0 - (-0.7) * 0.3 = 1.21
        assert!(weight > 1.2);
    }

    #[test]
    fn macrosystem_clone_eq() {
        let macro1 = MacrosystemContext::default();
        let macro2 = macro1.clone();
        assert_eq!(macro1, macro2);
    }

    #[test]
    fn macrosystem_debug() {
        let macro_ctx = MacrosystemContext::default();
        let debug = format!("{:?}", macro_ctx);
        assert!(debug.contains("MacrosystemContext"));
    }

    #[test]
    fn macrosystem_path_get_cultural_orientation() {
        let macro_ctx = MacrosystemContext::default();

        assert!(
            (macro_ctx.get_value(&MacrosystemPath::IndividualismCollectivism) - 0.0).abs()
                < f64::EPSILON
        );
        assert!((macro_ctx.get_value(&MacrosystemPath::PowerDistance) - 0.5).abs() < f64::EPSILON);
        assert!(
            (macro_ctx.get_value(&MacrosystemPath::UncertaintyAvoidance) - 0.5).abs()
                < f64::EPSILON
        );
    }

    #[test]
    fn macrosystem_path_get_institutional() {
        let macro_ctx = MacrosystemContext::default();

        assert!((macro_ctx.get_value(&MacrosystemPath::RuleOfLaw) - 0.6).abs() < f64::EPSILON);
        assert!((macro_ctx.get_value(&MacrosystemPath::SocialMobility) - 0.5).abs() < f64::EPSILON);
        assert!(
            (macro_ctx.get_value(&MacrosystemPath::CorruptionLevel) - 0.3).abs() < f64::EPSILON
        );
    }

    #[test]
    fn macrosystem_path_get_societal() {
        let macro_ctx = MacrosystemContext::default();

        assert!((macro_ctx.get_value(&MacrosystemPath::CulturalStress) - 0.2).abs() < f64::EPSILON);
        assert!(
            (macro_ctx.get_value(&MacrosystemPath::CollectiveTrauma) - 0.1).abs() < f64::EPSILON
        );
        assert!(
            (macro_ctx.get_value(&MacrosystemPath::EconomicInequality) - 0.4).abs() < f64::EPSILON
        );
    }

    // --- Required Phase 7 tests ---

    #[test]
    fn macrosystem_creation_default() {
        // MacrosystemContext creates with cultural defaults
        let macro_ctx = MacrosystemContext::default();

        // Verify cultural orientation defaults
        assert!(
            (macro_ctx.cultural_orientation.individualism_collectivism - 0.0).abs() < f64::EPSILON
        );
        assert!((macro_ctx.cultural_orientation.power_distance - 0.5).abs() < f64::EPSILON);
        assert!((macro_ctx.cultural_orientation.uncertainty_avoidance - 0.5).abs() < f64::EPSILON);

        // Verify institutional structure defaults
        assert!((macro_ctx.institutional_structure.rule_of_law - 0.6).abs() < f64::EPSILON);
        assert!((macro_ctx.institutional_structure.social_mobility - 0.5).abs() < f64::EPSILON);
        assert!((macro_ctx.institutional_structure.corruption_level - 0.3).abs() < f64::EPSILON);

        // Verify societal defaults
        assert!((macro_ctx.cultural_stress - 0.2).abs() < f64::EPSILON);
        assert!((macro_ctx.collective_trauma - 0.1).abs() < f64::EPSILON);
        assert!((macro_ctx.economic_inequality - 0.4).abs() < f64::EPSILON);

        // new() should produce same defaults
        let macro_new = MacrosystemContext::new();
        assert_eq!(macro_ctx, macro_new);
    }

    #[test]
    fn macrosystem_cultural_constraint_application() {
        // Culture limits available roles and behaviors
        // Test that cultural orientation values constrain behavioral weights

        // Create high power distance culture
        let mut high_pd = MacrosystemContext::default();
        high_pd.cultural_orientation.power_distance = 0.9;

        // Create low power distance culture
        let mut low_pd = MacrosystemContext::default();
        low_pd.cultural_orientation.power_distance = 0.2;

        // Verify the values are set correctly
        assert!(high_pd.is_high_power_distance());
        assert!(!low_pd.is_high_power_distance());

        // In high power distance cultures, hierarchical compliance is expected
        // (Cultural constraint on available roles)
        assert!(high_pd.cultural_orientation.power_distance > 0.6);
        assert!(low_pd.cultural_orientation.power_distance < 0.6);

        // Create individualist culture
        let mut individualist = MacrosystemContext::default();
        individualist
            .cultural_orientation
            .individualism_collectivism = 0.8;

        // Create collectivist culture
        let mut collectivist = MacrosystemContext::default();
        collectivist.cultural_orientation.individualism_collectivism = -0.8;

        assert!(individualist.is_individualist());
        assert!(!individualist.is_collectivist());
        assert!(collectivist.is_collectivist());
        assert!(!collectivist.is_individualist());
    }

    #[test]
    fn macrosystem_constraint_set_respects_power_distance() {
        let mut high_pd = MacrosystemContext::default();
        high_pd.cultural_orientation.power_distance = 0.9;
        let high_constraints = high_pd.constraint_set();
        assert!(high_constraints.allows_schema(RelationshipSchema::Subordinate));
        assert!(!high_constraints.allows_schema(RelationshipSchema::Romantic));
        assert!(high_constraints.hierarchy_violation_penalty > 0.0);

        let mut low_pd = MacrosystemContext::default();
        low_pd.cultural_orientation.power_distance = 0.2;
        let low_constraints = low_pd.constraint_set();
        assert!(low_constraints.allows_schema(RelationshipSchema::Romantic));
        assert!(!low_constraints.allows_schema(RelationshipSchema::Subordinate));
        assert!((low_constraints.hierarchy_violation_penalty - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn macrosystem_power_distance_modifies_compliance() {
        // High power distance increases compliance with authority
        // (Simulated via the power_distance dimension)

        let mut high_pd = MacrosystemContext::default();
        high_pd.cultural_orientation.power_distance = 0.85;

        let mut low_pd = MacrosystemContext::default();
        low_pd.cultural_orientation.power_distance = 0.15;

        // Verify high power distance classification
        assert!(high_pd.is_high_power_distance());
        assert!(!low_pd.is_high_power_distance());

        // The power distance value directly represents compliance expectation
        // High PD culture: individuals expected to comply with authority
        let high_pd_value = high_pd.get_value(&MacrosystemPath::PowerDistance);
        let low_pd_value = low_pd.get_value(&MacrosystemPath::PowerDistance);

        assert!((high_pd_value - 0.85).abs() < f64::EPSILON);
        assert!((low_pd_value - 0.15).abs() < f64::EPSILON);

        // Difference should be significant
        assert!(high_pd_value - low_pd_value > 0.5);
    }

    #[test]
    fn macrosystem_collectivism_modifies_belonging_weight() {
        // Cultural orientation affects belonging need weights
        // Collectivist cultures weight belonging needs higher

        // Create individualist culture
        let mut individualist = MacrosystemContext::default();
        individualist
            .cultural_orientation
            .individualism_collectivism = 0.7;

        // Create collectivist culture
        let mut collectivist = MacrosystemContext::default();
        collectivist.cultural_orientation.individualism_collectivism = -0.7;

        // Calculate belonging need weights
        let individualist_belonging = individualist.belonging_need_weight();
        let collectivist_belonging = collectivist.belonging_need_weight();

        // Collectivist cultures should weight belonging higher
        assert!(collectivist_belonging > individualist_belonging);

        // Verify expected values:
        // Individualist (ic=0.7): 1.0 - 0.7 * 0.2 = 0.86
        // Collectivist (ic=-0.7): 1.0 - (-0.7) * 0.3 = 1.21
        assert!((individualist_belonging - 0.86).abs() < 0.01);
        assert!((collectivist_belonging - 1.21).abs() < 0.01);

        // Also verify autonomy weight is inversely affected
        let individualist_autonomy = individualist.autonomy_need_weight();
        let collectivist_autonomy = collectivist.autonomy_need_weight();

        // Individualist cultures should weight autonomy higher
        assert!(individualist_autonomy > collectivist_autonomy);

        // Verify expected values:
        // Individualist (ic=0.7): 1.0 + 0.7 * 0.3 = 1.21
        // Collectivist (ic=-0.7): 1.0 + (-0.7) * 0.2 = 0.86
        assert!((individualist_autonomy - 1.21).abs() < 0.01);
        assert!((collectivist_autonomy - 0.86).abs() < 0.01);
    }

    // --- Coverage Gap Tests ---

    #[test]
    fn macrosystem_constraint_set_allows_schema() {
        let constraint = MacrosystemConstraintSet {
            allowed_relationship_schemas: vec![RelationshipSchema::Peer, RelationshipSchema::Mentor],
            hierarchy_violation_penalty: 0.1,
        };

        assert!(constraint.allows_schema(RelationshipSchema::Peer));
        assert!(constraint.allows_schema(RelationshipSchema::Mentor));
        assert!(!constraint.allows_schema(RelationshipSchema::Romantic));
    }

    #[test]
    fn macrosystem_constraint_set_default() {
        let constraint = MacrosystemConstraintSet::default();

        // Default allows all schemas
        assert!(constraint.allows_schema(RelationshipSchema::Peer));
        assert!(constraint.allows_schema(RelationshipSchema::Mentor));
        assert!(constraint.allows_schema(RelationshipSchema::Romantic));
        assert!(constraint.allows_schema(RelationshipSchema::Family));
        assert!(constraint.allows_schema(RelationshipSchema::Nuclear));
        assert!(constraint.allows_schema(RelationshipSchema::Extended));
        assert!(constraint.allows_schema(RelationshipSchema::Subordinate));

        // Default penalty is zero
        assert!((constraint.hierarchy_violation_penalty - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn macrosystem_modifier_apply_to_partial_overrides() {
        let base = MacrosystemContext::default();
        let mut modifier = MacrosystemModifier::default();

        // Set only some fields
        modifier.power_distance = Some(0.8);
        modifier.cultural_stress = Some(0.75);

        let modified = modifier.apply_to(&base);

        // Modified fields should change
        assert!((modified.cultural_orientation.power_distance - 0.8).abs() < f64::EPSILON);
        assert!((modified.cultural_stress - 0.75).abs() < f64::EPSILON);

        // Unmodified fields should remain as base
        assert_eq!(
            modified.cultural_orientation.individualism_collectivism,
            base.cultural_orientation.individualism_collectivism
        );
        assert_eq!(modified.collective_trauma, base.collective_trauma);
    }

    #[test]
    fn macrosystem_modifier_preserves_power_distance_when_none() {
        let base = MacrosystemContext::default();
        let mut modifier = MacrosystemModifier::default();

        modifier.cultural_stress = Some(0.42);

        let modified = modifier.apply_to(&base);

        assert_eq!(
            modified.cultural_orientation.power_distance,
            base.cultural_orientation.power_distance
        );
    }

    #[test]
    fn macrosystem_modifier_apply_to_all_fields() {
        let base = MacrosystemContext::default();
        let mut modifier = MacrosystemModifier::default();

        modifier.individualism_collectivism = Some(0.5);
        modifier.power_distance = Some(0.8);
        modifier.uncertainty_avoidance = Some(0.7);
        modifier.rule_of_law = Some(0.9);
        modifier.social_mobility = Some(0.4);
        modifier.corruption_level = Some(0.2);
        modifier.cultural_stress = Some(0.6);
        modifier.collective_trauma = Some(0.3);
        modifier.economic_inequality = Some(0.5);

        let modified = modifier.apply_to(&base);

        assert!((modified.cultural_orientation.individualism_collectivism - 0.5).abs() < f64::EPSILON);
        assert!((modified.cultural_orientation.power_distance - 0.8).abs() < f64::EPSILON);
        assert!((modified.cultural_orientation.uncertainty_avoidance - 0.7).abs() < f64::EPSILON);
        assert!((modified.institutional_structure.rule_of_law - 0.9).abs() < f64::EPSILON);
        assert!((modified.institutional_structure.social_mobility - 0.4).abs() < f64::EPSILON);
        assert!((modified.institutional_structure.corruption_level - 0.2).abs() < f64::EPSILON);
        assert!((modified.cultural_stress - 0.6).abs() < f64::EPSILON);
        assert!((modified.collective_trauma - 0.3).abs() < f64::EPSILON);
        assert!((modified.economic_inequality - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn macrosystem_modifier_apply_to_with_clamping() {
        let base = MacrosystemContext::default();
        let mut modifier = MacrosystemModifier::default();

        // Test out-of-range clamping
        modifier.individualism_collectivism = Some(1.5); // Should clamp to 1.0
        modifier.power_distance = Some(-0.5); // Should clamp to 0.0
        modifier.cultural_stress = Some(2.0); // Should clamp to 1.0

        let modified = modifier.apply_to(&base);

        assert!((modified.cultural_orientation.individualism_collectivism - 1.0).abs() < f64::EPSILON);
        assert!((modified.cultural_orientation.power_distance - 0.0).abs() < f64::EPSILON);
        assert!((modified.cultural_stress - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn macrosystem_constraint_set_low_power_distance_allows_peer() {
        let mut low_pd = MacrosystemContext::default();
        low_pd.cultural_orientation.power_distance = 0.4; // Below 0.6 threshold

        let constraint = low_pd.constraint_set();

        // Low power distance allows peer relationships
        assert!(constraint.allows_schema(RelationshipSchema::Peer));
        // Penalty is zero in low power distance
        assert!((constraint.hierarchy_violation_penalty - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn macrosystem_constraint_set_high_power_distance_no_peer() {
        let mut high_pd = MacrosystemContext::default();
        high_pd.cultural_orientation.power_distance = 0.75; // Above 0.6 threshold

        let constraint = high_pd.constraint_set();

        // High power distance does not allow peer relationships
        assert!(!constraint.allows_schema(RelationshipSchema::Peer));
        // Penalty is non-zero
        assert!(constraint.hierarchy_violation_penalty > 0.0);
        // Penalty should be: power_distance * 0.2 = 0.75 * 0.2 = 0.15
        assert!((constraint.hierarchy_violation_penalty - 0.15).abs() < f64::EPSILON);
    }

    #[test]
    fn macrosystem_autonomy_need_weight_neutral_individualism() {
        let mut ctx = MacrosystemContext::default();
        // Set ic to a value between -0.3 and 0.3 (neutral)
        ctx.cultural_orientation.individualism_collectivism = 0.0;

        let weight = ctx.autonomy_need_weight();
        // 1.0 + 0.0 * 0.2 = 1.0
        assert!((weight - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn macrosystem_autonomy_need_weight_slight_collectivist() {
        let mut ctx = MacrosystemContext::default();
        ctx.cultural_orientation.individualism_collectivism = -0.5;

        let weight = ctx.autonomy_need_weight();
        // ic < 0.3, so: 1.0 + (-0.5) * 0.2 = 0.9
        assert!((weight - 0.9).abs() < f64::EPSILON);
    }

    #[test]
    fn macrosystem_autonomy_need_weight_individualist_threshold() {
        let mut ctx = MacrosystemContext::default();
        // Set ic exactly at the threshold
        ctx.cultural_orientation.individualism_collectivism = 0.3;

        let weight = ctx.autonomy_need_weight();
        // At exactly 0.3, we're on boundary. The condition is ic > 0.3, so false branch applies.
        // This tests the boundary case.
        assert!(weight > 1.0);
    }

    #[test]
    fn macrosystem_belonging_need_weight_neutral_individualism() {
        let mut ctx = MacrosystemContext::default();
        ctx.cultural_orientation.individualism_collectivism = 0.0;

        let weight = ctx.belonging_need_weight();
        // 1.0 - 0.0 * 0.3 = 1.0
        assert!((weight - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn macrosystem_belonging_need_weight_slight_collectivist() {
        let mut ctx = MacrosystemContext::default();
        ctx.cultural_orientation.individualism_collectivism = -0.5;

        let weight = ctx.belonging_need_weight();
        // ic < 0.3, so: 1.0 - (-0.5) * 0.3 = 1.15
        assert!((weight - 1.15).abs() < f64::EPSILON);
    }

    #[test]
    fn macrosystem_belonging_need_weight_individualist_threshold() {
        let mut ctx = MacrosystemContext::default();
        ctx.cultural_orientation.individualism_collectivism = 0.3;

        let weight = ctx.belonging_need_weight();
        // At exactly 0.3, condition ic > 0.3 is false, so else branch applies
        assert!(weight > 0.9);
    }

    #[test]
    fn macrosystem_belonging_need_weight_positive_below_threshold() {
        let mut ctx = MacrosystemContext::default();
        ctx.cultural_orientation.individualism_collectivism = 0.2;

        let weight = ctx.belonging_need_weight();
        // ic = 0.2, which is <= 0.3, so else branch: 1.0 - 0.2 * 0.3 = 0.94
        assert!((weight - 0.94).abs() < f64::EPSILON);
    }

    #[test]
    fn for_subculture_with_nonexistent_key_returns_clone() {
        let ctx = MacrosystemContext::default();
        let result = ctx.for_subculture("nonexistent_key");

        // Should return a clone of the original
        assert_eq!(result, ctx);
    }

    #[test]
    fn macrosystem_set_value_power_distance() {
        let mut ctx = MacrosystemContext::default();
        ctx.set_value(&MacrosystemPath::PowerDistance, 0.85);
        assert!((ctx.cultural_orientation.power_distance - 0.85).abs() < f64::EPSILON);
    }

    #[test]
    fn macrosystem_set_value_uncertainty_avoidance() {
        let mut ctx = MacrosystemContext::default();
        ctx.set_value(&MacrosystemPath::UncertaintyAvoidance, 0.65);
        assert!((ctx.cultural_orientation.uncertainty_avoidance - 0.65).abs() < f64::EPSILON);
    }

    #[test]
    fn macrosystem_set_value_rule_of_law() {
        let mut ctx = MacrosystemContext::default();
        ctx.set_value(&MacrosystemPath::RuleOfLaw, 0.85);
        assert!((ctx.institutional_structure.rule_of_law - 0.85).abs() < f64::EPSILON);
    }

    #[test]
    fn macrosystem_set_value_social_mobility() {
        let mut ctx = MacrosystemContext::default();
        ctx.set_value(&MacrosystemPath::SocialMobility, 0.75);
        assert!((ctx.institutional_structure.social_mobility - 0.75).abs() < f64::EPSILON);
    }

    #[test]
    fn macrosystem_set_value_corruption_level() {
        let mut ctx = MacrosystemContext::default();
        ctx.set_value(&MacrosystemPath::CorruptionLevel, 0.55);
        assert!((ctx.institutional_structure.corruption_level - 0.55).abs() < f64::EPSILON);
    }

    #[test]
    fn macrosystem_set_value_collective_trauma() {
        let mut ctx = MacrosystemContext::default();
        ctx.set_value(&MacrosystemPath::CollectiveTrauma, 0.65);
        assert!((ctx.collective_trauma - 0.65).abs() < f64::EPSILON);
    }

    #[test]
    fn macrosystem_set_value_economic_inequality() {
        let mut ctx = MacrosystemContext::default();
        ctx.set_value(&MacrosystemPath::EconomicInequality, 0.75);
        assert!((ctx.economic_inequality - 0.75).abs() < f64::EPSILON);
    }

    #[test]
    fn macrosystem_is_individualist_boundary_above_threshold() {
        let mut ctx = MacrosystemContext::default();
        ctx.cultural_orientation.individualism_collectivism = 0.31;
        assert!(ctx.is_individualist());
    }

    #[test]
    fn macrosystem_is_individualist_boundary_below_threshold() {
        let mut ctx = MacrosystemContext::default();
        ctx.cultural_orientation.individualism_collectivism = 0.29;
        assert!(!ctx.is_individualist());
    }

    #[test]
    fn macrosystem_is_collectivist_boundary_below_threshold() {
        let mut ctx = MacrosystemContext::default();
        ctx.cultural_orientation.individualism_collectivism = -0.31;
        assert!(ctx.is_collectivist());
    }

    #[test]
    fn macrosystem_is_collectivist_boundary_above_threshold() {
        let mut ctx = MacrosystemContext::default();
        ctx.cultural_orientation.individualism_collectivism = -0.29;
        assert!(!ctx.is_collectivist());
    }

    #[test]
    fn macrosystem_is_high_power_distance_boundary_above() {
        let mut ctx = MacrosystemContext::default();
        ctx.cultural_orientation.power_distance = 0.61;
        assert!(ctx.is_high_power_distance());
    }

    #[test]
    fn macrosystem_is_high_power_distance_boundary_below() {
        let mut ctx = MacrosystemContext::default();
        ctx.cultural_orientation.power_distance = 0.59;
        assert!(!ctx.is_high_power_distance());
    }

    #[test]
    fn macrosystem_modifier_default_all_none() {
        let modifier = MacrosystemModifier::default();

        assert!(modifier.individualism_collectivism.is_none());
        assert!(modifier.power_distance.is_none());
        assert!(modifier.uncertainty_avoidance.is_none());
        assert!(modifier.rule_of_law.is_none());
        assert!(modifier.social_mobility.is_none());
        assert!(modifier.corruption_level.is_none());
        assert!(modifier.cultural_stress.is_none());
        assert!(modifier.collective_trauma.is_none());
        assert!(modifier.economic_inequality.is_none());
    }

    #[test]
    fn macrosystem_constraint_set_clone_eq() {
        let constraint1 = MacrosystemConstraintSet::default();
        let constraint2 = constraint1.clone();
        assert_eq!(constraint1, constraint2);
    }

    #[test]
    fn macrosystem_constraint_set_debug() {
        let constraint = MacrosystemConstraintSet::default();
        let debug_str = format!("{:?}", constraint);
        assert!(debug_str.contains("MacrosystemConstraintSet"));
    }

    #[test]
    fn macrosystem_modifier_clone_eq() {
        let modifier1 = MacrosystemModifier::default();
        let modifier2 = modifier1.clone();
        assert_eq!(modifier1, modifier2);
    }

    #[test]
    fn macrosystem_modifier_debug() {
        let modifier = MacrosystemModifier::default();
        let debug_str = format!("{:?}", modifier);
        assert!(debug_str.contains("MacrosystemModifier"));
    }

    #[test]
    fn macrosystem_subculture_none_exists() {
        let ctx = MacrosystemContext::default();
        let result = ctx.for_subculture("nonexistent");
        assert_eq!(result, ctx);
    }

    #[test]
    fn macrosystem_subculture_multiple_overrides() {
        let mut ctx = MacrosystemContext::default();

        let mut modifier1 = MacrosystemModifier::default();
        modifier1.power_distance = Some(0.9);
        modifier1.cultural_stress = Some(0.8);

        let mut modifier2 = MacrosystemModifier::default();
        modifier2.power_distance = Some(0.2);
        modifier2.economic_inequality = Some(0.1);

        ctx.subculture_overrides
            .insert("elite".to_string(), modifier1);
        ctx.subculture_overrides
            .insert("marginalized".to_string(), modifier2);

        let elite = ctx.for_subculture("elite");
        assert!((elite.cultural_orientation.power_distance - 0.9).abs() < f64::EPSILON);
        assert!((elite.cultural_stress - 0.8).abs() < f64::EPSILON);

        let marginalized = ctx.for_subculture("marginalized");
        assert!((marginalized.cultural_orientation.power_distance - 0.2).abs() < f64::EPSILON);
        assert!((marginalized.economic_inequality - 0.1).abs() < f64::EPSILON);
    }

    #[test]
    fn belonging_need_weight_above_threshold() {
        let mut ctx = MacrosystemContext::default();
        ctx.cultural_orientation.individualism_collectivism = 0.5;
        let weight = ctx.belonging_need_weight();
        assert!((weight - (1.0 - 0.5 * 0.2)).abs() < f64::EPSILON);
    }

    #[test]
    fn macrosystem_subculture_override_applied() {
        let mut ctx = MacrosystemContext::default();
        ctx.cultural_orientation.power_distance = 0.3;

        let mut modifier = MacrosystemModifier::default();
        modifier.power_distance = Some(0.9);

        ctx.subculture_overrides
            .insert("override".to_string(), modifier);

        let overridden = ctx.for_subculture("override");
        assert!((overridden.cultural_orientation.power_distance - 0.9).abs() < f64::EPSILON);
    }
}
