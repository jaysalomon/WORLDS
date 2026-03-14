# 10. Technical Architecture

## Purpose

This document defines the technical architecture for POLIS as a serious, multi-scale civilization simulator. It specifies the architectural layers, runtime contracts, dataflow rules, performance strategy, and integration boundaries required to make the system scientifically defensible, computationally tractable, and maintainable over time.

This document does not redefine the world ontology, state semantics, institutional model, biology, or presentation logic. Those are defined in the earlier numbered specifications and are treated here as architectural inputs.

## Scope

This specification covers:

- architectural layers and ownership boundaries
- runtime flow between state, systems, events, and persistence
- scheduling and multi-rate stepping
- data layout and access patterns
- replay, provenance, and experimentation infrastructure
- frontend and tooling integration
- performance and scalability strategy

This specification does not cover:

- detailed gameplay UX
- language-specific implementation details
- low-level renderer or graphics API choices
- final repository layout or milestone planning

## Architectural Stance

POLIS should be built as a specification-driven simulation platform rather than as a conventional game engine. The architecture should treat the conceptual model, simulation state, process logic, scheduling, persistence, experimentation, and presentation as distinct layers with narrow contracts and explicit non-responsibilities.

The core technical stance is:

- CPU-first for correctness, reference behavior, and broad portability
- data-oriented in hot paths without collapsing the conceptual ontology
- event-rich and provenance-aware rather than opaque and state-only
- frontend-decoupled, with presentation consuming snapshots, streams, and derived views rather than owning truth
- batch-capable from the start, not retrofitted after an interactive prototype

## Layered Architecture

POLIS should use seven primary architectural layers:

1. Ontology and Specification Layer
2. State Model Layer
3. Process and System Layer
4. Scheduling and Time Layer
5. Persistence and Provenance Layer
6. Experiment Orchestration Layer
7. Presentation and Tooling Layer

These layers are described below.

## 1. Ontology And Specification Layer

### Responsibilities

- Encode the formal ontology defined by the earlier specifications.
- Define entity classes, field types, resource categories, institutional forms, scale relations, and process families.
- Define scenario templates, structural variants, parameter sets, and experiment schemas.
- Provide machine-readable model descriptions that can be compiled into runtime state and process registrations.

### Must Not Own

- live runtime instances
- mutable simulation state
- scheduling logic
- frontend state
- backend-specific memory layout

### Notes

This layer is the declarative source of architectural truth. It should be versioned and inspectable. Runtime code should depend on compiled or validated specification artifacts, not on ad hoc hand-wired assumptions.

## 2. State Model Layer

### Responsibilities

- Materialize the conceptual ontology into authoritative runtime state.
- Store agent state, collective state, spatial fields, resource stocks, infrastructure state, networks, institutional state, and derived execution metadata required by the scheduler.
- Enforce ownership rules so each piece of authoritative state has exactly one home.
- Expose narrow read and mutation interfaces to process systems.

### Must Not Own

- domain behavior
- global time advancement
- experiment logic
- visualization logic

### Recommended State Structure

The state layer should be hybrid rather than dogmatically uniform:

- entity/component stores for heterogeneous actors and organizations
- structure-of-arrays layouts for hot homogeneous data
- dense field grids for soils, water, climate, pathogen pressure, and similar spatial variables
- graph or adjacency stores for social, political, trade, and kinship links
- aggregate caches only where explicitly marked as derived and disposable

This is ECS-like in discipline, but POLIS should not force all world concepts into a single generic ECS shape when fields or graphs are more appropriate.

## 3. Process And System Layer

### Responsibilities

- Implement all world dynamics as systems operating on state through declared contracts.
- Cover ecological, demographic, economic, institutional, military, discovery, and collective-agency processes.
- Emit commands, events, deltas, and diagnostics in standard forms.
- Support both individual-level and collective-level logic without duplicating ownership of state.

### Must Not Own

- authoritative state storage
- direct persistence concerns
- UI or presentation concerns
- top-level experiment orchestration

### System Contract

Every major system should declare:

- required inputs
- state it may mutate
- temporal cadence or trigger conditions
- emitted event types
- invariants and failure conditions

This keeps systems auditable and makes it possible to reason about correctness, ordering, and reproducibility.

## 4. Scheduling And Time Layer

### Responsibilities

- Advance simulation time.
- Coordinate fixed-step, variable-step, event-driven, and multi-rate processes.
- Resolve ordering between systems with explicit dependencies.
- Manage promotion and compression across scales where required by the state model and collective-agency rules.

### Must Not Own

- domain semantics beyond scheduling contracts
- persistence backends
- presentation state

### Recommended Time Model

POLIS should use a master scheduler with multi-rate execution:

- fast cadence for local actions, trade decisions, movement, disease contact, and tactical events
- medium cadence for household allocation, market adjustment, and institutional enforcement cycles
- slow cadence for seasonal biology, infrastructure wear, soil change, demographic shifts, and large political transitions

The scheduler should support:

- deterministic ordering where required
- explicit phase boundaries
- event queue insertion for irregular events
- replay-safe execution under fixed seed and fixed configuration

The scheduler should be authoritative. Systems may request cadence and emit future events, but they should not advance time independently.

## 5. Persistence And Provenance Layer

### Responsibilities

- Record the simulation lifecycle as a combination of snapshots, event logs, run metadata, and experiment metadata.
- Support replay, auditing, and scientific provenance.
- Capture model version, scenario definition, parameters, seeds, enabled modules, and numeric settings.
- Provide durable outputs for validation, calibration, and comparative experiments.

### Must Not Own

- simulation decisions
- scheduling authority
- analytical interpretation

### Recommended Persistence Model

POLIS should use a mixed snapshot-plus-event architecture:

- periodic full or partial snapshots for bounded replay cost
- append-only event streams for semantically meaningful changes
- structured metric series for analysis
- run manifests linking code version, spec version, inputs, and outputs

Not every microscopic mutation needs to be stored as a first-class event. The event layer should prioritize causally meaningful transitions, policy changes, commands, collective decisions, shocks, crises, births, deaths, migrations, and other audit-relevant changes.

## 6. Experiment Orchestration Layer

### Responsibilities

- Run parameter sweeps, ablations, calibration campaigns, stress tests, and comparative scenario families.
- Allocate runs across local or remote compute resources.
- Manage ensembles, seeds, stopping rules, and output bundles.
- Provide a first-class experiment object aligned with the validation framework in `08`.

### Must Not Own

- low-level world dynamics
- frontend rendering state
- direct mutation of simulation internals outside public runtime contracts

### Recommended Orchestration Model

Interactive runs and batch runs should share the same simulation core. The orchestrator should sit above the runtime and instantiate:

- simulation build and spec version
- scenario and parameter bundle
- randomization plan
- output policy
- analysis hooks

This avoids the common failure mode where exploratory and batch paths drift into separate, incompatible simulators.

## 7. Presentation And Tooling Layer

### Responsibilities

- Consume snapshots, event streams, metrics, and derived analytical products.
- Provide world views, overlays, inspections, replays, comparisons, and debugging tools.
- Support the dual-mode frontend defined in `09_FrontendAndPresentation.md`.

### Must Not Own

- authoritative world state
- causal world updates
- hidden simulation logic

### Integration Rule

The presentation layer is downstream of backend truth. It may request subscriptions, filtered views, replay frames, and derived summaries, but it must not be allowed to silently mutate or reinterpret authoritative state.

Any interactive intervention from the frontend should enter the backend as an explicit command subject to the same event, validation, and provenance rules as any other experiment input.

## Runtime And Dataflow Model

At runtime, POLIS should operate through a disciplined loop:

1. Load validated specifications, scenario bundle, seed plan, and runtime configuration.
2. Materialize authoritative state.
3. Register process systems and their declared contracts.
4. Initialize scheduler phases, cadence groups, and event queues.
5. Advance time through scheduler phases.
6. Let systems read allowed state, compute effects, and emit commands, deltas, metrics, and events.
7. Commit state changes through authoritative mutation channels.
8. Persist snapshots, event records, and metrics according to output policy.
9. Publish read-only streams or sampled state to tooling and presentation consumers.

This should produce a runtime where state is never updated by arbitrary side effects. Mutations should occur through explicit, inspectable paths.

## Module Boundaries

Several boundaries are especially important:

- Spec versus runtime: declarative model definitions must not be mixed with mutable execution state.
- State versus behavior: storage should not hide domain logic; systems should not own state.
- Scheduler versus systems: time control belongs to the scheduler, not to individual modules.
- Core runtime versus orchestration: experiment management should not leak into world dynamics.
- Backend versus frontend: visualization must not become a hidden simulation layer.
- Authoritative state versus analytical overlay: derived summaries are disposable and recomputable.

If these boundaries blur, POLIS will become difficult to validate, replay, and scale.

## Performance And Scalability Strategy

### CPU-First Baseline

POLIS should establish a strong CPU reference path first. This machine profile supports that approach well:

- modern AMD CPU
- 16 logical processors
- good SIMD capability
- moderate RAM budget
- no requirement for NVIDIA-specific tooling

The CPU path should be the correctness reference, the validation target, and the default development path.

### Data-Oriented Hot Paths

Performance-critical systems should use:

- structure-of-arrays layouts
- contiguous iteration over homogeneous component slices
- spatial partitioning where locality matters
- minimized pointer chasing in hot loops
- task-based parallelism across independent regions, cohorts, or system partitions

Not every subsystem needs the same optimization style. The architecture should optimize hot kernels while preserving conceptual clarity elsewhere.

### GPU Strategy

GPU acceleration should be optional and selective, not foundational. It is most defensible for:

- dense field updates
- diffusion or transport-like kernels
- reductions and aggregations
- embarrassingly parallel batch kernels

GPU acceleration is less appropriate as an early default for highly irregular institutional or collective-agency logic. Those paths should remain clean on CPU first.

### Batch Execution

The architecture should support:

- local multi-run ensembles
- resumable long runs
- deterministic reruns
- future scaling to remote workers or clusters

A batch job should be a packaged runtime invocation, not a separate code path.

### Approximation Policy

Approximation is allowed, but only under explicit rules:

- it must be declared in the spec or runtime configuration
- it must preserve interpretability for the target experiment class
- it must be measurable against the CPU reference path where possible
- it must not silently change ontology or causal semantics

This is especially important if surrogate models, reduced micro-detail, or compressed replay streams are introduced later.

## Frontend, Replay, And Tooling Integration

The frontend and analysis tools should attach through stable backend interfaces that expose:

- sampled or filtered state snapshots
- event streams
- metric streams
- replay manifests
- provenance bundles

Recommended tooling modes:

- live inspection during interactive runs
- post-run replay from snapshots plus events
- experiment comparison across run bundles
- debugging views tied to scheduler phases, invariants, and system diagnostics

The replay model should distinguish between:

- authoritative replay of recorded history
- approximate visual reconstruction
- narrative summaries for communication

These are not interchangeable and should never be presented as if they were the same thing.

## Reproducibility And Scientific Workflow Support

The technical architecture should directly support the validation framework in `08` by making the following first-class:

- run manifests
- deterministic seeds and RNG stream declarations
- machine-readable experiment specifications
- versioned scenario bundles
- metrics schemas
- snapshot and event compatibility rules
- explicit software and spec version capture

If an experiment result cannot be reconstructed from these artifacts, the architecture is incomplete.

## Risks And Failure Modes

The most important architectural mistakes to avoid are:

1. Building POLIS as a monolithic game loop where simulation, UI, orchestration, and persistence are interwoven.
2. Treating ECS as a universal solution and forcing every concept into one storage pattern, even when fields or graphs are the right abstraction.
3. Letting systems mutate state ad hoc without explicit mutation channels, making replay and debugging unreliable.
4. Allowing the frontend to become a second ontology or hidden source of simulation truth.
5. Creating separate implementations for interactive and batch execution.
6. Overcommitting to GPU acceleration before the CPU reference path is correct and validated.
7. Recording too little provenance to support serious experiments, or so much unstructured data that replay becomes unusable.
8. Mixing conceptual model changes with performance hacks in ways that destroy scientific clarity.
9. Hiding time semantics inside subsystems instead of keeping scheduling explicit.
10. Treating derived overlays, caches, and summaries as authoritative state.

## Final Recommendation

POLIS should be built around a specification-driven, CPU-first, event-rich simulation core with hybrid state storage, explicit process contracts, authoritative scheduling, provenance-aware persistence, a first-class experiment orchestration layer, and a strictly decoupled presentation stack.

The recommended architecture is not a generic game-engine stack. It is a scientific simulation architecture that borrows selectively from ECS, co-simulation, workflow provenance, and high-performance simulation practice while preserving the conceptual integrity established in `01` through `09`.

If implemented with discipline, this architecture will support:

- serious interactive exploration
- reproducible batch experiments
- validation and calibration workflows
- replay and causal inspection
- later acceleration and scaling without rewriting the conceptual model

That is the standard POLIS should aim for before major implementation begins.
