//! Trust context representing situational factors that moderate trust decisions.
//!
//! Per trust theory, context includes multiple dimensions that affect willingness
//! to be vulnerable. Rather than collapsing these to a single scalar multiplier,
//! we track them explicitly to preserve information about context type.

/// Situational context that moderates trust decisions.
///
/// Each dimension represents a factor that can increase or decrease
/// willingness to trust in a specific situation.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::relationship::TrustContext;
///
/// // High institutional support with time pressure
/// let context = TrustContext::new()
///     .with_institutional_support(0.8)
///     .with_time_pressure(0.7);
///
/// // Context multiplier reflects these factors
/// assert!(context.compute_multiplier() > 0.5);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct TrustContext {
    /// Social norms supporting or discouraging trust (0-1).
    /// Higher values indicate norms that encourage trusting behavior.
    social_norms: f32,

    /// Institutional safeguards that reduce risk (0-1).
    /// Higher values indicate stronger protections (contracts, insurance, etc.).
    institutional_safeguards: f32,

    /// Time pressure on the decision (0-1).
    /// Higher values indicate more pressure, which can reduce deliberation.
    time_pressure: f32,

    /// Institutional support for the relationship (0-1).
    /// Higher values indicate formal backing (employer, organization, etc.).
    institutional_support: f32,

    /// Cultural expectations about trust in this context (0-1).
    /// Higher values indicate culture expects/encourages trust.
    cultural_expectations: f32,
}

impl TrustContext {
    /// Creates a new TrustContext with neutral default values.
    ///
    /// All dimensions default to 0.5 (neutral effect).
    #[must_use]
    pub fn new() -> Self {
        TrustContext {
            social_norms: 0.5,
            institutional_safeguards: 0.5,
            time_pressure: 0.5,
            institutional_support: 0.5,
            cultural_expectations: 0.5,
        }
    }

    /// Sets the social norms dimension.
    #[must_use]
    pub fn with_social_norms(mut self, value: f32) -> Self {
        self.social_norms = value.clamp(0.0, 1.0);
        self
    }

    /// Sets the institutional safeguards dimension.
    #[must_use]
    pub fn with_institutional_safeguards(mut self, value: f32) -> Self {
        self.institutional_safeguards = value.clamp(0.0, 1.0);
        self
    }

    /// Sets the time pressure dimension.
    #[must_use]
    pub fn with_time_pressure(mut self, value: f32) -> Self {
        self.time_pressure = value.clamp(0.0, 1.0);
        self
    }

    /// Sets the institutional support dimension.
    #[must_use]
    pub fn with_institutional_support(mut self, value: f32) -> Self {
        self.institutional_support = value.clamp(0.0, 1.0);
        self
    }

    /// Sets the cultural expectations dimension.
    #[must_use]
    pub fn with_cultural_expectations(mut self, value: f32) -> Self {
        self.cultural_expectations = value.clamp(0.0, 1.0);
        self
    }

    /// Returns the social norms value.
    #[must_use]
    pub fn social_norms(&self) -> f32 {
        self.social_norms
    }

    /// Returns the institutional safeguards value.
    #[must_use]
    pub fn institutional_safeguards(&self) -> f32 {
        self.institutional_safeguards
    }

    /// Returns the time pressure value.
    #[must_use]
    pub fn time_pressure(&self) -> f32 {
        self.time_pressure
    }

    /// Returns the institutional support value.
    #[must_use]
    pub fn institutional_support(&self) -> f32 {
        self.institutional_support
    }

    /// Returns the cultural expectations value.
    #[must_use]
    pub fn cultural_expectations(&self) -> f32 {
        self.cultural_expectations
    }

    /// Computes the context multiplier from all dimensions.
    ///
    /// The multiplier is computed as a weighted combination of all dimensions,
    /// scaled to the range [0.5, 1.5] to moderate trust without dominating it.
    ///
    /// Factors that increase trust: social_norms, institutional_safeguards,
    /// institutional_support, cultural_expectations.
    ///
    /// Factors that decrease trust deliberation: time_pressure (high pressure
    /// may lead to snap judgments).
    ///
    /// # Returns
    ///
    /// A multiplier in the range [0.5, 1.5] where:
    /// - 1.0 = neutral context
    /// - < 1.0 = context discourages trust
    /// - > 1.0 = context encourages trust
    #[must_use]
    pub fn compute_multiplier(&self) -> f32 {
        // Weight the dimensions
        // Trust-encouraging factors: norms, safeguards, support, culture
        // Deliberation factor: time_pressure (high pressure = less careful decision)
        let trust_encouragement = (self.social_norms
            + self.institutional_safeguards
            + self.institutional_support
            + self.cultural_expectations)
            / 4.0;

        // Time pressure has a small negative effect on trust quality
        // (rushed decisions may be less calibrated)
        let pressure_penalty = self.time_pressure * 0.1;

        // Scale to [0.5, 1.5] range: base of 0.5 + up to 1.0 from encouragement
        let multiplier = 0.5 + trust_encouragement - pressure_penalty;

        multiplier.clamp(0.5, 1.5)
    }

    /// Creates a TrustContext from a legacy scalar multiplier.
    ///
    /// This is for backward compatibility with code that uses the old
    /// single-value context_multiplier pattern.
    #[must_use]
    pub fn from_multiplier(multiplier: f32) -> Self {
        let multiplier = multiplier.clamp(0.5, 1.5);
        // Reverse engineer: if multiplier = 0.5 + trust_encouragement - pressure_penalty
        // Assume neutral time_pressure (0.5), so pressure_penalty = 0.05
        // Then trust_encouragement = multiplier - 0.5 + 0.05 = multiplier - 0.45
        let trust_encouragement = (multiplier - 0.45).clamp(0.0, 1.0);

        TrustContext {
            social_norms: trust_encouragement,
            institutional_safeguards: trust_encouragement,
            time_pressure: 0.5,
            institutional_support: trust_encouragement,
            cultural_expectations: trust_encouragement,
        }
    }
}

impl Default for TrustContext {
    fn default() -> Self {
        TrustContext::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_neutral_context() {
        let ctx = TrustContext::new();
        assert!((ctx.social_norms() - 0.5).abs() < f32::EPSILON);
        assert!((ctx.institutional_safeguards() - 0.5).abs() < f32::EPSILON);
        assert!((ctx.time_pressure() - 0.5).abs() < f32::EPSILON);
        assert!((ctx.institutional_support() - 0.5).abs() < f32::EPSILON);
        assert!((ctx.cultural_expectations() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn default_equals_new() {
        let d = TrustContext::default();
        let n = TrustContext::new();
        assert_eq!(d, n);
    }

    #[test]
    fn with_methods_set_values() {
        let ctx = TrustContext::new()
            .with_social_norms(0.8)
            .with_institutional_safeguards(0.7)
            .with_time_pressure(0.3)
            .with_institutional_support(0.9)
            .with_cultural_expectations(0.6);

        assert!((ctx.social_norms() - 0.8).abs() < f32::EPSILON);
        assert!((ctx.institutional_safeguards() - 0.7).abs() < f32::EPSILON);
        assert!((ctx.time_pressure() - 0.3).abs() < f32::EPSILON);
        assert!((ctx.institutional_support() - 0.9).abs() < f32::EPSILON);
        assert!((ctx.cultural_expectations() - 0.6).abs() < f32::EPSILON);
    }

    #[test]
    fn with_methods_clamp_values() {
        let ctx = TrustContext::new()
            .with_social_norms(1.5)
            .with_time_pressure(-0.5);

        assert!((ctx.social_norms() - 1.0).abs() < f32::EPSILON);
        assert!(ctx.time_pressure().abs() < f32::EPSILON);
    }

    #[test]
    fn neutral_context_gives_neutral_multiplier() {
        let ctx = TrustContext::new();
        let multiplier = ctx.compute_multiplier();
        // Neutral values (0.5 each) should give approximately 1.0
        // trust_encouragement = 0.5, pressure_penalty = 0.05
        // multiplier = 0.5 + 0.5 - 0.05 = 0.95
        assert!((multiplier - 0.95).abs() < 0.01);
    }

    #[test]
    fn high_support_increases_multiplier() {
        let low = TrustContext::new().with_institutional_support(0.0);
        let high = TrustContext::new().with_institutional_support(1.0);

        assert!(high.compute_multiplier() > low.compute_multiplier());
    }

    #[test]
    fn high_pressure_decreases_multiplier() {
        let low_pressure = TrustContext::new().with_time_pressure(0.0);
        let high_pressure = TrustContext::new().with_time_pressure(1.0);

        assert!(low_pressure.compute_multiplier() > high_pressure.compute_multiplier());
    }

    #[test]
    fn multiplier_clamped_to_range() {
        // All high values
        let high_ctx = TrustContext::new()
            .with_social_norms(1.0)
            .with_institutional_safeguards(1.0)
            .with_time_pressure(0.0)
            .with_institutional_support(1.0)
            .with_cultural_expectations(1.0);

        assert!(high_ctx.compute_multiplier() <= 1.5);

        // All low values
        let low_ctx = TrustContext::new()
            .with_social_norms(0.0)
            .with_institutional_safeguards(0.0)
            .with_time_pressure(1.0)
            .with_institutional_support(0.0)
            .with_cultural_expectations(0.0);

        assert!(low_ctx.compute_multiplier() >= 0.5);
    }

    #[test]
    fn from_multiplier_backward_compat() {
        let ctx = TrustContext::from_multiplier(1.0);
        let computed = ctx.compute_multiplier();
        // Should be approximately 1.0
        assert!((computed - 1.0).abs() < 0.1);
    }

    #[test]
    fn clone_and_equality() {
        let c1 = TrustContext::new().with_social_norms(0.7);
        let c2 = c1.clone();
        assert_eq!(c1, c2);
    }

    #[test]
    fn debug_format() {
        let ctx = TrustContext::new();
        let debug = format!("{:?}", ctx);
        assert!(debug.contains("TrustContext"));
    }
}
