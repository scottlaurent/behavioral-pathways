//! Longitudinal test modules.
//!
//! Complex, long-running simulation tests spanning years or decades.
//! See tests/longitudinal/README.md for details and running instructions.
//!
//! All tests are marked #[ignore] - run explicitly with:
//! `cargo test --test longitudinal -- --ignored --nocapture`

#[path = "longitudinal/tribal_dynamics_ten_year.rs"]
mod tribal_dynamics_ten_year;

#[path = "longitudinal/riverside_threads.rs"]
mod riverside_threads;

#[path = "longitudinal/harborlight_resilience_cooperative.rs"]
mod harborlight_resilience_cooperative;

#[path = "longitudinal/resilient_physician_maya.rs"]
mod resilient_physician_maya;

#[path = "longitudinal/second_chance_marcus.rs"]
mod second_chance_marcus;
