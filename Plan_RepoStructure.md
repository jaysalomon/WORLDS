# POLIS Repository Structure

**Date:** 14 March 2026  
**Status:** Planning document  
**Purpose:** Define the intended workspace boundaries and repository organization for implementation, aligned to the current repository rather than an earlier hypothetical layout.

## 1. Guiding Principles

### 1.1 Crate boundaries follow authority boundaries

Crates should follow the ownership boundaries defined in the numbered specification suite. If a subsystem has unique write authority over a piece of state, that subsystem should expose a narrow API and keep its internals contained.

### 1.2 Canonical specs stay canonical

The numbered documents in the repository root are currently the canonical design baseline. Planning documents, concept notes, and legacy drafts must not override them.

### 1.3 Clear dependency direction

Dependencies should flow from low-level shared foundations toward orchestration, presentation, and narrative layers. Avoid circular dependencies and avoid letting UI crates leak into the simulation core.

### 1.4 Actual tree before imagined tree

This document should describe the repository that exists now plus clearly marked near-term additions. It should not pretend that files or directories already exist when they do not.

## 2. Current Repository Layout

The repository currently looks like this:

```text
WORLDS/
|- Cargo.toml
|- rust-toolchain.toml
|- README.md
|- SpecSuite.md
|- 01_WorldModel.md
|- 02_StateModel.md
|- 03_CollectiveAgency.md
|- 04_ResourcesAndMaterials.md
|- 05_DiscoveryHeuristics.md
|- 06_BiologyAndDomestication.md
|- 07_SocietyAndInstitutions.md
|- 08_ValidationAndExperiments.md
|- 09_FrontendAndPresentation.md
|- 10_TechnicalArchitecture.md
|- Worldspec1.md
|- DesignSpec.md
|- DiscoveryHeuristics.md
|- Concept_SwarmToSociety.md
|- Concept_MLAndNNUsage.md
|- Concept_FrontendDesign.md
|- Plan_RepoStructure.md
|- Plan_BuildOrder.md
|- CLAUDE.md
|- crates/
|  |- polis-app/
|  |- polis-core/
|  |- polis-world/
|  |- polis-agents/
|  |- polis-systems/
|  |- polis-compute/
|  |- polis-sim/
|  |- polis-export/
|  |- polis-frontend/
|  `- polis-narrative/
|- scenarios/
|- shaders/
|- assets/
`- tests/
```

## 3. Repository Roles

### 3.1 Root markdown documents

The repository root currently holds:

- canonical numbered specs
- supporting concept notes
- legacy draft documents
- planning documents

This is acceptable for now. A future `docs/` move is possible, but it is not yet the repository truth and should not be assumed in implementation planning.

### 3.2 Workspace crates

The Rust workspace is the implementation surface. The crate set already exists and should be treated as the intended initial module map unless later refactoring proves necessary.

### 3.3 Top-level data directories

- `scenarios/` holds scenario definitions
- `shaders/` holds GPU shader sources
- `assets/` holds frontend resources
- `tests/` holds cross-crate integration and validation tests

## 4. Crate Definitions

### 4.1 `polis-core`

Foundation crate for:

- IDs
- shared math and coordinate types
- RNG wrappers
- config and schema types
- common enums
- shared errors

Must not contain simulation domain logic.

### 4.2 `polis-world`

World substrate crate for:

- terrain and spatial indexing
- fields and environmental state
- resource systems and stocks
- place and structure substrate data

Must not own collective-agency logic or UI concerns.

### 4.3 `polis-agents`

Agent and collective state crate for:

- individuals
- households if represented explicitly
- collective actors
- memory, preferences, traits, and local state

Must not own top-level scheduling or frontend logic.

### 4.4 `polis-systems`

Process crate for:

- ecology
- demography
- social processes
- conflict
- discovery
- institutions
- biology

This crate should implement systems, not authoritative storage.

### 4.5 `polis-compute`

Optional acceleration and compute-support crate for:

- wgpu integration
- buffer and dispatch helpers
- shader loading
- reductions and field kernels

This crate should remain infrastructure-only. It must not become the primary owner of domain semantics.

### 4.6 `polis-sim`

Simulation orchestration crate for:

- runtime assembly
- scheduler phases and timesteps
- command and event flow
- snapshots and checkpoints
- scenario loading

This crate is where the authoritative runtime loop should live.

### 4.7 `polis-export`

Output crate for:

- metrics export
- event and snapshot serialization
- experiment bundle writing
- future columnar export support

### 4.8 `polis-frontend`

Presentation crate for:

- rendering
- UI panels
- overlays
- replay and inspection views
- input routing into backend commands

This crate must not own backend truth.

### 4.9 `polis-narrative`

Optional descriptive layer for:

- chronicle generation
- event text
- summaries
- public-facing narrative output

This crate must stay downstream of authoritative simulation events and state.

### 4.10 `polis-app`

Application entry point that wires together:

- runtime
- frontend
- export
- optional narrative services

If a separate headless binary becomes necessary later, it should be added only when the shared runtime is already stable.

## 5. Dependency Direction

The intended dependency shape is:

```text
polis-core
|- polis-world
|- polis-agents
|- polis-compute
|- polis-export
`- polis-narrative

polis-systems depends on:
- polis-core
- polis-world
- polis-agents

polis-sim depends on:
- polis-core
- polis-world
- polis-agents
- polis-systems
- polis-compute
- polis-export

polis-frontend depends on:
- polis-core
- polis-sim
- polis-export
- optional presentation-facing views from world or agent crates

polis-app depends on:
- polis-sim
- polis-frontend
- optional polis-narrative
```

This is a direction guide, not a claim that every dependency is already wired that way.

## 6. Shader Organization

For current planning, assume:

- WGSL is the preferred default for new shader work
- GLSL is transitional only if a specific kernel or migration path justifies it
- shader sources live under `shaders/`

No separate shader build pipeline should be assumed until it actually exists in the repository.

## 7. Scenario And Test Layout

### 7.1 Scenarios

Keep scenario files under `scenarios/`.

Near-term expectation:

- one default scenario
- a small number of hand-authored validation scenarios
- no large scenario taxonomy until runtime schemas stabilize

### 7.2 Tests

Use:

- crate-local unit tests for local logic
- `tests/integration/` for cross-crate behavior
- `tests/validation/` for determinism, conservation, and numerical checks

## 8. Planned Additions

The following may be added later, but should not be treated as already present:

- a dedicated `docs/` directory if the root becomes too crowded
- a separate headless binary if `polis-app` becomes too UI-heavy
- analysis scripts once export formats stabilize
- richer scenario packs once the schema is no longer moving

## 9. Relationship To The Spec Suite

This plan should be read after:

1. [01_WorldModel.md](/abs/path/e:/Drive/WORLDS/01_WorldModel.md)
2. [02_StateModel.md](/abs/path/e:/Drive/WORLDS/02_StateModel.md)
3. [10_TechnicalArchitecture.md](/abs/path/e:/Drive/WORLDS/10_TechnicalArchitecture.md)

Those documents define what the crates are trying to preserve. This plan only maps those constraints into repository organization.

## 10. Summary

The POLIS repository should stay organized around the authoritative numbered spec suite and a Rust workspace whose crate boundaries follow ownership boundaries from the architecture documents. The current tree already provides a workable scaffold; planning should now refine it rather than describe a different repository.
