# Behavioral Pathways API Reference

Complete API documentation for the behavioral-pathways computation library.

---

## API Model

The system is a **declarative computation library**. Consumers:
1. Create a simulation with a reference date
2. Add entities with known anchor states at specific timestamps
3. Add events and relationships at specific timestamps
4. Query state at any timestamp (system computes on-demand)

```
sim = Simulation(reference_date: "2024-01-01")
sim.add_entity(entity, timestamp)
sim.add_event(event, timestamp)
sim.add_relationship(a, b, type, timestamp)
state = sim.entity(id).state_at(timestamp)
```

---

## Simulation Container

| Item | Notes |
|------|-------|
| `Simulation` struct | Main container holding entities, relationships, events |
| `Simulation::new(reference_date)` | Constructor with absolute reference date |
| `sim.add_entity(entity, timestamp)` | Add entity with anchor state at timestamp |
| `sim.add_event(event, timestamp)` | Add event at absolute timestamp |
| `sim.add_relationship(a, b, schema, timestamp)` | Create relationship at timestamp |
| `sim.entity(id)` | Get entity query handle |
| `sim.entity(id).state_at(timestamp)` | Core API - compute state at any timestamp |
| `sim.entities()` | Iterate all entities |
| `sim.relationships_for(entity_id)` | Get relationships involving entity |
| `SimulationBuilder` | Fluent construction |

---

## Entity

| Item | Notes |
|------|-------|
| `Entity` struct | Core entity with id, species, state |
| `EntityBuilder` | Fluent construction (see below) |
| `entity.id()` | Get entity ID |
| `entity.species()` | Get species |
| `entity.age()` | Compute age from anchor_timestamp and query_timestamp |
| `entity.life_stage()` | Get current life stage |
| `entity.get_effective(StatePath)` | Get effective state value (base + delta) |
| `entity.get_base(StatePath)` | Get base value |
| `entity.get_delta(StatePath)` | Get delta value |
| `entity.set_state(StatePath, f64)` | Set state value |
| `entity.apply_delta(StatePath, f64)` | Apply delta to state |
| `entity.anchor_state` | The ONE known state for this entity |
| `entity.anchor_timestamp` | When the anchor state was known |

### EntityBuilder

Fluent builder for constructing entities with full state initialization.

| Method | Notes |
|--------|-------|
| `EntityBuilder::new()` | Create new builder |
| `.id(string)` | Set entity ID (auto-generated if omitted) |
| `.species(Species)` | **Required** - Human, Dog, Cat, etc. |
| `.age(Duration)` | Set age at anchor time |
| `.birth_date(Timestamp)` | Set birth date for age computation |
| `.life_stage(LifeStage)` | Override life stage (derived from age if omitted) |
| `.personality(PersonalityProfile)` | Set HEXACO via preset profile |
| `.hexaco(Hexaco)` | Set HEXACO directly |
| `.person_characteristics(PersonCharacteristics)` | Set PPCT factors |
| `.mood(Mood)` | Set PAD dimensions (valence, arousal, dominance bases) |
| `.needs(Needs)` | Set needs (fatigue, stress, purpose bases) |
| `.mental_health(MentalHealth)` | Set ITS factors (depression, hopelessness, acquired capability bases) |
| `.social_cognition(SocialCognition)` | Set interpersonal beliefs (loneliness, caring, liability bases) |
| `.disposition(Disposition)` | Set behavioral tendencies (empathy, aggression bases) |
| `.with_context(EcologicalContext)` | Set ecological context |
| `.build()` | Build entity, returns `Result<Entity, EntityBuildError>` |

**Example - Full state initialization from persisted data:**

```rust
let entity = EntityBuilder::new()
    .id("npc_001")
    .species(Species::Human)
    .birth_date(Timestamp::from_ymd_hms(1990, 6, 15, 0, 0, 0))
    .age(Duration::years(30))
    .hexaco(genesis.hexaco)
    .person_characteristics(genesis.person_characteristics)
    .mood(genesis.mood)
    .needs(genesis.needs)
    .mental_health(genesis.mental_health)
    .social_cognition(genesis.social_cognition)
    .disposition(genesis.disposition)
    .build()?;
```

### Entity State Access

| Item | Notes |
|------|-------|
| `StatePath` enum | Typed paths to all state dimensions |
| `StatePath::Mood(MoodPath)` | PAD dimensions (valence, arousal, dominance only) |
| `StatePath::Needs(NeedsPath)` | Physiological needs (fatigue, stress, purpose) |
| `StatePath::SocialCognition(SocialCognitionPath)` | Interpersonal perceptions (loneliness, liability, self_hate, caring) |
| `StatePath::MentalHealth(MentalHealthPath)` | ITS stored fields and computed factors |
| `StatePath::Disposition(DispositionPath)` | Behavioral tendencies |
| `StatePath::Hexaco(HexacoPath)` | Personality dimensions |
| `StatePath::PersonCharacteristics(...)` | PPCT person factors |

---

## State Components

### IndividualState

| Item | Notes |
|------|-------|
| `IndividualState` | Container for all psychological state |
| `Hexaco` | HEXACO personality model |
| `Mood` | PAD dimensions only (valence, arousal, dominance) |
| `Needs` | Physiological needs (fatigue, stress, purpose) |
| `SocialCognition` | Interpersonal perceptions (loneliness, perceived_liability, self_hate, reciprocal_caring) |
| `MentalHealth` | ITS stored factors (interpersonal_hopelessness, acquired_capability), depression |
| `Disposition` | Trust propensity (1-year decay), impulse control, empathy, etc. |
| `PersonCharacteristics` | Demand, resource, force (PPCT) |
| `EntityModelConfig` | Species-specific decay configuration |

### StateValue

| Item | Notes |
|------|-------|
| `StateValue` | Base + delta + decay pattern |
| `state_value.base` | Stable tendency |
| `state_value.delta` | Current deviation |
| `state_value.decay_half_life` | Optional, None = no decay |
| `state_value.effective()` | Returns base + delta |

### Formative Events Module

The formative module handles permanent personality changes from life events.

#### BaseShiftRecord

| Item | Notes |
|------|-------|
| `BaseShiftRecord` | Record of a personality base shift |
| `BaseShiftRecord::new(timestamp, trait, amount)` | Create shift record |
| `record.timestamp()` | When the shift occurred |
| `record.trait_path()` | Which HEXACO trait was shifted |
| `record.immediate()` | Initial shift magnitude |
| `record.settled()` | Final settled magnitude (after recovery) |
| `record.settling_days()` | Days to settle (0 if no settling) |
| `record.contribution_at(query_timestamp)` | Current contribution to effective base |

#### Stability Coefficients

Higher stability = more resistant to change.

| Trait | Coefficient | Trait Modifier |
|-------|-------------|----------------|
| Extraversion | 0.85 | 0.15 |
| Openness | 0.80 | 0.20 |
| Honesty-Humility | 0.75 | 0.25 |
| Conscientiousness | 0.70 | 0.30 |
| Agreeableness | 0.65 | 0.35 |
| Neuroticism | 0.60 | 0.40 |

```rust
use behavioral_pathways::state::stability_coefficient;
let coeff = stability_coefficient(HexacoPath::Extraversion); // 0.85
```

#### Age Plasticity

| Age Range | Plasticity |
|-----------|------------|
| < 18 | 1.30 |
| 18-29 | 1.00 (reference) |
| 30-49 | 0.80 |
| 50-69 | 0.70 |
| 70+ | 0.60 |

```rust
use behavioral_pathways::state::age_plasticity;
let plasticity = age_plasticity(15); // 1.30 for teenager
```

#### Sensitive Periods

Trait-specific age windows with amplified plasticity.

| Trait | Sensitive Period | Modifier |
|-------|------------------|----------|
| Neuroticism | 12-25 | 1.4x |
| Conscientiousness | 18-35 | 1.2x |
| Agreeableness | 25-40 | 1.2x |
| Extraversion | 13-22 | 1.2x |
| Openness | 15-30 | 1.2x |
| Honesty-Humility | 18-30 | 1.2x |

```rust
use behavioral_pathways::state::sensitive_period_modifier;
let modifier = sensitive_period_modifier(HexacoPath::Neuroticism, 20); // 1.4
```

#### Full Modifier Application

```rust
use behavioral_pathways::state::apply_formative_modifiers;

let applied_shift = apply_formative_modifiers(
    age_years,           // Entity age at event time
    HexacoPath::Agreeableness,
    requested_shift,     // e.g., -0.25
    cumulative_same_direction,  // For diminishing returns
);
```

#### Effective Base Computation

```rust
use behavioral_pathways::state::effective_base_at;

let effective = effective_base_at(
    anchor_value,        // Original personality value
    &shift_records,      // Vec of BaseShiftRecord
    query_timestamp,     // When to compute
);
```

---

## Relationships

| Item | Notes |
|------|-------|
| `Relationship` struct | Connection between two entities |
| `Relationship::new(from, to)` | Create relationship |
| `rel.stage()` | Stranger, Acquaintance, Established, Intimate, Estranged |
| `rel.set_stage(stage)` | Update stage |
| `RelationshipStage` enum | Stage values |
| `RelationshipSchema` enum | Romantic, Friendship, FamilyUpward, etc. |

### Trust

**Critical Distinction**: Trustworthiness (perceptions) vs. Trust (willingness to be vulnerable)

| Item | Notes |
|------|-------|
| `TrustworthinessFactors` | Competence, benevolence, integrity (PERCEPTIONS of trustee) |
| `TrustDecision` | Trust willingness computation (BEHAVIORAL decision) |
| `TrustContext` | Situational factors affecting trust (norms, safeguards, pressure) |
| `PerceivedRisk` | Downside risk, upside opportunity, vulnerabilities |
| `Vulnerability` | Type + stakes for explicit risk representation |
| `VulnerabilityType` | Identity, Resources, Safety, Relationship, Reputation, Emotional |
| `rel.get_trustworthiness(direction)` | Get trustworthiness perceptions for direction |
| `rel.compute_trust_decision(direction, propensity, stakes)` | Compute trust decision for direction |

### TrustworthinessFactors

| Item | Notes |
|------|-------|
| `competence` | Domain-specific via `HashMap<LifeDomain, StateValue>` |
| `benevolence` | Single StateValue (perceived caring about trustor) |
| `integrity` | Single StateValue (perceived ethical consistency) |
| `competence_for_domain(domain)` | Get competence for specific domain |
| `competence_effective()` | Weighted average across all known domains |
| `set_competence_for_domain(domain, val)` | Set competence for domain |
| `clear_competence_for_domain(domain)` | Remove domain-specific competence |

### TrustDecision

| Item | Notes |
|------|-------|
| `willingness` | Domain-specific willingness to be vulnerable |
| `decision_certainty` | How confident the trustor is in their decision |
| `trustee_confidence` | Perceived reliability of the trustee |
| `would_disclose(sensitivity)` | Would share info at sensitivity level? |
| `would_delegate_task(difficulty)` | Would delegate task at difficulty? |
| `would_accept_help(stakes)` | Would accept help at stakes level? |
| `overall_willingness()` | **DEPRECATED** - Trust is domain-specific |

### TrustContext

Situational factors that modify trust computation.

| Item | Notes |
|------|-------|
| `social_norms` | Expected trust level in this context (0-1) |
| `institutional_safeguards` | Protection from negative outcomes (0-1) |
| `time_pressure` | Urgency requiring quick decisions (0-1) |
| `institutional_support` | Available recourse if trust violated (0-1) |
| `cultural_expectations` | Cultural norms for trust in this situation (0-1) |
| `combined_effect()` | Net effect on trust computation |

### PerceivedRisk

| Item | Notes |
|------|-------|
| `downside` | Potential negative consequences (0-1) |
| `upside` | Potential positive outcomes (0-1) |
| `StakesLevel` | Low, Medium, High enum |
| `compute_for_trustor(stakes, trustor_sensitivity)` | Risk adjusted for trustor's sensitivity |
| `compute_subjective(stakes, stage_modifier, sensitivity)` | Full subjective risk computation |

### VulnerabilityType

| Value | Notes |
|-------|-------|
| `Identity` | Risk to sense of self or personal identity |
| `Resources` | Financial, material, or resource risk |
| `Safety` | Physical or psychological safety risk |
| `Relationship` | Risk to important relationships |
| `Reputation` | Risk to social standing or reputation |
| `Emotional` | Risk of emotional harm or distress |

### Vulnerability

| Item | Notes |
|------|-------|
| `vulnerability_type` | VulnerabilityType enum value |
| `stakes` | StakesLevel (Low, Medium, High) |

### Trust Antecedents

| Item | Notes |
|------|-------|
| `TrustAntecedent` | Historical observation affecting trustworthiness |
| `AntecedentType` | Competence, Benevolence, Integrity |
| `AntecedentDirection` | Positive, Negative |
| `life_domain` | Optional domain for competence antecedents |
| Temporal decay | 180-day half-life for antecedent impact |
| MAX_ANTECEDENT_HISTORY | 100 antecedents per direction |

### Shared/Directional Dimensions

| Item | Notes |
|------|-------|
| `SharedDimensions` | Symmetric dimensions (affinity, respect) |
| `DirectionalDimensions` | Asymmetric dimensions (warmth, resentment) |
| `RelPath` enum | Typed paths for relationship access |

---

## Events

| Item | Notes |
|------|-------|
| `Event` struct | Event with type, source, target, severity |
| `EventBuilder` | Fluent construction |
| `EventType` enum | 35 event types across 8 categories |
| `EventCategory` enum | 8 categories (SocialBelonging, Trauma, etc.) |
| `event.timestamp` | Absolute timestamp for event |
| `event.base_shifts()` | Personality base shifts attached to this event |
| `event.has_base_shifts()` | True if event has formative base shifts |

### EventBuilder

| Method | Notes |
|--------|-------|
| `EventBuilder::new(event_type)` | Create builder for event type |
| `.source(entity_id)` | Set source entity |
| `.target(entity_id)` | Set target entity |
| `.severity(f64)` | Set severity (0.0-1.0) |
| `.tag(EventTag)` | Add a tag |
| `.tags(Vec<EventTag>)` | Set all tags |
| `.payload(EventPayload)` | Set type-specific payload |
| `.timestamp(Duration)` | Set timestamp |
| `.context(MicrosystemId)` | Set microsystem context |
| `.with_base_shift(HexacoPath, f32)` | **Add formative personality shift** |
| `.build()` | Build event, returns `Result<Event, EventBuildError>` |

### Formative Events (Personality Base Shifts)

Events can permanently alter personality traits via the long-term pathway. This models major life transitions and trauma effects on personality (Roberts' Social Investment Theory, Tedeschi & Calhoun's Post-Traumatic Growth).

**Usage:**

```rust
let event = EventBuilder::new(EventType::Violence)
    .target(entity_id)
    .severity(0.9)
    .with_base_shift(HexacoPath::Neuroticism, 0.25)      // Increase emotional reactivity
    .with_base_shift(HexacoPath::Agreeableness, -0.15)  // Decrease trust/cooperation
    .build()?;
```

**How base shifts are computed:**

| Modifier | Description | Range |
|----------|-------------|-------|
| Age plasticity | Younger = more plastic | 1.3 (<18) to 0.6 (70+) |
| Trait stability | Some traits resist change | 0.60-0.85 |
| Sensitive periods | Age windows amplify specific traits | 1.0-1.4x |
| Diminishing returns | Repeated shifts in same direction | Asymptotic saturation |
| Severe shift recovery | Shifts >0.20 settle to 70% | 180 days |

**Applied shift formula:**
```
applied = requested * age_plasticity * (1 - trait_stability) * sensitive_period * saturation
```

### ITS Pathway Mapping

Events can affect multiple ITS pathways (Thwarted Belongingness, Perceived Burdensomeness, Acquired Capability):

| Method | Notes |
|--------|-------|
| `event_type.its_pathways()` | Returns `(affects_tb, affects_pb, affects_ac)` tuple |
| `event_type.affects_tb()` | True if event affects Thwarted Belongingness |
| `event_type.affects_pb()` | True if event affects Perceived Burdensomeness |
| `event_type.affects_ac()` | True if event affects Acquired Capability |
| `event_type.is_multi_pathway()` | True if event affects multiple ITS pathways |

### Multi-Pathway Event Types

| EventType | TB | PB | AC | Notes |
|-----------|:--:|:--:|:--:|-------|
| `Bereavement` | X | X | | Death of close person |
| `JobLoss` | X | X | | Employment termination |
| `SuicidalLoss` | X | | X | Suicide of someone close |

### TB (Thwarted Belongingness) Event Types

| EventType | Notes |
|-----------|-------|
| `SocialExclusion` | General social rejection |
| `SocialInclusion` | Social acceptance (reduces TB) |
| `Rejection` | Explicit rejection from group/relationship |
| `SocialIsolation` | Social isolation or withdrawal |
| `RelationshipEnd` | Relationship breakup or divorce |
| `GroupExclusion` | Exclusion from group activities |

### PB (Perceived Burdensomeness) Event Types

| EventType | Notes |
|-----------|-------|
| `BurdenFeedback` | Feedback indicating burden |
| `ShamingEvent` | Being shamed or criticized |
| `FinancialBurden` | Financial strain causing burden feelings |
| `ChronicIllnessOnset` | Chronic illness affecting self-perception |
| `FamilyDiscord` | Family conflict about support needs |

### AC (Acquired Capability) Event Types

| EventType | Notes |
|-----------|-------|
| `Violence` | Physical aggression |
| `TraumaticExposure` | Exposure to painful/frightening stimulus |
| `NonSuicidalSelfInjury` | NSSI (high specificity for AC) |
| `ChildhoodAbuse` | History of childhood maltreatment |
| `CombatExposure` | Military combat experience |
| `PhysicalInjury` | Physical injury with pain exposure |
| `ViolenceExposure` | Witnessing violence against others |
| `PriorSuicideAttempt` | Previous suicide attempt (highest AC) |

### Event Processing (Internal)

Internal functions used by the system to process events. Not part of the consumer API.

| Item | Notes |
|------|-------|
| `process_event(entity, event, config)` | Process event effects |
| `interpret_event(event, hexaco)` | Personality-based interpretation |
| `InterpretedEvent` | Event after interpretation |

---

## Memory

| Item | Notes |
|------|-------|
| `MemoryEntry` | Episodic memory with salience, emotional snapshot |
| `MemoryLayers` | Immediate, Short-term, Long-term, Legacy |
| `MemoryLayer` enum | Layer selection |
| `EmotionalSnapshot` | Frozen PAD at memory formation |
| `MemoryTag` enum | Categorization tags |
| `MemorySource` enum | Self, Observation, Report, etc. |

### Memory Operations

| Item | Notes |
|------|-------|
| `entity.create_memory(...)` | Store memory |
| `entity.retrieve_memories(query)` | Query by tag, salience, mood |
| `RetrievalQuery` | Query builder |
| `layers.retrieve_by_salience(threshold)` | Salience-based retrieval |
| `layers.retrieve_mood_congruent(mood, min)` | Mood-congruent retrieval |

### Memory Consolidation (Internal)

Internal functions for memory effects during state computation. Not part of the consumer API.

| Item | Notes |
|------|-------|
| `consolidate_memories(memories, mood)` | Priming effects during state computation |
| `compute_priming_deltas(memories, mood)` | Mood-congruent priming |

### Memory Maintenance (Internal)

Internal functions for memory layer transitions and decay. Not part of the consumer API.

| Item | Notes |
|------|-------|
| `promote_memory(layers, id)` | Promote by salience threshold |
| `check_decay(memory, threshold)` | Check if memory should decay |
| `compute_consolidation_window(base, arousal)` | Inverted-U arousal model |
| `apply_memory_maintenance(memories, elapsed)` | Run full maintenance cycle |
| `MaintenanceError` | Error type for maintenance operations |
| `MaintenanceReport` | Report of what changed |

---

## Ecological Context

| Item | Notes |
|------|-------|
| `EcologicalContext` | Container for all 5 Bronfenbrenner layers |
| `Microsystem` | Family, Work, Education, etc. |
| `MesosystemCache` | Computed linkages between microsystems |
| `ExosystemContext` | Indirect influences |
| `MacrosystemContext` | Cultural patterns |
| `ChronosystemContext` | Temporal patterns, turning points |
| `ContextPath` enum | Typed paths for context access |
| `entity.get_context(ContextPath)` | Read context value |
| `entity.set_context(ContextPath, f64)` | Write context value |
| `check_proximal_process_gate(...)` | PPCT validation |

### Context Effects (Internal)

Internal functions that apply contextual influences to entity state. Not part of the consumer API.

| Item | Notes |
|------|-------|
| `apply_context_effects(entity, context)` | Apply all context effects |
| `apply_microsystem_effects(entity, microsystems)` | Direct environment effects |
| `apply_mesosystem_spillover(entity, microsystems)` | Cross-context spillover |
| `apply_exosystem_effects(entity, exosystem)` | Indirect influences |
| `apply_macrosystem_effects(entity, macrosystem)` | Cultural constraints |
| `apply_chronosystem_effects(entity, chrono, timestamp)` | Historical/temporal effects |

---

## Developmental Processing (Internal)

Internal computation functions that modify event impact based on life stage. Not part of the consumer API.

| Item | Notes |
|------|-------|
| `apply_developmental_effects(entity, event, impact, age)` | Modify event impact based on development |
| `get_plasticity_modifier(life_stage, age_years)` | Continuous plasticity curve |
| `get_sensitive_period_multiplier(life_stage, category)` | Erikson-based amplification |
| `get_turning_point_boost(turning_points, age_days)` | Temporary plasticity boost |
| `DevelopmentalCategory` enum | Maps EventType to Erikson stages |

---

## State Processing (Internal)

Internal computation functions. Not part of the consumer API.

| Item | Notes |
|------|-------|
| `apply_decay(state, duration, config)` | Decay deltas toward baseline |
| `derive_emotion(v, a, d)` | PAD to discrete emotion |
| `compute_its_factors(state)` | ITS aggregation with threshold gates |
| `compute_thwarted_belongingness(social_cog)` | TB = (loneliness + (1 - caring)) / 2 |
| `compute_perceived_burdensomeness(social_cog)` | PB = perceived_liability * self_hate |
| `compute_suicidal_desire(tb, pb, hopelessness)` | Desire with threshold gates (all >= 0.5) |
| `compute_attempt_risk(desire, capability)` | Risk = desire * capability |
| `check_its_thresholds(factors)` | Mental health alerts |
| `apply_stress_spiral(state)` | Feedback loop |
| `apply_depression_spiral(state)` | Feedback loop |
| `reverse_decay(state_value, duration, config)` | Backward time computation |
| `DecayProcessor` trait | Decay abstraction |
| `StateDecayProcessor` | Real decay implementation |

---

## Alerts

| Item | Notes |
|------|-------|
| `Alert` struct | Trigger + severity |
| `AlertTrigger` enum | Threshold crossing types |
| `AlertSeverity` enum | Low, Medium, High, Critical |
| `check_thresholds(entity)` | Generate alerts for state |

---

## ITS (Interpersonal Theory of Suicide) System

### ItsFactors (Computed)

| Item | Notes |
|------|-------|
| `ItsFactors` struct | Computed ITS factors from entity state |
| `thwarted_belongingness` | TB = (loneliness + (1 - caring)) / 2 |
| `perceived_burdensomeness` | PB = liability * self_hate |
| `acquired_capability` | AC from mental_health (never decreases) |
| `suicidal_desire` | TB * PB when hopelessness threshold met |
| `attempt_risk` | desire * acquired_capability |
| `passive_ideation_present` | TB > 0.3 OR PB > 0.3 |
| `convergence_status` | ConvergenceStatus for risk matrix |
| `compute_its_factors(state)` | Compute factors from IndividualState |

### ItsProximalFactor

| Item | Notes |
|------|-------|
| `ItsProximalFactor` enum | The three proximal factors |
| `ThwartedBelongingness` | Unmet need to belong |
| `PerceivedBurdensomeness` | Belief of being a burden |
| `AcquiredCapability` | Habituation to pain/fear of death |
| `factor.name()` | Human-readable name |
| `factor.code()` | Short code ("TB", "PB", "AC") |
| `ItsProximalFactor::all()` | All three variants |

### ConvergenceStatus

Tracks which ITS factors are elevated (Joiner's risk matrix).

| Item | Notes |
|------|-------|
| `ConvergenceStatus` struct | Factor elevation tracking |
| `is_three_factor_convergent` | True if all three elevated (HIGH RISK) |
| `elevated_factor_count` | Count of elevated factors (0-3) |
| `highest_factor` | Factor with highest excess over threshold |
| `tb_elevated` | True if TB above threshold (0.5) |
| `pb_elevated` | True if PB above threshold (0.5) |
| `ac_elevated` | True if AC above threshold (0.3) |
| `has_desire()` | True if TB + PB elevated |
| `is_dormant_capability()` | AC elevated without desire |
| `has_desire_without_capability()` | Desire present, no AC |
| `elevated_factors()` | List of elevated factors |
| `ConvergenceStatus::from_factors(tb, pb, ac)` | Create from factor values |

### ItsAlert

Risk matrix alerts based on factor convergence.

| Item | Notes |
|------|-------|
| `ItsAlert` enum | 7 alert states matching Joiner matrix |
| `SingleFactorTb` | Only TB elevated (low risk) |
| `SingleFactorPb` | Only PB elevated (low risk) |
| `SingleFactorAc` | Only AC elevated (dormant capability) |
| `DesireWithoutCapability` | TB + PB elevated, no AC (moderate) |
| `TbWithCapability` | TB + AC elevated (moderate) |
| `PbWithCapability` | PB + AC elevated (moderate) |
| `ThreeFactorConvergence` | All three elevated (HIGH RISK) |
| `ItsAlert::from_convergence(status)` | Create from ConvergenceStatus |
| `alert.risk_level()` | 1 (low), 2 (moderate), 3 (high) |
| `alert.is_high_risk()` | True if three-factor convergence |
| `alert.has_desire()` | True if desire present |
| `alert.has_capability()` | True if AC elevated |
| `alert.elevated_factors()` | List of elevated factors |
| `alert.name()` | Human-readable name |
| `alert.intervention_focus()` | Clinical intervention guidance |

### ITS Contributors (Layer 2)

Tracks specific factors contributing to proximal factor elevation.

| Item | Notes |
|------|-------|
| `TbContributor` enum | 7 TB contributors |
| `PbContributor` enum | 8 PB contributors |
| `AcContributor` enum | 10 AC contributors |
| `ItsContributor` enum | Unified contributor (Tb/Pb/Ac variants) |
| `contributor.is_chronic()` | True if contributor persists |
| `contributor.proximal_factor()` | Which factor this affects |

### TbContributor (Thwarted Belongingness)

| Value | Chronic | Notes |
|-------|:-------:|-------|
| `SocialRejection` | | Rejection from individuals/groups |
| `Isolation` | X | Voluntary/involuntary isolation |
| `RelationshipLoss` | | Loss of close relationship |
| `RoleDisplacement` | X | Displacement from social role |
| `GroupExclusion` | | Exclusion from group activities |
| `InterpersonalConflict` | | Conflict damaging belonging |
| `SocialNetworkDisruption` | X | Geographic relocation disrupting network |

### PbContributor (Perceived Burdensomeness)

| Value | Chronic | Notes |
|-------|:-------:|-------|
| `DirectBurdenFeedback` | | Direct feedback of being a burden |
| `FinancialStrain` | X | Financial strain on others |
| `Shame` | | Shame about dependency |
| `RoleFailure` | | Perceived failure in role |
| `IllnessDependent` | X | Physical illness requiring care |
| `SelfLoathing` | X | Active self-loathing |
| `Uselessness` | | Feeling useless |
| `FamilyConflict` | | Conflict about support needs |

### AcContributor (Acquired Capability)

All AC contributors are chronic (capability doesn't diminish).

| Value | Weight | Notes |
|-------|:------:|-------|
| `PriorSuicideAttempt` | 1.0 | Strongest predictor |
| `NonSuicidalSelfInjury` | 0.8 | High specificity |
| `PhysicalAbuseExposure` | 0.6 | Physical abuse |
| `SexualAbuseExposure` | 0.6 | Sexual abuse |
| `CombatExposure` | 0.5 | Military combat |
| `ChronicPainExposure` | 0.4 | Chronic pain |
| `SuicideBereavement` | 0.4 | Suicide of close person |
| `ViolenceWitnessing` | 0.3 | Witnessing violence |
| `PhysicalInjury` | 0.3 | Injury with pain tolerance |
| `OccupationalExposure` | 0.2 | Healthcare, first responders |

### ItsContributors (State Tracking)

| Item | Notes |
|------|-------|
| `ItsContributors` struct | Tracks all active contributors |
| `ContributorActivation` struct | Single activation record |
| `activation.contributor` | The contributor type |
| `activation.activated_at` | When activated |
| `activation.initial_intensity` | Intensity (0-1) |
| `activation.is_chronic` | Whether it persists |
| `activation.intensity_at(time)` | Current intensity after decay |
| `activation.is_active_at(time)` | Active if intensity >= 0.1 |
| `contributors.activate(c, time, intensity)` | Activate a contributor |
| `contributors.active_at(time)` | All active activations |
| `contributors.tb_contribution_at(time)` | Sum of TB contributions |
| `contributors.pb_contribution_at(time)` | Sum of PB contributions |
| `contributors.ac_contribution_at(time)` | Weighted sum of AC contributions |
| `contributors.active_tb_contributors_at(time)` | Active TB contributors |
| `contributors.deactivate_chronic(c, time)` | Resolve chronic contributor |

### ITS Constants

| Constant | Value | Notes |
|----------|:-----:|-------|
| `TB_PRESENT_THRESHOLD` | 0.5 | TB elevation threshold |
| `PB_PRESENT_THRESHOLD` | 0.5 | PB elevation threshold |
| `AC_ELEVATED_THRESHOLD` | 0.3 | AC elevation threshold |
| `ACUTE_CONTRIBUTOR_DECAY_HALF_LIFE` | 7 days | Decay for non-chronic contributors |
| `CONTRIBUTOR_ACTIVATION_THRESHOLD` | 0.1 | Minimum to be "active" |

---

## Types

| Item | Notes |
|------|-------|
| `Duration` | Time duration (days, weeks, years) |
| `EntityId` | Typed entity identifier |
| `EventId` | Typed event identifier |
| `RelationshipId` | Typed relationship identifier |
| `MicrosystemId` | Typed context identifier |
| `MemoryId` | Typed memory identifier |
| `Species` enum | Human, Animal, RoboticEmergent, RoboticStateless |
| `LifeStage` enum | Infancy through Elderhood |
| `LifeDomain` enum | Work, Academic, Social, Athletic, Creative, Financial, Health, Relationship |
| `BondType` enum | Secure, Avoidant, Anxious, Disorganized |

### Timestamp Support

| Item | Notes |
|------|-------|
| `Timestamp` type | Absolute timestamp (YYYY-MM-DD HH:mm:ss) |
| `timestamp.duration_since(other)` | Get duration between timestamps |
| `timestamp.add_duration(duration)` | Add duration to timestamp |
