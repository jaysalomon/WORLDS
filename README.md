# POLIS

POLIS is a serious civilization sandbox with a watchable frontend and a research-grade backend.

This repository currently contains:

- design specifications in the repository root
- a Rust workspace scaffold for implementation
- CI and test wiring for early development

The canonical design baseline is the numbered suite indexed by [SpecSuite.md](/abs/path/e:/Drive/WORLDS/SpecSuite.md).

## Quick start

```powershell
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p polis-app
```

## Layout

- `crates/` - Rust workspace crates
- `scenarios/` - scenario files
- `shaders/` - GPU shader sources
- `assets/` - frontend assets
- `tests/` - integration and validation tests

Key design and planning documents:

- `SpecSuite.md`
- `01_WorldModel.md` through `10_TechnicalArchitecture.md`
- `Plan_RepoStructure.md`
- `Plan_BuildOrder.md`
