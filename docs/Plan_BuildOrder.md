# POLIS Build Order And Implementation Plan

**Date:** 14 March 2026  
**Status:** Planning document  
**Purpose:** Define the recommended implementation sequence for POLIS, aligned to the canonical numbered specs and the current Rust workspace scaffold.

## 1. Guiding Principles

### 1.1 The simulation core comes before polish

POLIS should establish authoritative state, scheduling, determinism, and validation scaffolding before investing heavily in frontend charm or narrative layers.

### 1.2 Visible artifacts still matter early

Even in a CPU-first architecture, early phases should produce something inspectable:

- a headless deterministic run
- a basic world view
- a minimal inspector

This keeps debugging practical and stops the backend from becoming invisible.

### 1.3 Batch and interactive paths must share one runtime

Do not build a toy interactive simulator and a separate research runner. Every phase should reinforce a single simulation core that can run both interactively and headlessly.

### 1.4 Validation is part of the build order

Each phase should add:

- tests
- diagnostics
- measurable outputs
- clear decision gates

If a phase cannot be validated, it is not complete.

## 2. Current Starting Point

The repository already contains:

- the canonical spec suite `01` through `10`
- a Rust workspace scaffold with named crates
- CI and test placeholders

That means implementation should start from a structured scaffold, not from a blank repository.

## 3. Phase 0: Core Runtime Foundation

### Goal

Establish the smallest authoritative runtime that can:

- load a scenario
- initialize world state
- advance deterministic time
- emit basic metrics and events

### Primary specs

- [01_WorldModel.md](/abs/path/e:/Drive/WORLDS/01_WorldModel.md)
- [02_StateModel.md](/abs/path/e:/Drive/WORLDS/02_StateModel.md)
- [10_TechnicalArchitecture.md](/abs/path/e:/Drive/WORLDS/10_TechnicalArchitecture.md)

### Crates

- `polis-core`
- `polis-world`
- `polis-sim`
- `polis-app`

### Deliverables

- scenario loading from `scenarios/default.ron`
- authoritative world state skeleton
- deterministic RNG wiring
- scheduler skeleton with at least one update phase
- event and metric output skeleton
- minimal app entry point that can run headless and print run status

### Validation

- same seed produces same state hash at tick `N`
- invalid scenario fails with a clear error
- event log and metric stream are produced in deterministic order

### Decision gate

Proceed only if deterministic stepping and authoritative state ownership are working.

## 4. Phase 1: World Substrate

### Goal

Build the physical and environmental substrate before meaningful agents exist.

### Primary specs

- [01_WorldModel.md](/abs/path/e:/Drive/WORLDS/01_WorldModel.md)
- [02_StateModel.md](/abs/path/e:/Drive/WORLDS/02_StateModel.md)
- [04_ResourcesAndMaterials.md](/abs/path/e:/Drive/WORLDS/04_ResourcesAndMaterials.md)

### Crates

- `polis-world`
- `polis-systems`
- `polis-sim`
- optional early `polis-frontend`

### Deliverables

- terrain or patch substrate
- environmental fields
- resource systems and resource stocks
- simple regeneration and diffusion kernels on CPU
- basic world-state inspection output

### Validation

- resource growth approaches carrying capacity without harvest
- diffusion behaves sensibly under reference cases
- no negative stocks or field values without explicit allowance

### Decision gate

Proceed only if substrate dynamics are numerically stable and inspectable.

## 5. Phase 2: Minimal Presentation Shell

### Goal

Expose the substrate through a minimal but real frontend without letting the frontend become part of the simulation contract.

### Primary specs

- [09_FrontendAndPresentation.md](/abs/path/e:/Drive/WORLDS/09_FrontendAndPresentation.md)
- [10_TechnicalArchitecture.md](/abs/path/e:/Drive/WORLDS/10_TechnicalArchitecture.md)
- [Concept_FrontendDesign.md](/abs/path/e:/Drive/WORLDS/Concept_FrontendDesign.md)

### Crates

- `polis-frontend`
- `polis-app`

### Deliverables

- windowed run mode
- map or grid rendering
- pause and speed controls
- one or two analytical overlays
- explicit command path from UI into backend

### Validation

- frontend does not mutate authoritative state except through commands
- paused and replayed states remain consistent with backend records

### Decision gate

Proceed only if the frontend is consuming backend truth cleanly.

## 6. Phase 3: Individuals And Basic Demography

### Goal

Introduce individuals with simple survival-driven behavior, movement, consumption, and demographic turnover.

### Primary specs

- [01_WorldModel.md](/abs/path/e:/Drive/WORLDS/01_WorldModel.md)
- [02_StateModel.md](/abs/path/e:/Drive/WORLDS/02_StateModel.md)
- [04_ResourcesAndMaterials.md](/abs/path/e:/Drive/WORLDS/04_ResourcesAndMaterials.md)

### Crates

- `polis-agents`
- `polis-systems`
- `polis-sim`
- `polis-frontend`

### Deliverables

- individual state model
- movement and consumption loop
- mortality and reproduction basics
- agent inspection view
- population and resource metrics

### Validation

- agents consume available resources
- starvation and recovery behave coherently
- population responds to substrate quality

### Decision gate

Proceed only if individuals create plausible population-resource coupling and remain deterministic under fixed seeds.

## 7. Phase 4: Social Fabric

### Goal

Add ties, trust, grievance, cooperation, and local conflict before true collective actors are enabled.

### Primary specs

- [03_CollectiveAgency.md](/abs/path/e:/Drive/WORLDS/03_CollectiveAgency.md)
- [07_SocietyAndInstitutions.md](/abs/path/e:/Drive/WORLDS/07_SocietyAndInstitutions.md)
- [Concept_SwarmToSociety.md](/abs/path/e:/Drive/WORLDS/Concept_SwarmToSociety.md)

### Crates

- `polis-agents`
- `polis-systems`
- `polis-sim`
- `polis-frontend`

### Deliverables

- social ties and local networks
- trust and grievance updates
- cooperation and local conflict rules
- social overlays and event markers

### Validation

- repeated interaction changes trust
- scarcity increases social tension and conflict hazard
- events and overlays explain what happened

### Decision gate

Proceed only if group-like patterns emerge without prematurely collapsing everything into collective actors.

## 8. Phase 5: Collective Agency

### Goal

Promote valid groups into real collective actors with explicit merge, split, and downward-causation mechanics.

### Primary specs

- [03_CollectiveAgency.md](/abs/path/e:/Drive/WORLDS/03_CollectiveAgency.md)
- [02_StateModel.md](/abs/path/e:/Drive/WORLDS/02_StateModel.md)
- [07_SocietyAndInstitutions.md](/abs/path/e:/Drive/WORLDS/07_SocietyAndInstitutions.md)

### Crates

- `polis-agents`
- `polis-systems`
- `polis-sim`
- `polis-frontend`

### Deliverables

- merge criteria
- split criteria with hysteresis
- collective decision paths
- retained individual sub-processes
- collective inspection tooling

### Validation

- no merge-split thrashing
- measurable difference between coordinated and uncoordinated populations
- approximation remains bounded where used

### Decision gate

Proceed only if collective agency is both scientifically defensible and technically stable.

## 9. Phase 6: Discovery, Biology, And Institutions

### Goal

Add the major medium-speed world process domains that turn populations into societies with path-dependent trajectories.

### Primary specs

- [05_DiscoveryHeuristics.md](/abs/path/e:/Drive/WORLDS/05_DiscoveryHeuristics.md)
- [06_BiologyAndDomestication.md](/abs/path/e:/Drive/WORLDS/06_BiologyAndDomestication.md)
- [07_SocietyAndInstitutions.md](/abs/path/e:/Drive/WORLDS/07_SocietyAndInstitutions.md)

### Crates

- `polis-systems`
- `polis-world`
- `polis-agents`
- `polis-sim`
- `polis-frontend`

### Deliverables

- discovery heuristics
- knowledge diffusion
- organism and agroecosystem basics
- hard-coded animal species archetypes with shared trait/capability model:
  - horse, cattle/ox, dog, sheep, goat, pig, poultry, waterfowl
  - optional wild baselines for prey/predator pressure where needed
  - trait-derived task outputs for transport, traction, hunting support, guarding, and secondary products
- early institutions and legitimacy mechanics
- disease pressure where biologically appropriate
- minimal conflict and territory mechanics:
  - combat power with bounded multipliers (tech, supply, cohesion, terrain, leadership)
  - conquest as control-transition process (not instant annexation)
  - partition-level territory states (claimed/contested/controlled/occupied)

### Validation

- discoveries depend on real affordances and process chains
- biological management creates different trajectories than pure foraging
- species specialisation produces measurable functional differences:
  - horses improve mobility and transport
  - cattle/ox improve traction and hauling
  - dogs improve hunting/guard outcomes
  - poultry/waterfowl contribute eggs/meat and waste loops
- institutions emerge from pressure rather than scripted stage gates
- conflict outcomes are sensitive to technology and supply but remain bounded
- territory turnover requires sustained control, not a single battle win

### Decision gate

Proceed only if these domains interact without breaking runtime clarity.

### Phase 6 Pass 2: Probabilistic Risk Inference (PLN Layer)

#### Goal

Add a bounded probabilistic logic layer for uncertainty-heavy domains without replacing deterministic core simulation.

#### Scope boundaries

- Keep substrate, demography, and core resource stepping deterministic.
- Use PLN/risk inference as an advisory layer that outputs probabilities consumed by existing decision and hazard systems.
- Evaluate at slower cadence (for example daily/weekly), not per-agent per-tick.

#### Priority domains

1. Zoonotic and epidemic pressure
2. Institutional enforcement failure
3. Trade cheating/default risk
4. Collective fracture/escalation risk
5. Local famine/crisis early warning

#### Required outputs

- partition-level probabilistic risk scores (`0..1` or `0..100`)
- event emissions for risk updates and realized incidents
- top-factor attribution per risk update for auditability
- deterministic seeded sampling path from probabilities to realized events

#### Validation

- serial/parallel parity preserved under fixed seed
- risk responds directionally to input drivers (for example higher carcass load -> higher spillover risk)
- calibration sanity checks against stylized expectations
- no significant runtime regression from PLN cadence

#### Decision gate

Proceed only if probabilistic inference improves explanatory power without introducing opaque or non-deterministic behavior in the core runtime.

## 10. Phase 7: Validation, Export, And Experiment Pipeline

### Goal

Make POLIS usable as a research instrument rather than only an interactive sandbox.

### Primary specs

- [08_ValidationAndExperiments.md](/abs/path/e:/Drive/WORLDS/08_ValidationAndExperiments.md)
- [10_TechnicalArchitecture.md](/abs/path/e:/Drive/WORLDS/10_TechnicalArchitecture.md)

### Crates

- `polis-export`
- `polis-sim`
- `polis-app`

### Deliverables

- headless execution path
- run manifests
- snapshot and metric export
- batch sweep support
- reproducibility checks

### Validation

- repeatable outputs from same configuration and seed
- experiment bundles can be re-run
- exported metrics support downstream analysis

### Decision gate

Proceed only if experimental runs can be reproduced and audited.

## 11. Phase 8: Performance And Selective Acceleration

### Goal

Optimize where the validated runtime proves it is necessary.

### Primary specs

- [08_ValidationAndExperiments.md](/abs/path/e:/Drive/WORLDS/08_ValidationAndExperiments.md)
- [10_TechnicalArchitecture.md](/abs/path/e:/Drive/WORLDS/10_TechnicalArchitecture.md)

### Crates

- `polis-compute`
- `polis-sim`
- `polis-world`
- `polis-frontend`

### Deliverables

- profiling-driven hot-path optimization
- selective GPU kernels for dense field or reduction work
- CPU reference versus accelerated path comparisons
- explicit backend contract (`CpuReference` / `GpuAccelerated`) in `polis-compute`
- parity harness for each accelerated kernel:
  - CPU reference output
  - accelerated output
  - tolerance and drift report
- acceleration target ordering:
  1. dense field kernels (diffusion/regeneration style math)
  2. partition/global reductions
  3. batch ensemble fan-out helpers
  4. only then consider selective agent-adjacent vectorizable paths

### Validation

- accelerated paths match CPU reference within tolerance
- speedups are real and not purchased by semantic drift
- determinism contract preserved in CPU reference mode for all scientific runs
- acceleration accepted only when profiling confirms material gain on representative scenarios

### Non-goals

- no GPU-first rewrite of institutional, social-network, or collective-governance logic
- no weakening of event ordering or provenance guarantees for speed

### Decision gate

Proceed only if acceleration preserves the validated model contract.

## 12. Phase 9: Narrative And Public-Facing Polish

### Goal

Add optional narrative, richer presentation, and public-facing readability after the scientific core is already stable.

### Primary specs

- [09_FrontendAndPresentation.md](/abs/path/e:/Drive/WORLDS/09_FrontendAndPresentation.md)
- [Concept_FrontendDesign.md](/abs/path/e:/Drive/WORLDS/Concept_FrontendDesign.md)
- [Concept_MLAndNNUsage.md](/abs/path/e:/Drive/WORLDS/Concept_MLAndNNUsage.md)

### Crates

- `polis-narrative`
- `polis-frontend`
- `polis-app`

### Deliverables

- chronicle or event text
- improved overlays and replay UX
- optional audio and presentation polish

### Validation

- narrative remains grounded in real events
- presentation does not hide uncertainty or replace instrumentation

## 13. Cross-Cutting Concerns

These apply from the beginning:

- deterministic RNG and reproducibility
- event logging
- profiling
- scenario-driven initialization
- unit, integration, and validation tests
- strict separation of backend truth and presentation

## 14. Relationship To Supporting Documents

Use the numbered suite as primary authority.

Use supporting docs only as advisory context:

- [Concept_SwarmToSociety.md](/abs/path/e:/Drive/WORLDS/Concept_SwarmToSociety.md)
- [Concept_MLAndNNUsage.md](/abs/path/e:/Drive/WORLDS/Concept_MLAndNNUsage.md)
- [Concept_FrontendDesign.md](/abs/path/e:/Drive/WORLDS/Concept_FrontendDesign.md)

Treat these legacy drafts as source history only:

- [Worldspec1.md](/abs/path/e:/Drive/WORLDS/Worldspec1.md)
- [DesignSpec.md](/abs/path/e:/Drive/WORLDS/DesignSpec.md)
- [DiscoveryHeuristics.md](/abs/path/e:/Drive/WORLDS/DiscoveryHeuristics.md)

## 15. Summary

The POLIS build order should now start from a real workspace scaffold and the completed numbered spec suite. The early focus should be deterministic runtime foundations, world substrate, inspectable agents, and collective agency, with experimentation infrastructure and selective acceleration following only after the core model is behaving coherently.
