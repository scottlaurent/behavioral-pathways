//! Typed path enums for relationship state access.
//!
//! These enums provide compile-time safety for accessing relationship dimensions.
//! No magic strings - all paths are typed enums.

/// Direction of a relationship dimension from one entity to another.
///
/// In `Relationship.between(entity_a, entity_b)`:
/// - `AToB` = first argument's perspective on second argument (A perceives B)
/// - `BToA` = second argument's perspective on first argument (B perceives A)
///
/// # Examples
///
/// ```
/// use behavioral_pathways::enums::Direction;
///
/// let dir = Direction::AToB;
/// assert_eq!(dir.opposite(), Direction::BToA);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    /// First entity's perspective on second entity.
    AToB,
    /// Second entity's perspective on first entity.
    BToA,
}

impl Direction {
    /// Returns the opposite direction.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::enums::Direction;
    ///
    /// assert_eq!(Direction::AToB.opposite(), Direction::BToA);
    /// assert_eq!(Direction::BToA.opposite(), Direction::AToB);
    /// ```
    #[must_use]
    pub const fn opposite(&self) -> Direction {
        match self {
            Direction::AToB => Direction::BToA,
            Direction::BToA => Direction::AToB,
        }
    }

    /// Returns a human-readable name for this direction.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Direction::AToB => "A to B",
            Direction::BToA => "B to A",
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Path to a trustworthiness factor dimension.
///
/// Per Mayer's model, trustworthiness has three input components:
/// - Competence: Perceived ability to perform
/// - Benevolence: Perceived caring and good intentions
/// - Integrity: Perceived adherence to principles
///
/// Additionally, SupportWillingness provides access to the computed
/// willingness for emotional support (derived from benevolence in trust decisions).
///
/// # Examples
///
/// ```
/// use behavioral_pathways::enums::TrustPath;
///
/// let path = TrustPath::Competence;
/// assert_eq!(path.name(), "Competence");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrustPath {
    /// Perceived ability to perform tasks competently.
    Competence,
    /// Perceived caring and benevolent intentions.
    Benevolence,
    /// Perceived adherence to principles and values.
    Integrity,
    /// Computed willingness for emotional support (based on benevolence).
    /// This is a computed output from trust decisions, not a stored input.
    SupportWillingness,
}

impl TrustPath {
    /// Returns a human-readable name for this path.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            TrustPath::Competence => "Competence",
            TrustPath::Benevolence => "Benevolence",
            TrustPath::Integrity => "Integrity",
            TrustPath::SupportWillingness => "SupportWillingness",
        }
    }

    /// Returns all trust paths (input factors only, not computed outputs).
    #[must_use]
    pub const fn all() -> [TrustPath; 3] {
        [
            TrustPath::Competence,
            TrustPath::Benevolence,
            TrustPath::Integrity,
        ]
    }

    /// Returns all trust paths including computed outputs.
    #[must_use]
    pub const fn all_with_computed() -> [TrustPath; 4] {
        [
            TrustPath::Competence,
            TrustPath::Benevolence,
            TrustPath::Integrity,
            TrustPath::SupportWillingness,
        ]
    }

    /// Returns true if this path is a computed output rather than a stored input.
    #[must_use]
    pub const fn is_computed(&self) -> bool {
        matches!(self, TrustPath::SupportWillingness)
    }
}

impl std::fmt::Display for TrustPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Path to a directional relationship dimension.
///
/// Directional dimensions are asymmetric - A's perception of B may differ
/// from B's perception of A.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::enums::{DirectionalPath, TrustPath};
///
/// let path = DirectionalPath::Trust(TrustPath::Competence);
/// assert!(matches!(path, DirectionalPath::Trust(_)));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DirectionalPath {
    /// Trustworthiness factor (competence, benevolence, integrity).
    Trust(TrustPath),
    /// Warmth toward the other entity.
    Warmth,
    /// Resentment toward the other entity.
    Resentment,
    /// Dependence on the other entity.
    Dependence,
    /// Attraction to the other entity.
    Attraction,
    /// Emotional attachment and fear of loss.
    Attachment,
    /// Jealousy and possessiveness.
    Jealousy,
    /// Fear and threat perception.
    Fear,
    /// Sense of duty or obligation.
    Obligation,
    /// Perceived risk when trusting the other entity.
    PerceivedRisk,
}

impl DirectionalPath {
    /// Returns a human-readable name for this path.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            DirectionalPath::Trust(tp) => tp.name(),
            DirectionalPath::Warmth => "Warmth",
            DirectionalPath::Resentment => "Resentment",
            DirectionalPath::Dependence => "Dependence",
            DirectionalPath::Attraction => "Attraction",
            DirectionalPath::Attachment => "Attachment",
            DirectionalPath::Jealousy => "Jealousy",
            DirectionalPath::Fear => "Fear",
            DirectionalPath::Obligation => "Obligation",
            DirectionalPath::PerceivedRisk => "Perceived Risk",
        }
    }
}

impl std::fmt::Display for DirectionalPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Path to a shared relationship dimension.
///
/// Shared dimensions are symmetric - both entities perceive the same value.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::enums::SharedPath;
///
/// let path = SharedPath::Affinity;
/// assert_eq!(path.name(), "Affinity");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SharedPath {
    /// General liking between entities.
    Affinity,
    /// Mutual respect and admiration.
    Respect,
    /// Unresolved conflict and tension.
    Tension,
    /// Emotional closeness and intimacy.
    Intimacy,
    /// Depth of shared experience (monotonically increasing).
    History,
}

impl SharedPath {
    /// Returns a human-readable name for this path.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            SharedPath::Affinity => "Affinity",
            SharedPath::Respect => "Respect",
            SharedPath::Tension => "Tension",
            SharedPath::Intimacy => "Intimacy",
            SharedPath::History => "History",
        }
    }

    /// Returns all shared paths.
    #[must_use]
    pub const fn all() -> [SharedPath; 5] {
        [
            SharedPath::Affinity,
            SharedPath::Respect,
            SharedPath::Tension,
            SharedPath::Intimacy,
            SharedPath::History,
        ]
    }
}

impl std::fmt::Display for SharedPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Top-level path for accessing relationship dimensions.
///
/// Used with `Relationship.get()` to access state values.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::enums::{RelPath, SharedPath, Direction, DirectionalPath, TrustPath};
///
/// // Access shared dimension
/// let affinity_path = RelPath::Shared(SharedPath::Affinity);
///
/// // Access directional dimension
/// let trust_path = RelPath::Directional(
///     Direction::AToB,
///     DirectionalPath::Trust(TrustPath::Competence)
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelPath {
    /// Path to a shared (symmetric) dimension.
    Shared(SharedPath),
    /// Path to a directional (asymmetric) dimension.
    Directional(Direction, DirectionalPath),
    /// Path to the relationship stage.
    Stage,
}

impl RelPath {
    /// Returns a human-readable description of this path.
    #[must_use]
    pub fn description(&self) -> String {
        match self {
            RelPath::Shared(sp) => format!("Shared.{}", sp.name()),
            RelPath::Directional(dir, dp) => format!("{}.{}", dir.name(), dp.name()),
            RelPath::Stage => "Stage".to_string(),
        }
    }
}

impl std::fmt::Display for RelPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Direction tests

    #[test]
    fn direction_opposite() {
        assert_eq!(Direction::AToB.opposite(), Direction::BToA);
        assert_eq!(Direction::BToA.opposite(), Direction::AToB);
    }

    #[test]
    fn direction_opposite_is_involutive() {
        assert_eq!(Direction::AToB.opposite().opposite(), Direction::AToB);
        assert_eq!(Direction::BToA.opposite().opposite(), Direction::BToA);
    }

    #[test]
    fn direction_name() {
        assert_eq!(Direction::AToB.name(), "A to B");
        assert_eq!(Direction::BToA.name(), "B to A");
    }

    #[test]
    fn direction_display() {
        assert_eq!(format!("{}", Direction::AToB), "A to B");
        assert_eq!(format!("{}", Direction::BToA), "B to A");
    }

    #[test]
    fn direction_equality() {
        assert_eq!(Direction::AToB, Direction::AToB);
        assert_ne!(Direction::AToB, Direction::BToA);
    }

    #[test]
    fn direction_clone_copy() {
        let d1 = Direction::AToB;
        let d2 = d1;
        let d3 = d1.clone();
        assert_eq!(d1, d2);
        assert_eq!(d1, d3);
    }

    #[test]
    fn direction_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(Direction::AToB);
        set.insert(Direction::AToB);
        assert_eq!(set.len(), 1);
        set.insert(Direction::BToA);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn direction_debug() {
        let debug = format!("{:?}", Direction::AToB);
        assert!(debug.contains("AToB"));
    }

    // TrustPath tests

    #[test]
    fn trust_path_name() {
        assert_eq!(TrustPath::Competence.name(), "Competence");
        assert_eq!(TrustPath::Benevolence.name(), "Benevolence");
        assert_eq!(TrustPath::Integrity.name(), "Integrity");
        assert_eq!(TrustPath::SupportWillingness.name(), "SupportWillingness");
    }

    #[test]
    fn trust_path_all() {
        let all = TrustPath::all();
        assert_eq!(all.len(), 3);
        assert!(all.contains(&TrustPath::Competence));
        assert!(all.contains(&TrustPath::Benevolence));
        assert!(all.contains(&TrustPath::Integrity));
        // SupportWillingness is computed, not in all()
        assert!(!all.contains(&TrustPath::SupportWillingness));
    }

    #[test]
    fn trust_path_all_with_computed() {
        let all = TrustPath::all_with_computed();
        assert_eq!(all.len(), 4);
        assert!(all.contains(&TrustPath::Competence));
        assert!(all.contains(&TrustPath::Benevolence));
        assert!(all.contains(&TrustPath::Integrity));
        assert!(all.contains(&TrustPath::SupportWillingness));
    }

    #[test]
    fn trust_path_is_computed() {
        assert!(!TrustPath::Competence.is_computed());
        assert!(!TrustPath::Benevolence.is_computed());
        assert!(!TrustPath::Integrity.is_computed());
        assert!(TrustPath::SupportWillingness.is_computed());
    }

    #[test]
    fn trust_path_display() {
        assert_eq!(format!("{}", TrustPath::Competence), "Competence");
    }

    #[test]
    fn trust_path_equality() {
        assert_eq!(TrustPath::Competence, TrustPath::Competence);
        assert_ne!(TrustPath::Competence, TrustPath::Benevolence);
    }

    #[test]
    fn trust_path_clone_copy() {
        let t1 = TrustPath::Integrity;
        let t2 = t1;
        let t3 = t1.clone();
        assert_eq!(t1, t2);
        assert_eq!(t1, t3);
    }

    #[test]
    fn trust_path_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(TrustPath::Competence);
        set.insert(TrustPath::Competence);
        assert_eq!(set.len(), 1);
        set.insert(TrustPath::Benevolence);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn trust_path_debug() {
        let debug = format!("{:?}", TrustPath::Integrity);
        assert!(debug.contains("Integrity"));
    }

    // DirectionalPath tests

    #[test]
    fn directional_path_name() {
        assert_eq!(DirectionalPath::Warmth.name(), "Warmth");
        assert_eq!(DirectionalPath::Resentment.name(), "Resentment");
        assert_eq!(DirectionalPath::Dependence.name(), "Dependence");
        assert_eq!(DirectionalPath::Attraction.name(), "Attraction");
        assert_eq!(DirectionalPath::Attachment.name(), "Attachment");
        assert_eq!(DirectionalPath::Jealousy.name(), "Jealousy");
        assert_eq!(DirectionalPath::Fear.name(), "Fear");
        assert_eq!(DirectionalPath::Obligation.name(), "Obligation");
        assert_eq!(DirectionalPath::PerceivedRisk.name(), "Perceived Risk");
    }

    #[test]
    fn directional_path_trust_name() {
        let path = DirectionalPath::Trust(TrustPath::Competence);
        assert_eq!(path.name(), "Competence");
    }

    #[test]
    fn directional_path_display() {
        assert_eq!(format!("{}", DirectionalPath::Warmth), "Warmth");
        assert_eq!(
            format!("{}", DirectionalPath::Trust(TrustPath::Benevolence)),
            "Benevolence"
        );
    }

    #[test]
    fn directional_path_equality() {
        assert_eq!(DirectionalPath::Warmth, DirectionalPath::Warmth);
        assert_ne!(DirectionalPath::Warmth, DirectionalPath::Fear);
        assert_eq!(
            DirectionalPath::Trust(TrustPath::Competence),
            DirectionalPath::Trust(TrustPath::Competence)
        );
        assert_ne!(
            DirectionalPath::Trust(TrustPath::Competence),
            DirectionalPath::Trust(TrustPath::Benevolence)
        );
    }

    #[test]
    fn directional_path_clone_copy() {
        let d1 = DirectionalPath::Attachment;
        let d2 = d1;
        let d3 = d1.clone();
        assert_eq!(d1, d2);
        assert_eq!(d1, d3);
    }

    #[test]
    fn directional_path_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(DirectionalPath::Warmth);
        set.insert(DirectionalPath::Warmth);
        assert_eq!(set.len(), 1);
        set.insert(DirectionalPath::Trust(TrustPath::Competence));
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn directional_path_debug() {
        let debug = format!("{:?}", DirectionalPath::Jealousy);
        assert!(debug.contains("Jealousy"));
    }

    // SharedPath tests

    #[test]
    fn shared_path_name() {
        assert_eq!(SharedPath::Affinity.name(), "Affinity");
        assert_eq!(SharedPath::Respect.name(), "Respect");
        assert_eq!(SharedPath::Tension.name(), "Tension");
        assert_eq!(SharedPath::Intimacy.name(), "Intimacy");
        assert_eq!(SharedPath::History.name(), "History");
    }

    #[test]
    fn shared_path_all() {
        let all = SharedPath::all();
        assert_eq!(all.len(), 5);
        assert!(all.contains(&SharedPath::Affinity));
        assert!(all.contains(&SharedPath::Respect));
        assert!(all.contains(&SharedPath::Tension));
        assert!(all.contains(&SharedPath::Intimacy));
        assert!(all.contains(&SharedPath::History));
    }

    #[test]
    fn shared_path_display() {
        assert_eq!(format!("{}", SharedPath::Affinity), "Affinity");
    }

    #[test]
    fn shared_path_equality() {
        assert_eq!(SharedPath::Affinity, SharedPath::Affinity);
        assert_ne!(SharedPath::Affinity, SharedPath::Tension);
    }

    #[test]
    fn shared_path_clone_copy() {
        let s1 = SharedPath::Intimacy;
        let s2 = s1;
        let s3 = s1.clone();
        assert_eq!(s1, s2);
        assert_eq!(s1, s3);
    }

    #[test]
    fn shared_path_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(SharedPath::Affinity);
        set.insert(SharedPath::Affinity);
        assert_eq!(set.len(), 1);
        set.insert(SharedPath::Tension);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn shared_path_debug() {
        let debug = format!("{:?}", SharedPath::History);
        assert!(debug.contains("History"));
    }

    // RelPath tests

    #[test]
    fn rel_path_shared_description() {
        let path = RelPath::Shared(SharedPath::Affinity);
        assert_eq!(path.description(), "Shared.Affinity");
    }

    #[test]
    fn rel_path_directional_description() {
        let path = RelPath::Directional(Direction::AToB, DirectionalPath::Warmth);
        assert_eq!(path.description(), "A to B.Warmth");
    }

    #[test]
    fn rel_path_directional_trust_description() {
        let path = RelPath::Directional(
            Direction::BToA,
            DirectionalPath::Trust(TrustPath::Integrity),
        );
        assert_eq!(path.description(), "B to A.Integrity");
    }

    #[test]
    fn rel_path_stage_description() {
        let path = RelPath::Stage;
        assert_eq!(path.description(), "Stage");
    }

    #[test]
    fn rel_path_display() {
        let path = RelPath::Shared(SharedPath::Tension);
        assert_eq!(format!("{}", path), "Shared.Tension");
    }

    #[test]
    fn rel_path_equality() {
        assert_eq!(
            RelPath::Shared(SharedPath::Affinity),
            RelPath::Shared(SharedPath::Affinity)
        );
        assert_ne!(
            RelPath::Shared(SharedPath::Affinity),
            RelPath::Shared(SharedPath::Tension)
        );
        assert_ne!(RelPath::Shared(SharedPath::Affinity), RelPath::Stage);
    }

    #[test]
    fn rel_path_clone_copy() {
        let r1 = RelPath::Stage;
        let r2 = r1;
        let r3 = r1.clone();
        assert_eq!(r1, r2);
        assert_eq!(r1, r3);
    }

    #[test]
    fn rel_path_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(RelPath::Stage);
        set.insert(RelPath::Stage);
        assert_eq!(set.len(), 1);
        set.insert(RelPath::Shared(SharedPath::Affinity));
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn rel_path_debug() {
        let debug = format!("{:?}", RelPath::Stage);
        assert!(debug.contains("Stage"));
    }
}
