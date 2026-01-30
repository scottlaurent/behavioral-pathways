//! Trust decision domains.
//!
//! These domains indicate which willingness dimension is affected
//! by an antecedent or interaction.

/// Trust domains for willingness decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrustDomain {
    /// Willingness to delegate tasks.
    Task,
    /// Willingness to seek/provide support.
    Support,
    /// Willingness to disclose vulnerabilities.
    Disclosure,
}
