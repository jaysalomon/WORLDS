# Contributing to POLIS

Thanks for your interest. POLIS is a long-horizon research project; contributions that
advance simulation correctness, crate quality, or documentation quality are very welcome.

---

## Design authority

The numbered spec suite (`01_WorldModel.md` through `10_TechnicalArchitecture.md`) is the
canonical design baseline — indexed by [docs/SpecSuite.md](docs/SpecSuite.md). Code follows spec, not
the other way around. If you believe a spec decision is wrong, open a discussion issue first
before writing code that conflicts with it.

---

## Build requirements

- Rust 1.94+ stable (`rustup update stable`)
- No additional system dependencies for headless builds
- A Vulkan / DX12 / Metal capable GPU if you are working on the renderer

---

## Before submitting a PR

```powershell
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

All three must pass cleanly. CI enforces this on every push.

---

## Crate boundaries

The dependency graph is strictly downward (`polis-core` → … → `polis-app`). Do not
introduce cycles or upward dependencies. See
See [docs/Plan_RepoStructure.md](docs/Plan_RepoStructure.md) for the full dependency graph and the
intended responsibility of each crate.

---

## Simulation correctness

- All causal state changes must be deterministic and seed-reproducible.
- Rendering, audio, and narrative code must never influence simulation state.
- Prefer explicit, inspectable logic over opaque heuristics.
- Every non-trivial state mutation should be expressible as a logged event.

---

## Opening issues

Bug reports and feature requests are welcome — please use the issue templates provided.
For larger design proposals, open a discussion issue and reference the relevant spec section.
