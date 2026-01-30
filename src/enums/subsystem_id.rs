//! Subsystem identifiers for entity processing.
//!
//! Each subsystem handles a specific aspect of entity behavior and state.
//! Different entity types may have different active subsystems based on
//! their psychological complexity.

use serde::{Deserialize, Serialize};

/// Identifies a subsystem that can be active or inactive for an entity.
///
/// Subsystems are modular components that handle specific aspects of
/// psychological processing. Not all entities have all subsystems active.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::enums::SubsystemId;
///
/// let active = vec![SubsystemId::State, SubsystemId::Memory, SubsystemId::Relationship];
/// assert!(active.contains(&SubsystemId::State));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SubsystemId {
    /// State system: decay, drift, feedback loops, threshold checks.
    ///
    /// Handles the core psychological state processing including
    /// applying decay to deltas, processing needs growth, and
    /// computing derived values like emotions.
    State,

    /// Memory system: capture, retrieval, consolidation.
    ///
    /// Manages memory formation from events, promotion through
    /// memory layers, and mood-congruent retrieval.
    Memory,

    /// Relationship system: trust, affinity, interaction effects.
    ///
    /// Processes relationship state changes, trust antecedents,
    /// and relationship decay over time.
    Relationship,

    /// Event system: event processing and cascades.
    ///
    /// Handles event interpretation, effect application, and
    /// triggering of downstream events.
    Event,

    /// Behavioral decision system: compliance, coping, reactions.
    ///
    /// Determines how entities respond to commands, stressors,
    /// and opportunities based on personality and state.
    BehavioralDecision,

    /// Developmental system: life stages, plasticity, cohort effects.
    ///
    /// Manages long-term developmental changes including
    /// personality crystallization and sensitive periods.
    Developmental,

    /// Ecological context system: spillover effects between contexts.
    ///
    /// Processes interactions between microsystems, exosystem
    /// effects, and macrosystem constraints.
    EcologicalContext,

    /// Interaction system: ambient exchanges, conversations.
    ///
    /// Handles spontaneous interactions based on proximity
    /// and relationship state.
    Interaction,
}

impl SubsystemId {
    /// Returns all subsystem IDs.
    ///
    /// Useful for iterating over all possible subsystems.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::enums::SubsystemId;
    ///
    /// let all = SubsystemId::all();
    /// assert_eq!(all.len(), 8);
    /// ```
    #[must_use]
    pub const fn all() -> [SubsystemId; 8] {
        [
            SubsystemId::State,
            SubsystemId::Memory,
            SubsystemId::Relationship,
            SubsystemId::Event,
            SubsystemId::BehavioralDecision,
            SubsystemId::Developmental,
            SubsystemId::EcologicalContext,
            SubsystemId::Interaction,
        ]
    }

    /// Returns a human-readable name for this subsystem.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::enums::SubsystemId;
    ///
    /// assert_eq!(SubsystemId::State.name(), "State");
    /// assert_eq!(SubsystemId::BehavioralDecision.name(), "Behavioral Decision");
    /// ```
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            SubsystemId::State => "State",
            SubsystemId::Memory => "Memory",
            SubsystemId::Relationship => "Relationship",
            SubsystemId::Event => "Event",
            SubsystemId::BehavioralDecision => "Behavioral Decision",
            SubsystemId::Developmental => "Developmental",
            SubsystemId::EcologicalContext => "Ecological Context",
            SubsystemId::Interaction => "Interaction",
        }
    }
}

impl std::fmt::Display for SubsystemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn all_subsystems_listed() {
        let all = SubsystemId::all();
        assert_eq!(all.len(), 8);

        // Verify all expected subsystems are present
        let set: HashSet<_> = all.iter().collect();
        assert!(set.contains(&SubsystemId::State));
        assert!(set.contains(&SubsystemId::Memory));
        assert!(set.contains(&SubsystemId::Relationship));
        assert!(set.contains(&SubsystemId::Event));
        assert!(set.contains(&SubsystemId::BehavioralDecision));
        assert!(set.contains(&SubsystemId::Developmental));
        assert!(set.contains(&SubsystemId::EcologicalContext));
        assert!(set.contains(&SubsystemId::Interaction));
    }

    #[test]
    fn subsystem_names() {
        assert_eq!(SubsystemId::State.name(), "State");
        assert_eq!(SubsystemId::Memory.name(), "Memory");
        assert_eq!(SubsystemId::Relationship.name(), "Relationship");
        assert_eq!(SubsystemId::Event.name(), "Event");
        assert_eq!(
            SubsystemId::BehavioralDecision.name(),
            "Behavioral Decision"
        );
        assert_eq!(SubsystemId::Developmental.name(), "Developmental");
        assert_eq!(SubsystemId::EcologicalContext.name(), "Ecological Context");
        assert_eq!(SubsystemId::Interaction.name(), "Interaction");
    }

    #[test]
    fn display_format() {
        assert_eq!(format!("{}", SubsystemId::State), "State");
        assert_eq!(
            format!("{}", SubsystemId::BehavioralDecision),
            "Behavioral Decision"
        );
    }

    #[test]
    fn equality() {
        assert_eq!(SubsystemId::State, SubsystemId::State);
        assert_ne!(SubsystemId::State, SubsystemId::Memory);
    }

    #[test]
    fn hashable() {
        let mut set = HashSet::new();
        set.insert(SubsystemId::State);
        set.insert(SubsystemId::State); // Duplicate

        assert_eq!(set.len(), 1);

        set.insert(SubsystemId::Memory);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn copy_and_clone() {
        let original = SubsystemId::State;
        let copied = original; // Copy
        let cloned = original.clone(); // Clone

        assert_eq!(original, copied);
        assert_eq!(original, cloned);
    }

    #[test]
    fn debug_format() {
        let debug = format!("{:?}", SubsystemId::State);
        assert!(debug.contains("State"));
    }
}
