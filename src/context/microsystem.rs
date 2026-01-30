//! Microsystem implementation for immediate face-to-face environments.
//!
//! Microsystems are the innermost layer of Bronfenbrenner's ecological model,
//! representing settings where the individual has direct, face-to-face
//! interactions with others.
//!
//! # Value Type
//!
//! Microsystem values are static f64, NOT StateValue. Unlike mood/needs which
//! have base+delta+decay, context values represent environmental conditions
//! that change via explicit events, not gradual decay.

use crate::enums::{
    EducationPath, FamilyPath, HealthcarePath, MicrosystemPath, NeighborhoodPath, ReligiousPath,
    SocialPath, WorkPath,
};
use crate::types::EntityId;

/// Type of microsystem environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MicrosystemType {
    /// Work environment.
    Work,
    /// Family environment.
    Family,
    /// Social/friendship environment.
    Social,
    /// Educational environment.
    Education,
    /// Healthcare environment.
    Healthcare,
    /// Religious community environment.
    Religious,
    /// Neighborhood environment.
    Neighborhood,
}

/// Role within a family context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FamilyRole {
    /// Child role in family.
    Child,
    /// Parent role in family.
    Parent,
    /// Spouse/partner role.
    Spouse,
    /// Sibling role.
    Sibling,
    /// Extended family member.
    Extended,
    /// No family role (default).
    #[default]
    None,
}

impl FamilyRole {
    /// Returns the effect multiplier for this role.
    ///
    /// Per spec: Parents amplify caregiving events 1.5x,
    /// children absorb parent stress 1.3x, etc.
    #[must_use]
    pub const fn effect_multiplier(&self) -> f64 {
        match self {
            FamilyRole::Parent => 1.5,
            FamilyRole::Child => 1.3,
            FamilyRole::Spouse => 1.2,
            FamilyRole::Sibling => 1.1,
            FamilyRole::Extended => 1.0,
            FamilyRole::None => 1.0,
        }
    }
}

/// Interaction profile for a microsystem.
///
/// Tracks the frequency and complexity of interactions, which determine
/// whether proximal processes can occur.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct InteractionProfile {
    /// Interaction frequency (0-1, normalized).
    pub interaction_frequency: f64,

    /// Progressive complexity of shared tasks (0-1).
    pub interaction_complexity: f64,

    /// Primary dyadic relationships in this context.
    pub primary_dyadic_relationships: Vec<EntityId>,
}

impl InteractionProfile {
    /// Creates a new InteractionProfile with default values.
    #[must_use]
    pub fn new() -> Self {
        InteractionProfile {
            interaction_frequency: 0.5,
            interaction_complexity: 0.5,
            primary_dyadic_relationships: Vec::new(),
        }
    }

    /// Creates an interaction profile with specified frequency and complexity.
    #[must_use]
    pub fn with_values(frequency: f64, complexity: f64) -> Self {
        InteractionProfile {
            interaction_frequency: frequency.clamp(0.0, 1.0),
            interaction_complexity: complexity.clamp(0.0, 1.0),
            primary_dyadic_relationships: Vec::new(),
        }
    }
}

/// Work context dimensions.
#[derive(Debug, Clone, PartialEq)]
pub struct WorkContext {
    /// Current work pressure (0-1).
    pub workload_stress: f64,

    /// Satisfaction with job role (0-1).
    pub role_satisfaction: f64,

    /// Emotional supportiveness of workplace (0-1).
    pub warmth: f64,

    /// Degree of antagonism/conflict (0-1).
    pub hostility: f64,

    /// Clear expectations and responsibilities (0-1).
    pub role_clarity: f64,

    /// Routine and consistent environment (0-1).
    pub predictability: f64,

    /// Mental engagement and challenge (0-1).
    pub cognitive_stimulation: f64,

    /// Security and constancy of position (0-1).
    pub stability: f64,

    /// Interaction profile for this context.
    pub interaction_profile: InteractionProfile,

    /// Supervisor entity (if any).
    pub supervisor_id: Option<EntityId>,

    /// Coworker entities.
    pub peer_ids: Vec<EntityId>,
}

impl Default for WorkContext {
    fn default() -> Self {
        WorkContext {
            workload_stress: 0.3,
            role_satisfaction: 0.5,
            warmth: 0.5,
            hostility: 0.2,
            role_clarity: 0.6,
            predictability: 0.5,
            cognitive_stimulation: 0.5,
            stability: 0.6,
            interaction_profile: InteractionProfile::new(),
            supervisor_id: None,
            peer_ids: Vec::new(),
        }
    }
}

impl WorkContext {
    /// Gets a value by path.
    #[must_use]
    pub fn get_value(&self, path: WorkPath) -> f64 {
        match path {
            WorkPath::WorkloadStress => self.workload_stress,
            WorkPath::RoleSatisfaction => self.role_satisfaction,
            WorkPath::Warmth => self.warmth,
            WorkPath::Hostility => self.hostility,
            WorkPath::RoleClarity => self.role_clarity,
            WorkPath::Predictability => self.predictability,
            WorkPath::CognitiveStimulation => self.cognitive_stimulation,
            WorkPath::Stability => self.stability,
            WorkPath::InteractionFrequency => self.interaction_profile.interaction_frequency,
            WorkPath::InteractionComplexity => self.interaction_profile.interaction_complexity,
        }
    }

    /// Sets a value by path.
    pub fn set_value(&mut self, path: WorkPath, value: f64) {
        let clamped = value.clamp(0.0, 1.0);
        match path {
            WorkPath::WorkloadStress => self.workload_stress = clamped,
            WorkPath::RoleSatisfaction => self.role_satisfaction = clamped,
            WorkPath::Warmth => self.warmth = clamped,
            WorkPath::Hostility => self.hostility = clamped,
            WorkPath::RoleClarity => self.role_clarity = clamped,
            WorkPath::Predictability => self.predictability = clamped,
            WorkPath::CognitiveStimulation => self.cognitive_stimulation = clamped,
            WorkPath::Stability => self.stability = clamped,
            WorkPath::InteractionFrequency => {
                self.interaction_profile.interaction_frequency = clamped
            }
            WorkPath::InteractionComplexity => {
                self.interaction_profile.interaction_complexity = clamped
            }
        }
    }
}

/// Family context dimensions.
#[derive(Debug, Clone, PartialEq)]
pub struct FamilyContext {
    /// Overall family satisfaction (0-1).
    pub family_satisfaction: f64,

    /// Caregiving burden (0-1).
    pub caregiving_burden: f64,

    /// Emotional warmth (0-1).
    pub warmth: f64,

    /// Active hostility (0-1).
    pub hostility: f64,

    /// Clear family role expectations (0-1).
    pub role_clarity: f64,

    /// Consistent family routines (0-1).
    pub predictability: f64,

    /// Family stability (0-1).
    pub stability: f64,

    /// Role within family.
    pub family_role: FamilyRole,

    /// Interaction profile for this context.
    pub interaction_profile: InteractionProfile,

    /// Family member entities.
    pub family_unit: Vec<EntityId>,
}

impl Default for FamilyContext {
    fn default() -> Self {
        FamilyContext {
            family_satisfaction: 0.6,
            caregiving_burden: 0.2,
            warmth: 0.6,
            hostility: 0.1,
            role_clarity: 0.5,
            predictability: 0.5,
            stability: 0.6,
            family_role: FamilyRole::None,
            interaction_profile: InteractionProfile::new(),
            family_unit: Vec::new(),
        }
    }
}

impl FamilyContext {
    /// Gets a value by path.
    #[must_use]
    pub fn get_value(&self, path: FamilyPath) -> f64 {
        match path {
            FamilyPath::FamilySatisfaction => self.family_satisfaction,
            FamilyPath::CaregivingBurden => self.caregiving_burden,
            FamilyPath::Warmth => self.warmth,
            FamilyPath::Hostility => self.hostility,
            FamilyPath::RoleClarity => self.role_clarity,
            FamilyPath::Predictability => self.predictability,
            FamilyPath::Stability => self.stability,
            FamilyPath::InteractionFrequency => self.interaction_profile.interaction_frequency,
            FamilyPath::InteractionComplexity => self.interaction_profile.interaction_complexity,
        }
    }

    /// Sets a value by path.
    pub fn set_value(&mut self, path: FamilyPath, value: f64) {
        let clamped = value.clamp(0.0, 1.0);
        match path {
            FamilyPath::FamilySatisfaction => self.family_satisfaction = clamped,
            FamilyPath::CaregivingBurden => self.caregiving_burden = clamped,
            FamilyPath::Warmth => self.warmth = clamped,
            FamilyPath::Hostility => self.hostility = clamped,
            FamilyPath::RoleClarity => self.role_clarity = clamped,
            FamilyPath::Predictability => self.predictability = clamped,
            FamilyPath::Stability => self.stability = clamped,
            FamilyPath::InteractionFrequency => {
                self.interaction_profile.interaction_frequency = clamped
            }
            FamilyPath::InteractionComplexity => {
                self.interaction_profile.interaction_complexity = clamped
            }
        }
    }
}

/// Social context dimensions.
#[derive(Debug, Clone, PartialEq)]
pub struct SocialContext {
    /// Standing in social group (0-1).
    pub group_standing: f64,

    /// Supportiveness of social circle (0-1).
    pub warmth: f64,

    /// Gossip, exclusion, rivalry (0-1).
    pub hostility: f64,

    /// Reliability of social connections (0-1).
    pub predictability: f64,

    /// Interaction profile for this context.
    pub interaction_profile: InteractionProfile,

    /// Close friend entities.
    pub close_friends: Vec<EntityId>,
}

impl Default for SocialContext {
    fn default() -> Self {
        SocialContext {
            group_standing: 0.5,
            warmth: 0.5,
            hostility: 0.2,
            predictability: 0.5,
            interaction_profile: InteractionProfile::new(),
            close_friends: Vec::new(),
        }
    }
}

impl SocialContext {
    /// Gets a value by path.
    #[must_use]
    pub fn get_value(&self, path: SocialPath) -> f64 {
        match path {
            SocialPath::GroupStanding => self.group_standing,
            SocialPath::Warmth => self.warmth,
            SocialPath::Hostility => self.hostility,
            SocialPath::Predictability => self.predictability,
            SocialPath::InteractionFrequency => self.interaction_profile.interaction_frequency,
            SocialPath::InteractionComplexity => self.interaction_profile.interaction_complexity,
        }
    }

    /// Sets a value by path.
    pub fn set_value(&mut self, path: SocialPath, value: f64) {
        let clamped = value.clamp(0.0, 1.0);
        match path {
            SocialPath::GroupStanding => self.group_standing = clamped,
            SocialPath::Warmth => self.warmth = clamped,
            SocialPath::Hostility => self.hostility = clamped,
            SocialPath::Predictability => self.predictability = clamped,
            SocialPath::InteractionFrequency => {
                self.interaction_profile.interaction_frequency = clamped
            }
            SocialPath::InteractionComplexity => {
                self.interaction_profile.interaction_complexity = clamped
            }
        }
    }
}

/// Education context dimensions.
#[derive(Debug, Clone, PartialEq)]
pub struct EducationContext {
    /// Intellectual challenge level (0-1).
    pub cognitive_demand: f64,

    /// Tutoring, mentorship availability (0-1).
    pub competence_support: f64,

    /// Emotional support (0-1).
    pub warmth: f64,

    /// Active antagonism (0-1).
    pub hostility: f64,

    /// Interaction profile for this context.
    pub interaction_profile: InteractionProfile,

    /// Instructor entities.
    pub instructors: Vec<EntityId>,

    /// Peer student entities.
    pub peer_ids: Vec<EntityId>,
}

impl Default for EducationContext {
    fn default() -> Self {
        EducationContext {
            cognitive_demand: 0.5,
            competence_support: 0.5,
            warmth: 0.5,
            hostility: 0.2,
            interaction_profile: InteractionProfile::new(),
            instructors: Vec::new(),
            peer_ids: Vec::new(),
        }
    }
}

impl EducationContext {
    /// Gets a value by path.
    #[must_use]
    pub fn get_value(&self, path: EducationPath) -> f64 {
        match path {
            EducationPath::CognitiveDemand => self.cognitive_demand,
            EducationPath::CompetenceSupport => self.competence_support,
            EducationPath::Warmth => self.warmth,
            EducationPath::Hostility => self.hostility,
            EducationPath::InteractionFrequency => self.interaction_profile.interaction_frequency,
            EducationPath::InteractionComplexity => self.interaction_profile.interaction_complexity,
        }
    }

    /// Sets a value by path.
    pub fn set_value(&mut self, path: EducationPath, value: f64) {
        let clamped = value.clamp(0.0, 1.0);
        match path {
            EducationPath::CognitiveDemand => self.cognitive_demand = clamped,
            EducationPath::CompetenceSupport => self.competence_support = clamped,
            EducationPath::Warmth => self.warmth = clamped,
            EducationPath::Hostility => self.hostility = clamped,
            EducationPath::InteractionFrequency => {
                self.interaction_profile.interaction_frequency = clamped
            }
            EducationPath::InteractionComplexity => {
                self.interaction_profile.interaction_complexity = clamped
            }
        }
    }
}

/// Healthcare context dimensions.
#[derive(Debug, Clone, PartialEq)]
pub struct HealthcareContext {
    /// Healthcare access frequency (0-1).
    pub access_frequency: f64,

    /// Provider responsiveness (0-1).
    pub responsiveness: f64,

    /// Provider warmth (0-1).
    pub warmth: f64,

    /// Provider hostility (0-1).
    pub hostility: f64,

    /// Interaction profile for this context.
    pub interaction_profile: InteractionProfile,

    /// Primary provider entity (if any).
    pub primary_provider_id: Option<EntityId>,
}

impl Default for HealthcareContext {
    fn default() -> Self {
        HealthcareContext {
            access_frequency: 0.3,
            responsiveness: 0.5,
            warmth: 0.5,
            hostility: 0.1,
            interaction_profile: InteractionProfile::new(),
            primary_provider_id: None,
        }
    }
}

impl HealthcareContext {
    /// Gets a value by path.
    #[must_use]
    pub fn get_value(&self, path: HealthcarePath) -> f64 {
        match path {
            HealthcarePath::AccessFrequency => self.access_frequency,
            HealthcarePath::Responsiveness => self.responsiveness,
            HealthcarePath::Warmth => self.warmth,
            HealthcarePath::Hostility => self.hostility,
            HealthcarePath::InteractionFrequency => self.interaction_profile.interaction_frequency,
            HealthcarePath::InteractionComplexity => {
                self.interaction_profile.interaction_complexity
            }
        }
    }

    /// Sets a value by path.
    pub fn set_value(&mut self, path: HealthcarePath, value: f64) {
        let clamped = value.clamp(0.0, 1.0);
        match path {
            HealthcarePath::AccessFrequency => self.access_frequency = clamped,
            HealthcarePath::Responsiveness => self.responsiveness = clamped,
            HealthcarePath::Warmth => self.warmth = clamped,
            HealthcarePath::Hostility => self.hostility = clamped,
            HealthcarePath::InteractionFrequency => {
                self.interaction_profile.interaction_frequency = clamped
            }
            HealthcarePath::InteractionComplexity => {
                self.interaction_profile.interaction_complexity = clamped
            }
        }
    }
}

/// Religious context dimensions.
#[derive(Debug, Clone, PartialEq)]
pub struct ReligiousContext {
    /// Ritual participation frequency (0-1).
    pub ritual_frequency: f64,

    /// Community warmth (0-1).
    pub warmth: f64,

    /// Community hostility (0-1).
    pub hostility: f64,

    /// Interaction profile for this context.
    pub interaction_profile: InteractionProfile,

    /// Community leader entity (if any).
    pub leader_id: Option<EntityId>,
}

impl Default for ReligiousContext {
    fn default() -> Self {
        ReligiousContext {
            ritual_frequency: 0.3,
            warmth: 0.5,
            hostility: 0.1,
            interaction_profile: InteractionProfile::new(),
            leader_id: None,
        }
    }
}

impl ReligiousContext {
    /// Gets a value by path.
    #[must_use]
    pub fn get_value(&self, path: ReligiousPath) -> f64 {
        match path {
            ReligiousPath::RitualFrequency => self.ritual_frequency,
            ReligiousPath::Warmth => self.warmth,
            ReligiousPath::Hostility => self.hostility,
            ReligiousPath::InteractionFrequency => self.interaction_profile.interaction_frequency,
            ReligiousPath::InteractionComplexity => self.interaction_profile.interaction_complexity,
        }
    }

    /// Sets a value by path.
    pub fn set_value(&mut self, path: ReligiousPath, value: f64) {
        let clamped = value.clamp(0.0, 1.0);
        match path {
            ReligiousPath::RitualFrequency => self.ritual_frequency = clamped,
            ReligiousPath::Warmth => self.warmth = clamped,
            ReligiousPath::Hostility => self.hostility = clamped,
            ReligiousPath::InteractionFrequency => {
                self.interaction_profile.interaction_frequency = clamped
            }
            ReligiousPath::InteractionComplexity => {
                self.interaction_profile.interaction_complexity = clamped
            }
        }
    }
}

/// Neighborhood context dimensions.
#[derive(Debug, Clone, PartialEq)]
pub struct NeighborhoodContext {
    /// Neighborhood safety (0-1).
    pub safety: f64,

    /// Community cohesion (0-1).
    pub cohesion: f64,

    /// Neighbor warmth (0-1).
    pub warmth: f64,

    /// Neighbor hostility (0-1).
    pub hostility: f64,

    /// Interaction profile for this context.
    pub interaction_profile: InteractionProfile,

    /// Proximity network entities.
    pub proximity_network: Vec<EntityId>,
}

impl Default for NeighborhoodContext {
    fn default() -> Self {
        NeighborhoodContext {
            safety: 0.6,
            cohesion: 0.5,
            warmth: 0.5,
            hostility: 0.2,
            interaction_profile: InteractionProfile::new(),
            proximity_network: Vec::new(),
        }
    }
}

impl NeighborhoodContext {
    /// Gets a value by path.
    #[must_use]
    pub fn get_value(&self, path: NeighborhoodPath) -> f64 {
        match path {
            NeighborhoodPath::Safety => self.safety,
            NeighborhoodPath::Cohesion => self.cohesion,
            NeighborhoodPath::Warmth => self.warmth,
            NeighborhoodPath::Hostility => self.hostility,
            NeighborhoodPath::InteractionFrequency => {
                self.interaction_profile.interaction_frequency
            }
            NeighborhoodPath::InteractionComplexity => {
                self.interaction_profile.interaction_complexity
            }
        }
    }

    /// Sets a value by path.
    pub fn set_value(&mut self, path: NeighborhoodPath, value: f64) {
        let clamped = value.clamp(0.0, 1.0);
        match path {
            NeighborhoodPath::Safety => self.safety = clamped,
            NeighborhoodPath::Cohesion => self.cohesion = clamped,
            NeighborhoodPath::Warmth => self.warmth = clamped,
            NeighborhoodPath::Hostility => self.hostility = clamped,
            NeighborhoodPath::InteractionFrequency => {
                self.interaction_profile.interaction_frequency = clamped
            }
            NeighborhoodPath::InteractionComplexity => {
                self.interaction_profile.interaction_complexity = clamped
            }
        }
    }
}

/// A microsystem instance containing one of the context types.
#[derive(Debug, Clone, PartialEq)]
pub enum Microsystem {
    /// Work environment context.
    Work(WorkContext),
    /// Family environment context.
    Family(FamilyContext),
    /// Social environment context.
    Social(SocialContext),
    /// Education environment context.
    Education(EducationContext),
    /// Healthcare environment context.
    Healthcare(HealthcareContext),
    /// Religious environment context.
    Religious(ReligiousContext),
    /// Neighborhood environment context.
    Neighborhood(NeighborhoodContext),
}

impl Microsystem {
    /// Creates a work microsystem from a WorkContext.
    #[must_use]
    pub fn new_work(context: WorkContext) -> Self {
        Microsystem::Work(context)
    }

    /// Creates a family microsystem from a FamilyContext.
    #[must_use]
    pub fn new_family(context: FamilyContext) -> Self {
        Microsystem::Family(context)
    }

    /// Creates a social microsystem from a SocialContext.
    #[must_use]
    pub fn new_social(context: SocialContext) -> Self {
        Microsystem::Social(context)
    }

    /// Creates an education microsystem from an EducationContext.
    #[must_use]
    pub fn new_education(context: EducationContext) -> Self {
        Microsystem::Education(context)
    }

    /// Creates a healthcare microsystem from a HealthcareContext.
    #[must_use]
    pub fn new_healthcare(context: HealthcareContext) -> Self {
        Microsystem::Healthcare(context)
    }

    /// Creates a religious microsystem from a ReligiousContext.
    #[must_use]
    pub fn new_religious(context: ReligiousContext) -> Self {
        Microsystem::Religious(context)
    }

    /// Creates a neighborhood microsystem from a NeighborhoodContext.
    #[must_use]
    pub fn new_neighborhood(context: NeighborhoodContext) -> Self {
        Microsystem::Neighborhood(context)
    }

    /// Returns the type of this microsystem.
    #[must_use]
    pub fn microsystem_type(&self) -> MicrosystemType {
        match self {
            Microsystem::Work(_) => MicrosystemType::Work,
            Microsystem::Family(_) => MicrosystemType::Family,
            Microsystem::Social(_) => MicrosystemType::Social,
            Microsystem::Education(_) => MicrosystemType::Education,
            Microsystem::Healthcare(_) => MicrosystemType::Healthcare,
            Microsystem::Religious(_) => MicrosystemType::Religious,
            Microsystem::Neighborhood(_) => MicrosystemType::Neighborhood,
        }
    }

    /// Gets a value by microsystem path.
    #[must_use]
    pub fn get_value(&self, path: &MicrosystemPath) -> f64 {
        match (self, path) {
            (Microsystem::Work(w), MicrosystemPath::Work(p)) => w.get_value(*p),
            (Microsystem::Family(f), MicrosystemPath::Family(p)) => f.get_value(*p),
            (Microsystem::Social(s), MicrosystemPath::Social(p)) => s.get_value(*p),
            (Microsystem::Education(e), MicrosystemPath::Education(p)) => e.get_value(*p),
            (Microsystem::Healthcare(h), MicrosystemPath::Healthcare(p)) => h.get_value(*p),
            (Microsystem::Religious(r), MicrosystemPath::Religious(p)) => r.get_value(*p),
            (Microsystem::Neighborhood(n), MicrosystemPath::Neighborhood(p)) => n.get_value(*p),
            // Type mismatch - return 0.0 as a safe default
            _ => 0.0,
        }
    }

    /// Sets a value by microsystem path.
    ///
    /// Returns true if the path matches the microsystem type, false otherwise.
    pub fn set_value(&mut self, path: &MicrosystemPath, value: f64) -> bool {
        match (self, path) {
            (Microsystem::Work(w), MicrosystemPath::Work(p)) => {
                w.set_value(*p, value);
                true
            }
            (Microsystem::Family(f), MicrosystemPath::Family(p)) => {
                f.set_value(*p, value);
                true
            }
            (Microsystem::Social(s), MicrosystemPath::Social(p)) => {
                s.set_value(*p, value);
                true
            }
            (Microsystem::Education(e), MicrosystemPath::Education(p)) => {
                e.set_value(*p, value);
                true
            }
            (Microsystem::Healthcare(h), MicrosystemPath::Healthcare(p)) => {
                h.set_value(*p, value);
                true
            }
            (Microsystem::Religious(r), MicrosystemPath::Religious(p)) => {
                r.set_value(*p, value);
                true
            }
            (Microsystem::Neighborhood(n), MicrosystemPath::Neighborhood(p)) => {
                n.set_value(*p, value);
                true
            }
            _ => false,
        }
    }

    /// Returns a reference to the work context, if this is a work microsystem.
    #[must_use]
    pub fn work(&self) -> Option<&WorkContext> {
        match self {
            Microsystem::Work(w) => Some(w),
            _ => None,
        }
    }

    /// Returns a mutable reference to the work context.
    pub fn work_mut(&mut self) -> Option<&mut WorkContext> {
        match self {
            Microsystem::Work(w) => Some(w),
            _ => None,
        }
    }

    /// Returns a reference to the family context, if this is a family microsystem.
    #[must_use]
    pub fn family(&self) -> Option<&FamilyContext> {
        match self {
            Microsystem::Family(f) => Some(f),
            _ => None,
        }
    }

    /// Returns a mutable reference to the family context.
    pub fn family_mut(&mut self) -> Option<&mut FamilyContext> {
        match self {
            Microsystem::Family(f) => Some(f),
            _ => None,
        }
    }

    /// Returns a reference to the social context, if this is a social microsystem.
    #[must_use]
    pub fn social(&self) -> Option<&SocialContext> {
        match self {
            Microsystem::Social(s) => Some(s),
            _ => None,
        }
    }

    /// Returns a mutable reference to the social context.
    pub fn social_mut(&mut self) -> Option<&mut SocialContext> {
        match self {
            Microsystem::Social(s) => Some(s),
            _ => None,
        }
    }

    /// Returns a reference to the education context, if this is an education microsystem.
    #[must_use]
    pub fn education(&self) -> Option<&EducationContext> {
        match self {
            Microsystem::Education(e) => Some(e),
            _ => None,
        }
    }

    /// Returns a mutable reference to the education context.
    pub fn education_mut(&mut self) -> Option<&mut EducationContext> {
        match self {
            Microsystem::Education(e) => Some(e),
            _ => None,
        }
    }

    /// Returns a reference to the healthcare context, if this is a healthcare microsystem.
    #[must_use]
    pub fn healthcare(&self) -> Option<&HealthcareContext> {
        match self {
            Microsystem::Healthcare(h) => Some(h),
            _ => None,
        }
    }

    /// Returns a mutable reference to the healthcare context.
    pub fn healthcare_mut(&mut self) -> Option<&mut HealthcareContext> {
        match self {
            Microsystem::Healthcare(h) => Some(h),
            _ => None,
        }
    }

    /// Returns a reference to the religious context, if this is a religious microsystem.
    #[must_use]
    pub fn religious(&self) -> Option<&ReligiousContext> {
        match self {
            Microsystem::Religious(r) => Some(r),
            _ => None,
        }
    }

    /// Returns a mutable reference to the religious context.
    pub fn religious_mut(&mut self) -> Option<&mut ReligiousContext> {
        match self {
            Microsystem::Religious(r) => Some(r),
            _ => None,
        }
    }

    /// Returns a reference to the neighborhood context, if this is a neighborhood microsystem.
    #[must_use]
    pub fn neighborhood(&self) -> Option<&NeighborhoodContext> {
        match self {
            Microsystem::Neighborhood(n) => Some(n),
            _ => None,
        }
    }

    /// Returns a mutable reference to the neighborhood context.
    pub fn neighborhood_mut(&mut self) -> Option<&mut NeighborhoodContext> {
        match self {
            Microsystem::Neighborhood(n) => Some(n),
            _ => None,
        }
    }

    /// Returns the warmth value for this microsystem.
    #[must_use]
    pub fn warmth(&self) -> f64 {
        match self {
            Microsystem::Work(w) => w.warmth,
            Microsystem::Family(f) => f.warmth,
            Microsystem::Social(s) => s.warmth,
            Microsystem::Education(e) => e.warmth,
            Microsystem::Healthcare(h) => h.warmth,
            Microsystem::Religious(r) => r.warmth,
            Microsystem::Neighborhood(n) => n.warmth,
        }
    }

    /// Returns the hostility value for this microsystem.
    #[must_use]
    pub fn hostility(&self) -> f64 {
        match self {
            Microsystem::Work(w) => w.hostility,
            Microsystem::Family(f) => f.hostility,
            Microsystem::Social(s) => s.hostility,
            Microsystem::Education(e) => e.hostility,
            Microsystem::Healthcare(h) => h.hostility,
            Microsystem::Religious(r) => r.hostility,
            Microsystem::Neighborhood(n) => n.hostility,
        }
    }

    /// Returns the interaction frequency for this microsystem.
    #[must_use]
    pub fn interaction_frequency(&self) -> f64 {
        match self {
            Microsystem::Work(w) => w.interaction_profile.interaction_frequency,
            Microsystem::Family(f) => f.interaction_profile.interaction_frequency,
            Microsystem::Social(s) => s.interaction_profile.interaction_frequency,
            Microsystem::Education(e) => e.interaction_profile.interaction_frequency,
            Microsystem::Healthcare(h) => h.interaction_profile.interaction_frequency,
            Microsystem::Religious(r) => r.interaction_profile.interaction_frequency,
            Microsystem::Neighborhood(n) => n.interaction_profile.interaction_frequency,
        }
    }

    /// Returns the interaction complexity for this microsystem.
    #[must_use]
    pub fn interaction_complexity(&self) -> f64 {
        match self {
            Microsystem::Work(w) => w.interaction_profile.interaction_complexity,
            Microsystem::Family(f) => f.interaction_profile.interaction_complexity,
            Microsystem::Social(s) => s.interaction_profile.interaction_complexity,
            Microsystem::Education(e) => e.interaction_profile.interaction_complexity,
            Microsystem::Healthcare(h) => h.interaction_profile.interaction_complexity,
            Microsystem::Religious(r) => r.interaction_profile.interaction_complexity,
            Microsystem::Neighborhood(n) => n.interaction_profile.interaction_complexity,
        }
    }

    /// Returns the stress level for this microsystem.
    ///
    /// Returns a computed stress value based on the microsystem type.
    #[must_use]
    pub fn stress_level(&self) -> f64 {
        match self {
            Microsystem::Work(w) => w.workload_stress,
            Microsystem::Family(f) => f.caregiving_burden * 0.7 + f.hostility * 0.3,
            Microsystem::Social(s) => s.hostility * 0.6 + (1.0 - s.group_standing) * 0.4,
            Microsystem::Education(e) => e.cognitive_demand * 0.5 + e.hostility * 0.5,
            Microsystem::Healthcare(h) => h.hostility * 0.5 + (1.0 - h.responsiveness) * 0.5,
            Microsystem::Religious(r) => r.hostility,
            Microsystem::Neighborhood(n) => (1.0 - n.safety) * 0.6 + n.hostility * 0.4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- MicrosystemType tests ---

    #[test]
    fn microsystem_type_all_variants() {
        let _ = MicrosystemType::Work;
        let _ = MicrosystemType::Family;
        let _ = MicrosystemType::Social;
        let _ = MicrosystemType::Education;
        let _ = MicrosystemType::Healthcare;
        let _ = MicrosystemType::Religious;
        let _ = MicrosystemType::Neighborhood;
    }

    // --- FamilyRole tests ---

    #[test]
    fn family_role_effect_multipliers() {
        assert!((FamilyRole::Parent.effect_multiplier() - 1.5).abs() < f64::EPSILON);
        assert!((FamilyRole::Child.effect_multiplier() - 1.3).abs() < f64::EPSILON);
        assert!((FamilyRole::Spouse.effect_multiplier() - 1.2).abs() < f64::EPSILON);
        assert!((FamilyRole::Sibling.effect_multiplier() - 1.1).abs() < f64::EPSILON);
        assert!((FamilyRole::Extended.effect_multiplier() - 1.0).abs() < f64::EPSILON);
        assert!((FamilyRole::None.effect_multiplier() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn family_role_default() {
        assert_eq!(FamilyRole::default(), FamilyRole::None);
    }

    // --- InteractionProfile tests ---

    #[test]
    fn interaction_profile_new() {
        let profile = InteractionProfile::new();
        assert!((profile.interaction_frequency - 0.5).abs() < f64::EPSILON);
        assert!((profile.interaction_complexity - 0.5).abs() < f64::EPSILON);
        assert!(profile.primary_dyadic_relationships.is_empty());
    }

    #[test]
    fn interaction_profile_with_values() {
        let profile = InteractionProfile::with_values(0.8, 0.6);
        assert!((profile.interaction_frequency - 0.8).abs() < f64::EPSILON);
        assert!((profile.interaction_complexity - 0.6).abs() < f64::EPSILON);
    }

    #[test]
    fn interaction_profile_with_values_clamped() {
        let profile = InteractionProfile::with_values(1.5, -0.5);
        assert!((profile.interaction_frequency - 1.0).abs() < f64::EPSILON);
        assert!((profile.interaction_complexity - 0.0).abs() < f64::EPSILON);
    }

    // --- WorkContext tests ---

    #[test]
    fn microsystem_work_context_creation() {
        let work = WorkContext::default();
        assert!(work.workload_stress >= 0.0 && work.workload_stress <= 1.0);
        assert!(work.warmth >= 0.0 && work.warmth <= 1.0);
    }

    #[test]
    fn work_context_get_value() {
        let work = WorkContext::default();
        assert!((work.get_value(WorkPath::WorkloadStress) - 0.3).abs() < f64::EPSILON);
        assert!((work.get_value(WorkPath::RoleSatisfaction) - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn work_context_set_value() {
        let mut work = WorkContext::default();
        work.set_value(WorkPath::WorkloadStress, 0.8);
        assert!((work.workload_stress - 0.8).abs() < f64::EPSILON);
    }

    #[test]
    fn work_context_set_value_clamped() {
        let mut work = WorkContext::default();
        work.set_value(WorkPath::Warmth, 1.5);
        assert!((work.warmth - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn work_context_all_paths() {
        let work = WorkContext::default();
        for path in WorkPath::all() {
            let value = work.get_value(path);
            assert!(value >= 0.0 && value <= 1.0);
        }
    }

    // --- FamilyContext tests ---

    #[test]
    fn microsystem_family_context_creation() {
        let family = FamilyContext::default();
        assert!(family.warmth >= 0.0 && family.warmth <= 1.0);
        assert_eq!(family.family_role, FamilyRole::None);
    }

    #[test]
    fn family_context_get_value() {
        let family = FamilyContext::default();
        assert!((family.get_value(FamilyPath::FamilySatisfaction) - 0.6).abs() < f64::EPSILON);
    }

    #[test]
    fn family_context_set_value() {
        let mut family = FamilyContext::default();
        family.set_value(FamilyPath::CaregivingBurden, 0.7);
        assert!((family.caregiving_burden - 0.7).abs() < f64::EPSILON);
    }

    #[test]
    fn family_context_all_paths() {
        let family = FamilyContext::default();
        for path in FamilyPath::all() {
            let value = family.get_value(path);
            assert!(value >= 0.0 && value <= 1.0);
        }
    }

    // --- SocialContext tests ---

    #[test]
    fn microsystem_social_context_creation() {
        let social = SocialContext::default();
        assert!(social.warmth >= 0.0 && social.warmth <= 1.0);
    }

    #[test]
    fn social_context_get_value() {
        let social = SocialContext::default();
        assert!((social.get_value(SocialPath::GroupStanding) - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn social_context_set_value() {
        let mut social = SocialContext::default();
        social.set_value(SocialPath::Warmth, 0.9);
        assert!((social.warmth - 0.9).abs() < f64::EPSILON);
    }

    #[test]
    fn social_context_all_paths() {
        let social = SocialContext::default();
        for path in SocialPath::all() {
            let value = social.get_value(path);
            assert!(value >= 0.0 && value <= 1.0);
        }
    }

    // --- EducationContext tests ---

    #[test]
    fn education_context_default() {
        let edu = EducationContext::default();
        assert!((edu.cognitive_demand - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn education_context_all_paths() {
        let edu = EducationContext::default();
        for path in EducationPath::all() {
            let value = edu.get_value(path);
            assert!(value >= 0.0 && value <= 1.0);
        }
    }

    #[test]
    fn education_context_set_value() {
        let mut edu = EducationContext::default();
        edu.set_value(EducationPath::CognitiveDemand, 0.8);
        assert!((edu.cognitive_demand - 0.8).abs() < f64::EPSILON);
    }

    // --- HealthcareContext tests ---

    #[test]
    fn healthcare_context_default() {
        let hc = HealthcareContext::default();
        assert!((hc.access_frequency - 0.3).abs() < f64::EPSILON);
    }

    #[test]
    fn healthcare_context_all_paths() {
        let hc = HealthcareContext::default();
        for path in HealthcarePath::all() {
            let value = hc.get_value(path);
            assert!(value >= 0.0 && value <= 1.0);
        }
    }

    #[test]
    fn healthcare_context_set_value() {
        let mut hc = HealthcareContext::default();
        hc.set_value(HealthcarePath::Responsiveness, 0.9);
        assert!((hc.responsiveness - 0.9).abs() < f64::EPSILON);
    }

    // --- ReligiousContext tests ---

    #[test]
    fn religious_context_default() {
        let rel = ReligiousContext::default();
        assert!((rel.ritual_frequency - 0.3).abs() < f64::EPSILON);
    }

    #[test]
    fn religious_context_all_paths() {
        let rel = ReligiousContext::default();
        for path in ReligiousPath::all() {
            let value = rel.get_value(path);
            assert!(value >= 0.0 && value <= 1.0);
        }
    }

    #[test]
    fn religious_context_set_value() {
        let mut rel = ReligiousContext::default();
        rel.set_value(ReligiousPath::Warmth, 0.8);
        assert!((rel.warmth - 0.8).abs() < f64::EPSILON);
    }

    // --- NeighborhoodContext tests ---

    #[test]
    fn neighborhood_context_default() {
        let nb = NeighborhoodContext::default();
        assert!((nb.safety - 0.6).abs() < f64::EPSILON);
    }

    #[test]
    fn neighborhood_context_all_paths() {
        let nb = NeighborhoodContext::default();
        for path in NeighborhoodPath::all() {
            let value = nb.get_value(path);
            assert!(value >= 0.0 && value <= 1.0);
        }
    }

    #[test]
    fn neighborhood_context_set_value() {
        let mut nb = NeighborhoodContext::default();
        nb.set_value(NeighborhoodPath::Safety, 0.9);
        assert!((nb.safety - 0.9).abs() < f64::EPSILON);
    }

    // --- Microsystem tests ---

    #[test]
    fn microsystem_work_creation() {
        let m = Microsystem::new_work(WorkContext::default());
        assert_eq!(m.microsystem_type(), MicrosystemType::Work);
    }

    #[test]
    fn microsystem_family_creation() {
        let m = Microsystem::new_family(FamilyContext::default());
        assert_eq!(m.microsystem_type(), MicrosystemType::Family);
    }

    #[test]
    fn microsystem_social_creation() {
        let m = Microsystem::new_social(SocialContext::default());
        assert_eq!(m.microsystem_type(), MicrosystemType::Social);
    }

    #[test]
    fn microsystem_education_creation() {
        let m = Microsystem::new_education(EducationContext::default());
        assert_eq!(m.microsystem_type(), MicrosystemType::Education);
    }

    #[test]
    fn microsystem_healthcare_creation() {
        let m = Microsystem::new_healthcare(HealthcareContext::default());
        assert_eq!(m.microsystem_type(), MicrosystemType::Healthcare);
    }

    #[test]
    fn microsystem_religious_creation() {
        let m = Microsystem::new_religious(ReligiousContext::default());
        assert_eq!(m.microsystem_type(), MicrosystemType::Religious);
    }

    #[test]
    fn microsystem_neighborhood_creation() {
        let m = Microsystem::new_neighborhood(NeighborhoodContext::default());
        assert_eq!(m.microsystem_type(), MicrosystemType::Neighborhood);
    }

    #[test]
    fn microsystem_get_value_matching_type() {
        let m = Microsystem::new_work(WorkContext::default());
        let value = m.get_value(&MicrosystemPath::Work(WorkPath::Warmth));
        assert!(value >= 0.0 && value <= 1.0);
    }

    #[test]
    fn microsystem_get_value_mismatched_type() {
        let m = Microsystem::new_work(WorkContext::default());
        let value = m.get_value(&MicrosystemPath::Family(FamilyPath::Warmth));
        assert!((value - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn microsystem_set_value_matching_type() {
        let mut m = Microsystem::new_work(WorkContext::default());
        let result = m.set_value(&MicrosystemPath::Work(WorkPath::Warmth), 0.9);
        assert!(result);
        assert!((m.warmth() - 0.9).abs() < f64::EPSILON);
    }

    #[test]
    fn microsystem_set_value_mismatched_type() {
        let mut m = Microsystem::new_work(WorkContext::default());
        let result = m.set_value(&MicrosystemPath::Family(FamilyPath::Warmth), 0.9);
        assert!(!result);
    }

    #[test]
    fn microsystem_typed_accessors() {
        let m = Microsystem::new_work(WorkContext::default());
        assert!(m.work().is_some());
        assert!(m.family().is_none());

        let m = Microsystem::new_family(FamilyContext::default());
        assert!(m.family().is_some());
        assert!(m.work().is_none());

        let m = Microsystem::new_social(SocialContext::default());
        assert!(m.social().is_some());
        assert!(m.work().is_none());

        let m = Microsystem::new_education(EducationContext::default());
        assert!(m.education().is_some());

        let m = Microsystem::new_healthcare(HealthcareContext::default());
        assert!(m.healthcare().is_some());

        let m = Microsystem::new_religious(ReligiousContext::default());
        assert!(m.religious().is_some());

        let m = Microsystem::new_neighborhood(NeighborhoodContext::default());
        assert!(m.neighborhood().is_some());
    }

    #[test]
    fn microsystem_typed_mut_accessors() {
        let mut m = Microsystem::new_work(WorkContext::default());
        assert!(m.work_mut().is_some());
        assert!(m.family_mut().is_none());

        let mut m = Microsystem::new_family(FamilyContext::default());
        assert!(m.family_mut().is_some());

        let mut m = Microsystem::new_social(SocialContext::default());
        assert!(m.social_mut().is_some());

        let mut m = Microsystem::new_education(EducationContext::default());
        assert!(m.education_mut().is_some());

        let mut m = Microsystem::new_healthcare(HealthcareContext::default());
        assert!(m.healthcare_mut().is_some());

        let mut m = Microsystem::new_religious(ReligiousContext::default());
        assert!(m.religious_mut().is_some());

        let mut m = Microsystem::new_neighborhood(NeighborhoodContext::default());
        assert!(m.neighborhood_mut().is_some());
    }

    #[test]
    fn microsystem_warmth() {
        let m = Microsystem::new_work(WorkContext::default());
        let warmth = m.warmth();
        assert!(warmth >= 0.0 && warmth <= 1.0);
    }

    #[test]
    fn microsystem_hostility() {
        let m = Microsystem::new_family(FamilyContext::default());
        let hostility = m.hostility();
        assert!(hostility >= 0.0 && hostility <= 1.0);
    }

    #[test]
    fn microsystem_interaction_frequency() {
        let m = Microsystem::new_work(WorkContext::default());
        let freq = m.interaction_frequency();
        assert!(freq >= 0.0 && freq <= 1.0);
    }

    #[test]
    fn microsystem_interaction_complexity() {
        let m = Microsystem::new_social(SocialContext::default());
        let complexity = m.interaction_complexity();
        assert!(complexity >= 0.0 && complexity <= 1.0);
    }

    #[test]
    fn microsystem_stress_level() {
        let m = Microsystem::new_work(WorkContext::default());
        let stress = m.stress_level();
        assert!(stress >= 0.0 && stress <= 1.0);
    }

    #[test]
    fn microsystem_stress_level_all_types() {
        let types = [
            Microsystem::new_work(WorkContext::default()),
            Microsystem::new_family(FamilyContext::default()),
            Microsystem::new_social(SocialContext::default()),
            Microsystem::new_education(EducationContext::default()),
            Microsystem::new_healthcare(HealthcareContext::default()),
            Microsystem::new_religious(ReligiousContext::default()),
            Microsystem::new_neighborhood(NeighborhoodContext::default()),
        ];

        for m in types {
            let stress = m.stress_level();
            assert!(stress >= 0.0 && stress <= 1.0);
        }
    }

    #[test]
    fn microsystem_clone_and_eq() {
        let m1 = Microsystem::new_work(WorkContext::default());
        let m2 = m1.clone();
        assert_eq!(m1, m2);
    }

    #[test]
    fn microsystem_debug() {
        let m = Microsystem::new_work(WorkContext::default());
        let debug = format!("{:?}", m);
        assert!(debug.contains("Work"));
    }

    // --- Additional coverage tests for set_value paths ---

    #[test]
    fn work_context_set_value_all_paths() {
        let mut work = WorkContext::default();
        for path in WorkPath::all() {
            work.set_value(path, 0.7);
            assert!((work.get_value(path) - 0.7).abs() < f64::EPSILON);
        }
    }

    #[test]
    fn family_context_set_value_all_paths() {
        let mut family = FamilyContext::default();
        for path in FamilyPath::all() {
            family.set_value(path, 0.7);
            assert!((family.get_value(path) - 0.7).abs() < f64::EPSILON);
        }
    }

    #[test]
    fn social_context_set_value_all_paths() {
        let mut social = SocialContext::default();
        for path in SocialPath::all() {
            social.set_value(path, 0.7);
            assert!((social.get_value(path) - 0.7).abs() < f64::EPSILON);
        }
    }

    #[test]
    fn education_context_set_value_all_paths() {
        let mut edu = EducationContext::default();
        for path in EducationPath::all() {
            edu.set_value(path, 0.7);
            assert!((edu.get_value(path) - 0.7).abs() < f64::EPSILON);
        }
    }

    #[test]
    fn healthcare_context_set_value_all_paths() {
        let mut hc = HealthcareContext::default();
        for path in HealthcarePath::all() {
            hc.set_value(path, 0.7);
            assert!((hc.get_value(path) - 0.7).abs() < f64::EPSILON);
        }
    }

    #[test]
    fn religious_context_set_value_all_paths() {
        let mut rel = ReligiousContext::default();
        for path in ReligiousPath::all() {
            rel.set_value(path, 0.7);
            assert!((rel.get_value(path) - 0.7).abs() < f64::EPSILON);
        }
    }

    #[test]
    fn neighborhood_context_set_value_all_paths() {
        let mut nb = NeighborhoodContext::default();
        for path in NeighborhoodPath::all() {
            nb.set_value(path, 0.7);
            assert!((nb.get_value(path) - 0.7).abs() < f64::EPSILON);
        }
    }

    // --- Tests for Microsystem get_value/set_value for all types ---

    #[test]
    fn microsystem_get_value_family() {
        let m = Microsystem::new_family(FamilyContext::default());
        let value = m.get_value(&MicrosystemPath::Family(FamilyPath::Warmth));
        assert!(value >= 0.0 && value <= 1.0);
    }

    #[test]
    fn microsystem_get_value_social() {
        let m = Microsystem::new_social(SocialContext::default());
        let value = m.get_value(&MicrosystemPath::Social(SocialPath::Warmth));
        assert!(value >= 0.0 && value <= 1.0);
    }

    #[test]
    fn microsystem_get_value_education() {
        let m = Microsystem::new_education(EducationContext::default());
        let value = m.get_value(&MicrosystemPath::Education(EducationPath::Warmth));
        assert!(value >= 0.0 && value <= 1.0);
    }

    #[test]
    fn microsystem_get_value_healthcare() {
        let m = Microsystem::new_healthcare(HealthcareContext::default());
        let value = m.get_value(&MicrosystemPath::Healthcare(HealthcarePath::Warmth));
        assert!(value >= 0.0 && value <= 1.0);
    }

    #[test]
    fn microsystem_get_value_religious() {
        let m = Microsystem::new_religious(ReligiousContext::default());
        let value = m.get_value(&MicrosystemPath::Religious(ReligiousPath::Warmth));
        assert!(value >= 0.0 && value <= 1.0);
    }

    #[test]
    fn microsystem_get_value_neighborhood() {
        let m = Microsystem::new_neighborhood(NeighborhoodContext::default());
        let value = m.get_value(&MicrosystemPath::Neighborhood(NeighborhoodPath::Warmth));
        assert!(value >= 0.0 && value <= 1.0);
    }

    #[test]
    fn microsystem_set_value_family() {
        let mut m = Microsystem::new_family(FamilyContext::default());
        let result = m.set_value(&MicrosystemPath::Family(FamilyPath::Warmth), 0.9);
        assert!(result);
        assert!((m.warmth() - 0.9).abs() < f64::EPSILON);
    }

    #[test]
    fn microsystem_set_value_social() {
        let mut m = Microsystem::new_social(SocialContext::default());
        let result = m.set_value(&MicrosystemPath::Social(SocialPath::Warmth), 0.9);
        assert!(result);
        assert!((m.warmth() - 0.9).abs() < f64::EPSILON);
    }

    #[test]
    fn microsystem_set_value_education() {
        let mut m = Microsystem::new_education(EducationContext::default());
        let result = m.set_value(&MicrosystemPath::Education(EducationPath::Warmth), 0.9);
        assert!(result);
        assert!((m.warmth() - 0.9).abs() < f64::EPSILON);
    }

    #[test]
    fn microsystem_set_value_healthcare() {
        let mut m = Microsystem::new_healthcare(HealthcareContext::default());
        let result = m.set_value(&MicrosystemPath::Healthcare(HealthcarePath::Warmth), 0.9);
        assert!(result);
        assert!((m.warmth() - 0.9).abs() < f64::EPSILON);
    }

    #[test]
    fn microsystem_set_value_religious() {
        let mut m = Microsystem::new_religious(ReligiousContext::default());
        let result = m.set_value(&MicrosystemPath::Religious(ReligiousPath::Warmth), 0.9);
        assert!(result);
        assert!((m.warmth() - 0.9).abs() < f64::EPSILON);
    }

    #[test]
    fn microsystem_set_value_neighborhood() {
        let mut m = Microsystem::new_neighborhood(NeighborhoodContext::default());
        let result = m.set_value(
            &MicrosystemPath::Neighborhood(NeighborhoodPath::Warmth),
            0.9,
        );
        assert!(result);
        assert!((m.warmth() - 0.9).abs() < f64::EPSILON);
    }

    // --- Tests for warmth/hostility/interaction_frequency/complexity for all types ---

    #[test]
    fn microsystem_warmth_all_types() {
        let types = [
            Microsystem::new_work(WorkContext::default()),
            Microsystem::new_family(FamilyContext::default()),
            Microsystem::new_social(SocialContext::default()),
            Microsystem::new_education(EducationContext::default()),
            Microsystem::new_healthcare(HealthcareContext::default()),
            Microsystem::new_religious(ReligiousContext::default()),
            Microsystem::new_neighborhood(NeighborhoodContext::default()),
        ];
        for m in types {
            let warmth = m.warmth();
            assert!(warmth >= 0.0 && warmth <= 1.0);
        }
    }

    #[test]
    fn microsystem_hostility_all_types() {
        let types = [
            Microsystem::new_work(WorkContext::default()),
            Microsystem::new_family(FamilyContext::default()),
            Microsystem::new_social(SocialContext::default()),
            Microsystem::new_education(EducationContext::default()),
            Microsystem::new_healthcare(HealthcareContext::default()),
            Microsystem::new_religious(ReligiousContext::default()),
            Microsystem::new_neighborhood(NeighborhoodContext::default()),
        ];
        for m in types {
            let hostility = m.hostility();
            assert!(hostility >= 0.0 && hostility <= 1.0);
        }
    }

    #[test]
    fn microsystem_interaction_frequency_all_types() {
        let types = [
            Microsystem::new_work(WorkContext::default()),
            Microsystem::new_family(FamilyContext::default()),
            Microsystem::new_social(SocialContext::default()),
            Microsystem::new_education(EducationContext::default()),
            Microsystem::new_healthcare(HealthcareContext::default()),
            Microsystem::new_religious(ReligiousContext::default()),
            Microsystem::new_neighborhood(NeighborhoodContext::default()),
        ];
        for m in types {
            let freq = m.interaction_frequency();
            assert!(freq >= 0.0 && freq <= 1.0);
        }
    }

    #[test]
    fn microsystem_interaction_complexity_all_types() {
        let types = [
            Microsystem::new_work(WorkContext::default()),
            Microsystem::new_family(FamilyContext::default()),
            Microsystem::new_social(SocialContext::default()),
            Microsystem::new_education(EducationContext::default()),
            Microsystem::new_healthcare(HealthcareContext::default()),
            Microsystem::new_religious(ReligiousContext::default()),
            Microsystem::new_neighborhood(NeighborhoodContext::default()),
        ];
        for m in types {
            let complexity = m.interaction_complexity();
            assert!(complexity >= 0.0 && complexity <= 1.0);
        }
    }

    // --- Tests for mut accessor returning None ---

    #[test]
    fn microsystem_typed_mut_accessors_none_variants() {
        let mut m = Microsystem::new_work(WorkContext::default());
        assert!(m.social_mut().is_none());
        assert!(m.education_mut().is_none());
        assert!(m.healthcare_mut().is_none());
        assert!(m.religious_mut().is_none());
        assert!(m.neighborhood_mut().is_none());

        let mut m = Microsystem::new_family(FamilyContext::default());
        assert!(m.work_mut().is_none());
        assert!(m.social_mut().is_none());
        assert!(m.education_mut().is_none());
        assert!(m.healthcare_mut().is_none());
        assert!(m.religious_mut().is_none());
        assert!(m.neighborhood_mut().is_none());

        let mut m = Microsystem::new_social(SocialContext::default());
        assert!(m.family_mut().is_none());

        let mut m = Microsystem::new_education(EducationContext::default());
        assert!(m.work_mut().is_none());

        let mut m = Microsystem::new_healthcare(HealthcareContext::default());
        assert!(m.work_mut().is_none());

        let mut m = Microsystem::new_religious(ReligiousContext::default());
        assert!(m.work_mut().is_none());

        let mut m = Microsystem::new_neighborhood(NeighborhoodContext::default());
        assert!(m.work_mut().is_none());
    }

    // --- Tests for read accessor returning None ---

    #[test]
    fn microsystem_typed_accessors_none_variants() {
        // Test that immutable accessors return None for non-matching types
        let m = Microsystem::new_work(WorkContext::default());
        assert!(m.social().is_none());
        assert!(m.education().is_none());
        assert!(m.healthcare().is_none());
        assert!(m.religious().is_none());
        assert!(m.neighborhood().is_none());

        let m = Microsystem::new_family(FamilyContext::default());
        assert!(m.social().is_none());
        assert!(m.education().is_none());
        assert!(m.healthcare().is_none());
        assert!(m.religious().is_none());
        assert!(m.neighborhood().is_none());

        let m = Microsystem::new_social(SocialContext::default());
        assert!(m.education().is_none());
        assert!(m.healthcare().is_none());
        assert!(m.religious().is_none());
        assert!(m.neighborhood().is_none());

        let m = Microsystem::new_education(EducationContext::default());
        assert!(m.social().is_none());
        assert!(m.healthcare().is_none());
        assert!(m.religious().is_none());
        assert!(m.neighborhood().is_none());

        let m = Microsystem::new_healthcare(HealthcareContext::default());
        assert!(m.social().is_none());
        assert!(m.education().is_none());
        assert!(m.religious().is_none());
        assert!(m.neighborhood().is_none());

        let m = Microsystem::new_religious(ReligiousContext::default());
        assert!(m.social().is_none());
        assert!(m.education().is_none());
        assert!(m.healthcare().is_none());
        assert!(m.neighborhood().is_none());

        let m = Microsystem::new_neighborhood(NeighborhoodContext::default());
        assert!(m.social().is_none());
        assert!(m.education().is_none());
        assert!(m.healthcare().is_none());
        assert!(m.religious().is_none());
    }

    // --- Required Phase 7 tests ---

    #[test]
    fn microsystem_family_support_modifies_valence() {
        // High family support (warmth) increases valence
        // This tests that family warmth is a measurable dimension
        // that can influence entity state through microsystem effects

        // Create high-support family
        let mut high_support = FamilyContext::default();
        high_support.warmth = 0.9;
        high_support.hostility = 0.1;
        high_support.family_satisfaction = 0.8;

        // Create low-support family
        let mut low_support = FamilyContext::default();
        low_support.warmth = 0.2;
        low_support.hostility = 0.7;
        low_support.family_satisfaction = 0.3;

        let high_family = Microsystem::new_family(high_support);
        let low_family = Microsystem::new_family(low_support);

        // High support family has higher warmth
        assert!(high_family.warmth() > low_family.warmth());
        assert!((high_family.warmth() - 0.9).abs() < f64::EPSILON);
        assert!((low_family.warmth() - 0.2).abs() < f64::EPSILON);

        // High support family has lower stress level
        let high_stress = high_family.stress_level();
        let low_stress = low_family.stress_level();
        assert!(high_stress < low_stress);
    }

    #[test]
    fn microsystem_social_isolation_increases_loneliness() {
        // Low social support raises loneliness
        // Tests that low social interaction and warmth correlate with isolation

        // Create socially connected context
        let mut connected = SocialContext::default();
        connected.warmth = 0.8;
        connected.group_standing = 0.7;
        connected.hostility = 0.1;
        connected.interaction_profile = InteractionProfile::with_values(0.8, 0.7);

        // Create isolated context
        let mut isolated = SocialContext::default();
        isolated.warmth = 0.2;
        isolated.group_standing = 0.2;
        isolated.hostility = 0.6;
        isolated.interaction_profile = InteractionProfile::with_values(0.1, 0.1);

        let connected_micro = Microsystem::new_social(connected);
        let isolated_micro = Microsystem::new_social(isolated);

        // Isolated context has lower warmth
        assert!(isolated_micro.warmth() < connected_micro.warmth());

        // Isolated context has higher hostility
        assert!(isolated_micro.hostility() > connected_micro.hostility());

        // Isolated context has lower interaction frequency
        assert!(isolated_micro.interaction_frequency() < connected_micro.interaction_frequency());

        // Isolated context has higher stress level
        assert!(isolated_micro.stress_level() > connected_micro.stress_level());

        // Social microsystem stress is influenced by hostility and low standing
        // stress = hostility * 0.6 + (1.0 - group_standing) * 0.4
        let isolated_expected = 0.6 * 0.6 + (1.0 - 0.2) * 0.4; // 0.36 + 0.32 = 0.68
        assert!((isolated_micro.stress_level() - isolated_expected).abs() < 0.01);
    }

    #[test]
    fn microsystem_set_value_mismatched_type_returns_false() {
        // Test that setting a work path on a family microsystem returns false
        let mut family_micro = Microsystem::new_family(FamilyContext::default());
        let work_path = MicrosystemPath::Work(WorkPath::WorkloadStress);
        let result = family_micro.set_value(&work_path, 0.5);
        assert!(!result);

        // Test setting a family path on a work microsystem returns false
        let mut work_micro = Microsystem::new_work(WorkContext::default());
        let family_path = MicrosystemPath::Family(FamilyPath::Warmth);
        let result = work_micro.set_value(&family_path, 0.5);
        assert!(!result);
    }

    #[test]
    fn microsystem_get_value_mismatched_type_returns_zero() {
        // Test that getting a work path from a family microsystem returns 0.0
        let family_micro = Microsystem::new_family(FamilyContext::default());
        let work_path = MicrosystemPath::Work(WorkPath::WorkloadStress);
        let value = family_micro.get_value(&work_path);
        assert!((value - 0.0).abs() < f64::EPSILON);

        // Test getting a family path from a work microsystem returns 0.0
        let work_micro = Microsystem::new_work(WorkContext::default());
        let family_path = MicrosystemPath::Family(FamilyPath::Warmth);
        let value = work_micro.get_value(&family_path);
        assert!((value - 0.0).abs() < f64::EPSILON);
    }
}
