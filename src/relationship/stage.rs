//! Relationship stage definitions.
//!
//! Stages represent the depth and development level of a relationship.

/// The development stage of a relationship.
///
/// Stages progress from Stranger through increasing levels of trust
/// and intimacy. The Estranged stage represents a broken relationship.
///
/// # Stage Progression
///
/// ```text
/// Stranger -> Acquaintance -> Established -> Intimate
///     |           |              |            |
///     +---------> +------------> +----------> +----> Estranged
/// ```
///
/// Any stage can transition to Estranged following betrayal or sustained conflict.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::relationship::RelationshipStage;
///
/// let stage = RelationshipStage::Stranger;
/// assert_eq!(stage.propensity_weight(), 0.6);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RelationshipStage {
    /// No significant interaction history.
    #[default]
    Stranger,
    /// Limited interactions, beginning to form impressions.
    Acquaintance,
    /// Regular relationship with consistent patterns.
    Established,
    /// Deep trust and extensive history.
    Intimate,
    /// Previously close relationship that has deteriorated.
    Estranged,
}

impl RelationshipStage {
    /// Returns the propensity weight for trust computation at this stage.
    ///
    /// Propensity weight diminishes as relationships develop because
    /// direct experience becomes more important than general disposition.
    ///
    /// - Stranger: 0.6 (heavily relies on disposition)
    /// - Acquaintance: 0.4 (balanced)
    /// - Established: 0.2 (mostly experience-based)
    /// - Intimate: 0.1 (almost entirely experience-based)
    /// - Estranged: 0.3 (some defensive dispositional caution)
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::relationship::RelationshipStage;
    ///
    /// assert!((RelationshipStage::Stranger.propensity_weight() - 0.6).abs() < f32::EPSILON);
    /// assert!((RelationshipStage::Intimate.propensity_weight() - 0.1).abs() < f32::EPSILON);
    /// ```
    #[must_use]
    pub const fn propensity_weight(&self) -> f32 {
        match self {
            RelationshipStage::Stranger => 0.6,
            RelationshipStage::Acquaintance => 0.4,
            RelationshipStage::Established => 0.2,
            RelationshipStage::Intimate => 0.1,
            RelationshipStage::Estranged => 0.3,
        }
    }

    /// Returns the trustworthiness weight for trust computation at this stage.
    ///
    /// This is the complement of propensity weight, representing how much
    /// perceived trustworthiness matters at each stage.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::relationship::RelationshipStage;
    ///
    /// let stage = RelationshipStage::Established;
    /// let total = stage.propensity_weight() + stage.trustworthiness_weight();
    /// assert!((total - 1.0).abs() < f32::EPSILON);
    /// ```
    #[must_use]
    pub const fn trustworthiness_weight(&self) -> f32 {
        match self {
            RelationshipStage::Stranger => 0.4,
            RelationshipStage::Acquaintance => 0.6,
            RelationshipStage::Established => 0.8,
            RelationshipStage::Intimate => 0.9,
            RelationshipStage::Estranged => 0.7,
        }
    }

    /// Returns the risk modifier for perceived risk at this stage.
    ///
    /// Positive values increase perceived risk, negative values decrease it.
    ///
    /// - Stranger: +0.3 (high uncertainty)
    /// - Acquaintance: +0.2 (some uncertainty)
    /// - Established: 0.0 (neutral)
    /// - Intimate: -0.1 (trusted, reduces perceived risk)
    /// - Estranged: +0.4 (high risk due to past hurt)
    #[must_use]
    pub const fn risk_modifier(&self) -> f32 {
        match self {
            RelationshipStage::Stranger => 0.3,
            RelationshipStage::Acquaintance => 0.2,
            RelationshipStage::Established => 0.0,
            RelationshipStage::Intimate => -0.1,
            RelationshipStage::Estranged => 0.4,
        }
    }

    /// Returns a human-readable name for this stage.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            RelationshipStage::Stranger => "Stranger",
            RelationshipStage::Acquaintance => "Acquaintance",
            RelationshipStage::Established => "Established",
            RelationshipStage::Intimate => "Intimate",
            RelationshipStage::Estranged => "Estranged",
        }
    }

    /// Returns a description of this stage.
    #[must_use]
    pub const fn description(&self) -> &'static str {
        match self {
            RelationshipStage::Stranger => "No significant interaction history",
            RelationshipStage::Acquaintance => "Limited interactions, forming impressions",
            RelationshipStage::Established => "Regular relationship with consistent patterns",
            RelationshipStage::Intimate => "Deep trust and extensive history",
            RelationshipStage::Estranged => "Previously close but now deteriorated",
        }
    }

    /// Returns all relationship stages.
    #[must_use]
    pub const fn all() -> [RelationshipStage; 5] {
        [
            RelationshipStage::Stranger,
            RelationshipStage::Acquaintance,
            RelationshipStage::Established,
            RelationshipStage::Intimate,
            RelationshipStage::Estranged,
        ]
    }

    /// Returns true if this is a positive relationship stage.
    ///
    /// Positive stages are those where the relationship is functioning
    /// and has potential for growth.
    #[must_use]
    pub const fn is_positive(&self) -> bool {
        matches!(
            self,
            RelationshipStage::Acquaintance
                | RelationshipStage::Established
                | RelationshipStage::Intimate
        )
    }

    /// Returns true if this stage represents a developed relationship.
    ///
    /// Developed relationships have significant shared history.
    #[must_use]
    pub const fn is_developed(&self) -> bool {
        matches!(
            self,
            RelationshipStage::Established | RelationshipStage::Intimate
        )
    }
}

impl std::fmt::Display for RelationshipStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn propensity_weight_values() {
        assert!((RelationshipStage::Stranger.propensity_weight() - 0.6).abs() < f32::EPSILON);
        assert!((RelationshipStage::Acquaintance.propensity_weight() - 0.4).abs() < f32::EPSILON);
        assert!((RelationshipStage::Established.propensity_weight() - 0.2).abs() < f32::EPSILON);
        assert!((RelationshipStage::Intimate.propensity_weight() - 0.1).abs() < f32::EPSILON);
        assert!((RelationshipStage::Estranged.propensity_weight() - 0.3).abs() < f32::EPSILON);
    }

    #[test]
    fn trustworthiness_weight_values() {
        assert!((RelationshipStage::Stranger.trustworthiness_weight() - 0.4).abs() < f32::EPSILON);
        assert!(
            (RelationshipStage::Acquaintance.trustworthiness_weight() - 0.6).abs() < f32::EPSILON
        );
        assert!(
            (RelationshipStage::Established.trustworthiness_weight() - 0.8).abs() < f32::EPSILON
        );
        assert!((RelationshipStage::Intimate.trustworthiness_weight() - 0.9).abs() < f32::EPSILON);
        assert!((RelationshipStage::Estranged.trustworthiness_weight() - 0.7).abs() < f32::EPSILON);
    }

    #[test]
    fn weights_sum_to_one() {
        for stage in RelationshipStage::all() {
            let sum = stage.propensity_weight() + stage.trustworthiness_weight();
            let message = format!("Stage {:?} weights sum to {}, not 1.0", stage, sum);
            assert!((sum - 1.0).abs() < f32::EPSILON, "{}", message);
        }
    }

    #[test]
    fn risk_modifier_values() {
        assert!((RelationshipStage::Stranger.risk_modifier() - 0.3).abs() < f32::EPSILON);
        assert!((RelationshipStage::Acquaintance.risk_modifier() - 0.2).abs() < f32::EPSILON);
        assert!(RelationshipStage::Established.risk_modifier().abs() < f32::EPSILON);
        assert!((RelationshipStage::Intimate.risk_modifier() - (-0.1)).abs() < f32::EPSILON);
        assert!((RelationshipStage::Estranged.risk_modifier() - 0.4).abs() < f32::EPSILON);
    }

    #[test]
    fn propensity_weight_diminishes_with_stage() {
        assert!(
            RelationshipStage::Stranger.propensity_weight()
                > RelationshipStage::Acquaintance.propensity_weight()
        );
        assert!(
            RelationshipStage::Acquaintance.propensity_weight()
                > RelationshipStage::Established.propensity_weight()
        );
        assert!(
            RelationshipStage::Established.propensity_weight()
                > RelationshipStage::Intimate.propensity_weight()
        );
    }

    #[test]
    fn name_values() {
        assert_eq!(RelationshipStage::Stranger.name(), "Stranger");
        assert_eq!(RelationshipStage::Acquaintance.name(), "Acquaintance");
        assert_eq!(RelationshipStage::Established.name(), "Established");
        assert_eq!(RelationshipStage::Intimate.name(), "Intimate");
        assert_eq!(RelationshipStage::Estranged.name(), "Estranged");
    }

    #[test]
    fn descriptions_not_empty() {
        for stage in RelationshipStage::all() {
            assert!(!stage.description().is_empty());
        }
    }

    #[test]
    fn all_stages() {
        let all = RelationshipStage::all();
        assert_eq!(all.len(), 5);
    }

    #[test]
    fn is_positive() {
        assert!(!RelationshipStage::Stranger.is_positive());
        assert!(RelationshipStage::Acquaintance.is_positive());
        assert!(RelationshipStage::Established.is_positive());
        assert!(RelationshipStage::Intimate.is_positive());
        assert!(!RelationshipStage::Estranged.is_positive());
    }

    #[test]
    fn is_developed() {
        assert!(!RelationshipStage::Stranger.is_developed());
        assert!(!RelationshipStage::Acquaintance.is_developed());
        assert!(RelationshipStage::Established.is_developed());
        assert!(RelationshipStage::Intimate.is_developed());
        assert!(!RelationshipStage::Estranged.is_developed());
    }

    #[test]
    fn default_is_stranger() {
        assert_eq!(RelationshipStage::default(), RelationshipStage::Stranger);
    }

    #[test]
    fn display_format() {
        assert_eq!(format!("{}", RelationshipStage::Intimate), "Intimate");
    }

    #[test]
    fn equality() {
        assert_eq!(RelationshipStage::Stranger, RelationshipStage::Stranger);
        assert_ne!(RelationshipStage::Stranger, RelationshipStage::Intimate);
    }

    #[test]
    fn clone_copy() {
        let s1 = RelationshipStage::Established;
        let s2 = s1;
        let s3 = s1.clone();
        assert_eq!(s1, s2);
        assert_eq!(s1, s3);
    }

    #[test]
    fn hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(RelationshipStage::Stranger);
        set.insert(RelationshipStage::Stranger);
        assert_eq!(set.len(), 1);
        set.insert(RelationshipStage::Intimate);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn debug_format() {
        let debug = format!("{:?}", RelationshipStage::Acquaintance);
        assert!(debug.contains("Acquaintance"));
    }
}
