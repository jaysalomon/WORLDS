# POLIS

> A research-grade civilization simulator.

[![CI](https://github.com/jaysalomon/WORLDS/actions/workflows/ci.yml/badge.svg)](https://github.com/jaysalomon/WORLDS/actions/workflows/ci.yml)
![Rust](https://img.shields.io/badge/rust-1.94%2B-orange?logo=rust)
![Edition](https://img.shields.io/badge/edition-2024-blue)
![License: MIT](https://img.shields.io/badge/license-MIT-blue)

POLIS is a scientific civilization sandbox. The backend models world and social dynamics
through explicit, inspectable simulation steps. Current implementation focus is deterministic
runtime foundations and exportable research artifacts.

## What makes it different

| Typical sandbox | POLIS |
|---|---|
| Scripted progression | Mechanism-driven simulation |
| Black-box behavior | Auditable state + event logs |
| Single-run storytelling | Batch-capable, reproducible runs |
| Weak reproducibility | Seeded deterministic runtime + checkpoints |

## Architecture

Ten Rust crates with strict dependency direction:

```text
polis-core          <- shared types, RNG, config, manifest schema
|- polis-world      <- world substrate and partition state
|- polis-agents     <- agent-domain placeholder crate
|- polis-compute    <- compute/backend placeholder crate
|- polis-export     <- JSON/JSONL export utilities and bundle helpers
`- polis-narrative  <- narrative placeholder crate
        `- polis-systems   <- runtime phase logic
                `- polis-sim       <- deterministic runtime and scheduler
                        `- polis-frontend  <- frontend placeholder crate
                                `- polis-app   <- CLI entry point
```

Key properties:

- CPU-first correctness
- Deterministic serial/parallel parity
- Headless-first CLI workflows
- Checkpoint save/load and replay-resume support

## Quick start

```powershell
# Run tests across all crates
cargo test --workspace

# One-command demo run (exports artifacts)
cargo run -p polis-app -- --demo --parallel

# Single run (serial)
cargo run -p polis-app -- --ticks 1000

# Single run (parallel internal phase execution)
cargo run -p polis-app -- --ticks 1000 --parallel

# Batch sweep
cargo run -p polis-app -- --ticks 500 --batch 32 --parallel

# Export run artifacts
cargo run -p polis-app -- --ticks 200 --export-dir target/exports/example

# Save and resume from checkpoint
cargo run -p polis-app -- --ticks 200 --save-checkpoint target/checkpoints/t200.json
cargo run -p polis-app -- --ticks 800 --load-checkpoint target/checkpoints/t200.json

# Run an explicit scenario file
cargo run -p polis-app -- --scenario scenarios/demo_v1.ron --ticks 3000 --parallel
```

## Documentation

Canonical design baseline:

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

Planning and implementation order:

- [docs/Plan_RepoStructure.md](docs/Plan_RepoStructure.md)
- [docs/Plan_BuildOrder.md](docs/Plan_BuildOrder.md)
- [docs/DemoPlaybook.md](docs/DemoPlaybook.md)
- [TRACKING.md](TRACKING.md)

## Project status

Phase 0 (Core Runtime Foundation) is complete, and Phase 1 (World Substrate) is underway:

- scenario-driven deterministic runtime with serial/parallel parity
- authoritative partitioned world state
- deterministic event and metric streams
- checkpoint save/load and replay-resume path
- early substrate dynamics: regeneration, diffusion, field evolution
- waste/byproduct loop: consumption creates waste and waste is naturally processed over time

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). The numbered spec suite in `docs/` is the authority.
Code follows spec, not the other way around.

## License

MIT - see [LICENSE](LICENSE).
