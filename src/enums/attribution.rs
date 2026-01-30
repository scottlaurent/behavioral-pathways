//! Attribution enums for causal attribution of events.
//!
//! These enums model how entities attribute causes to events,
//! which affects how events modify psychological state.

use crate::types::EntityId;

/// Stability dimension of causal attribution.
///
/// Per attribution theory, stable attributions (perceived as permanent)
/// have different psychological effects than unstable (temporary) ones.
/// In ITS theory, stable attributions to TB/PB states feed into
/// interpersonal hopelessness.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::enums::AttributionStability;
///
/// let stability = AttributionStability::Stable;
/// assert!(stability.is_stable());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AttributionStability {
    /// Perceived as permanent/unchangeable.
    Stable,
    /// Perceived as temporary/changeable.
    #[default]
    Unstable,
}

impl AttributionStability {
    /// Returns true if this attribution is stable (permanent).
    #[must_use]
    pub const fn is_stable(&self) -> bool {
        matches!(self, AttributionStability::Stable)
    }

    /// Returns true if this attribution is unstable (temporary).
    #[must_use]
    pub const fn is_unstable(&self) -> bool {
        matches!(self, AttributionStability::Unstable)
    }

    /// Returns a human-readable name for this stability.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            AttributionStability::Stable => "Stable",
            AttributionStability::Unstable => "Unstable",
        }
    }
}

/// Causal attribution for an event.
///
/// Attribution theory states that people attribute causes to events,
/// and these attributions affect emotional and behavioral responses.
/// The locus of causality (self, other, situation) combined with
/// stability determines the psychological impact.
///
/// # ITS Integration
///
/// In Joiner's Interpersonal Theory of Suicide:
/// - Self-attributions with stable stability for negative events
///   increase perceived burdensomeness
/// - Stable attributions for social exclusion events increase
///   interpersonal hopelessness
///
/// # Examples
///
/// ```
/// use behavioral_pathways::enums::{Attribution, AttributionStability};
/// use behavioral_pathways::types::EntityId;
///
/// // Entity blames themselves for a failure (stable = trait-like)
/// let self_attribution = Attribution::SelfCaused(AttributionStability::Stable);
///
/// // Entity blames another person (unstable = situational)
/// let other_id = EntityId::new("person_001").unwrap();
/// let other_attribution = Attribution::Other(other_id, AttributionStability::Unstable);
///
/// // Entity blames circumstances
/// let situational = Attribution::Situational(AttributionStability::Unstable);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Attribution {
    /// Entity attributes cause to themselves.
    SelfCaused(AttributionStability),
    /// Entity attributes cause to another entity.
    Other(EntityId, AttributionStability),
    /// Entity attributes cause to circumstances/situation.
    Situational(AttributionStability),
    /// Entity is uncertain about cause.
    #[default]
    Unknown,
}

impl Attribution {
    /// Creates a self-attribution with the given stability.
    #[must_use]
    pub const fn self_caused(stability: AttributionStability) -> Self {
        Attribution::SelfCaused(stability)
    }

    /// Creates an other-attribution with the given entity and stability.
    #[must_use]
    pub fn other(entity_id: EntityId, stability: AttributionStability) -> Self {
        Attribution::Other(entity_id, stability)
    }

    /// Creates a situational attribution with the given stability.
    #[must_use]
    pub const fn situational(stability: AttributionStability) -> Self {
        Attribution::Situational(stability)
    }

    /// Creates an unknown attribution.
    #[must_use]
    pub const fn unknown() -> Self {
        Attribution::Unknown
    }

    /// Returns the stability of this attribution, or None for Unknown.
    #[must_use]
    pub fn stability(&self) -> Option<AttributionStability> {
        match self {
            Attribution::SelfCaused(s) => Some(*s),
            Attribution::Other(_, s) => Some(*s),
            Attribution::Situational(s) => Some(*s),
            Attribution::Unknown => None,
        }
    }

    /// Returns true if this attribution is stable (permanent).
    #[must_use]
    pub fn is_stable(&self) -> bool {
        self.stability().is_some_and(|s| s.is_stable())
    }

    /// Returns true if this is a self-attribution.
    #[must_use]
    pub const fn is_self_caused(&self) -> bool {
        matches!(self, Attribution::SelfCaused(_))
    }

    /// Returns true if this is an other-attribution.
    #[must_use]
    pub const fn is_other(&self) -> bool {
        matches!(self, Attribution::Other(_, _))
    }

    /// Returns true if this is a situational attribution.
    #[must_use]
    pub const fn is_situational(&self) -> bool {
        matches!(self, Attribution::Situational(_))
    }

    /// Returns true if attribution is unknown.
    #[must_use]
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Attribution::Unknown)
    }

    /// Returns the entity ID for other-attributions, or None otherwise.
    #[must_use]
    pub fn other_entity(&self) -> Option<&EntityId> {
        match self {
            Attribution::Other(id, _) => Some(id),
            _ => None,
        }
    }

    /// Returns a human-readable name for this attribution type.
    #[must_use]
    pub const fn type_name(&self) -> &'static str {
        match self {
            Attribution::SelfCaused(_) => "Self-Caused",
            Attribution::Other(_, _) => "Other",
            Attribution::Situational(_) => "Situational",
            Attribution::Unknown => "Unknown",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attribution_stability_is_stable() {
        assert!(AttributionStability::Stable.is_stable());
        assert!(!AttributionStability::Unstable.is_stable());
    }

    #[test]
    fn attribution_stability_is_unstable() {
        assert!(AttributionStability::Unstable.is_unstable());
        assert!(!AttributionStability::Stable.is_unstable());
    }

    #[test]
    fn attribution_stability_default_is_unstable() {
        assert_eq!(
            AttributionStability::default(),
            AttributionStability::Unstable
        );
    }

    #[test]
    fn attribution_stability_names() {
        assert_eq!(AttributionStability::Stable.name(), "Stable");
        assert_eq!(AttributionStability::Unstable.name(), "Unstable");
    }

    #[test]
    fn attribution_self_caused_creation() {
        let attr = Attribution::self_caused(AttributionStability::Stable);
        assert!(attr.is_self_caused());
        assert!(attr.is_stable());
        assert_eq!(attr.stability(), Some(AttributionStability::Stable));
    }

    #[test]
    fn attribution_other_creation() {
        let entity = EntityId::new("person_001").unwrap();
        let attr = Attribution::other(entity.clone(), AttributionStability::Unstable);
        assert!(attr.is_other());
        assert!(!attr.is_stable());
        assert_eq!(attr.other_entity(), Some(&entity));
    }

    #[test]
    fn attribution_situational_creation() {
        let attr = Attribution::situational(AttributionStability::Stable);
        assert!(attr.is_situational());
        assert!(attr.is_stable());
    }

    #[test]
    fn attribution_unknown_creation() {
        let attr = Attribution::unknown();
        assert!(attr.is_unknown());
        assert!(!attr.is_stable());
        assert_eq!(attr.stability(), None);
    }

    #[test]
    fn attribution_default_is_unknown() {
        assert_eq!(Attribution::default(), Attribution::Unknown);
    }

    #[test]
    fn attribution_type_names() {
        let self_attr = Attribution::SelfCaused(AttributionStability::Stable);
        assert_eq!(self_attr.type_name(), "Self-Caused");

        let entity = EntityId::new("person").unwrap();
        let other_attr = Attribution::Other(entity, AttributionStability::Unstable);
        assert_eq!(other_attr.type_name(), "Other");

        let situational = Attribution::Situational(AttributionStability::Stable);
        assert_eq!(situational.type_name(), "Situational");

        let unknown = Attribution::Unknown;
        assert_eq!(unknown.type_name(), "Unknown");
    }

    #[test]
    fn attribution_other_entity_none_for_non_other() {
        let self_attr = Attribution::SelfCaused(AttributionStability::Stable);
        assert!(self_attr.other_entity().is_none());

        let situational = Attribution::Situational(AttributionStability::Stable);
        assert!(situational.other_entity().is_none());

        let unknown = Attribution::Unknown;
        assert!(unknown.other_entity().is_none());
    }

    #[test]
    fn attribution_is_predicates() {
        let self_attr = Attribution::SelfCaused(AttributionStability::Stable);
        assert!(self_attr.is_self_caused());
        assert!(!self_attr.is_other());
        assert!(!self_attr.is_situational());
        assert!(!self_attr.is_unknown());

        let entity = EntityId::new("person").unwrap();
        let other_attr = Attribution::Other(entity, AttributionStability::Unstable);
        assert!(!other_attr.is_self_caused());
        assert!(other_attr.is_other());
        assert!(!other_attr.is_situational());
        assert!(!other_attr.is_unknown());
    }

    #[test]
    fn attribution_stability_is_copy() {
        let s1 = AttributionStability::Stable;
        let s2 = s1;
        assert_eq!(s1, s2);
    }

    #[test]
    fn attribution_clone() {
        let entity = EntityId::new("person").unwrap();
        let attr = Attribution::Other(entity, AttributionStability::Stable);
        let cloned = attr.clone();
        assert_eq!(attr, cloned);
    }

    #[test]
    fn attribution_debug_format() {
        let attr = Attribution::SelfCaused(AttributionStability::Stable);
        let debug = format!("{:?}", attr);
        assert!(debug.contains("SelfCaused"));
        assert!(debug.contains("Stable"));
    }

    #[test]
    fn attribution_stability_debug_format() {
        let stability = AttributionStability::Stable;
        let debug = format!("{:?}", stability);
        assert!(debug.contains("Stable"));
    }

    #[test]
    fn attribution_is_stable_false_for_unknown() {
        let attr = Attribution::Unknown;
        assert!(!attr.is_stable());
    }
}
