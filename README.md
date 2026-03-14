# POLIS

> A research-grade civilization simulator you can actually watch.

[![CI](https://github.com/jaysalomon/WORLDS/actions/workflows/ci.yml/badge.svg)](https://github.com/jaysalomon/WORLDS/actions/workflows/ci.yml)
![Rust](https://img.shields.io/badge/rust-1.94%2B-orange?logo=rust)
![Edition](https://img.shields.io/badge/edition-2024-blue)
![License: MIT](https://img.shields.io/badge/license-MIT-blue)

POLIS is a scientific civilization sandbox. The backend models terrain, resources, biology,
agent cognition, and collective behaviour through explicit, inspectable simulation steps.
The frontend gives you a live window into that process — zoom in on a single settlement, pull
back to watch continental migration patterns, or run headless and export everything to Parquet
for offline analysis.

Think of it as the serious engine that WorldBox wishes it had.

---

## What makes it different

| Other sandboxes | POLIS |
|---|---|
| Handcrafted "feel-good" rules | Physics-derived causality chains |
| Black-box AI behaviour | Auditable agent state, logged to disk |
| Fixed map scale | Continuous zoom: tile → biome → continent |
| Play-to-win objectives | Observer-first: configure, launch, study |
| No reproducibility | Deterministic RNG, snapshot + event log |

---

## Architecture

Ten Rust crates with a strict downward dependency graph:

```
polis-core          ← shared types, RNG, config, event schema
├── polis-world     ← terrain, climate, geography substrate
├── polis-agents    ← individual agent state and cognition
├── polis-compute   ← wgpu GPU abstraction (compute + render)
├── polis-export    ← Arrow / Parquet data pipeline
└── polis-narrative ← optional SLM narrative surface
        └── polis-systems   ← domain subsystems (ecology, trade, …)
                └── polis-sim       ← tick orchestrator, scenario runner
                        └── polis-frontend  ← wgpu renderer + egui UI
                                └── polis-app   ← binary entry point
```

Key properties:

- **CPU-first correctness** — simulation logic is deterministic and fully testable without a GPU
- **GPU in, GPU out** — the tilemap lives in a storage buffer shared between compute and render
- **Headless mode** — `--features headless` strips all rendering dependencies for batch runs
- **Reproducible** — every run carries a seed; any run can be replayed from its event log

---

## Quick start

```powershell
# Run tests across all crates
cargo test --workspace

# Open the simulation window
cargo run -p polis-app
```

Requires Rust 1.94+ (`rustup update stable`).

---

## Documentation

The canonical design is in the numbered spec suite:

| Doc | Topic |
|-----|-------|
| [SpecSuite.md](SpecSuite.md) | Index and guiding principles |
| [01_WorldModel.md](01_WorldModel.md) | Terrain, climate, geography |
| [02_StateModel.md](02_StateModel.md) | State representation and ownership |
| [03_CollectiveAgency.md](03_CollectiveAgency.md) | Collective behaviour and governance |
| [04_ResourcesAndMaterials.md](04_ResourcesAndMaterials.md) | Material flows and economics |
| [05_DiscoveryHeuristics.md](05_DiscoveryHeuristics.md) | Technology and discovery |
| [06_BiologyAndDomestication.md](06_BiologyAndDomestication.md) | Biology, species, domestication |
| [07_SocietyAndInstitutions.md](07_SocietyAndInstitutions.md) | Social structures and institutions |
| [08_ValidationAndExperiments.md](08_ValidationAndExperiments.md) | Validation methodology |
| [09_FrontendAndPresentation.md](09_FrontendAndPresentation.md) | Rendering and UI design |
| [10_TechnicalArchitecture.md](10_TechnicalArchitecture.md) | Architecture and subsystem layout |

Implementation planning:

- [Plan_RepoStructure.md](Plan_RepoStructure.md) — crate definitions and dependency graph
- [Plan_BuildOrder.md](Plan_BuildOrder.md) — phased build roadmap

---

## Project status

Early scaffold. All crates compile cleanly; `polis-core` has initial passing tests.
Active development is beginning at Phase 0: wgpu window, deterministic tilemap, egui HUD.

See [Plan_BuildOrder.md](Plan_BuildOrder.md) for the full phase-by-phase roadmap.

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). In brief: the numbered spec suite is the canonical
design baseline — code follows spec, not the other way around.
`cargo clippy -- -D warnings` must pass before any PR.

---

## License

MIT — see [LICENSE](LICENSE).
