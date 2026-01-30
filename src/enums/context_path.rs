//! Typed context access paths for compile-time validated context queries.
//!
//! These enums provide type-safe paths for accessing ecological context
//! dimensions without using magic strings. They follow the same pattern
//! as [`StatePath`] for state access.
//!
//! # Examples
//!
//! ```
//! use behavioral_pathways::enums::{
//!     ContextPath, MacrosystemPath, MicrosystemPath, WorkPath
//! };
//! use behavioral_pathways::types::MicrosystemId;
//!
//! // Type-safe context path construction
//! let macro_path = ContextPath::Macrosystem(MacrosystemPath::PowerDistance);
//!
//! // Microsystem paths require an ID to identify which instance
//! let work_id = MicrosystemId::new("work_acme").unwrap();
//! let work_path = ContextPath::Microsystem(work_id, MicrosystemPath::Work(WorkPath::WorkloadStress));
//! ```
//!
//! [`StatePath`]: crate::enums::StatePath

use crate::types::MicrosystemId;

/// Top-level context access path.
///
/// This is the root enum for accessing any ecological context dimension.
/// Use this when you need to specify a path to any part of an entity's context.
///
/// # Multi-Instance Access
///
/// Microsystem paths require a `MicrosystemId` because entities can have
/// multiple microsystems of the same type (e.g., two jobs). Single-instance
/// contexts (Macrosystem, Exosystem, Chronosystem) do not require an ID.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContextPath {
    /// Path to a microsystem dimension.
    /// Requires MicrosystemId to identify which instance.
    Microsystem(MicrosystemId, MicrosystemPath),

    /// Path to an exosystem dimension.
    Exosystem(ExosystemPath),

    /// Path to a macrosystem dimension.
    Macrosystem(MacrosystemPath),

    /// Path to a chronosystem dimension.
    Chronosystem(ChronosystemPath),
}

/// Path to microsystem context dimensions.
///
/// Microsystems are immediate environments with face-to-face interactions.
/// Each entity can have multiple microsystems of each type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MicrosystemPath {
    /// Work context dimensions.
    Work(WorkPath),

    /// Family context dimensions.
    Family(FamilyPath),

    /// Social context dimensions.
    Social(SocialPath),

    /// Educational context dimensions.
    Education(EducationPath),

    /// Healthcare context dimensions.
    Healthcare(HealthcarePath),

    /// Religious context dimensions.
    Religious(ReligiousPath),

    /// Neighborhood context dimensions.
    Neighborhood(NeighborhoodPath),
}

/// Path to work context dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorkPath {
    /// Current work pressure (0-1).
    WorkloadStress,

    /// Satisfaction with job role (0-1).
    RoleSatisfaction,

    /// Emotional supportiveness of workplace (0-1).
    Warmth,

    /// Degree of antagonism/conflict (0-1).
    Hostility,

    /// Clear expectations and responsibilities (0-1).
    RoleClarity,

    /// Routine and consistent environment (0-1).
    Predictability,

    /// Mental engagement and challenge (0-1).
    CognitiveStimulation,

    /// Security and constancy of position (0-1).
    Stability,

    /// Interaction frequency normalized (0-1).
    InteractionFrequency,

    /// Progressive complexity of shared tasks (0-1).
    InteractionComplexity,
}

/// Path to family context dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FamilyPath {
    /// Overall family satisfaction (0-1).
    FamilySatisfaction,

    /// Caregiving burden (0-1).
    CaregivingBurden,

    /// Emotional warmth (0-1).
    Warmth,

    /// Active hostility (0-1).
    Hostility,

    /// Clear family role expectations (0-1).
    RoleClarity,

    /// Consistent family routines (0-1).
    Predictability,

    /// Family stability (0-1).
    Stability,

    /// Interaction frequency normalized (0-1).
    InteractionFrequency,

    /// Progressive complexity of shared tasks (0-1).
    InteractionComplexity,
}

/// Path to social context dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SocialPath {
    /// Standing in social group (0-1).
    GroupStanding,

    /// Supportiveness of social circle (0-1).
    Warmth,

    /// Gossip, exclusion, rivalry (0-1).
    Hostility,

    /// Reliability of social connections (0-1).
    Predictability,

    /// Interaction frequency normalized (0-1).
    InteractionFrequency,

    /// Progressive complexity of shared tasks (0-1).
    InteractionComplexity,
}

/// Path to education context dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EducationPath {
    /// Intellectual challenge level (0-1).
    CognitiveDemand,

    /// Tutoring, mentorship availability (0-1).
    CompetenceSupport,

    /// Emotional support (0-1).
    Warmth,

    /// Active antagonism (0-1).
    Hostility,

    /// Interaction frequency normalized (0-1).
    InteractionFrequency,

    /// Progressive complexity of shared tasks (0-1).
    InteractionComplexity,
}

/// Path to healthcare context dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HealthcarePath {
    /// Healthcare access frequency (0-1).
    AccessFrequency,

    /// Provider responsiveness (0-1).
    Responsiveness,

    /// Provider warmth (0-1).
    Warmth,

    /// Provider hostility (0-1).
    Hostility,

    /// Interaction frequency normalized (0-1).
    InteractionFrequency,

    /// Progressive complexity of shared tasks (0-1).
    InteractionComplexity,
}

/// Path to religious context dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReligiousPath {
    /// Ritual participation frequency (0-1).
    RitualFrequency,

    /// Community warmth (0-1).
    Warmth,

    /// Community hostility (0-1).
    Hostility,

    /// Interaction frequency normalized (0-1).
    InteractionFrequency,

    /// Progressive complexity of shared tasks (0-1).
    InteractionComplexity,
}

/// Path to neighborhood context dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NeighborhoodPath {
    /// Neighborhood safety (0-1).
    Safety,

    /// Community cohesion (0-1).
    Cohesion,

    /// Neighbor warmth (0-1).
    Warmth,

    /// Neighbor hostility (0-1).
    Hostility,

    /// Interaction frequency normalized (0-1).
    InteractionFrequency,

    /// Progressive complexity of shared tasks (0-1).
    InteractionComplexity,
}

/// Path to exosystem context dimensions.
///
/// Exosystem settings affect the individual indirectly through
/// their influence on microsystems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExosystemPath {
    /// Quality of healthcare access (0-1).
    HealthSystemAccess,

    /// Quality of education available (0-1).
    EducationalSystemQuality,

    /// Access to community resources (0-1).
    CommunityServicesAvailability,

    /// General resource access (0-1).
    ResourceAvailability,

    /// Support from institutions (0-1).
    InstitutionalSupport,

    /// Parent work stress level (0-1) - for children.
    ParentWorkStress,

    /// Parent schedule flexibility (0-1) - for children.
    ParentScheduleFlexibility,

    /// Parent income stability (0-1) - for children.
    ParentIncomeStability,
}

/// Path to macrosystem context dimensions.
///
/// Macrosystem represents overarching cultural, societal, and
/// ideological patterns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MacrosystemPath {
    /// Individualism vs collectivism (-1 collectivist to 1 individualist).
    IndividualismCollectivism,

    /// Acceptance of hierarchy (0-1).
    PowerDistance,

    /// Discomfort with ambiguity (0-1).
    UncertaintyAvoidance,

    /// Society-level stress (0-1).
    CulturalStress,

    /// Shared historical trauma (0-1).
    CollectiveTrauma,

    /// Wealth disparity level (0-1).
    EconomicInequality,

    /// Legal system reliability (0-1).
    RuleOfLaw,

    /// Ability to change status (0-1).
    SocialMobility,

    /// Institutional corruption (0-1).
    CorruptionLevel,
}

/// Path to chronosystem context dimensions.
///
/// Chronosystem represents the temporal dimension including
/// life transitions and historical events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChronosystemPath {
    /// Societal stability level (0-1).
    StabilityLevel,

    /// Resource scarcity (0-1).
    ResourceScarcity,

    /// Trust in institutions (0-1).
    InstitutionalTrust,

    /// Current plasticity modifier (0-1).
    PlasticityModifier,
}

// Implement name and all() methods for each path enum

impl WorkPath {
    /// Returns all WorkPath variants.
    #[must_use]
    pub const fn all() -> [WorkPath; 10] {
        [
            WorkPath::WorkloadStress,
            WorkPath::RoleSatisfaction,
            WorkPath::Warmth,
            WorkPath::Hostility,
            WorkPath::RoleClarity,
            WorkPath::Predictability,
            WorkPath::CognitiveStimulation,
            WorkPath::Stability,
            WorkPath::InteractionFrequency,
            WorkPath::InteractionComplexity,
        ]
    }

    /// Returns a human-readable name for this path.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            WorkPath::WorkloadStress => "Workload Stress",
            WorkPath::RoleSatisfaction => "Role Satisfaction",
            WorkPath::Warmth => "Warmth",
            WorkPath::Hostility => "Hostility",
            WorkPath::RoleClarity => "Role Clarity",
            WorkPath::Predictability => "Predictability",
            WorkPath::CognitiveStimulation => "Cognitive Stimulation",
            WorkPath::Stability => "Stability",
            WorkPath::InteractionFrequency => "Interaction Frequency",
            WorkPath::InteractionComplexity => "Interaction Complexity",
        }
    }
}

impl FamilyPath {
    /// Returns all FamilyPath variants.
    #[must_use]
    pub const fn all() -> [FamilyPath; 9] {
        [
            FamilyPath::FamilySatisfaction,
            FamilyPath::CaregivingBurden,
            FamilyPath::Warmth,
            FamilyPath::Hostility,
            FamilyPath::RoleClarity,
            FamilyPath::Predictability,
            FamilyPath::Stability,
            FamilyPath::InteractionFrequency,
            FamilyPath::InteractionComplexity,
        ]
    }

    /// Returns a human-readable name for this path.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            FamilyPath::FamilySatisfaction => "Family Satisfaction",
            FamilyPath::CaregivingBurden => "Caregiving Burden",
            FamilyPath::Warmth => "Warmth",
            FamilyPath::Hostility => "Hostility",
            FamilyPath::RoleClarity => "Role Clarity",
            FamilyPath::Predictability => "Predictability",
            FamilyPath::Stability => "Stability",
            FamilyPath::InteractionFrequency => "Interaction Frequency",
            FamilyPath::InteractionComplexity => "Interaction Complexity",
        }
    }
}

impl SocialPath {
    /// Returns all SocialPath variants.
    #[must_use]
    pub const fn all() -> [SocialPath; 6] {
        [
            SocialPath::GroupStanding,
            SocialPath::Warmth,
            SocialPath::Hostility,
            SocialPath::Predictability,
            SocialPath::InteractionFrequency,
            SocialPath::InteractionComplexity,
        ]
    }

    /// Returns a human-readable name for this path.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            SocialPath::GroupStanding => "Group Standing",
            SocialPath::Warmth => "Warmth",
            SocialPath::Hostility => "Hostility",
            SocialPath::Predictability => "Predictability",
            SocialPath::InteractionFrequency => "Interaction Frequency",
            SocialPath::InteractionComplexity => "Interaction Complexity",
        }
    }
}

impl EducationPath {
    /// Returns all EducationPath variants.
    #[must_use]
    pub const fn all() -> [EducationPath; 6] {
        [
            EducationPath::CognitiveDemand,
            EducationPath::CompetenceSupport,
            EducationPath::Warmth,
            EducationPath::Hostility,
            EducationPath::InteractionFrequency,
            EducationPath::InteractionComplexity,
        ]
    }

    /// Returns a human-readable name for this path.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            EducationPath::CognitiveDemand => "Cognitive Demand",
            EducationPath::CompetenceSupport => "Competence Support",
            EducationPath::Warmth => "Warmth",
            EducationPath::Hostility => "Hostility",
            EducationPath::InteractionFrequency => "Interaction Frequency",
            EducationPath::InteractionComplexity => "Interaction Complexity",
        }
    }
}

impl HealthcarePath {
    /// Returns all HealthcarePath variants.
    #[must_use]
    pub const fn all() -> [HealthcarePath; 6] {
        [
            HealthcarePath::AccessFrequency,
            HealthcarePath::Responsiveness,
            HealthcarePath::Warmth,
            HealthcarePath::Hostility,
            HealthcarePath::InteractionFrequency,
            HealthcarePath::InteractionComplexity,
        ]
    }

    /// Returns a human-readable name for this path.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            HealthcarePath::AccessFrequency => "Access Frequency",
            HealthcarePath::Responsiveness => "Responsiveness",
            HealthcarePath::Warmth => "Warmth",
            HealthcarePath::Hostility => "Hostility",
            HealthcarePath::InteractionFrequency => "Interaction Frequency",
            HealthcarePath::InteractionComplexity => "Interaction Complexity",
        }
    }
}

impl ReligiousPath {
    /// Returns all ReligiousPath variants.
    #[must_use]
    pub const fn all() -> [ReligiousPath; 5] {
        [
            ReligiousPath::RitualFrequency,
            ReligiousPath::Warmth,
            ReligiousPath::Hostility,
            ReligiousPath::InteractionFrequency,
            ReligiousPath::InteractionComplexity,
        ]
    }

    /// Returns a human-readable name for this path.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            ReligiousPath::RitualFrequency => "Ritual Frequency",
            ReligiousPath::Warmth => "Warmth",
            ReligiousPath::Hostility => "Hostility",
            ReligiousPath::InteractionFrequency => "Interaction Frequency",
            ReligiousPath::InteractionComplexity => "Interaction Complexity",
        }
    }
}

impl NeighborhoodPath {
    /// Returns all NeighborhoodPath variants.
    #[must_use]
    pub const fn all() -> [NeighborhoodPath; 6] {
        [
            NeighborhoodPath::Safety,
            NeighborhoodPath::Cohesion,
            NeighborhoodPath::Warmth,
            NeighborhoodPath::Hostility,
            NeighborhoodPath::InteractionFrequency,
            NeighborhoodPath::InteractionComplexity,
        ]
    }

    /// Returns a human-readable name for this path.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            NeighborhoodPath::Safety => "Safety",
            NeighborhoodPath::Cohesion => "Cohesion",
            NeighborhoodPath::Warmth => "Warmth",
            NeighborhoodPath::Hostility => "Hostility",
            NeighborhoodPath::InteractionFrequency => "Interaction Frequency",
            NeighborhoodPath::InteractionComplexity => "Interaction Complexity",
        }
    }
}

impl MicrosystemPath {
    /// Returns a human-readable name for this path.
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            MicrosystemPath::Work(p) => p.name(),
            MicrosystemPath::Family(p) => p.name(),
            MicrosystemPath::Social(p) => p.name(),
            MicrosystemPath::Education(p) => p.name(),
            MicrosystemPath::Healthcare(p) => p.name(),
            MicrosystemPath::Religious(p) => p.name(),
            MicrosystemPath::Neighborhood(p) => p.name(),
        }
    }

    /// Returns the microsystem type name.
    #[must_use]
    pub const fn type_name(&self) -> &'static str {
        match self {
            MicrosystemPath::Work(_) => "Work",
            MicrosystemPath::Family(_) => "Family",
            MicrosystemPath::Social(_) => "Social",
            MicrosystemPath::Education(_) => "Education",
            MicrosystemPath::Healthcare(_) => "Healthcare",
            MicrosystemPath::Religious(_) => "Religious",
            MicrosystemPath::Neighborhood(_) => "Neighborhood",
        }
    }
}

impl ExosystemPath {
    /// Returns all ExosystemPath variants.
    #[must_use]
    pub const fn all() -> [ExosystemPath; 8] {
        [
            ExosystemPath::HealthSystemAccess,
            ExosystemPath::EducationalSystemQuality,
            ExosystemPath::CommunityServicesAvailability,
            ExosystemPath::ResourceAvailability,
            ExosystemPath::InstitutionalSupport,
            ExosystemPath::ParentWorkStress,
            ExosystemPath::ParentScheduleFlexibility,
            ExosystemPath::ParentIncomeStability,
        ]
    }

    /// Returns a human-readable name for this path.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            ExosystemPath::HealthSystemAccess => "Health System Access",
            ExosystemPath::EducationalSystemQuality => "Educational System Quality",
            ExosystemPath::CommunityServicesAvailability => "Community Services Availability",
            ExosystemPath::ResourceAvailability => "Resource Availability",
            ExosystemPath::InstitutionalSupport => "Institutional Support",
            ExosystemPath::ParentWorkStress => "Parent Work Stress",
            ExosystemPath::ParentScheduleFlexibility => "Parent Schedule Flexibility",
            ExosystemPath::ParentIncomeStability => "Parent Income Stability",
        }
    }
}

impl MacrosystemPath {
    /// Returns all MacrosystemPath variants.
    #[must_use]
    pub const fn all() -> [MacrosystemPath; 9] {
        [
            MacrosystemPath::IndividualismCollectivism,
            MacrosystemPath::PowerDistance,
            MacrosystemPath::UncertaintyAvoidance,
            MacrosystemPath::CulturalStress,
            MacrosystemPath::CollectiveTrauma,
            MacrosystemPath::EconomicInequality,
            MacrosystemPath::RuleOfLaw,
            MacrosystemPath::SocialMobility,
            MacrosystemPath::CorruptionLevel,
        ]
    }

    /// Returns a human-readable name for this path.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            MacrosystemPath::IndividualismCollectivism => "Individualism-Collectivism",
            MacrosystemPath::PowerDistance => "Power Distance",
            MacrosystemPath::UncertaintyAvoidance => "Uncertainty Avoidance",
            MacrosystemPath::CulturalStress => "Cultural Stress",
            MacrosystemPath::CollectiveTrauma => "Collective Trauma",
            MacrosystemPath::EconomicInequality => "Economic Inequality",
            MacrosystemPath::RuleOfLaw => "Rule of Law",
            MacrosystemPath::SocialMobility => "Social Mobility",
            MacrosystemPath::CorruptionLevel => "Corruption Level",
        }
    }
}

impl ChronosystemPath {
    /// Returns all ChronosystemPath variants.
    #[must_use]
    pub const fn all() -> [ChronosystemPath; 4] {
        [
            ChronosystemPath::StabilityLevel,
            ChronosystemPath::ResourceScarcity,
            ChronosystemPath::InstitutionalTrust,
            ChronosystemPath::PlasticityModifier,
        ]
    }

    /// Returns a human-readable name for this path.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            ChronosystemPath::StabilityLevel => "Stability Level",
            ChronosystemPath::ResourceScarcity => "Resource Scarcity",
            ChronosystemPath::InstitutionalTrust => "Institutional Trust",
            ChronosystemPath::PlasticityModifier => "Plasticity Modifier",
        }
    }
}

impl std::fmt::Display for ContextPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextPath::Microsystem(id, path) => {
                write!(
                    f,
                    "Microsystem[{}]::{}::{}",
                    id,
                    path.type_name(),
                    path.name()
                )
            }
            ContextPath::Exosystem(p) => write!(f, "Exosystem::{}", p.name()),
            ContextPath::Macrosystem(p) => write!(f, "Macrosystem::{}", p.name()),
            ContextPath::Chronosystem(p) => write!(f, "Chronosystem::{}", p.name()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_path_macrosystem_variants_exist() {
        let _ = MacrosystemPath::PowerDistance;
        let _ = MacrosystemPath::IndividualismCollectivism;
        let _ = MacrosystemPath::UncertaintyAvoidance;
        let _ = ContextPath::Macrosystem(MacrosystemPath::PowerDistance);
    }

    #[test]
    fn context_path_exosystem_variants_exist() {
        let _ = ExosystemPath::ResourceAvailability;
        let _ = ExosystemPath::InstitutionalSupport;
        let _ = ContextPath::Exosystem(ExosystemPath::ResourceAvailability);
    }

    #[test]
    fn context_path_microsystem_with_id() {
        let id = MicrosystemId::new("work_acme").unwrap();
        let path = ContextPath::Microsystem(id, MicrosystemPath::Work(WorkPath::WorkloadStress));
        let _ = path;
    }

    #[test]
    fn context_path_chronosystem_variants_exist() {
        let _ = ChronosystemPath::StabilityLevel;
        let _ = ChronosystemPath::ResourceScarcity;
        let _ = ContextPath::Chronosystem(ChronosystemPath::StabilityLevel);
    }

    #[test]
    fn work_path_all() {
        let all = WorkPath::all();
        assert_eq!(all.len(), 10);
    }

    #[test]
    fn family_path_all() {
        let all = FamilyPath::all();
        assert_eq!(all.len(), 9);
    }

    #[test]
    fn social_path_all() {
        let all = SocialPath::all();
        assert_eq!(all.len(), 6);
    }

    #[test]
    fn education_path_all() {
        let all = EducationPath::all();
        assert_eq!(all.len(), 6);
    }

    #[test]
    fn healthcare_path_all() {
        let all = HealthcarePath::all();
        assert_eq!(all.len(), 6);
    }

    #[test]
    fn religious_path_all() {
        let all = ReligiousPath::all();
        assert_eq!(all.len(), 5);
    }

    #[test]
    fn neighborhood_path_all() {
        let all = NeighborhoodPath::all();
        assert_eq!(all.len(), 6);
    }

    #[test]
    fn exosystem_path_all() {
        let all = ExosystemPath::all();
        assert_eq!(all.len(), 8);
    }

    #[test]
    fn macrosystem_path_all() {
        let all = MacrosystemPath::all();
        assert_eq!(all.len(), 9);
    }

    #[test]
    fn chronosystem_path_all() {
        let all = ChronosystemPath::all();
        assert_eq!(all.len(), 4);
    }

    #[test]
    fn work_path_names() {
        for p in WorkPath::all() {
            assert!(!p.name().is_empty());
        }
    }

    #[test]
    fn family_path_names() {
        for p in FamilyPath::all() {
            assert!(!p.name().is_empty());
        }
    }

    #[test]
    fn social_path_names() {
        for p in SocialPath::all() {
            assert!(!p.name().is_empty());
        }
    }

    #[test]
    fn education_path_names() {
        for p in EducationPath::all() {
            assert!(!p.name().is_empty());
        }
    }

    #[test]
    fn healthcare_path_names() {
        for p in HealthcarePath::all() {
            assert!(!p.name().is_empty());
        }
    }

    #[test]
    fn religious_path_names() {
        for p in ReligiousPath::all() {
            assert!(!p.name().is_empty());
        }
    }

    #[test]
    fn neighborhood_path_names() {
        for p in NeighborhoodPath::all() {
            assert!(!p.name().is_empty());
        }
    }

    #[test]
    fn exosystem_path_names() {
        for p in ExosystemPath::all() {
            assert!(!p.name().is_empty());
        }
    }

    #[test]
    fn macrosystem_path_names() {
        for p in MacrosystemPath::all() {
            assert!(!p.name().is_empty());
        }
    }

    #[test]
    fn chronosystem_path_names() {
        for p in ChronosystemPath::all() {
            assert!(!p.name().is_empty());
        }
    }

    #[test]
    fn microsystem_path_type_names() {
        assert_eq!(MicrosystemPath::Work(WorkPath::Warmth).type_name(), "Work");
        assert_eq!(
            MicrosystemPath::Family(FamilyPath::Warmth).type_name(),
            "Family"
        );
        assert_eq!(
            MicrosystemPath::Social(SocialPath::Warmth).type_name(),
            "Social"
        );
        assert_eq!(
            MicrosystemPath::Education(EducationPath::Warmth).type_name(),
            "Education"
        );
        assert_eq!(
            MicrosystemPath::Healthcare(HealthcarePath::Warmth).type_name(),
            "Healthcare"
        );
        assert_eq!(
            MicrosystemPath::Religious(ReligiousPath::Warmth).type_name(),
            "Religious"
        );
        assert_eq!(
            MicrosystemPath::Neighborhood(NeighborhoodPath::Warmth).type_name(),
            "Neighborhood"
        );
    }

    #[test]
    fn microsystem_path_names() {
        assert_eq!(MicrosystemPath::Work(WorkPath::Warmth).name(), "Warmth");
        assert_eq!(MicrosystemPath::Family(FamilyPath::Warmth).name(), "Warmth");
        assert_eq!(MicrosystemPath::Social(SocialPath::Warmth).name(), "Warmth");
        assert_eq!(
            MicrosystemPath::Education(EducationPath::Warmth).name(),
            "Warmth"
        );
        assert_eq!(
            MicrosystemPath::Healthcare(HealthcarePath::Warmth).name(),
            "Warmth"
        );
        assert_eq!(
            MicrosystemPath::Religious(ReligiousPath::Warmth).name(),
            "Warmth"
        );
        assert_eq!(
            MicrosystemPath::Neighborhood(NeighborhoodPath::Warmth).name(),
            "Warmth"
        );
    }

    #[test]
    fn context_path_display() {
        let id = MicrosystemId::new("work_acme").unwrap();
        let micro_path =
            ContextPath::Microsystem(id, MicrosystemPath::Work(WorkPath::WorkloadStress));
        let display = format!("{}", micro_path);
        assert!(display.contains("Microsystem"));
        assert!(display.contains("work_acme"));
        assert!(display.contains("Work"));
        assert!(display.contains("Workload Stress"));

        let macro_path = ContextPath::Macrosystem(MacrosystemPath::PowerDistance);
        let display = format!("{}", macro_path);
        assert!(display.contains("Macrosystem"));
        assert!(display.contains("Power Distance"));

        let exo_path = ContextPath::Exosystem(ExosystemPath::ResourceAvailability);
        let display = format!("{}", exo_path);
        assert!(display.contains("Exosystem"));
        assert!(display.contains("Resource Availability"));

        let chrono_path = ContextPath::Chronosystem(ChronosystemPath::StabilityLevel);
        let display = format!("{}", chrono_path);
        assert!(display.contains("Chronosystem"));
        assert!(display.contains("Stability Level"));
    }

    #[test]
    fn paths_are_copy_except_context_path() {
        // MicrosystemPath and leaf paths are Copy
        let p1 = WorkPath::Warmth;
        let p2 = p1;
        assert_eq!(p1, p2);

        let mp1 = MacrosystemPath::PowerDistance;
        let mp2 = mp1;
        assert_eq!(mp1, mp2);
    }

    #[test]
    fn context_path_clone() {
        let id = MicrosystemId::new("work_acme").unwrap();
        let path = ContextPath::Microsystem(id, MicrosystemPath::Work(WorkPath::WorkloadStress));
        let cloned = path.clone();
        assert_eq!(path, cloned);
    }

    #[test]
    fn paths_are_hashable() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(MacrosystemPath::PowerDistance);
        set.insert(MacrosystemPath::PowerDistance);
        assert_eq!(set.len(), 1);

        set.insert(MacrosystemPath::CulturalStress);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn context_path_hashable() {
        use std::collections::HashSet;

        let id = MicrosystemId::new("work_acme").unwrap();
        let path1 =
            ContextPath::Microsystem(id.clone(), MicrosystemPath::Work(WorkPath::WorkloadStress));
        let path2 = ContextPath::Microsystem(id, MicrosystemPath::Work(WorkPath::WorkloadStress));

        let mut set = HashSet::new();
        set.insert(path1.clone());
        set.insert(path2);
        assert_eq!(set.len(), 1);

        let path3 = ContextPath::Macrosystem(MacrosystemPath::PowerDistance);
        set.insert(path3);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn debug_format() {
        let path = MacrosystemPath::PowerDistance;
        let debug = format!("{:?}", path);
        assert!(debug.contains("PowerDistance"));

        let id = MicrosystemId::new("work_acme").unwrap();
        let context_path = ContextPath::Microsystem(id, MicrosystemPath::Work(WorkPath::Warmth));
        let debug = format!("{:?}", context_path);
        assert!(debug.contains("Microsystem"));
    }

    #[test]
    fn equality() {
        assert_eq!(
            MacrosystemPath::PowerDistance,
            MacrosystemPath::PowerDistance
        );
        assert_ne!(
            MacrosystemPath::PowerDistance,
            MacrosystemPath::CulturalStress
        );

        let id = MicrosystemId::new("work_acme").unwrap();
        let path1 = ContextPath::Microsystem(id.clone(), MicrosystemPath::Work(WorkPath::Warmth));
        let path2 = ContextPath::Microsystem(id, MicrosystemPath::Work(WorkPath::Warmth));
        assert_eq!(path1, path2);
    }

    #[test]
    fn all_work_path_names() {
        for p in WorkPath::all() {
            match p {
                WorkPath::WorkloadStress => assert_eq!(p.name(), "Workload Stress"),
                WorkPath::RoleSatisfaction => assert_eq!(p.name(), "Role Satisfaction"),
                WorkPath::Warmth => assert_eq!(p.name(), "Warmth"),
                WorkPath::Hostility => assert_eq!(p.name(), "Hostility"),
                WorkPath::RoleClarity => assert_eq!(p.name(), "Role Clarity"),
                WorkPath::Predictability => assert_eq!(p.name(), "Predictability"),
                WorkPath::CognitiveStimulation => assert_eq!(p.name(), "Cognitive Stimulation"),
                WorkPath::Stability => assert_eq!(p.name(), "Stability"),
                WorkPath::InteractionFrequency => assert_eq!(p.name(), "Interaction Frequency"),
                WorkPath::InteractionComplexity => assert_eq!(p.name(), "Interaction Complexity"),
            }
        }
    }

    #[test]
    fn all_family_path_names() {
        for p in FamilyPath::all() {
            match p {
                FamilyPath::FamilySatisfaction => assert_eq!(p.name(), "Family Satisfaction"),
                FamilyPath::CaregivingBurden => assert_eq!(p.name(), "Caregiving Burden"),
                FamilyPath::Warmth => assert_eq!(p.name(), "Warmth"),
                FamilyPath::Hostility => assert_eq!(p.name(), "Hostility"),
                FamilyPath::RoleClarity => assert_eq!(p.name(), "Role Clarity"),
                FamilyPath::Predictability => assert_eq!(p.name(), "Predictability"),
                FamilyPath::Stability => assert_eq!(p.name(), "Stability"),
                FamilyPath::InteractionFrequency => assert_eq!(p.name(), "Interaction Frequency"),
                FamilyPath::InteractionComplexity => assert_eq!(p.name(), "Interaction Complexity"),
            }
        }
    }

    #[test]
    fn all_social_path_names() {
        for p in SocialPath::all() {
            match p {
                SocialPath::GroupStanding => assert_eq!(p.name(), "Group Standing"),
                SocialPath::Warmth => assert_eq!(p.name(), "Warmth"),
                SocialPath::Hostility => assert_eq!(p.name(), "Hostility"),
                SocialPath::Predictability => assert_eq!(p.name(), "Predictability"),
                SocialPath::InteractionFrequency => assert_eq!(p.name(), "Interaction Frequency"),
                SocialPath::InteractionComplexity => assert_eq!(p.name(), "Interaction Complexity"),
            }
        }
    }

    #[test]
    fn all_education_path_names() {
        for p in EducationPath::all() {
            match p {
                EducationPath::CognitiveDemand => assert_eq!(p.name(), "Cognitive Demand"),
                EducationPath::CompetenceSupport => assert_eq!(p.name(), "Competence Support"),
                EducationPath::Warmth => assert_eq!(p.name(), "Warmth"),
                EducationPath::Hostility => assert_eq!(p.name(), "Hostility"),
                EducationPath::InteractionFrequency => {
                    assert_eq!(p.name(), "Interaction Frequency")
                }
                EducationPath::InteractionComplexity => {
                    assert_eq!(p.name(), "Interaction Complexity")
                }
            }
        }
    }

    #[test]
    fn all_healthcare_path_names() {
        for p in HealthcarePath::all() {
            match p {
                HealthcarePath::AccessFrequency => assert_eq!(p.name(), "Access Frequency"),
                HealthcarePath::Responsiveness => assert_eq!(p.name(), "Responsiveness"),
                HealthcarePath::Warmth => assert_eq!(p.name(), "Warmth"),
                HealthcarePath::Hostility => assert_eq!(p.name(), "Hostility"),
                HealthcarePath::InteractionFrequency => {
                    assert_eq!(p.name(), "Interaction Frequency")
                }
                HealthcarePath::InteractionComplexity => {
                    assert_eq!(p.name(), "Interaction Complexity")
                }
            }
        }
    }

    #[test]
    fn all_religious_path_names() {
        for p in ReligiousPath::all() {
            match p {
                ReligiousPath::RitualFrequency => assert_eq!(p.name(), "Ritual Frequency"),
                ReligiousPath::Warmth => assert_eq!(p.name(), "Warmth"),
                ReligiousPath::Hostility => assert_eq!(p.name(), "Hostility"),
                ReligiousPath::InteractionFrequency => {
                    assert_eq!(p.name(), "Interaction Frequency")
                }
                ReligiousPath::InteractionComplexity => {
                    assert_eq!(p.name(), "Interaction Complexity")
                }
            }
        }
    }

    #[test]
    fn all_neighborhood_path_names() {
        for p in NeighborhoodPath::all() {
            match p {
                NeighborhoodPath::Safety => assert_eq!(p.name(), "Safety"),
                NeighborhoodPath::Cohesion => assert_eq!(p.name(), "Cohesion"),
                NeighborhoodPath::Warmth => assert_eq!(p.name(), "Warmth"),
                NeighborhoodPath::Hostility => assert_eq!(p.name(), "Hostility"),
                NeighborhoodPath::InteractionFrequency => {
                    assert_eq!(p.name(), "Interaction Frequency")
                }
                NeighborhoodPath::InteractionComplexity => {
                    assert_eq!(p.name(), "Interaction Complexity")
                }
            }
        }
    }

    #[test]
    fn all_exosystem_path_names() {
        for p in ExosystemPath::all() {
            match p {
                ExosystemPath::HealthSystemAccess => assert_eq!(p.name(), "Health System Access"),
                ExosystemPath::EducationalSystemQuality => {
                    assert_eq!(p.name(), "Educational System Quality")
                }
                ExosystemPath::CommunityServicesAvailability => {
                    assert_eq!(p.name(), "Community Services Availability")
                }
                ExosystemPath::ResourceAvailability => {
                    assert_eq!(p.name(), "Resource Availability")
                }
                ExosystemPath::InstitutionalSupport => {
                    assert_eq!(p.name(), "Institutional Support")
                }
                ExosystemPath::ParentWorkStress => assert_eq!(p.name(), "Parent Work Stress"),
                ExosystemPath::ParentScheduleFlexibility => {
                    assert_eq!(p.name(), "Parent Schedule Flexibility")
                }
                ExosystemPath::ParentIncomeStability => {
                    assert_eq!(p.name(), "Parent Income Stability")
                }
            }
        }
    }

    #[test]
    fn all_macrosystem_path_names() {
        for p in MacrosystemPath::all() {
            match p {
                MacrosystemPath::IndividualismCollectivism => {
                    assert_eq!(p.name(), "Individualism-Collectivism")
                }
                MacrosystemPath::PowerDistance => assert_eq!(p.name(), "Power Distance"),
                MacrosystemPath::UncertaintyAvoidance => {
                    assert_eq!(p.name(), "Uncertainty Avoidance")
                }
                MacrosystemPath::CulturalStress => assert_eq!(p.name(), "Cultural Stress"),
                MacrosystemPath::CollectiveTrauma => assert_eq!(p.name(), "Collective Trauma"),
                MacrosystemPath::EconomicInequality => assert_eq!(p.name(), "Economic Inequality"),
                MacrosystemPath::RuleOfLaw => assert_eq!(p.name(), "Rule of Law"),
                MacrosystemPath::SocialMobility => assert_eq!(p.name(), "Social Mobility"),
                MacrosystemPath::CorruptionLevel => assert_eq!(p.name(), "Corruption Level"),
            }
        }
    }

    #[test]
    fn all_chronosystem_path_names() {
        for p in ChronosystemPath::all() {
            match p {
                ChronosystemPath::StabilityLevel => assert_eq!(p.name(), "Stability Level"),
                ChronosystemPath::ResourceScarcity => assert_eq!(p.name(), "Resource Scarcity"),
                ChronosystemPath::InstitutionalTrust => assert_eq!(p.name(), "Institutional Trust"),
                ChronosystemPath::PlasticityModifier => assert_eq!(p.name(), "Plasticity Modifier"),
            }
        }
    }
}
