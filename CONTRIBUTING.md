# Contributing to POLIS

Thanks for your interest. POLIS is a long-horizon research project; contributions that improve simulation correctness, crate quality, or documentation quality are welcome.

## Design Authority

The numbered spec suite (`01_WorldModel.md` through `10_TechnicalArchitecture.md`) is the canonical design baseline, indexed by [docs/SpecSuite.md](docs/SpecSuite.md).

Code follows spec, not the other way around. If you believe a spec decision is wrong, open a design discussion before submitting conflicting code.

## Build Requirements

- Rust stable toolchain (see `rust-toolchain.toml`)
- No additional system dependencies for headless builds
- A GPU is only required when working on the frontend and compute acceleration paths

## Before Submitting a PR

```powershell
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

All three should pass locally before opening a PR.

## Crate Boundaries

The dependency graph is strictly downward (`polis-core` -> ... -> `polis-app`). Do not introduce cycles or upward dependencies.

See [docs/Plan_RepoStructure.md](docs/Plan_RepoStructure.md) for expected responsibilities and dependency direction.

## Simulation Correctness Rules

- Causal state changes must be deterministic and seed-reproducible.
- Rendering, audio, and narrative code must never mutate authoritative simulation state.
- Prefer explicit, inspectable logic over opaque heuristics.
- Every non-trivial state mutation should be representable in the event stream.
- Parallel execution is allowed, but deterministic reduction order is required for reproducibility.

## Issues and Proposals

- Use issue templates for bugs and feature requests.
- For larger design changes, open a discussion issue and reference the relevant numbered spec section.
