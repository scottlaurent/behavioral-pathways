//! Exosystem implementation for indirect environmental influences.
//!
//! The exosystem represents settings that affect the individual indirectly
//! through their influence on microsystems. The individual is not a direct
//! participant in these settings.
//!
//! Examples:
//! - Parent's workplace affects child through parent's stress/schedule
//! - Local school board policies affect education quality
//! - Healthcare system access affects available care

use crate::enums::ExosystemPath;

/// Parent work environment quality.
///
/// For children, this captures how the parent's workplace affects
/// their availability and emotional state.
#[derive(Debug, Clone, PartialEq)]
pub struct ParentWorkQuality {
    /// Parent's work stress level (0-1).
    pub stress_level: f64,

    /// Ability to attend to family needs (0-1).
    pub schedule_flexibility: f64,

    /// Financial security (0-1).
    pub income_stability: f64,
}

impl Default for ParentWorkQuality {
    fn default() -> Self {
        ParentWorkQuality {
            stress_level: 0.3,
            schedule_flexibility: 0.5,
            income_stability: 0.6,
        }
    }
}

impl ParentWorkQuality {
    /// Computes overall parent capacity from work environment.
    ///
    /// Per spec: capacity = (1.0 - (stress * 0.3 + (1.0 - flexibility) * 0.2)) * income_stability
    /// Returns value in range [0.0, 1.0].
    #[must_use]
    pub fn parent_capacity(&self) -> f64 {
        let base_capacity = 1.0
            - (self.stress_level * 0.3 + (1.0 - self.schedule_flexibility) * 0.2);
        (base_capacity * self.income_stability).clamp(0.0, 1.0)
    }
}

/// Exosystem context for indirect environmental influences.
///
/// These settings affect the individual through their influence on
/// microsystems rather than through direct interaction.
#[derive(Debug, Clone, PartialEq)]
pub struct ExosystemContext {
    /// Quality of healthcare access (0-1).
    pub health_system_access: f64,

    /// Quality of education available (0-1).
    pub educational_system_quality: f64,

    /// Access to community resources (0-1).
    pub community_services_availability: f64,

    /// General resource access (0-1).
    pub resource_availability: f64,

    /// Support from institutions (0-1).
    pub institutional_support: f64,

    /// Parent work environment (for children).
    pub parent_work_environment: Option<ParentWorkQuality>,
}

impl Default for ExosystemContext {
    fn default() -> Self {
        ExosystemContext {
            health_system_access: 0.6,
            educational_system_quality: 0.6,
            community_services_availability: 0.5,
            resource_availability: 0.6,
            institutional_support: 0.5,
            parent_work_environment: None,
        }
    }
}

impl ExosystemContext {
    /// Creates a new ExosystemContext with default values.
    #[must_use]
    pub fn new() -> Self {
        ExosystemContext::default()
    }

    /// Gets a value by exosystem path.
    #[must_use]
    pub fn get_value(&self, path: &ExosystemPath) -> f64 {
        match path {
            ExosystemPath::HealthSystemAccess => self.health_system_access,
            ExosystemPath::EducationalSystemQuality => self.educational_system_quality,
            ExosystemPath::CommunityServicesAvailability => self.community_services_availability,
            ExosystemPath::ResourceAvailability => self.resource_availability,
            ExosystemPath::InstitutionalSupport => self.institutional_support,
            ExosystemPath::ParentWorkStress => self
                .parent_work_environment
                .as_ref()
                .map_or(0.0, |p| p.stress_level),
            ExosystemPath::ParentScheduleFlexibility => self
                .parent_work_environment
                .as_ref()
                .map_or(0.5, |p| p.schedule_flexibility),
            ExosystemPath::ParentIncomeStability => self
                .parent_work_environment
                .as_ref()
                .map_or(0.5, |p| p.income_stability),
        }
    }

    /// Sets a value by exosystem path.
    pub fn set_value(&mut self, path: &ExosystemPath, value: f64) {
        let clamped = value.clamp(0.0, 1.0);
        match path {
            ExosystemPath::HealthSystemAccess => self.health_system_access = clamped,
            ExosystemPath::EducationalSystemQuality => self.educational_system_quality = clamped,
            ExosystemPath::CommunityServicesAvailability => {
                self.community_services_availability = clamped
            }
            ExosystemPath::ResourceAvailability => self.resource_availability = clamped,
            ExosystemPath::InstitutionalSupport => self.institutional_support = clamped,
            ExosystemPath::ParentWorkStress => {
                let parent = self
                    .parent_work_environment
                    .get_or_insert_with(ParentWorkQuality::default);
                parent.stress_level = clamped;
            }
            ExosystemPath::ParentScheduleFlexibility => {
                let parent = self
                    .parent_work_environment
                    .get_or_insert_with(ParentWorkQuality::default);
                parent.schedule_flexibility = clamped;
            }
            ExosystemPath::ParentIncomeStability => {
                let parent = self
                    .parent_work_environment
                    .get_or_insert_with(ParentWorkQuality::default);
                parent.income_stability = clamped;
            }
        }
    }

    /// Computes the average parent capacity.
    ///
    /// Returns None if no parent work environment is set.
    #[must_use]
    pub fn parent_capacity(&self) -> Option<f64> {
        self.parent_work_environment
            .as_ref()
            .map(|p| p.parent_capacity())
    }

    /// Returns whether this context has a parent work environment set.
    #[must_use]
    pub fn has_parent_work_environment(&self) -> bool {
        self.parent_work_environment.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- ParentWorkQuality tests ---

    #[test]
    fn parent_work_quality_default() {
        let pwq = ParentWorkQuality::default();
        assert!((pwq.stress_level - 0.3).abs() < f64::EPSILON);
        assert!((pwq.schedule_flexibility - 0.5).abs() < f64::EPSILON);
        assert!((pwq.income_stability - 0.6).abs() < f64::EPSILON);
    }

    #[test]
    fn parent_work_quality_parent_capacity() {
        let pwq = ParentWorkQuality::default();
        let capacity = pwq.parent_capacity();
        // (1.0 - (0.3 * 0.3 + (1.0 - 0.5) * 0.2)) * 0.6 = 0.81 * 0.6 = 0.486
        assert!((capacity - 0.486).abs() < 0.01);
    }

    #[test]
    fn parent_work_quality_high_stress_low_capacity() {
        let pwq = ParentWorkQuality {
            stress_level: 1.0,
            schedule_flexibility: 0.0,
            income_stability: 0.5,
        };
        let capacity = pwq.parent_capacity();
        // (1.0 - (1.0 * 0.3 + (1.0 - 0.0) * 0.2)) * 0.5 = 0.5 * 0.5 = 0.25
        assert!((capacity - 0.25).abs() < f64::EPSILON);
    }

    #[test]
    fn parent_work_quality_clone_eq() {
        let pwq1 = ParentWorkQuality::default();
        let pwq2 = pwq1.clone();
        assert_eq!(pwq1, pwq2);
    }

    // --- ExosystemContext tests ---

    #[test]
    fn exosystem_context_default() {
        let exo = ExosystemContext::default();
        assert!((exo.health_system_access - 0.6).abs() < f64::EPSILON);
        assert!((exo.educational_system_quality - 0.6).abs() < f64::EPSILON);
        assert!((exo.community_services_availability - 0.5).abs() < f64::EPSILON);
        assert!((exo.resource_availability - 0.6).abs() < f64::EPSILON);
        assert!((exo.institutional_support - 0.5).abs() < f64::EPSILON);
        assert!(exo.parent_work_environment.is_none());
    }

    #[test]
    fn exosystem_context_new() {
        let exo = ExosystemContext::new();
        assert!((exo.health_system_access - 0.6).abs() < f64::EPSILON);
    }

    #[test]
    fn exosystem_get_value_all_paths() {
        let exo = ExosystemContext::default();
        for path in ExosystemPath::all() {
            let value = exo.get_value(&path);
            assert!(value >= 0.0 && value <= 1.0);
        }
    }

    #[test]
    fn exosystem_set_value() {
        let mut exo = ExosystemContext::default();
        exo.set_value(&ExosystemPath::HealthSystemAccess, 0.9);
        assert!((exo.health_system_access - 0.9).abs() < f64::EPSILON);
    }

    #[test]
    fn exosystem_set_value_clamped() {
        let mut exo = ExosystemContext::default();
        exo.set_value(&ExosystemPath::ResourceAvailability, 1.5);
        assert!((exo.resource_availability - 1.0).abs() < f64::EPSILON);

        exo.set_value(&ExosystemPath::ResourceAvailability, -0.5);
        assert!((exo.resource_availability - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn exosystem_parent_work_stress_creates_environment() {
        let mut exo = ExosystemContext::default();
        assert!(!exo.has_parent_work_environment());

        exo.set_value(&ExosystemPath::ParentWorkStress, 0.7);
        assert!(exo.has_parent_work_environment());
        assert!((exo.get_value(&ExosystemPath::ParentWorkStress) - 0.7).abs() < f64::EPSILON);
    }

    #[test]
    fn exosystem_parent_flexibility_creates_environment() {
        let mut exo = ExosystemContext::default();
        exo.set_value(&ExosystemPath::ParentScheduleFlexibility, 0.8);
        assert!(exo.has_parent_work_environment());
        assert!(
            (exo.get_value(&ExosystemPath::ParentScheduleFlexibility) - 0.8).abs() < f64::EPSILON
        );
    }

    #[test]
    fn exosystem_parent_income_creates_environment() {
        let mut exo = ExosystemContext::default();
        exo.set_value(&ExosystemPath::ParentIncomeStability, 0.9);
        assert!(exo.has_parent_work_environment());
        assert!((exo.get_value(&ExosystemPath::ParentIncomeStability) - 0.9).abs() < f64::EPSILON);
    }

    #[test]
    fn exosystem_parent_capacity_none() {
        let exo = ExosystemContext::default();
        assert!(exo.parent_capacity().is_none());
    }

    #[test]
    fn exosystem_parent_capacity_some() {
        let mut exo = ExosystemContext::default();
        exo.parent_work_environment = Some(ParentWorkQuality::default());
        let capacity = exo.parent_capacity();
        assert!(capacity.is_some());
        assert!(capacity.unwrap() > 0.4);
    }

    #[test]
    fn exosystem_get_parent_values_no_environment() {
        let exo = ExosystemContext::default();
        // Without parent environment, should return defaults
        assert!((exo.get_value(&ExosystemPath::ParentWorkStress) - 0.0).abs() < f64::EPSILON);
        assert!(
            (exo.get_value(&ExosystemPath::ParentScheduleFlexibility) - 0.5).abs() < f64::EPSILON
        );
        assert!((exo.get_value(&ExosystemPath::ParentIncomeStability) - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn exosystem_set_all_values() {
        let mut exo = ExosystemContext::default();

        for path in ExosystemPath::all() {
            exo.set_value(&path, 0.75);
        }

        // Check all direct values are set
        assert!((exo.health_system_access - 0.75).abs() < f64::EPSILON);
        assert!((exo.educational_system_quality - 0.75).abs() < f64::EPSILON);
        assert!((exo.community_services_availability - 0.75).abs() < f64::EPSILON);
        assert!((exo.resource_availability - 0.75).abs() < f64::EPSILON);
        assert!((exo.institutional_support - 0.75).abs() < f64::EPSILON);

        // Parent values should also be set
        assert!(exo.has_parent_work_environment());
    }

    #[test]
    fn exosystem_clone_eq() {
        let exo1 = ExosystemContext::default();
        let exo2 = exo1.clone();
        assert_eq!(exo1, exo2);
    }

    #[test]
    fn exosystem_debug() {
        let exo = ExosystemContext::default();
        let debug = format!("{:?}", exo);
        assert!(debug.contains("ExosystemContext"));
    }

    #[test]
    fn exosystem_with_parent_environment() {
        let mut exo = ExosystemContext::default();
        exo.parent_work_environment = Some(ParentWorkQuality {
            stress_level: 0.8,
            schedule_flexibility: 0.2,
            income_stability: 0.4,
        });

        assert!((exo.get_value(&ExosystemPath::ParentWorkStress) - 0.8).abs() < f64::EPSILON);
        assert!(
            (exo.get_value(&ExosystemPath::ParentScheduleFlexibility) - 0.2).abs() < f64::EPSILON
        );
        assert!((exo.get_value(&ExosystemPath::ParentIncomeStability) - 0.4).abs() < f64::EPSILON);
    }

    // --- Required Phase 7 tests ---

    #[test]
    fn exosystem_creation_default() {
        // ExosystemContext creates with defaults
        let exo = ExosystemContext::default();

        // Verify all defaults per spec
        assert!((exo.health_system_access - 0.6).abs() < f64::EPSILON);
        assert!((exo.educational_system_quality - 0.6).abs() < f64::EPSILON);
        assert!((exo.community_services_availability - 0.5).abs() < f64::EPSILON);
        assert!((exo.resource_availability - 0.6).abs() < f64::EPSILON);
        assert!((exo.institutional_support - 0.5).abs() < f64::EPSILON);

        // No parent work environment by default
        assert!(!exo.has_parent_work_environment());
        assert!(exo.parent_work_environment.is_none());

        // new() should produce same defaults
        let exo_new = ExosystemContext::new();
        assert_eq!(exo, exo_new);
    }

    #[test]
    fn exosystem_parent_job_loss_affects_child() {
        // Parent job loss (high stress, low flexibility, low income) affects child
        // through reduced parent capacity

        // Normal parent work environment
        let normal_parent = ParentWorkQuality {
            stress_level: 0.3,
            schedule_flexibility: 0.6,
            income_stability: 0.7,
        };

        // Parent after job loss
        let job_loss_parent = ParentWorkQuality {
            stress_level: 0.9,         // High stress
            schedule_flexibility: 0.2, // Less flexibility (job hunting)
            income_stability: 0.1,     // Very low income stability
        };

        let normal_capacity = normal_parent.parent_capacity();
        let job_loss_capacity = job_loss_parent.parent_capacity();

        // Job loss should reduce parent capacity
        assert!(job_loss_capacity < normal_capacity);

        // Verify job loss has significant impact
        // Normal: (1.0 - (0.3 * 0.3 + (1.0 - 0.6) * 0.2)) * 0.7 = 0.83 * 0.7 = 0.581
        // Job loss: (1.0 - (0.9 * 0.3 + (1.0 - 0.2) * 0.2)) * 0.1 = 0.57 * 0.1 = 0.057
        assert!((job_loss_capacity - 0.057).abs() < 0.01);
        assert!((normal_capacity - 0.581).abs() < 0.01);
    }

    #[test]
    fn exosystem_local_policy_modifies_resources() {
        // Policy changes affect resource availability
        let mut exo = ExosystemContext::default();

        // Initial resource availability
        let initial_resources = exo.resource_availability;
        assert!((initial_resources - 0.6).abs() < f64::EPSILON);

        // Simulate policy that reduces community resources (e.g., budget cuts)
        exo.set_value(&ExosystemPath::ResourceAvailability, 0.3);
        exo.set_value(&ExosystemPath::CommunityServicesAvailability, 0.2);

        // Resources should be reduced
        assert!((exo.resource_availability - 0.3).abs() < f64::EPSILON);
        assert!((exo.community_services_availability - 0.2).abs() < f64::EPSILON);

        // Simulate positive policy change
        exo.set_value(&ExosystemPath::ResourceAvailability, 0.9);
        exo.set_value(&ExosystemPath::CommunityServicesAvailability, 0.85);

        // Resources should be increased
        assert!((exo.resource_availability - 0.9).abs() < f64::EPSILON);
        assert!((exo.community_services_availability - 0.85).abs() < f64::EPSILON);
    }

    #[test]
    fn exosystem_institutional_support_buffer() {
        // Institutional support buffers stress
        // Higher support = more stress buffering capacity

        let mut low_support = ExosystemContext::default();
        low_support.institutional_support = 0.2;

        let mut high_support = ExosystemContext::default();
        high_support.institutional_support = 0.9;

        // Verify values are set correctly
        assert!(
            (low_support.get_value(&ExosystemPath::InstitutionalSupport) - 0.2).abs()
                < f64::EPSILON
        );
        assert!(
            (high_support.get_value(&ExosystemPath::InstitutionalSupport) - 0.9).abs()
                < f64::EPSILON
        );

        // The stress buffer effect is computed during internal state computation:
        // exo_stress_buffer = ((institutional_support - 0.5) * 0.1)
        // Low support: (0.2 - 0.5) * 0.1 = -0.03 (adds stress)
        // High support: (0.9 - 0.5) * 0.1 = 0.04 (buffers stress)

        let low_buffer_effect = (low_support.institutional_support - 0.5) * 0.1;
        let high_buffer_effect = (high_support.institutional_support - 0.5) * 0.1;

        assert!(low_buffer_effect < 0.0);
        assert!(high_buffer_effect > 0.0);

        // The difference in effect should be significant
        let buffer_difference = high_buffer_effect - low_buffer_effect;
        assert!((buffer_difference - 0.07).abs() < 0.001);
    }
}
