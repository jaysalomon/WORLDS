# POLIS State Model And Simulation Contract

**Version:** 0.1  
**Date:** 14 March 2026  
**Document:** 02 of the POLIS spec suite  
**Purpose:** Define what state exists in POLIS, who owns it, how it changes, what is logged, and how multi-scale consistency is maintained.

## 1. Scope

This document defines the simulation state contract for POLIS.

It answers:

- what categories of state exist
- which entity or subsystem owns which state
- what may be stored directly
- what must be derived
- what exists only transiently during processing
- what must be logged for replay and audit
- how state may change
- how state remains coherent across scale transitions

This document does **not** define:

- the full ontology of entities
- detailed numerical methods
- low-level implementation architecture
- specific storage engines
- UI state or rendering state

Those belong in other documents.

## 2. Dependencies

This document depends on the ontology defined in:

- [01_WorldModel.md](/abs/path/e:/Drive/WORLDS/01_WorldModel.md)

It establishes contracts for later documents, especially:

- `03_CollectiveAgency.md`
- `04_ResourcesAndMaterials.md`
- `05_DiscoveryHeuristics.md`
- `07_SocietyAndInstitutions.md`
- `08_ValidationAndExperiments.md`
- `10_TechnicalArchitecture.md`

## 3. Guiding Principle

POLIS must treat state as disciplined scientific model state, not as a convenient pile of variables.

The state contract must guarantee:

- unique authority
- causal traceability
- reproducibility
- multi-scale consistency
- separation between authoritative facts and derived summaries

If those guarantees fail, the simulator is not scientifically serious.

## 4. State Categories

POLIS uses four primary state categories.

### 4.1 Persistent State

`PersistentState` is the minimum authoritative state required to reconstruct the active world at a given simulation time.

It includes:

- enduring entity attributes
- current controlled quantities
- current rule definitions
- current field values
- current memberships and ownership relations

Persistent state is long-lived and must survive save, load, replay checkpointing, and experimental replication.

### 4.2 Derived State

`DerivedState` is computed from authoritative state and event history.

It includes:

- aggregate indicators
- summary metrics
- projections used for inspection
- cached spatial indices
- temporary multi-scale summaries

Derived state may be cached for efficiency, but it is never the primary source of truth.

### 4.3 Transient State

`TransientState` exists only during active computation or local process execution.

It includes:

- commands awaiting validation
- pathfinding buffers
- negotiation scratch data
- local message queues
- temporary process workspaces

Transient state must not be relied upon for long-term world identity.

### 4.4 Logged State

`LoggedState` is the durable record of state transitions and simulation provenance.

It includes:

- emitted events
- causal links between events
- simulation time stamps
- scenario and run metadata
- checkpoint metadata

Logged state exists for replay, debugging, audit, and scientific traceability.

## 5. Authority Model

The simulation must obey the `Unique Authority Principle`.

### 5.1 Unique Authority Principle

Every discrete piece of authoritative state must have exactly one current write authority.

This means:

- one owner for each primary field
- one subsystem responsible for validating changes
- no parallel mutation of the same authoritative value by multiple systems

Other systems may:

- read authoritative state
- cache summaries
- propose changes through valid commands

They may not directly mutate state they do not own.

### 5.2 Authority Types

Authority may belong to:

- an entity class
- a process authority
- a domain subsystem
- a mediated shared authority for contested substrate

The exact runtime implementation is deferred, but the ownership contract is mandatory.

### 5.3 Mediated Conflict Resolution

When multiple actors or systems seek to affect the same authoritative state, a designated mediator must resolve the conflict before change is committed.

Examples:

- two actors drawing from the same water source
- multiple groups claiming the same territory
- overlapping directives on the same subordinate actor

Mediation is part of authority, not an afterthought.

## 6. Ownership Rules By Meta-Category

This section defines what kinds of state belong to each ontology family.

### 6.1 Actors own

Actors may own:

- internal condition
- beliefs and memory
- goals or preferences
- learned capabilities
- directly controlled portable inventory
- commitments and current intents

Actors do not own:

- institutions as rule systems
- environmental fields
- public aggregate indicators
- another entity's internal state

### 6.2 Rule systems own

Rule systems may own:

- permissions
- obligations
- sanctions
- role definitions
- participation constraints
- jurisdiction and scope

Rule systems do not own:

- physical resource quantities
- bodily states of actors
- private intentions of actors

### 6.3 Substrate entities own

Substrate entities may own:

- location
- physical quantities
- material properties
- ecological properties
- field intensities
- structural capacities
- stock levels

Substrate entities do not own:

- decisions
- role occupancy
- social intent

### 6.4 Processes own

Processes may own:

- participation records
- contextual parameters
- validation status
- transition records
- temporary process-local state

Processes do not own:

- long-term world identity
- enduring actor beliefs
- durable substrate capacities except by validated effect

### 6.5 Aggregates own

Aggregates may own:

- query definitions
- derived summaries
- cached indicators

Aggregates do not own:

- primary causal state
- direct action authority

## 7. Persistent State Rules

Persistent state should store only what is required for authoritative world continuity.

### 7.1 Persistent state should include

- identity-defining attributes
- current quantities that materially affect future outcomes
- current memberships and bindings
- current institutional rules in force
- current environmental and material state
- currently valid ownership and control links

### 7.2 Persistent state should exclude

- values easily recomputable from authoritative facts
- presentation-only labels
- temporary process buffers
- UI-specific summaries
- convenience duplicates of another owner’s data

### 7.3 Identity minimums

Every entity type must have a defined `IdentityState` minimum.

This minimum is the irreducible persistent subset needed to:

- distinguish the entity from others
- preserve causal continuity
- support valid replay from a checkpoint

The full per-entity identity minimums are defined in later domain documents and the eventual schema catalog.

## 8. Derived State Rules

Derived state exists to make the simulation legible and tractable without corrupting authority.

### 8.1 Valid derived state

Valid derived state includes:

- social cohesion summaries
- aggregate output measures
- regional scarcity indices
- settlement stability estimates
- social network centrality measures
- field summaries

### 8.2 Derived state constraints

Derived state must:

- be reproducible from authoritative state and valid logs
- be invalidated or refreshed when source state changes
- never silently override authoritative facts

### 8.3 Double-update prohibition

No macro-level derived variable may be independently updated as if it were authoritative when it is supposed to summarize micro-level facts.

This rule exists to prevent:

- state drift
- contradictory cross-scale values
- hidden causal duplication

## 9. Transient State Rules

Transient state is necessary, but dangerous if allowed to leak into model semantics.

### 9.1 Valid transient state

Valid transient state includes:

- commands pending validation
- message envelopes
- local arbitration workspace
- route proposals
- temporary action scoring results
- process-local buffers

### 9.2 Constraints on transient state

Transient state must:

- have bounded lifetime
- be clearable without loss of world identity
- not serve as hidden persistence

If transient state must survive beyond its local cycle, it is no longer transient and must be reclassified.

## 10. Logged State And Provenance

POLIS must maintain a durable record of why state changed.

### 10.1 Purpose of logged state

Logged state exists for:

- replay
- audit
- debugging
- causal inspection
- scientific validation
- experiment comparison

### 10.2 Required event metadata

Each logged event must include enough metadata to identify:

- what changed
- which entities were involved
- which authority validated the change
- when it occurred in simulation time
- which prior event or condition caused it

### 10.3 Minimum provenance fields

Each event record should have at minimum:

- event identifier
- event type
- simulation time stamp
- step or phase stamp
- participating entity references
- responsible authority reference
- causal parent or parent set
- payload sufficient to reconstruct the transition

### 10.4 Causal linkage

Events must be causally traceable.

That does not require a single-parent chain in all cases. Some events may depend on:

- multiple prior events
- environmental thresholds
- institutional state
- aggregated conditions

But the causal basis must still be representable.

## 11. Command-Event-Effect Contract

POLIS must not allow unconstrained direct mutation of authoritative state.

All authoritative state change must pass through the `Command-Event-Effect` contract.

### 11.1 Command

A `Command` is a request or proposed action.

It is:

- intentional
- provisional
- not yet fact

Examples:

- move to location
- extract resource
- form alliance
- adopt rule
- transfer goods

### 11.2 Event

An `Event` is an accepted state transition.

It exists only after:

- validation against current state
- validation against applicable rules
- mediation of contested access when necessary

Once emitted, the event becomes part of logged history.

### 11.3 Effect

An `Effect` is the application of the validated event to authoritative state and allowed derived projections.

Effects may update:

- actor state
- substrate state
- rule-system state
- derived summaries
- audit logs

### 11.4 Direct mutation prohibition

No authoritative state may be mutated outside a valid effect path.

This is a hard contract.

## 12. Reproducibility Contract

POLIS must support repeatable scientific execution.

### 12.1 Reproducibility requirement

For a fixed scenario, initial state, rule set, and controlled runtime conditions, the simulator must reproduce the same authoritative trajectory within the defined reproducibility guarantee of the engine.

The exact technical guarantee is specified later in architecture documents, but reproducibility itself is non-negotiable.

### 12.2 Reproducibility inputs

Each run must preserve:

- scenario definition
- initial state or seed state
- rule configuration
- random seed material
- build and schema identifiers
- checkpoint references
- relevant execution metadata

### 12.3 Randomness discipline

Randomness must be scoped and controlled so that:

- stochastic decisions are traceable
- adding unrelated entities does not silently perturb unrelated stochastic paths
- experiments remain comparable across runs

The exact PRNG scheme is an implementation concern, but scoped determinism is required by the contract.

## 13. Checkpoints And Snapshots

Long runs require checkpointing without losing causal discipline.

### 13.1 Snapshot role

A snapshot is a durable capture of current persistent state plus the metadata needed to resume execution.

### 13.2 Snapshot requirements

A valid snapshot must include:

- simulation time
- authoritative persistent state
- enough run metadata to resume consistently
- reference to the relevant event-log boundary

### 13.3 Snapshot rule

Snapshots are optimization and recovery tools, not substitutes for provenance.

## 14. Multi-Scale State Bridging

POLIS must support state transitions across resolution and scale without duplicating authority.

### 14.1 Proxy pattern

When a lower-level population is represented at a higher level, a `ProxyEntity` may hold aggregate active state for that scale.

If this occurs:

- underlying micro-state must remain recoverable
- the proxy must have clearly bounded authority
- the proxy must not silently invent contradictory micro-facts

### 14.2 Dormant micro-state rule

If micro-agents are abstracted out of active simulation, their dormant state must remain available for later reactivation or validated reconstruction.

They are not to be treated as destroyed unless the model explicitly defines irreversible elimination.

### 14.3 Delegation pattern

Higher-level actors may influence lower-level actors through directives, incentives, constraints, or modified action weights.

They may not bypass the state contract by directly rewriting another actor’s internal state without a defined mediation rule.

### 14.4 Summarization pattern

Macro-level indicators and observer entities may summarize lower-level events and state.

These summaries:

- support inspection
- support decision support
- support later institutions

But they do not replace lower-level authority unless a later scale transition explicitly does so.

## 15. Constraints And Anti-Patterns

The following are prohibited or strongly disfavoured.

### 15.1 Duplicate authority

The same authoritative value stored and updated in multiple places.

### 15.2 Hidden persistence

Temporary buffers that quietly become enduring model state.

### 15.3 UI-state contamination

Frontend summaries or labels treated as backend truth.

### 15.4 Macro-state drift

Aggregate values updated independently from the micro-state they summarize.

### 15.5 Unlogged authoritative change

Any important state mutation that cannot be traced through valid effect history.

### 15.6 Scale incoherence

Micro and macro forms of the same population evolving independently without a defined bridging contract.

## 16. Open Questions

These questions remain for later documents:

- Which domain states must always remain micro-resolved even under scale abstraction?
- Which institutions own persistent state directly versus only rule definitions?
- Which derived summaries are required for agent perception versus only for analysis?
- How fine-grained should causal parentage be for dense environmental updates?

These are design questions for later specifications, not blockers for this contract.

## 17. Summary

POLIS should treat simulation state as a disciplined contract built on:

- persistent authoritative facts
- derived but non-authoritative summaries
- bounded transient process state
- logged causal history
- unique write authority
- command-event-effect mutation
- reproducible checkpoints and replay
- explicit multi-scale bridging rules

This provides the backbone needed for a serious simulation backend before implementation details are chosen.
