# Behavioral Pathways: A Framework for Modeling Individual Psychology and Social Dynamics

**Conceptual Overview**
**Audience**: Architects, researchers, game designers, AI developers

---

## 1. Vision and Philosophy

### The Problem We Are Solving

How do you model a believable individual? Not their appearance or their dialogue, but their *inner life* - the psychological substrate that makes behavior coherent across time, consistent with personality, and responsive to experience.

This question arises in multiple domains:

- **Games** need NPCs that remember betrayals, hold grudges, form attachments, and respond differently based on who they are and what they have experienced
- **Social simulations** need agents whose behavior emerges from psychological state rather than scripted rules
- **AI agent systems** need models for how autonomous agents develop trust, manage relationships, and make decisions under emotional influence
- **Research tools** need standardized implementations of psychological theories for agent-based modeling experiments

Yet each domain reinvents the wheel. Game A implements mood with three variables; Game B uses twelve. Simulation C models trust as a single number; Simulation D decomposes it into components. There is no shared vocabulary, no reusable foundation, no accumulated wisdom.

**Behavioral Pathways** addresses this by providing a general-purpose framework for modeling individual psychology and social dynamics - a foundation that can be customized for any domain while sharing core concepts, algorithms, and accumulated refinements.

### What This Is Not

Behavioral Pathways is not:

- A game engine or game framework
- An AI/ML system for learning behavior
- A natural language processing system
- A character animation system
- A dialogue generation system

It is purely concerned with the *internal state* of individuals and relationships - the psychological ground truth from which behavior, dialogue, and decisions can be derived by consuming applications.

### Scope: What The System Computes

**Behavioral Pathways is a pure computation library.** It computes psychological state at any point in time given declared inputs.

**In Scope - The System Computes:**

| Capability | Description |
|------------|-------------|
| State projection | Given entity state + events + time, compute future state |
| State regression | Given entity state + events, infer what prior state would have led there |
| State at any timestamp | Query entity state at any point in the timeline |
| Psychological evolution | How personality, mood, needs, mental health change through events and time |
| Relationship dynamics | Trust formation, stage progression, behavioral predictions |
| Memory formation | Events create memories; memories influence retrieval and state |
| Context effects | Ecological layers (micro/meso/exo/macro/chrono) modify state |
| ITS risk computation | Thwarted belongingness, perceived burdensomeness, acquired capability |

**Out of Scope - Consumer Responsibility:**

| Capability | Why Out of Scope |
|------------|------------------|
| Persistence/Serialization | Consumer chooses storage format and mechanism |
| Database integration | Library doesn't care where data comes from |
| API server/networking | It's a crate, not a service |
| Real-time processing | Simulation operates on timestamps, not wall-clock |
| UI/Visualization | Consumer builds their own presentation layer |
| Event orchestration | Consumer decides which entities receive which events |
| State comparison | Consumer compares computed states to hypotheses |
| Multi-process coordination | Consumer manages their own concurrency |

### Usage Model

The system operates declaratively:

```
// 1. Create simulation with reference date
sim = Simulation(reference_date: "2024-01-01")

// 2. Add entity with ONE known anchor state at specific timestamp
sim.add_entity(entity_with_state, timestamp: "2004-01-01")  // born this date

// 3. Add events at absolute timestamps
sim.add_event(trauma_event, timestamp: "2008-01-01")  // age 4
sim.add_event(achievement_event, timestamp: "2013-01-01")  // age 9
sim.add_relationship(A, B, type: Friendship, timestamp: "2020-01-01")

// 4. Query state at ANY timestamp (system computes on-demand)
state_future = sim.entity(id).state_at("2044-01-01")  // age 40, projected forward
state_past = sim.entity(id).state_at("2010-01-01")    // age 6, regressed backward
state_anchor = sim.entity(id).state_at("2004-01-01") // age 0, the known state

// 5. Consumer does their own comparison/analysis
diff = consumer_compare(state_future, hypothesis_state)
```

**Key Constraints:**

| Rule | Description |
|------|-------------|
| One anchor per entity | Each entity has exactly one known state at a specific timestamp; all computation radiates from there |
| Timestamps are absolute | Consumer provides YYYY-MM-DD HH:mm:ss timestamps |
| System handles time conversion | Translates to species-appropriate internal units automatically |
| No persistence | Each query computes fresh from declared data |
| No built-in comparison | System computes state; consumer compares to their hypotheses |
| Deterministic | Same inputs always produce same outputs |

**Backward Regression Example:**

If you create an entity at age 20 with known state X, add an event at age 10, and query state at age 10, the system regresses: it computes what state at age 10 would have led to state X at age 20, given the event occurred at age 10.

### Design Principles

**1. Separation of State from Presentation**

The framework models *what an individual is feeling* and *how relationships stand*, not how these are displayed, described, or acted upon. A game might show a character's stress through animation; a simulation might log it to a file; a research tool might aggregate it for analysis. The framework is agnostic. Logic makes decisions; presentation renders outcomes.

**2. Theoretical Grounding**

Where possible, models are grounded in established psychological research:
- Affect modeling draws from dimensional emotion theory (valence-arousal-dominance)
- Relationship trust decomposes per Mayer/Davis/Schoorman's model
- Developmental trajectories follow Bronfenbrenner's Process-Person-Context-Time framework
- Crisis detection incorporates Joiner's Interpersonal Theory of Suicide

This grounding provides legitimacy for research applications and coherent behavior for entertainment applications.

**3. Two Pathways for Psychological Change**

The framework models psychological change through two complementary pathways:

**Short-Term Pathway: Base + Delta + Decay**

The fundamental insight underlying everyday psychological fluctuations: states have both stable components (personality traits) and transient components (current mood). The transient components naturally decay toward baseline over time unless reinforced.

This pattern - **base** (what you tend toward), **delta** (where you are now), **decay** (how you return to baseline) - unifies mood, disposition, relationship dimensions, and skill-like values under a single conceptual model.

**Long-Term Pathway: Formative Events + Base Shifts**

Major life events can permanently alter personality traits. Grounded in:
- Roberts' Social Investment Theory (role demands change personality)
- Tedeschi & Calhoun's Post-Traumatic Growth (trauma can shift traits)
- Bleidorn's Life Events Research (empirical effect sizes)

Base shifts are constrained by empirical research:
- Age plasticity (younger = more plastic)
- Trait stability coefficients (some traits resist change)
- Sensitive periods (specific ages amplify specific traits)
- Diminishing returns (repeated shifts saturate)

| Pathway | Mechanism | Duration | Example |
|---------|-----------|----------|---------|
| Short-term | Delta + decay | Hours to weeks | Bad day lowers mood |
| Long-term | Base shift | Permanent | Trauma increases Neuroticism |

Both pathways can operate simultaneously - a traumatic event creates immediate distress (delta) AND permanent personality change (base shift).

**4. Configuration Over Specialization**

Rather than creating separate systems for humans, animals, or custom entity types, the framework uses configuration. Different entity types share the same core mechanics but vary in:
- Which psychological dimensions are active
- Which subsystems run in the processing loop
- How quickly states decay (scaled by lifespan)
- Which feedback loops and thresholds apply

**5. Real-Time Time Model**

All time is expressed in real time (hours, days, weeks). Each entity has a `time_scale` derived from lifespan, so the same half-life can unfold faster or slower across species while keeping the math consistent.

**6. Determinism for Testability**

Given identical inputs, the framework produces identical outputs. There is no hidden randomness. This enables:
- Reproducible test cases
- Debugging by replaying event sequences
- Research requiring replicable results
- Save/load without state drift

Consuming applications can add randomness at their layer if desired.

---

## 2. Core Concepts

### The Individual

An **Individual** represents any entity capable of having psychological states. This could be:
- A human character in a game
- An AI agent in a multi-agent system
- An animal in a wildlife simulation
- A fictional species with custom psychology
- A simplified social actor in an economic model

Every Individual has:

- **Demographical + Demand Characteristics**: Identity, age, and observable cues that shape first impressions
- **Core Personality**: Stable trait dimensions (HEXACO) plus categorical traits
- **Attachment Style**: Patterns in how the individual forms and maintains close bonds
- **Mood**: Current affective state (temporary, decays over time)
- **Event Flags**: Short-lived indicators (e.g., recent moral violation)
- **Needs**: Drives that grow when unmet
- **Disposition**: Behavioral tendencies (impulse control, aggression, empathy, reactance, trust propensity)
- **Mental Health**: Longer-term psychological wellbeing indicators (human-focused)
- **Force + Resource Characteristics**: Motivation style and available coping resources
- **Drives**: Persistent motivational pressures (e.g., status)
- **Reputation**: Public perception dimensions (trusted, feared, hated)
- **Developmental Context**: Life stage, plasticity, cohort effects, turning points
- **Ecological Context**: Work/family/social environments, cultural context, and historical time

Not all entity types use all components. An animal simulation might omit mental health modeling; an economic agent might have minimal personality. The framework is composable.

### State Value

The **State Value** is the atomic unit of psychological modeling. It represents any dimension that can vary over time:

- Has a **base** (stable tendency, like personality)
- Has a **delta** (current deviation from base)
- Has a **decay half-life** (real time, human baseline) that is scaled by `time_scale`
- Has **min/max bounds** to clamp values

The *effective value* at any moment is base + delta (clamped to bounds).

Examples:
- Mood valence: base = 0.3 (generally positive person), delta = -0.5 (just received bad news), effective = -0.2
- Grievance: base = 0.1 (low baseline), delta = 0.4 (recent injustice), effective = 0.5
- Impulse control: base = 0.7 (personality trait), delta = -0.4 (exhausted and stressed), effective = 0.3

This pattern enables both stable individual differences AND temporary state changes, with natural return to baseline.

### Crystallization (Drift) vs. Formative Events

The framework offers two mechanisms for permanent state changes:

**Crystallization (Drift)** - Gradual incorporation of sustained deltas into base

Sometimes temporary states become permanent. A chronically stressed person develops a lower baseline for stress tolerance. A repeatedly betrayed individual develops lower baseline trust.

**Crystallization** is the process by which sustained deltas gradually incorporate into the base value. This models:
- Personality change through sustained experience
- Relationship deepening (or degradation) over time
- Skill development through practice

The rate of crystallization can vary by:
- Life stage (children crystallize faster than adults)
- State type (mood crystallizes slowly; trust crystallizes faster)
- Entity configuration (some entity types are more plastic)

**Formative Events** - Immediate base shifts from significant events

Major life events (trauma, role transitions, transformative experiences) can immediately and permanently shift personality traits. This complements crystallization:

| Mechanism | Trigger | Speed | Use Case |
|-----------|---------|-------|----------|
| Crystallization | Sustained delta over time | Gradual (months/years) | Slow adaptation to environment |
| Formative Events | Single significant event | Immediate | Major life transitions, trauma |

Both mechanisms can coexist and operate on the same entity.

### Mood

**Mood** represents the current affective state of an individual. Following dimensional emotion theory, it includes:

- **Valence**: Positive to negative feeling tone (bipolar -1 to 1)
- **Arousal**: Activation level (bipolar -1 to 1, deactivated to activated)
- **Dominance**: Sense of control (bipolar -1 to 1, powerless to in-control)
- **Fatigue**: Energy level (0 to 1, rested to exhausted)
- **Stress**: Accumulated pressure (0 to 1, relaxed to pressured)

Each is a State Value with its own decay half-life. Moods are *fast* - they shift quickly in response to events and decay back to baseline within hours to days.

**Disgust** is derived from event flags (e.g., recent moral violation) rather than stored as a persistent mood dimension.

Mood influences behavior through modulation: an individual in a negative, high-arousal, low-dominance state will react to provocation differently than the same individual in a positive, calm, in-control state.

### Event Flags

**Event Flags** are short-lived indicators derived from recent events (e.g., moral violations) that drive derived states without adding permanent mood dimensions.

### Needs

**Needs** represent drives that create motivation when unmet. Unlike mood (which is reactive), needs *grow* over time and create pressure toward action.

Examples for human individuals:
- **Loneliness**: Social connection need (thwarted belongingness)
- **Purpose**: Meaning and contribution need
- **Perceived Reciprocal Caring**: Belief that others genuinely care

Needs follow an inverted State Value pattern: they *increase* over time (delta grows) and are *satisfied* by events (delta decreases).

Different entity types can define custom needs in configuration (e.g., mission alignment, system integrity, input quality).

### Personality

**Personality** represents stable individual differences that influence how someone perceives, processes, and responds to their environment.

For human individuals, the framework supports the HEXACO model (modeled on a -1 to 1 scale):
- **Honesty-Humility**: Sincerity, fairness, lack of greed
- **Neuroticism**: Emotional instability, anxiety, moodiness
- **Extraversion**: Sociability, assertiveness, positive affect
- **Agreeableness**: Cooperation, trust, flexibility
- **Conscientiousness**: Organization, diligence, self-discipline
- **Openness**: Curiosity, creativity, preference for novelty

Personality is modeled as State Values with very slow decay - personality can change, but slowly, through sustained experience.

Other entity types can use different personality models:
- Animals: Boldness, activity level, sociability, exploration
- AI agents: Risk tolerance, goal flexibility, collaboration tendency
- Custom: Whatever dimensions matter for the domain

### Attachment Style

**Attachment Style** describes patterns in how individuals form and maintain close relationships, based on attachment theory:

- **Anxiety**: Fear of abandonment, need for reassurance, sensitivity to rejection
- **Avoidance**: Discomfort with closeness, preference for independence, suppression of needs

These dimensions combine into four styles:
- Secure (low anxiety, low avoidance): Comfortable with intimacy and autonomy
- Anxious-Preoccupied (high anxiety, low avoidance): Seeks closeness but fears rejection
- Dismissive-Avoidant (low anxiety, high avoidance): Values independence, minimizes attachment
- Fearful-Avoidant (high anxiety, high avoidance): Wants closeness but fears it

Attachment influences how individuals:
- Respond to relationship threats
- Seek or avoid intimacy
- Process relationship-relevant events
- Develop (or resist developing) trust

### Disposition

**Disposition** captures behavioral tendencies that are personality-adjacent but more malleable:

- **Impulse Control**: Ability to delay gratification, resist urges
- **Empathy**: Responsiveness to others' emotional states
- **Aggression**: Tendency toward hostile response
- **Grievance**: Accumulated sense of injustice
- **Reactance**: Resistance to perceived control attempts
- **Trustor Propensity**: Baseline willingness to trust others

These are State Values with medium decay rates - more stable than mood, more changeable than personality.

### Mental Health

**Mental Health** models longer-term psychological wellbeing, including clinical-adjacent phenomena:

- **Depression**: Persistent low mood and anhedonia
- **Self-Worth / Self-Hate**: Global valuation of self (positive and negative)
- **Hopelessness**: Cognitive belief about future outcomes
- **Burdensomeness**: Perception of being a burden to others
- **Acquired Capability**: Habituation to pain/fear through experience
- **Suicidal Ideation / Attempt Risk**: Computed from ITS components

These have slow decay rates and can trigger threshold alerts when they cross critical values (human-focused).

### Force and Resource Characteristics

**Force Characteristics** capture motivation style beyond traits:
- **Baseline Motivation**: Drive to initiate action
- **Persistence**: Tendency to continue despite difficulty
- **Curiosity**: Drive for information and novelty
- **Domain Self-Efficacy**: Confidence by domain (e.g., "social", "technical")

**Resource Characteristics** capture available coping resources:
- **Cognitive Ability**: Reasoning/problem solving capacity
- **Emotional Regulation Assets**: Learned coping tools
- **Social Capital**: Access to supportive relationships
- **Material Security**: Access to food, shelter, savings
- **Experience Diversity**: Breadth of life domains encountered

### Drives

**Drives** are persistent motivational pressures, such as:
- **Status**: Ambition and dominance seeking

### Reputation

**Reputation** captures public perception:
- **Trusted**, **Feared**, **Hated**

### Developmental Context

**Developmental Context** captures how the individual's current state relates to their life trajectory:

- **Life Stage**: Child, adolescent, young adult, adult, elder
- **Plasticity**: How much current experiences shape long-term development
- **Sensitive Periods**: Whether the individual is in a critical period for certain types of development
- **Cohort Effects**: How the era/environment of upbringing shapes expectations and values
- **Turning Points**: Major life events that redirected development

Life stage influences:
- Crystallization rates (children are more plastic)
- Which needs are salient (e.g., belonging vs. purpose)
- How relationships are processed (children: attachment-focused; adults: reciprocity-focused)

### Ecological Context

**Ecological Context** models the environments that shape behavior:

- **Microsystem**: Direct settings (work, family, social, education, healthcare, neighborhood)
- **Mesosystem**: Interactions between microsystems (e.g., work-family conflict)
- **Exosystem**: Indirect influences (health systems, community resources)
- **Macrosystem**: Cultural and institutional patterns
- **Chronosystem**: Historical era, turning points, and cohort effects
- **Spillover**: Cross-context effects and person-context fit

Context variables influence needs, stress, and relationship dynamics, while entities actively shape their contexts through behavior.

---

## 3. Entity Model

### Conceptual Diagram

```
+--------------------------------------------------------------+
|                        INDIVIDUAL                             |
|  +----------------------+  +----------------+  +-----------+ |
|  | Demographical/Demand |  | Core Personality|  | Attachment| |
|  | - id, age, signals   |  | - hexaco        |  | - anxiety | |
|  +----------------------+  | - traits        |  | - avoid.  | |
|                            +-----------------+  +-----------+ |
|  +----------------------+  +----------------+  +-----------+ |
|  | Mood + Event Flags   |  | Needs          |  | Disposition| |
|  | - val/ar/dominance   |  | - loneliness   |  | - impulse | |
|  | - fatigue/stress     |  | - purpose      |  | - empathy | |
|  | - moral_violation    |  | - recip_care   |  | - grievance|
|  +----------------------+  +----------------+  | - trustor | |
|                            +----------------+  +-----------+ |
|  +----------------------+  +----------------+  +-----------+ |
|  | Mental Health        |  | Force/Resource |  | Drives/Rep| |
|  | - depression         |  | - motivation   |  | - status  | |
|  | - self-worth/hate    |  | - assets       |  | - trusted | |
|  | - hopelessness       |  +----------------+  | - feared  | |
|  +----------------------+                      | - hated   | |
|  +----------------------+  +-----------------------------+ |
|  | Developmental        |  | Ecological Context          | |
|  | - life stage         |  | - micro/meso/exo/macro/chrono|
|  +----------------------+  +-----------------------------+ |
+--------------------------------------------------------------+
           |
           | has many
           v
+--------------------------------------------------------------+
|                        RELATIONSHIP                           |
|  +----------------------+  +-----------------------------+   |
|  | Shared Dimensions    |  | Directional Dimensions      |   |
|  | - affinity/respect   |  | - warmth/resentment         |   |
|  | - tension/intimacy   |  | - dependence/attraction     |   |
|  | - history            |  | - attachment/jealousy/fear  |   |
|  +----------------------+  +-----------------------------+   |
|  +----------------------+  +-----------------------------+   |
|  | Trustworthiness      |  | Trust State (A->B, B->A)     |   |
|  | - competence         |  | - task/support/disclosure   |   |
|  | - benevolence        |  |   willingness               |   |
|  | - integrity          |  |                             |   |
|  +----------------------+  +-----------------------------+   |
|  +----------------------+  +-----------------------------+   |
|  | Bonds/Schema/Stage   |  | Interaction Pattern         |   |
|  | - bond labels        |  | - frequency/consistency     |   |
|  | - relationship type  |  | - last interaction          |   |
|  +----------------------+  +-----------------------------+   |
+--------------------------------------------------------------+
           |
           | recorded in
           v
+--------------------------------------------------------------+
|                          MEMORY                              |
|  +------------------------------------------------------+   |
|  |                    Memory Entry                      |   |
|  |  - event_id, timestamp                               |   |
|  |  - participants, tags                                |   |
|  |  - source + confidence                               |   |
|  |  - emotional_snapshot (val/ar/dominance)             |   |
|  |  - salience, deltas_applied, summary                 |   |
|  +------------------------------------------------------+   |
|  +------------------------------------------------------+   |
|  |                  Memory Layers                       |   |
|  |  IMMEDIATE(10) -> SHORT-TERM(20) -> LONG-TERM(50)    |   |
|  |  -> LEGACY(unlimited, milestone-triggered)          |   |
|  +------------------------------------------------------+   |
+--------------------------------------------------------------+
```

### Relationship Structure

Relationships are bidirectional connections between two Individuals. They include:

**Shared dimensions** (symmetric - same value from both perspectives):
- Affinity (general mutual liking)
- Respect (mutual admiration)
- Tension (unresolved conflict)
- Intimacy (emotional closeness)
- History (depth of shared experience - monotonically increasing)

**Trustworthiness factors** (perceived qualities of the other):
- Competence, benevolence, integrity

**Trust state** (directional willingness to be vulnerable):
- Task, support, and disclosure willingness (A->B, B->A)

**Relationship framing**:
- Bond labels, schema (peer/mentor/etc.), and stage (stranger -> intimate)
- Interaction pattern (frequency, consistency, last interaction)

**Directional dimensions** (asymmetric - can differ by direction):
- Warmth, resentment, dependence, attraction
- Attachment, jealousy, fear, obligation

This asymmetry models reality: one person can feel warmly toward another who resents them; attachment can be one-sided; fear and obligation often flow in one direction.

### Memory Architecture

Memory serves multiple purposes in the framework:

1. **Context for behavior**: An individual who remembers a betrayal will behave differently than one who doesn't
2. **Mood-congruent retrieval**: Current mood biases which memories surface
3. **Narrative coherence**: Accumulated memories form the basis of an individual's story
4. **Relationship history**: Memories of shared experiences anchor relationship depth

The layered architecture reflects cognitive science and is capacity-limited:
- **Immediate**: Minutes-hours (cap 10)
- **Short-term**: Days-weeks (cap 20)
- **Long-term**: Months-years (cap 50)
- **Legacy**: Milestone-triggered, unlimited

Each memory entry stores source (self, witness, rumor) with a confidence weight, a PAD emotional snapshot (valence/arousal/dominance), tags, salience, and deltas applied. Memories are promoted based on salience and retained for narrative inspection.

---

## 4. Capabilities Overview

### Processing Loop (Per Entity)

1. **Event processing**: apply event effects, create memories, queue responses
2. **State processing**: decay deltas, grow needs, run feedback loops, compute derived values
3. **Relationship updates**: apply interaction effects and decay
4. **Memory processing**: consolidate and promote salient memories
5. **Context updates**: apply micro/meso/exo/macro/chrono effects
6. **Developmental updates** (less frequent): life-stage shifts, plasticity, cohort effects

### State and Time

| Capability | Description |
|------------|-------------|
| **apply_decay** | Decay all deltas toward baseline using real-time half-life scaled by `time_scale` |
| **apply_needs_growth** | Increase unmet needs over time |
| **crystallize** | Convert sustained deltas into base value changes |
| **derive_emotions** | Compute derived affective indicators (e.g., disgust from event flags) |

### Behavioral Decisions

| Capability | Description |
|------------|-------------|
| **predict_compliance** | Predict compliance outcome and confidence |
| **select_coping_strategy** | Choose coping style (problem-focused, social support, avoidance, etc.) |
| **predict_reaction** | Predict reaction type (ignore, flee, de-escalate, confront, fight back, rage) |
| **select_context_focus** | Choose microsystem focus based on needs and characteristics |
| **compute_context_shaping** | Modify microsystem warmth/hostility based on entity influence |

### Threshold Detection

| Capability | Description |
|------------|-------------|
| **check_thresholds** | Scan state for alerts (suicidality, rebellion risk, burnout, rupture) |

### Relationship Processing

| Capability | Description |
|------------|-------------|
| **update_trustworthiness** | Apply antecedents to competence/benevolence/integrity |
| **update_trust_willingness** | Compute task/support/disclosure willingness per direction |
| **decay_relationship** | Apply time-based decay to tension and unused ties |
| **update_interaction_pattern** | Track frequency and consistency of interactions |

### Interaction Processing

| Capability | Description |
|------------|-------------|
| **process_interaction** | Apply conversation/encounter effects and feed relationship + memory updates |

### Memory Operations

| Capability | Description |
|------------|-------------|
| **encode_memory** | Create a memory entry with salience and PAD snapshot |
| **consolidate_memories** | Promote immediate memories to longer-term layers |
| **retrieve_memories** | Query memories with mood-congruent bias and source confidence |

### Context and Development

| Capability | Description |
|------------|-------------|
| **apply_microsystem_effects** | Apply local environment effects (work, family, social) |
| **apply_mesosystem_spillover** | Apply cross-context effects and role conflict |
| **apply_exosystem_effects** | Apply indirect system-level influences |
| **apply_macrosystem_effects** | Apply cultural and institutional modifiers |
| **apply_chronosystem_effects** | Apply historical period and turning point impacts |
| **apply_developmental_effects** | Update life stage, plasticity, and cohort effects |

---

## 5. Extensibility Model

### Entity Type Configuration

The framework is designed to support multiple entity types through configuration rather than code modification. An entity type configuration specifies:

**1. Active Dimensions**

Which psychological dimensions this entity type uses:
- Does it have all HEXACO personality dimensions, or a subset?
- Which needs are relevant?
- Is mental health modeled?
- What mood dimensions are active?

**2. Time Scale and Decay Rates**

How quickly different state types return to baseline (scaled by lifespan):
- Fast decay (hours): Immediate emotional reactions
- Medium decay (days): Situational mood states
- Slow decay (weeks): Dispositional states
- Very slow decay (months): Personality-adjacent traits

Different entity types have different time scales. An animal might process emotions in minutes; an organization might process reputation over years.

**3. Active Subsystems and Feedback Loops**

Which subsystems and feedback dynamics operate for this entity type:
- Biological entities: Stress-fatigue loops, mood-behavior loops
- AI entities: Performance-confidence loops, goal-conflict loops
- Social entities: Reputation-opportunity loops

**4. Threshold Definitions**

What state combinations trigger alerts:
- Humans: Suicidality thresholds, rebellion thresholds
- AI agents: System degradation thresholds, misalignment thresholds
- Custom: Whatever matters for the domain

### Example Configurations

The reference model defines **Human** and **Animal** entity types; additional types are supported via configuration.

**Human Configuration**
- Full HEXACO personality
- Needs: loneliness, purpose, perceived reciprocal caring
- Mental health: depression, self-worth, self-hate, hopelessness, burdensomeness, acquired capability
- ITS thresholds and rebellion thresholds defined
- Time scale: 1.0x (baseline)

**AI Agent Configuration**
- Simplified personality: risk tolerance, collaboration tendency, goal flexibility
- Needs: mission alignment, system integrity, input quality
- No mental health modeling (or simplified "system degradation")
- Feedback loops: performance-confidence, goal-conflict
- Thresholds: misalignment detection, degradation alerts

**Animal Configuration**
- Simplified personality: boldness, sociability, activity level
- Needs: hunger, safety, social (for pack animals), territory
- Minimal or no mental health modeling
- Feedback loops: stress-fatigue only
- Thresholds: acute stress, social isolation (for social species)

**Fictional Species Configuration**
- Custom personality dimensions appropriate to lore
- Custom needs reflecting species biology/psychology
- Custom feedback loops as appropriate
- Custom thresholds for species-specific crises

### Domain-Specific Extensions

Consuming applications can extend the framework with domain-specific concepts:

**Game-Specific Extensions**:
- Skill systems that use State Value patterns
- Faction reputation modeling
- Context-specific behavior (work vs. social vs. family)
- Narrative state tracking

**Research-Specific Extensions**:
- Data export for analysis
- Intervention modeling
- Population-level aggregation
- Parameter sweep capabilities

**AI System Extensions**:
- Goal representation
- Belief systems
- Planning integration
- Communication modeling

---

## 6. Use Case Scenarios

### Scenario 1: Colony Survival Game (TOI)

**Context**: A narrative survival game where player manages a colony of individuals with complex inner lives. NPCs have dialogue generated by LLMs, using psychological state to inform responses.

**How Behavioral Pathways Applies**:

The game uses Human Configuration for colonists. Each colonist is an Individual with full psychological modeling.

When a colonist's friend dies:
1. Framework receives event, computes salience (high - novel, threatening, personal)
2. Applies mood deltas: valence drops, arousal spikes, stress increases
3. Updates relationship (friend is gone, but memories persist)
4. Creates memory entry with emotional snapshot
5. Threshold check: Do thwarted belongingness, perceived burdensomeness, and hopelessness cross warning threshold (and acquired capability for critical)?

Over subsequent game ticks:
1. Mood deltas decay, but slowly due to salience
2. Loneliness need grows (lost social connection)
3. Feedback loops process: stress -> fatigue -> reduced coping
4. Depression may develop if multiple stressors accumulate

When player issues orders:
1. Framework predicts compliance based on colonist state and relationship with player
2. Grievance and reactance factor into prediction
3. Result informs whether colonist complies, delays, argues, or refuses

When colonist has dialogue:
1. Game queries current state, recent memories, relationship with conversation partner
2. This context informs LLM prompt
3. Dialogue reflects psychological state authentically

### Scenario 2: Multi-Agent AI Research Platform

**Context**: A research platform studying emergent behavior in populations of AI agents with social dynamics.

**How Behavioral Pathways Applies**:

Researchers configure AI Agent entities with simplified psychological models focused on trust, collaboration, and goal alignment.

Agents have:
- Simplified personality (3-4 dimensions)
- Needs focused on mission completion and system integrity
- Relationships with trust and interaction patterns
- Memory of past interactions

Research questions enabled:
- How does trust develop in agent networks?
- What conditions lead to cooperation vs. competition?
- How do betrayals affect future behavior?
- Can agents "repair" damaged relationships?

The framework provides:
- Deterministic computation for reproducibility
- Configurable parameters for systematic variation
- State export for analysis
- Batch processing for large populations

### Scenario 3: Educational Social Simulation

**Context**: A training simulation teaching interpersonal skills (conflict resolution, leadership, cross-cultural communication).

**How Behavioral Pathways Applies**:

Simulated characters use Human Configuration. Trainees interact with characters through dialogue choices and actions.

Characters respond based on:
- Personality (some are agreeable, some are confrontational)
- Current mood (stressed characters respond differently)
- Relationship with trainee (trust affects openness)
- Recent history (bringing up past conflicts triggers memories)

The framework enables:
- Consistent character behavior across scenarios
- Trackable trainee impact on character states
- Graduated difficulty (vary character disposition)
- Assessment based on relationship outcomes

### Scenario 4: Wildlife Behavior Simulation

**Context**: An ecological simulation studying predator-prey dynamics with realistic animal behavior.

**How Behavioral Pathways Applies**:

Animals use Animal Configuration with simplified psychology:
- Personality: boldness, activity level
- Needs: hunger, safety, territorial (for territorial species)
- Mood: arousal, stress
- No mental health modeling

The framework models:
- Individual differences in risk-taking (bold vs. shy animals)
- State-dependent behavior (hungry animals take more risks)
- Stress effects on reproduction and foraging
- Social dynamics for pack/herd species

Benefits over simple behavioral rules:
- Individual variation creates realistic population dynamics
- State-dependent behavior produces emergent patterns
- Memory of threats affects future behavior

---

## 7. Theoretical Foundations

### Dimensional Emotion Theory

The mood model is grounded in dimensional approaches to emotion (Russell's circumplex model, Mehrabian's PAD model):

- **Valence**: The pleasure-displeasure dimension
- **Arousal**: The activation-deactivation dimension
- **Dominance**: The control-powerlessness dimension

This approach, rather than discrete emotions, enables:
- Smooth transitions between states
- Mathematical operations on emotional state
- Clear semantics for combination effects

### HEXACO Model of Personality

For human entities, personality uses the HEXACO model (Ashton & Lee, 2007), which extends Big Five with Honesty-Humility:

- Empirically derived from lexical studies across multiple languages
- Better predicts certain behaviors (workplace deviance, cooperation)
- More nuanced than Big Five in interpersonal domains

### Attachment Theory

Attachment style modeling follows Bartholomew & Horowitz (1991):

- Two-dimensional model (anxiety x avoidance)
- Predicts relationship behavior across the lifespan
- Influences how relationship events are processed

### Trust Decomposition

Trust modeling follows Mayer, Davis & Schoorman (1995):

- **Competence**: Can they perform?
- **Benevolence**: Do they care about my interests?
- **Integrity**: Are they honest and ethical?

This decomposition enables nuanced trust dynamics - you can trust someone's competence but not their intentions.

### Joiner's Interpersonal Theory of Suicide

Crisis detection incorporates Joiner's ITS (2005):

- Suicidal desire requires both thwarted belongingness AND perceived burdensomeness
- Suicidal capability requires acquired capability (habituation to pain/fear)
- Hopelessness is included as a cognitive amplifier in warning thresholds

**Ethical note**: This modeling is for simulation purposes. The framework is not a clinical tool and should not be used for real-world suicide risk assessment.

### Bronfenbrenner's PPCT Model

Developmental modeling draws from Bronfenbrenner's Process-Person-Context-Time framework:

- **Proximal processes**: Sustained, regular interactions drive development
- **Person characteristics**: Individual differences affect developmental outcomes
- **Context**: Environment shapes what development is possible
- **Time**: Both historical time (cohort effects) and life stage matter

This informs how the framework weights experiences for long-term impact.

### Mood-Congruent Processing

Memory retrieval incorporates mood-congruent processing (Bower, 1981):

- Current mood biases which memories are retrieved
- Negative mood activates negative memories, creating feedback potential
- This creates realistic rumination patterns

---

## 8. Open Questions

### Conceptual Questions

**Q1: What is the boundary of an "individual"?**

Is a faction an individual? A family unit? A company? The current design assumes individuals are atomic entities with their own psychology. Group-level phenomena (mob behavior, organizational culture) would need separate modeling or emergent treatment.

**Q2: How do we model genuinely alien psychologies?**

The framework is grounded in human psychology research. For truly alien entities (not just fictional species with human-like minds), what should "mood" or "attachment" even mean? Is there a more general abstraction?

**Q3: How should context/environment be modeled?**

The current design focuses on individuals and relationships. But context matters enormously - behavior at work differs from behavior at home. Should context be a first-class concept, or a consumer-layer concern?

**Q4: What about collective memory and reputation?**

An individual has memories of interactions. But what about an individual's *reputation* - what others remember about them? This is currently left to consumers but might warrant core support.

### Technical Questions

**Q5: How should time be abstracted?**

The framework uses "ticks" but consumers have different time scales. Is tick-based sufficient, or should there be first-class support for real-time, game-time, and wall-clock-time mappings?

**Q6: What serialization format?**

For save/load and data export, what formats should be supported? Just the language's native serialization, or standard formats (JSON, MessagePack, etc.)?

**Q7: How to handle impossibly large relationship graphs?**

For N individuals, there are N*(N-1)/2 potential relationships. At scale (thousands of individuals), this is intractable. Should relationships be sparse? Created on-demand? What's the expected relationship density?

**Q8: What observability/debugging support is needed?**

When behavior is unexpected, how do consumers trace causation? State snapshots? Event logs? Change tracking?

### Research Questions

**Q9: How should parameters be validated?**

The framework will ship with default parameters (decay rates, thresholds, coefficients). How should these be validated? Against what criteria?

**Q10: How do we handle the replication crisis?**

Some psychological theories are better supported than others. How much should the framework commit to specific theories vs. remain agnostic?

**Q11: Is there a simpler core?**

Could the essential insights (base + delta + decay, feedback loops, threshold detection) be expressed in a much simpler framework that users extend, rather than a rich framework users configure?

---

## Appendix: Glossary

| Term | Definition |
|------|------------|
| **Individual** | Any entity capable of psychological states |
| **State Value** | A dimension with base, delta, and decay components |
| **Base** | The stable tendency for a state dimension |
| **Delta** | Current temporary deviation from base |
| **Decay** | The rate at which delta returns to zero |
| **Time Scale** | Multiplier that converts real time into psychological time |
| **Crystallization** | Process of delta becoming incorporated into base |
| **Mood** | Fast-moving affective state dimensions (PAD + fatigue/stress) |
| **Needs** | Drives that grow over time and motivate behavior |
| **Personality** | Stable individual differences in perception/processing/response |
| **Disposition** | Behavioral tendencies more malleable than personality |
| **Attachment Style** | Patterns in relationship formation and maintenance |
| **Relationship** | Bidirectional connection with shared, directional, and trust components |
| **Trustworthiness** | Perceived competence, benevolence, and integrity of another |
| **Trust State** | Willingness to be vulnerable across task/support/disclosure domains |
| **Shared Dimension** | Relationship aspect that is symmetric |
| **Directional Dimension** | Relationship aspect that can differ by direction |
| **Memory** | Record of past experience with source confidence and emotional context |
| **Salience** | How memorable/important an experience is |
| **Feedback Loop** | Self-reinforcing cycle in psychological dynamics |
| **Threshold Alert** | Notification that a state combination has crossed a significant boundary |
| **Ecological Context** | Micro/meso/exo/macro/chrono systems shaping behavior |
| **Entity Configuration** | Settings that customize framework behavior for an entity type |

---

*This document describes the conceptual architecture of Behavioral Pathways. Implementation details, API specifications, and integration guides will be provided in separate technical documentation.*
