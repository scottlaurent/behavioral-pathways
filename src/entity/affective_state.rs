//! Affective state structs for PAD dimensions and physiological needs.
//!
//! Following the theoretical purity of the PAD (Pleasure-Arousal-Dominance) model,
//! affective state is strictly limited to the three core dimensions. Physiological
//! states like fatigue and stress are handled separately.

/// Pure PAD affective state containing only the three core dimensions.
///
/// Per the Mehrabian-Russell PAD model, affect is represented by three
/// orthogonal dimensions:
/// - **Valence** (Pleasure): pleasantness of the experience
/// - **Arousal**: activation or energy level
/// - **Dominance**: sense of control
///
/// Physiological states (fatigue, stress) are NOT part of affect and should
/// be queried separately via `query_physiological_state()`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AffectiveState {
    /// Valence: pleasantness (-1 to 1).
    pub valence: f32,
    /// Arousal: activation (-1 to 1).
    pub arousal: f32,
    /// Dominance: control (-1 to 1).
    pub dominance: f32,
}

/// Physiological state containing fatigue and stress.
///
/// These are NOT part of the PAD affective model but are often relevant
/// for behavioral modeling. They are kept separate to maintain theoretical
/// purity of the affective state construct.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhysiologicalState {
    /// Fatigue: tiredness (0 to 1).
    pub fatigue: f32,
    /// Stress: tension (0 to 1).
    pub stress: f32,
}
