# Simulation Tests

End-to-end tests via the public API. These are the most important tests in the codebase.

## Architecture

```
API CALLS (same as consumer would make)
  - Entity.create(), sim.advance(), entity.get()
  - Relationship.between(), rel.apply_event()
  - EventBus.dispatch(), entity.recall()
        |
        v
SIMULATION PROGRESSION
  - Multiple stages with time advancement
  - State changes verified at each stage
        |
        v
ASSERTIONS
  - Verify expected psychological state
  - Check relationship dynamics
  - Validate memory formation/decay
```

## File Naming

**ONE FILE = ONE SCENARIO = ONE TEST FUNCTION**

The filename describes the scenario. The function name matches the filename.

```
CORRECT:
  positive_event_increases_valence.rs
  fn positive_event_increases_valence() { ... }

WRONG:
  affect_tests.rs           # Vague, grouped
  test_valence.rs           # Generic
```

## Folder Structure

### By Domain (Theoretical Framework)

| Folder | Framework | Tests |
|--------|-----------|-------|
| `affect/` | PAD Model | Valence, arousal, dominance, emotion derivation |
| `trust/` | Trust Decomposition | Competence, benevolence, integrity |
| `mental_health/` | Joiner's ITS | TB, PB, AC, suicidal risk |
| `ecology/` | Bronfenbrenner | Micro through chronosystem |

### By System

| Folder | System | Tests |
|--------|--------|-------|
| `entity/` | Entity lifecycle | Creation, aging, state changes |
| `relationships/` | Relationship system | Formation, dynamics, influence |
| `memory/` | Memory system | Formation, retrieval, decay |
| `development/` | Developmental system | Life stages, plasticity, sensitive periods |
| `events/` | Event system | Processing, interpretation |
| `time/` | Time processing | Forward simulation, backward regression |

### By Scale (Complexity)

| Folder | Scale | Tests |
|--------|-------|-------|
| `single_entity/` | 1 entity | Simple to complex individual scenarios |
| `dyadic/` | 2 entities | Relationship-focused scenarios |
| `group/` | 3+ entities | Group dynamics, contagion |
| `longitudinal/` | Long time spans | Multi-year/generational scenarios |

### Invariants

| Folder | Purpose |
|--------|---------|
| `invariants/determinism/` | Same inputs = same outputs |
| `invariants/conservation/` | State values stay in bounds |
| `invariants/consistency/` | Serialization roundtrips correctly |

## Test Pattern

```rust
#[test]
fn scenario_name_matching_filename() {
    // ========================================================================
    // SETUP
    // What we're doing: [Plain English description]
    // ========================================================================

    let sim = Simulation::new();
    let entity = Entity::create("test_entity")
        .species(Human)
        .age(30.years());
    sim.add_entity(entity);

    // ========================================================================
    // STAGE 1: [Stage description]
    // What we're testing: [Expected outcome]
    // ========================================================================

    sim.advance(1.day());
    assert!(entity.get("mood.valence") > 0.5);

    // ========================================================================
    // STAGE 2: [Stage description]
    // What we're testing: [Expected outcome]
    // ========================================================================

    // ... more stages ...
}
```

## Rules

### Absolute Rules

- ONE file = ONE test function
- Filename MUST match function name
- Use API calls, not internal methods
- Test scenarios consumers would actually use
- Write failing tests when API is incomplete

### Standard Rules

- Check existing tests before adding new ones
- Use helpers.rs for shared utilities
- Plain English stage comments
- Readable by non-programmers
- Run with `make test`
