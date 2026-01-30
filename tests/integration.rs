//! Integration tests for behavioral-pathways.
//!
//! Tests interaction between 1-2 systems. These are minimal tests for
//! validating that systems communicate correctly before full simulation tests.

mod integration {
    pub mod context;
    pub mod memory;
    pub mod relationship;
    pub mod simulation;
    pub mod state;
    pub mod state_interpreter;
    pub mod types;
}
