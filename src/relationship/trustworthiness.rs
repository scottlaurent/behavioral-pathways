//! Trustworthiness factors based on Mayer's trust model.
//!
//! Per Mayer's model, trustworthiness has three components:
//! - Competence (ability): Perceived ability to perform tasks, domain-specific
//! - Benevolence: Perceived caring and good intentions toward the trustor
//! - Integrity: Perceived adherence to principles acceptable to the trustor
//!
//! Competence is domain-specific per Mayer: being competent in one domain
//! (e.g., medical advice) does not imply competence in another (e.g., financial advice).

use std::collections::HashMap;

use crate::enums::{LifeDomain, TrustPath};
use crate::relationship::{AntecedentDirection, AntecedentType, TrustAntecedent};
use crate::state::StateValue;
use crate::types::{Duration, Timestamp};

/// Decay half-life for perceived competence (30 days).
const COMPETENCE_DECAY_HALF_LIFE: Duration = Duration::days(30);

/// Decay half-life for perceived benevolence (14 days).
const BENEVOLENCE_DECAY_HALF_LIFE: Duration = Duration::days(14);

/// Decay half-life for perceived integrity (60 days).
const INTEGRITY_DECAY_HALF_LIFE: Duration = Duration::days(60);

/// Default base value for trustworthiness factors.
const DEFAULT_BASE: f32 = 0.3;

/// Exponential smoothing factor for antecedent recomputation.
const ANTECEDENT_SMOOTHING_ALPHA: f32 = 0.4;

/// Weight multiplier for negative antecedents.
const NEGATIVE_ANTECEDENT_WEIGHT: f32 = 2.5;

/// Weight multiplier for positive antecedents during rebuilding.
const REBUILDING_POSITIVE_WEIGHT: f32 = 0.7;

/// Rebuilding window after a negative antecedent.
const REBUILDING_WINDOW: Duration = Duration::days(180);

/// Half-life for antecedent temporal decay in days.
/// Antecedent impact decays exponentially: impact = base * exp(-age_days / half_life_days).
/// Per trust-antecedents spec, ~180 days half-life means an antecedent from
/// 6 months ago has ~50% the impact of a recent one.
const ANTECEDENT_DECAY_HALF_LIFE_DAYS: f64 = 180.0;

/// Perceived trustworthiness factors of one entity toward another.
///
/// These represent the trustor's perceptions of the trustee's:
/// - **Competence**: Ability to perform in the relevant domain (domain-specific)
/// - **Benevolence**: Caring and good intentions toward the trustor
/// - **Integrity**: Adherence to principles the trustor finds acceptable
///
/// Competence is tracked per-domain per Mayer's model: competence in one
/// domain (e.g., medical) does not imply competence in another (e.g., financial).
///
/// Each factor uses the StateValue pattern with decay. Competence perceptions
/// decay slowest, benevolence faster (requires ongoing demonstration),
/// and integrity slowest of all.
///
/// # Examples
///
/// ```
/// use behavioral_pathways::relationship::TrustworthinessFactors;
/// use behavioral_pathways::enums::{TrustPath, LifeDomain};
///
/// let mut factors = TrustworthinessFactors::new();
///
/// // Demonstrate competence in a specific domain
/// factors.add_competence_delta_in(LifeDomain::Work, 0.2);
/// assert!(factors.competence_in(LifeDomain::Work) > 0.4);
///
/// // Other domains remain at default
/// assert!((factors.competence_in(LifeDomain::Health) - 0.3).abs() < 0.01);
///
/// // Get overall trustworthiness
/// let overall = factors.overall();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct TrustworthinessFactors {
    /// Perceived ability to perform tasks competently, per domain.
    /// Per Mayer: competence is domain-specific.
    competence: HashMap<LifeDomain, StateValue>,

    /// Perceived caring and benevolent intentions.
    benevolence: StateValue,

    /// Perceived adherence to principles and values.
    integrity: StateValue,
}

impl TrustworthinessFactors {
    /// Creates new TrustworthinessFactors with default values.
    ///
    /// Default base is 0.3 for all factors (slight trust baseline).
    /// Competence is initialized for all domains with the default base.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::relationship::TrustworthinessFactors;
    /// use behavioral_pathways::enums::LifeDomain;
    ///
    /// let factors = TrustworthinessFactors::new();
    /// assert!((factors.competence_in(LifeDomain::Work) - 0.3).abs() < f32::EPSILON);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        TrustworthinessFactors {
            competence: Self::create_domain_competence_map(DEFAULT_BASE),
            benevolence: StateValue::new(DEFAULT_BASE)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(BENEVOLENCE_DECAY_HALF_LIFE),
            integrity: StateValue::new(DEFAULT_BASE)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(INTEGRITY_DECAY_HALF_LIFE),
        }
    }

    /// Creates TrustworthinessFactors with specified base values.
    ///
    /// The competence base is applied to all domains uniformly.
    ///
    /// # Arguments
    ///
    /// * `competence` - Base competence perception (0-1), applied to all domains
    /// * `benevolence` - Base benevolence perception (0-1)
    /// * `integrity` - Base integrity perception (0-1)
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::relationship::TrustworthinessFactors;
    /// use behavioral_pathways::enums::LifeDomain;
    ///
    /// let factors = TrustworthinessFactors::with_bases(0.5, 0.6, 0.7);
    /// assert!((factors.competence_in(LifeDomain::Work) - 0.5).abs() < f32::EPSILON);
    /// ```
    #[must_use]
    pub fn with_bases(competence: f32, benevolence: f32, integrity: f32) -> Self {
        TrustworthinessFactors {
            competence: Self::create_domain_competence_map(competence),
            benevolence: StateValue::new(benevolence)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(BENEVOLENCE_DECAY_HALF_LIFE),
            integrity: StateValue::new(integrity)
                .with_bounds(0.0, 1.0)
                .with_decay_half_life(INTEGRITY_DECAY_HALF_LIFE),
        }
    }

    /// Creates a competence map with the given base value for all domains.
    fn create_domain_competence_map(base: f32) -> HashMap<LifeDomain, StateValue> {
        let domains = [
            LifeDomain::Work,
            LifeDomain::Academic,
            LifeDomain::Social,
            LifeDomain::Athletic,
            LifeDomain::Creative,
            LifeDomain::Financial,
            LifeDomain::Health,
            LifeDomain::Relationship,
        ];

        domains
            .into_iter()
            .map(|domain| {
                let state = StateValue::new(base)
                    .with_bounds(0.0, 1.0)
                    .with_decay_half_life(COMPETENCE_DECAY_HALF_LIFE);
                (domain, state)
            })
            .collect()
    }

    // Effective value accessors

    /// Returns the effective competence perception in a specific domain.
    ///
    /// Per Mayer's model, competence is domain-specific: being competent
    /// in medical advice does not imply competence in financial advice.
    ///
    /// # Arguments
    ///
    /// * `domain` - The life domain to query competence for
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::relationship::TrustworthinessFactors;
    /// use behavioral_pathways::enums::LifeDomain;
    ///
    /// let factors = TrustworthinessFactors::new();
    /// let work_competence = factors.competence_in(LifeDomain::Work);
    /// ```
    #[must_use]
    pub fn competence_in(&self, domain: LifeDomain) -> f32 {
        self.competence
            .get(&domain)
            .map(|sv| sv.effective())
            .unwrap_or(DEFAULT_BASE)
    }

    /// Returns the average competence across all domains.
    ///
    /// This is provided for backward compatibility and general queries.
    /// For domain-specific trust decisions, use `competence_in(domain)`.
    #[must_use]
    pub fn competence_effective(&self) -> f32 {
        // Competence is always initialized with all LifeDomain entries in new(),
        // so it's never empty and division is safe.
        let sum: f32 = self.competence.values().map(|sv| sv.effective()).sum();
        sum / self.competence.len() as f32
    }

    /// Returns the effective benevolence perception (base + delta).
    #[must_use]
    pub fn benevolence_effective(&self) -> f32 {
        self.benevolence.effective()
    }

    /// Returns the effective integrity perception (base + delta).
    #[must_use]
    pub fn integrity_effective(&self) -> f32 {
        self.integrity.effective()
    }

    /// Returns the overall trustworthiness (average of all factors).
    ///
    /// Uses average competence across domains. For domain-specific trust
    /// decisions, use `competence_in(domain)` directly.
    ///
    /// This is a simple average. For weighted combinations, use
    /// individual factors with custom weights.
    ///
    /// # Examples
    ///
    /// ```
    /// use behavioral_pathways::relationship::TrustworthinessFactors;
    ///
    /// let factors = TrustworthinessFactors::with_bases(0.6, 0.6, 0.6);
    /// assert!((factors.overall() - 0.6).abs() < f32::EPSILON);
    /// ```
    #[must_use]
    pub fn overall(&self) -> f32 {
        (self.competence_effective() + self.benevolence_effective() + self.integrity_effective())
            / 3.0
    }

    // StateValue references

    /// Returns a reference to the competence StateValue for a specific domain.
    ///
    /// Returns None if the domain is not tracked (should not happen with default init).
    #[must_use]
    pub fn competence(&self, domain: LifeDomain) -> Option<&StateValue> {
        self.competence.get(&domain)
    }

    /// Returns a reference to the benevolence StateValue.
    #[must_use]
    pub fn benevolence(&self) -> &StateValue {
        &self.benevolence
    }

    /// Returns a reference to the integrity StateValue.
    #[must_use]
    pub fn integrity(&self) -> &StateValue {
        &self.integrity
    }

    /// Returns a mutable reference to the competence StateValue for a domain.
    ///
    /// Returns None if the domain is not tracked.
    pub fn competence_mut(&mut self, domain: LifeDomain) -> Option<&mut StateValue> {
        self.competence.get_mut(&domain)
    }

    /// Returns a mutable reference to the benevolence StateValue.
    pub fn benevolence_mut(&mut self) -> &mut StateValue {
        &mut self.benevolence
    }

    /// Returns a mutable reference to the integrity StateValue.
    pub fn integrity_mut(&mut self) -> &mut StateValue {
        &mut self.integrity
    }

    /// Returns a reference to the StateValue for the given trust path.
    ///
    /// For Competence, returns the Work domain by default.
    /// Use `competence(domain)` for domain-specific access.
    /// Returns None for computed paths like SupportWillingness.
    #[must_use]
    pub fn get(&self, path: TrustPath) -> Option<&StateValue> {
        match path {
            TrustPath::Competence => self.competence.get(&LifeDomain::Work),
            TrustPath::Benevolence => Some(&self.benevolence),
            TrustPath::Integrity => Some(&self.integrity),
            TrustPath::SupportWillingness => None, // Computed, not stored
        }
    }

    /// Returns a mutable reference to the StateValue for the given trust path.
    ///
    /// For Competence, returns the Work domain by default.
    /// Use `competence_mut(domain)` for domain-specific access.
    /// Returns None for computed paths like SupportWillingness.
    pub fn get_mut(&mut self, path: TrustPath) -> Option<&mut StateValue> {
        match path {
            TrustPath::Competence => self.competence.get_mut(&LifeDomain::Work),
            TrustPath::Benevolence => Some(&mut self.benevolence),
            TrustPath::Integrity => Some(&mut self.integrity),
            TrustPath::SupportWillingness => None, // Computed, not stored
        }
    }

    // Delta modifiers

    /// Adds to the competence delta for a specific domain.
    ///
    /// Per Mayer's model, competence is domain-specific.
    pub fn add_competence_delta_in(&mut self, domain: LifeDomain, amount: f32) {
        if let Some(sv) = self.competence.get_mut(&domain) {
            sv.add_delta(amount);
        }
    }

    /// Adds to the competence delta for all domains.
    ///
    /// This is provided for backward compatibility. For domain-specific
    /// updates, use `add_competence_delta_in(domain, amount)`.
    pub fn add_competence_delta(&mut self, amount: f32) {
        for sv in self.competence.values_mut() {
            sv.add_delta(amount);
        }
    }

    /// Adds to the benevolence delta.
    pub fn add_benevolence_delta(&mut self, amount: f32) {
        self.benevolence.add_delta(amount);
    }

    /// Adds to the integrity delta.
    pub fn add_integrity_delta(&mut self, amount: f32) {
        self.integrity.add_delta(amount);
    }

    /// Adds to the delta for the specified trust path.
    ///
    /// For Competence, applies to the Work domain by default.
    /// Use `add_competence_delta_in(domain, amount)` for domain-specific updates.
    /// Does nothing for computed paths like SupportWillingness.
    pub fn add_delta(&mut self, path: TrustPath, amount: f32) {
        match path {
            TrustPath::Competence => {
                if let Some(sv) = self.competence.get_mut(&LifeDomain::Work) {
                    sv.add_delta(amount);
                }
            }
            TrustPath::Benevolence => self.benevolence.add_delta(amount),
            TrustPath::Integrity => self.integrity.add_delta(amount),
            TrustPath::SupportWillingness => {} // Computed, cannot modify
        }
    }

    /// Recomputes trustworthiness deltas from trust antecedent history.
    ///
    /// Uses exponential smoothing with asymmetric weights:
    /// - Negative antecedents are weighted by 2.5x
    /// - Positive antecedents are weighted by 0.7x during rebuilding
    /// - Temporal decay: older antecedents have less impact (half-life 180 days)
    ///
    /// For ability antecedents, competence is updated per-domain when
    /// a life_domain is specified. Antecedents without a life_domain
    /// update all domains (backward compatibility).
    pub fn recompute_from_antecedents(&mut self, antecedents: &[TrustAntecedent]) {
        if antecedents.is_empty() {
            self.reset_deltas();
            return;
        }

        let mut sorted = antecedents.to_vec();
        sorted.sort_by_key(|entry| entry.timestamp());

        // Use the most recent antecedent as reference for temporal decay
        let reference_ts = sorted.last().map(|a| a.timestamp()).unwrap();

        // Track EMA per domain for competence
        let mut competence_emas: HashMap<LifeDomain, f32> = HashMap::new();
        let mut benevolence_ema = 0.0;
        let mut integrity_ema = 0.0;
        let mut last_negative: Option<Timestamp> = None;

        for antecedent in sorted {
            // Compute temporal decay based on age relative to reference
            let age_duration = reference_ts - antecedent.timestamp();
            let age_days = age_duration.as_seconds() as f64 / 86400.0;
            let decay_factor =
                (-age_days * std::f64::consts::LN_2 / ANTECEDENT_DECAY_HALF_LIFE_DAYS).exp() as f32;

            let weight = match antecedent.direction() {
                AntecedentDirection::Negative => {
                    last_negative = Some(antecedent.timestamp());
                    NEGATIVE_ANTECEDENT_WEIGHT
                }
                AntecedentDirection::Positive => {
                    if let Some(last_negative_ts) = last_negative {
                        if antecedent.timestamp() - last_negative_ts <= REBUILDING_WINDOW {
                            REBUILDING_POSITIVE_WEIGHT
                        } else {
                            1.0
                        }
                    } else {
                        1.0
                    }
                }
            };

            // Apply temporal decay to the weighted impact
            let signed = match antecedent.direction() {
                AntecedentDirection::Positive => antecedent.magnitude(),
                AntecedentDirection::Negative => -antecedent.magnitude(),
            } * weight
                * decay_factor;

            match antecedent.antecedent_type() {
                AntecedentType::Ability => {
                    if let Some(domain) = antecedent.life_domain() {
                        // Update specific domain
                        let current = competence_emas.get(&domain).copied().unwrap_or(0.0);
                        competence_emas.insert(domain, update_ema(current, signed));
                    } else {
                        // No domain specified - update all domains (backward compat)
                        for domain in self.competence.keys().cloned().collect::<Vec<_>>() {
                            let current = competence_emas.get(&domain).copied().unwrap_or(0.0);
                            competence_emas.insert(domain, update_ema(current, signed));
                        }
                    }
                }
                AntecedentType::Benevolence => {
                    benevolence_ema = update_ema(benevolence_ema, signed);
                }
                AntecedentType::Integrity => {
                    integrity_ema = update_ema(integrity_ema, signed);
                }
            }
        }

        // Apply EMAs to competence domains
        for (domain, ema) in competence_emas {
            if let Some(sv) = self.competence.get_mut(&domain) {
                apply_ema_to_state_value(sv, ema);
            }
        }
        apply_ema_to_state_value(&mut self.benevolence, benevolence_ema);
        apply_ema_to_state_value(&mut self.integrity, integrity_ema);
    }

    // Decay

    /// Applies decay to all trustworthiness factors over the specified duration.
    pub fn apply_decay(&mut self, elapsed: Duration) {
        for sv in self.competence.values_mut() {
            sv.apply_decay(elapsed);
        }
        self.benevolence.apply_decay(elapsed);
        self.integrity.apply_decay(elapsed);
    }

    /// Resets all deltas to zero.
    pub fn reset_deltas(&mut self) {
        for sv in self.competence.values_mut() {
            sv.reset_delta();
        }
        self.benevolence.reset_delta();
        self.integrity.reset_delta();
    }
}

fn update_ema(previous: f32, value: f32) -> f32 {
    (1.0 - ANTECEDENT_SMOOTHING_ALPHA) * previous + ANTECEDENT_SMOOTHING_ALPHA * value
}

fn apply_ema_to_state_value(value: &mut StateValue, ema: f32) {
    let base = value.base();
    let target = (base + ema).clamp(0.0, 1.0);
    value.set_delta(target - base);
}

impl Default for TrustworthinessFactors {
    fn default() -> Self {
        TrustworthinessFactors::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_default_values() {
        let factors = TrustworthinessFactors::new();
        assert!((factors.competence_effective() - DEFAULT_BASE).abs() < f32::EPSILON);
        assert!((factors.benevolence_effective() - DEFAULT_BASE).abs() < f32::EPSILON);
        assert!((factors.integrity_effective() - DEFAULT_BASE).abs() < f32::EPSILON);
    }

    #[test]
    fn with_bases_creates_custom_values() {
        let factors = TrustworthinessFactors::with_bases(0.5, 0.6, 0.7);
        assert!((factors.competence_effective() - 0.5).abs() < f32::EPSILON);
        assert!((factors.benevolence_effective() - 0.6).abs() < f32::EPSILON);
        assert!((factors.integrity_effective() - 0.7).abs() < f32::EPSILON);
    }

    #[test]
    fn overall_is_average() {
        let factors = TrustworthinessFactors::with_bases(0.3, 0.6, 0.9);
        let expected = (0.3 + 0.6 + 0.9) / 3.0;
        assert!((factors.overall() - expected).abs() < f32::EPSILON);
    }

    #[test]
    fn add_competence_delta_in_domain() {
        let mut factors = TrustworthinessFactors::new();
        factors.add_competence_delta_in(LifeDomain::Work, 0.2);

        // Work domain should have the delta
        assert!((factors.competence(LifeDomain::Work).unwrap().delta() - 0.2).abs() < f32::EPSILON);
        assert!((factors.competence_in(LifeDomain::Work) - 0.5).abs() < f32::EPSILON);

        // Other domains should be unaffected
        assert!((factors.competence(LifeDomain::Health).unwrap().delta()).abs() < f32::EPSILON);
    }

    #[test]
    fn add_competence_delta_in_ignores_missing_domain() {
        let mut factors = TrustworthinessFactors::new();
        factors.competence.remove(&LifeDomain::Work);

        factors.add_competence_delta_in(LifeDomain::Work, 0.2);

        assert!(factors.competence(LifeDomain::Work).is_none());
    }

    #[test]
    fn add_competence_delta_all_domains() {
        let mut factors = TrustworthinessFactors::new();
        factors.add_competence_delta(0.2);

        // All domains should have the delta
        assert!((factors.competence(LifeDomain::Work).unwrap().delta() - 0.2).abs() < f32::EPSILON);
        assert!((factors.competence(LifeDomain::Health).unwrap().delta() - 0.2).abs() < f32::EPSILON);
        assert!((factors.competence_effective() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn add_benevolence_delta() {
        let mut factors = TrustworthinessFactors::new();
        factors.add_benevolence_delta(0.3);
        assert!((factors.benevolence().delta() - 0.3).abs() < f32::EPSILON);
    }

    #[test]
    fn add_integrity_delta() {
        let mut factors = TrustworthinessFactors::new();
        factors.add_integrity_delta(-0.1);
        assert!((factors.integrity().delta() - (-0.1)).abs() < f32::EPSILON);
    }

    #[test]
    fn add_delta_by_path() {
        let mut factors = TrustworthinessFactors::new();
        factors.add_delta(TrustPath::Competence, 0.1);
        factors.add_delta(TrustPath::Benevolence, 0.2);
        factors.add_delta(TrustPath::Integrity, 0.3);

        // Competence via path defaults to Work domain
        assert!((factors.competence(LifeDomain::Work).unwrap().delta() - 0.1).abs() < f32::EPSILON);
        assert!((factors.benevolence().delta() - 0.2).abs() < f32::EPSILON);
        assert!((factors.integrity().delta() - 0.3).abs() < f32::EPSILON);
    }

    #[test]
    fn add_delta_skips_missing_competence_domain() {
        let mut factors = TrustworthinessFactors::new();
        factors.competence.remove(&LifeDomain::Work);

        factors.add_delta(TrustPath::Competence, 0.1);

        assert!(factors.competence(LifeDomain::Work).is_none());
    }

    #[test]
    fn get_by_path() {
        let factors = TrustworthinessFactors::with_bases(0.4, 0.5, 0.6);

        // Competence via path defaults to Work domain
        assert!(
            (factors.get(TrustPath::Competence).unwrap().effective() - 0.4).abs() < f32::EPSILON
        );
        assert!(
            (factors.get(TrustPath::Benevolence).unwrap().effective() - 0.5).abs() < f32::EPSILON
        );
        assert!(
            (factors.get(TrustPath::Integrity).unwrap().effective() - 0.6).abs() < f32::EPSILON
        );
    }

    #[test]
    fn get_by_path_returns_none_for_computed() {
        let factors = TrustworthinessFactors::new();
        assert!(factors.get(TrustPath::SupportWillingness).is_none());
    }

    #[test]
    fn get_mut_by_path() {
        let mut factors = TrustworthinessFactors::new();
        factors
            .get_mut(TrustPath::Competence)
            .unwrap()
            .add_delta(0.1);
        factors
            .get_mut(TrustPath::Benevolence)
            .unwrap()
            .add_delta(0.2);
        factors
            .get_mut(TrustPath::Integrity)
            .unwrap()
            .add_delta(0.3);

        // Competence via path defaults to Work domain
        assert!((factors.competence(LifeDomain::Work).unwrap().delta() - 0.1).abs() < f32::EPSILON);
        assert!((factors.benevolence().delta() - 0.2).abs() < f32::EPSILON);
        assert!((factors.integrity().delta() - 0.3).abs() < f32::EPSILON);
    }

    #[test]
    fn get_mut_by_path_returns_none_for_computed() {
        let mut factors = TrustworthinessFactors::new();
        assert!(factors.get_mut(TrustPath::SupportWillingness).is_none());
    }

    #[test]
    fn add_delta_on_computed_does_nothing() {
        let mut factors = TrustworthinessFactors::new();
        let before = factors.overall();
        factors.add_delta(TrustPath::SupportWillingness, 0.5);
        let after = factors.overall();
        assert!((before - after).abs() < f32::EPSILON);
    }

    #[test]
    fn recompute_from_empty_resets_deltas() {
        let mut factors = TrustworthinessFactors::new();
        factors.add_competence_delta(0.2);
        factors.add_benevolence_delta(0.3);
        factors.add_integrity_delta(0.4);

        factors.recompute_from_antecedents(&[]);

        assert!(factors.competence(LifeDomain::Work).unwrap().delta().abs() < f32::EPSILON);
        assert!(factors.benevolence().delta().abs() < f32::EPSILON);
        assert!(factors.integrity().delta().abs() < f32::EPSILON);
    }

    #[test]
    fn recompute_handles_benevolence_antecedents() {
        let ts = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
        let antecedent = TrustAntecedent::new(
            ts,
            AntecedentType::Benevolence,
            AntecedentDirection::Positive,
            0.3,
            "support",
        );

        let mut factors = TrustworthinessFactors::new();
        factors.recompute_from_antecedents(&[antecedent]);

        assert!(factors.benevolence().delta() > 0.0);
    }

    #[test]
    fn competence_decays_over_30_days() {
        let mut factors = TrustworthinessFactors::new();
        factors.add_competence_delta_in(LifeDomain::Work, 0.4);

        // After 30 days, delta should be halved
        factors.apply_decay(Duration::days(30));
        assert!((factors.competence(LifeDomain::Work).unwrap().delta() - 0.2).abs() < 0.01);
    }

    #[test]
    fn benevolence_decays_over_14_days() {
        let mut factors = TrustworthinessFactors::new();
        factors.add_benevolence_delta(0.4);

        // After 14 days, delta should be halved
        factors.apply_decay(Duration::days(14));
        assert!((factors.benevolence().delta() - 0.2).abs() < 0.01);
    }

    #[test]
    fn integrity_decays_over_60_days() {
        let mut factors = TrustworthinessFactors::new();
        factors.add_integrity_delta(0.4);

        // After 60 days, delta should be halved
        factors.apply_decay(Duration::days(60));
        assert!((factors.integrity().delta() - 0.2).abs() < 0.01);
    }

    #[test]
    fn benevolence_decays_faster_than_competence() {
        let mut factors = TrustworthinessFactors::new();
        factors.add_competence_delta_in(LifeDomain::Work, 0.4);
        factors.add_benevolence_delta(0.4);

        factors.apply_decay(Duration::days(14));

        // Benevolence should have decayed more (half-life reached)
        // Competence should have decayed less
        assert!(factors.benevolence().delta() < factors.competence(LifeDomain::Work).unwrap().delta());
    }

    #[test]
    fn integrity_decays_slower_than_competence() {
        let mut factors = TrustworthinessFactors::new();
        factors.add_competence_delta_in(LifeDomain::Work, 0.4);
        factors.add_integrity_delta(0.4);

        factors.apply_decay(Duration::days(30));

        // Competence should have decayed more (half-life reached)
        // Integrity should have decayed less
        assert!(factors.competence(LifeDomain::Work).unwrap().delta() < factors.integrity().delta());
    }

    #[test]
    fn reset_deltas() {
        let mut factors = TrustworthinessFactors::new();
        factors.add_competence_delta(0.1);
        factors.add_benevolence_delta(0.2);
        factors.add_integrity_delta(0.3);

        factors.reset_deltas();

        assert!(factors.competence(LifeDomain::Work).unwrap().delta().abs() < f32::EPSILON);
        assert!(factors.benevolence().delta().abs() < f32::EPSILON);
        assert!(factors.integrity().delta().abs() < f32::EPSILON);
    }

    #[test]
    fn mutable_references_work() {
        let mut factors = TrustworthinessFactors::new();
        factors.competence_mut(LifeDomain::Work).unwrap().add_delta(0.1);
        factors.benevolence_mut().add_delta(0.2);
        factors.integrity_mut().add_delta(0.3);

        assert!((factors.competence(LifeDomain::Work).unwrap().delta() - 0.1).abs() < f32::EPSILON);
        assert!((factors.benevolence().delta() - 0.2).abs() < f32::EPSILON);
        assert!((factors.integrity().delta() - 0.3).abs() < f32::EPSILON);
    }

    #[test]
    fn default_equals_new() {
        let d = TrustworthinessFactors::default();
        let n = TrustworthinessFactors::new();
        assert_eq!(d, n);
    }

    #[test]
    fn clone_and_equality() {
        let f1 = TrustworthinessFactors::with_bases(0.5, 0.6, 0.7);
        let f2 = f1.clone();
        assert_eq!(f1, f2);
    }

    #[test]
    fn debug_format() {
        let factors = TrustworthinessFactors::new();
        let debug = format!("{:?}", factors);
        assert!(debug.contains("TrustworthinessFactors"));
    }

    #[test]
    fn values_clamped_to_bounds() {
        let mut factors = TrustworthinessFactors::new();
        factors.add_competence_delta(2.0); // Way over 1.0

        // Effective should be clamped to 1.0
        assert!((factors.competence_effective() - 1.0).abs() < f32::EPSILON);

        // But delta is not clamped (check via one domain)
        assert!((factors.competence(LifeDomain::Work).unwrap().delta() - 2.0).abs() < f32::EPSILON);
    }

    #[test]
    fn betrayal_applies_2_5x_negative_weight() {
        let ts = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
        let positive = TrustAntecedent::new(
            ts,
            AntecedentType::Integrity,
            AntecedentDirection::Positive,
            0.2,
            "support",
        );
        let negative = TrustAntecedent::new(
            ts,
            AntecedentType::Integrity,
            AntecedentDirection::Negative,
            0.2,
            "betrayal",
        );

        let mut positive_factors = TrustworthinessFactors::new();
        positive_factors.recompute_from_antecedents(&[positive]);
        let positive_delta = positive_factors.integrity().delta().abs();

        let mut negative_factors = TrustworthinessFactors::new();
        negative_factors.recompute_from_antecedents(&[negative]);
        let negative_delta = negative_factors.integrity().delta().abs();

        let ratio = negative_delta / positive_delta;
        assert!((ratio - NEGATIVE_ANTECEDENT_WEIGHT).abs() < 0.01);
    }

    #[test]
    fn rebuilding_after_betrayal_reduces_positive_weight() {
        let base_time = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
        let within_window = base_time + Duration::days(30);
        let outside_window = base_time + Duration::days(200);

        let negative = TrustAntecedent::new(
            base_time,
            AntecedentType::Integrity,
            AntecedentDirection::Negative,
            0.2,
            "betrayal",
        );
        let positive_within = TrustAntecedent::new(
            within_window,
            AntecedentType::Integrity,
            AntecedentDirection::Positive,
            0.2,
            "rebuild",
        );
        let positive_outside = TrustAntecedent::new(
            outside_window,
            AntecedentType::Integrity,
            AntecedentDirection::Positive,
            0.2,
            "rebuild",
        );

        let mut within = TrustworthinessFactors::new();
        within.recompute_from_antecedents(&[negative.clone(), positive_within]);

        let mut outside = TrustworthinessFactors::new();
        outside.recompute_from_antecedents(&[negative, positive_outside]);

        assert!(within.integrity_effective() < outside.integrity_effective());
    }

    #[test]
    fn exponential_smoothing_weights_recent_events_higher() {
        let base_time = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
        let later_time = base_time + Duration::days(1);

        // Use ability antecedents without domain (applies to all domains)
        let first = TrustAntecedent::new(
            base_time,
            AntecedentType::Ability,
            AntecedentDirection::Positive,
            0.3,
            "task_completed_well",
        );
        let second = TrustAntecedent::new(
            later_time,
            AntecedentType::Ability,
            AntecedentDirection::Positive,
            0.3,
            "task_completed_well",
        );

        let mut factors = TrustworthinessFactors::new();
        factors.recompute_from_antecedents(&[first, second]);

        let expected_ema = ANTECEDENT_SMOOTHING_ALPHA * 0.3
            + (1.0 - ANTECEDENT_SMOOTHING_ALPHA) * ANTECEDENT_SMOOTHING_ALPHA * 0.3;
        // Check one domain (they should all have same delta since no domain was specified)
        assert!((factors.competence(LifeDomain::Work).unwrap().delta() - expected_ema).abs() < 0.001);

        let recent_contribution = ANTECEDENT_SMOOTHING_ALPHA * 0.3;
        let older_contribution =
            (1.0 - ANTECEDENT_SMOOTHING_ALPHA) * ANTECEDENT_SMOOTHING_ALPHA * 0.3;
        assert!(recent_contribution > older_contribution);
    }

    #[test]
    fn domain_specific_competence_antecedents() {
        let ts = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);

        // Create an ability antecedent for Health domain only
        let health_competence = TrustAntecedent::new(
            ts,
            AntecedentType::Ability,
            AntecedentDirection::Positive,
            0.5,
            "gave_good_medical_advice",
        )
        .with_life_domain(LifeDomain::Health);

        let mut factors = TrustworthinessFactors::new();
        factors.recompute_from_antecedents(&[health_competence]);

        // Health domain should have increased competence
        assert!(factors.competence(LifeDomain::Health).unwrap().delta() > 0.0);

        // Other domains should be unaffected
        assert!(factors.competence(LifeDomain::Work).unwrap().delta().abs() < f32::EPSILON);
        assert!(factors.competence(LifeDomain::Financial).unwrap().delta().abs() < f32::EPSILON);
    }

    #[test]
    fn recompute_skips_missing_competence_domain() {
        let mut factors = TrustworthinessFactors::new();
        factors.competence.remove(&LifeDomain::Work);

        let ts = Timestamp::from_ymd_hms(2024, 1, 1, 0, 0, 0);
        let antecedent = TrustAntecedent::new(
            ts,
            AntecedentType::Ability,
            AntecedentDirection::Positive,
            0.5,
            "test",
        )
        .with_life_domain(LifeDomain::Work);

        factors.recompute_from_antecedents(&[antecedent]);

        assert!(factors.competence(LifeDomain::Work).is_none());
    }

    #[test]
    fn competence_in_returns_domain_specific_value() {
        let mut factors = TrustworthinessFactors::new();
        factors.add_competence_delta_in(LifeDomain::Work, 0.3);
        factors.add_competence_delta_in(LifeDomain::Health, 0.1);

        assert!((factors.competence_in(LifeDomain::Work) - 0.6).abs() < f32::EPSILON);
        assert!((factors.competence_in(LifeDomain::Health) - 0.4).abs() < f32::EPSILON);
        assert!((factors.competence_in(LifeDomain::Financial) - DEFAULT_BASE).abs() < f32::EPSILON);
    }

    #[test]
    fn older_antecedents_have_less_impact_due_to_temporal_decay() {
        let recent_ts = Timestamp::from_ymd_hms(2024, 6, 1, 0, 0, 0);
        let old_ts = Timestamp::from_ymd_hms(2023, 6, 1, 0, 0, 0); // 1 year ago

        // Create two antecedents: one recent, one old
        let recent = TrustAntecedent::new(
            recent_ts,
            AntecedentType::Benevolence,
            AntecedentDirection::Positive,
            0.5,
            "recent_kindness",
        );
        let old = TrustAntecedent::new(
            old_ts,
            AntecedentType::Benevolence,
            AntecedentDirection::Positive,
            0.5,
            "old_kindness",
        );

        // Test with only the recent antecedent
        let mut recent_only = TrustworthinessFactors::new();
        recent_only.recompute_from_antecedents(&[recent.clone()]);
        let recent_impact = recent_only.benevolence_effective();

        // Test with only the old antecedent
        // Note: When there's only one antecedent, it's the reference so no decay applies
        // So we compare with both antecedents where the old one gets decay applied
        let mut both = TrustworthinessFactors::new();
        both.recompute_from_antecedents(&[old.clone(), recent]);
        let combined_impact = both.benevolence_effective();

        // With temporal decay, the old antecedent contributes less, so:
        // combined should be less than 2x the single recent antecedent impact
        // because the old one is decayed (1 year ~ 2 half-lives = ~25% remaining)
        assert!(combined_impact < recent_impact * 1.5);
    }

    #[test]
    fn antecedent_at_exactly_half_life_has_half_impact() {
        let recent_ts = Timestamp::from_ymd_hms(2024, 6, 1, 0, 0, 0);
        let half_life_ago = recent_ts - Duration::days(180); // Exactly 180 days ago

        let recent = TrustAntecedent::new(
            recent_ts,
            AntecedentType::Integrity,
            AntecedentDirection::Positive,
            0.6,
            "recent_honesty",
        );
        let half_life_old = TrustAntecedent::new(
            half_life_ago,
            AntecedentType::Integrity,
            AntecedentDirection::Positive,
            0.6,
            "half_life_old_honesty",
        );

        // The EMA with decay: recent gets decay_factor=1.0, half_life_old gets decay_factor=0.5
        // So the old one contributes half as much to the EMA calculation
        let mut factors = TrustworthinessFactors::new();
        factors.recompute_from_antecedents(&[half_life_old, recent]);

        // The impact should show decay - the combined is not equal to 2x single
        let mut single = TrustworthinessFactors::new();
        single.recompute_from_antecedents(&[TrustAntecedent::new(
            recent_ts,
            AntecedentType::Integrity,
            AntecedentDirection::Positive,
            0.6,
            "single_honesty",
        )]);

        // With decay, two antecedents (one at half impact) should be less than 2x
        assert!(factors.integrity_effective() < single.integrity_effective() * 2.0);
        assert!(factors.integrity_effective() > single.integrity_effective());
    }

    #[test]
    fn all_domains_initialized() {
        let factors = TrustworthinessFactors::new();

        // All 8 life domains should be initialized
        assert!(factors.competence(LifeDomain::Work).is_some());
        assert!(factors.competence(LifeDomain::Academic).is_some());
        assert!(factors.competence(LifeDomain::Social).is_some());
        assert!(factors.competence(LifeDomain::Athletic).is_some());
        assert!(factors.competence(LifeDomain::Creative).is_some());
        assert!(factors.competence(LifeDomain::Financial).is_some());
        assert!(factors.competence(LifeDomain::Health).is_some());
        assert!(factors.competence(LifeDomain::Relationship).is_some());
    }
}
