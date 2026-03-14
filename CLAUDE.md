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

- [docs/SpecSuite.md](docs/SpecSuite.md)
- [docs/01_WorldModel.md](docs/01_WorldModel.md)
- [docs/02_StateModel.md](docs/02_StateModel.md)
- [docs/03_CollectiveAgency.md](docs/03_CollectiveAgency.md)
- [docs/04_ResourcesAndMaterials.md](docs/04_ResourcesAndMaterials.md)
- [docs/05_DiscoveryHeuristics.md](docs/05_DiscoveryHeuristics.md)
- [docs/06_BiologyAndDomestication.md](docs/06_BiologyAndDomestication.md)
- [docs/07_SocietyAndInstitutions.md](docs/07_SocietyAndInstitutions.md)
- [docs/08_ValidationAndExperiments.md](docs/08_ValidationAndExperiments.md)
- [docs/09_FrontendAndPresentation.md](docs/09_FrontendAndPresentation.md)
- [docs/10_TechnicalArchitecture.md](docs/10_TechnicalArchitecture.md)

Legacy drafts such as [Worldspec1.md](docs/legacy/Worldspec1.md), [DesignSpec.md](docs/legacy/DesignSpec.md), and [DiscoveryHeuristics.md](docs/legacy/DiscoveryHeuristics.md) are source history only. They should not override the numbered suite.

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

- [docs/Plan_RepoStructure.md](docs/Plan_RepoStructure.md)
- [docs/Plan_BuildOrder.md](docs/Plan_BuildOrder.md)

These planning docs should be treated as implementation aids subordinate to the numbered suite.
