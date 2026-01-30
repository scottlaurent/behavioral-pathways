//! Tests for state_at() API - the core consumer query interface.
//!
//! These tests validate that state_at() correctly computes entity state
//! at any timestamp via forward projection or backward regression.

mod event_at_anchor_excluded_forward;
mod event_at_query_excluded_backward;
mod event_at_target_included_forward;
mod state_at_20_years_future_shows_decay;
mod state_at_anchor_returns_anchor_state;
mod state_at_childhood_regresses_from_adult;
