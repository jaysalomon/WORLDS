# CLAUDE.md

This file gives repository-level guidance for coding agents working in this workspace.

## Project Overview

POLIS is a serious civilization simulator with a watchable frontend and a research-grade backend.

The repository now contains both:

- the canonical numbered specification suite
- an initial Rust workspace scaffold for implementation

The project should be treated as a scientific simulation platform first, not as a conventional game prototype.

## Canonical Documents

The authoritative design baseline is:

- [SpecSuite.md](/abs/path/e:/Drive/WORLDS/SpecSuite.md)
- [01_WorldModel.md](/abs/path/e:/Drive/WORLDS/01_WorldModel.md)
- [02_StateModel.md](/abs/path/e:/Drive/WORLDS/02_StateModel.md)
- [03_CollectiveAgency.md](/abs/path/e:/Drive/WORLDS/03_CollectiveAgency.md)
- [04_ResourcesAndMaterials.md](/abs/path/e:/Drive/WORLDS/04_ResourcesAndMaterials.md)
- [05_DiscoveryHeuristics.md](/abs/path/e:/Drive/WORLDS/05_DiscoveryHeuristics.md)
- [06_BiologyAndDomestication.md](/abs/path/e:/Drive/WORLDS/06_BiologyAndDomestication.md)
- [07_SocietyAndInstitutions.md](/abs/path/e:/Drive/WORLDS/07_SocietyAndInstitutions.md)
- [08_ValidationAndExperiments.md](/abs/path/e:/Drive/WORLDS/08_ValidationAndExperiments.md)
- [09_FrontendAndPresentation.md](/abs/path/e:/Drive/WORLDS/09_FrontendAndPresentation.md)
- [10_TechnicalArchitecture.md](/abs/path/e:/Drive/WORLDS/10_TechnicalArchitecture.md)

Legacy drafts such as [Worldspec1.md](/abs/path/e:/Drive/WORLDS/Worldspec1.md), [DesignSpec.md](/abs/path/e:/Drive/WORLDS/DesignSpec.md), and [DiscoveryHeuristics.md](/abs/path/e:/Drive/WORLDS/DiscoveryHeuristics.md) are source history only. They should not override the numbered suite.

## Current Repository State

The repository already has a Rust workspace scaffold with these crates:

- `polis-app`
- `polis-core`
- `polis-world`
- `polis-agents`
- `polis-systems`
- `polis-compute`
- `polis-sim`
- `polis-export`
- `polis-frontend`
- `polis-narrative`

Do not assume the repository is still design-only.

## Technical Direction

The current architectural direction is:

- CPU-first simulation core for correctness and validation
- data-oriented hot paths where justified
- optional and selective GPU acceleration
- explicit state ownership and authoritative scheduling
- snapshot-plus-event persistence
- strictly decoupled frontend and tooling layers

Do not assume:

- Vulkan-first architecture is mandatory
- CUDA or NVIDIA-specific tooling is available
- raw GPU compute should dominate early implementation choices

## Working Rules

When editing or adding code:

1. Respect the ownership boundaries defined in `01` through `10`.
2. Prefer explicit, inspectable simulation logic over opaque shortcuts.
3. Keep backend causality separate from presentation and narrative layers.
4. Keep batch and interactive execution on the same simulation core.
5. Avoid introducing architecture that contradicts the spec suite without updating the specs.

## Planning Documents

Implementation planning is currently tracked in:

- [Plan_RepoStructure.md](/abs/path/e:/Drive/WORLDS/Plan_RepoStructure.md)
- [Plan_BuildOrder.md](/abs/path/e:/Drive/WORLDS/Plan_BuildOrder.md)

These planning docs should be treated as implementation aids subordinate to the numbered suite.
