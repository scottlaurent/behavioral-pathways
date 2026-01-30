# Longitudinal Tests

**Purpose**: Complex, long-running simulation tests that span years or decades with multiple entities. These tests are separated from the main test suite because they:

1. Are primarily for demonstration and validation of complex scenarios
2. Serve as documentation for how to use the system for realistic use cases

## Running Longitudinal Tests

These tests are marked with `#[ignore]` and must be run explicitly:

```bash
# Run a specific longitudinal test with output
cargo test tribal_dynamics_over_ten_years -- --ignored --nocapture

# Run all longitudinal tests
cargo test --test longitudinal -- --ignored --nocapture

# Run without capturing output (see println! statements)
cargo test tribal_dynamics_over_ten_years -- --ignored --show-output
```

## Example Test

### `tribal_dynamics_ten_year.rs`

**Scenario**: Five coworkers over 10 years (2010-2020)
- **Entities**: John (trauma survivor), Sue (victim/leader), Maria (supporter), David (mediator), Chen (opportunist)
- **Events**: ~70-80 events including violence, betrayal, job loss, pandemic
- **Duration**: 10 years simulated time
- **Demonstrates**:
  - Childhood trauma persistence (AC never decays)
  - Alliance formation and in-group/out-group dynamics
  - Social support buffering negative outcomes
  - ITS risk convergence (all 3 factors)
  - Backward regression (querying past states)
  - Forward projection (predicting future states)
  - Historical events affecting all entities

See the EVENT MAPPING section at the top of the file for how domain-specific events map to behavioral pathways events.

## Test Structure

All longitudinal tests follow this pattern:

1. **Event Mapping Section**: Shows how domain events map to behavioral pathways events
2. **Entity Setup**: Create entities with distinct personalities and histories
3. **Event Sequence**: Apply events chronologically with explanatory comments
4. **Analysis Stages**: Check state at key milestones with assertions
5. **Temporal Queries**: Demonstrate backward/forward state queries
6. **Summary**: Print comprehensive analysis of outcomes

## Adding New Longitudinal Tests

When adding a new longitudinal test:

1. Create a new `.rs` file in this directory
2. Follow the existing template structure
3. Add `#[ignore]` attribute to the test function
4. Include clear event mapping for your domain
5. Add extensive `println!` statements for inspection