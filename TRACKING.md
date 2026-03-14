# POLIS Build Tracking

Last updated: **2026-03-14**
Current status: **Phase 3 Complete** - Individuals and Basic Demography implemented

## Dashboard

| Area | Status | Notes |
|---|---|---|
| Phase 0 - Core Runtime | Done | Deterministic runtime foundation complete |
| Phase 1 - World Substrate | Done | Includes waste loop and biology extension scaffold |
| Phase 2 - Presentation Shell | Done | Windowed shell and read-only state contract validated |
| Phase 3 - Individuals and Demography | Done | Agents with needs, movement, consumption, mortality, reproduction |
| Phase 7 - Reproducibility Audit | Partial | Helpers exist; full completion pending |

## Maintenance Protocol

- Update this file after each meaningful implementation step.
- Always update:
  - `Last updated`
  - dashboard/phase status
  - checkboxes
  - update log entry

## Phase Checklists

### Phase 0 - Foundation

#### Completed

- [x] Scenario loading from `scenarios/default.ron`
- [x] Deterministic world initialization with seeded RNG
- [x] Authoritative partitioned world state
- [x] Multi-phase scheduler skeleton (`Perception`, `Decision`, `Commit`)
- [x] Deterministic serial/parallel execution parity
- [x] Deterministic event stream (`TickStarted`, `PhaseApplied`, `TickCompleted`)
- [x] Deterministic per-tick metrics stream
- [x] Run manifest generation
- [x] Batch run scaffolding (serial + parallel fan-out)
- [x] Export outputs:
  - `manifest.json`
  - `events.jsonl`
  - `metrics.jsonl`
  - `checkpoint.json`
  - `bundle-index.json`
- [x] Checkpoint save/load and replay-resume path
- [x] Tests:
  - determinism (single run + batch + serial/parallel parity)
  - scenario read/parse errors
  - checkpoint round-trip and resume parity

#### Blocked

- [ ] None

---

### Phase 1 - World Substrate

#### Completed

- [x] Resource types: `ResourceKind` (Food, Water, Material, Fuel, Ore, Waste, Knowledge)
- [x] `ResourceStock` with quantity, quality, extract/deposit operations
- [x] Environmental fields: `FieldKind` (Temperature, Moisture, Fertility, SolarRadiation, BioticPressure)
- [x] `FieldCell` with diffusion support
- [x] `PartitionState` expanded with all resource stocks and environmental fields
- [x] `regenerate_resources()` with carrying capacity behavior
- [x] `process_waste()` with fertility and biotic-pressure coupling
- [x] `diffuse_resources()` 1D ring diffusion for food/water
- [x] `evolve_fields()` seasonal and degradation dynamics
- [x] `validate_partition()` field/resource bounds + NaN/Inf checks
- [x] `check_carrying_capacity_convergence()` helper
- [x] `polis-systems` integrated with resource/field model
- [x] Waste loop integrated in runtime:
  - Decision phase creates waste
  - Commit phase applies field evolution + regeneration + waste processing
- [x] `polis-sim` metrics consume partition resource stocks
- [x] Simulation metrics track `total_waste`
- [x] Resource/field tests including convergence, diffusion, seasonal evolution
- [x] Parallel animal co-evolution scaffold:
  - `herbivore_population`, `predator_population`, `proto_domestic_population`
  - `domestication_tameness` bounded in `[0,1]`
  - `evolve_animal_populations()` integrated in commit phase
  - hunting/predation tied to food, waste, and social-pressure proxies
- [x] Biology metrics:
  - `total_herbivores`, `total_predators`, `total_proto_domestic`, `average_tameness_ppm`
- [x] Docs updates:
  - `docs/06_BiologyAndDomestication.md` implementation status note
  - `docs/EventSchema.md` planned biology interaction events

#### In Progress

- [ ] None

#### Next

- [ ] Phase 4: Collective Agency (Institutions and Factions)

#### Blocked

- [ ] None

---

### Phase 3 - Individuals and Basic Demography

#### Completed

- [x] `Individual` agent struct with health, hunger, thirst, age, metabolism, mobility
- [x] Agent lifecycle: initialization, needs update, consumption, mortality
- [x] Agent movement based on needs and resource availability
- [x] Reproduction with inheritance (metabolism) and cooldown
- [x] `AgentPopulation` collection with statistics and cleanup
- [x] Three-phase agent system integration:
  - `agent_perception_phase()` - movement decisions
  - `agent_decision_phase()` - consumption and reproduction
  - `agent_commit_phase()` - needs update and mortality
- [x] Agent metrics in `TickMetrics`: total_agents, average_agent_health, average_agent_hunger, average_agent_thirst
- [x] Deterministic agent behavior with seeded RNG
- [x] Phase 3 validation tests (17 tests in `crates/polis-sim/tests/agent_dynamics.rs`):
  - Agents consume available resources
  - Starvation reduces health, recovery restores health
  - Population responds to substrate quality
  - Reproduction increases population
  - Cleanup removes dead agents
  - Determinism verified with agents

#### In Progress

- [ ] None

#### Next

- [ ] Phase 4: Collective Agency (Institutions and Factions)

#### Blocked

- [ ] None

#### Completed

- [x] Windowed run mode (`--windowed`)
- [x] `macroquad` integration
- [x] Grid rendering of partitions (resource/field/demand overlays)
- [x] Pause/Resume (`SPACE`)
- [x] Step single tick (`S`)
- [x] Speed controls (`1/2/3/4`)
- [x] Overlay toggles (`R/F/D/N`)
- [x] Click-to-select partition + detail panel
- [x] Hover tooltip summary
- [x] Explicit command path (`SimCommand`) from UI to backend
- [x] `PresentationShell` frontend state separated from simulation state
- [x] Validation tests:
  - frontend state separation
  - read-only world access
  - command-only mutation path

#### In Progress

- [ ] None

#### Next

- [ ] Phase 4: Collective Agency (Institutions and Factions)

#### Blocked

- [ ] None

---

### Phase 7 - Audit Snapshot

#### Done

- [x] Headless execution path
- [x] Run manifests
- [x] Snapshot and metric export
- [x] Batch sweep support

#### Partial

- [ ] Reproducibility verification helpers exist (needs final hardening)
- [ ] Experiment bundle helpers exist (needs full operational workflow)

## Active Work Queue

1. Phase 4: Collective Agency (Institutions and Factions)

## Update Log

### 2026-03-14

- Completed Phase 3: Individuals and Basic Demography:
  - `polis-agents` crate with `Individual` and `AgentPopulation` structs
  - Agent lifecycle: needs (hunger/thirst), consumption, health, mortality, reproduction
  - Movement system based on resource seeking and mobility
  - Three-phase integration: perception (movement), decision (consumption/reproduction), commit (needs/mortality)
  - Deterministic agent behavior with seeded RNG
  - 17 validation tests in `crates/polis-sim/tests/agent_dynamics.rs`
  - All 101 tests passing across workspace

### 2026-03-14

- Updated Phase 2 frontend to visualize Phase 1 biology extension:
  - Added Animals section to partition detail panel (herbivores, predators, proto-domestic, tameness)
  - Added animal summary to hover tooltip (herbivore/predator counts)
  - Frontend now displays full world state including animal co-evolution scaffold
  - All 73 tests passing

### 2026-03-14
  - Extended partition state with herbivore/predator/proto-domestic populations and tameness.
  - Added `evolve_animal_populations()` in commit flow after field evolution.
  - Extended tick metrics with animal/tameness aggregates.
  - Added validation test for animal evolution bounds.
  - Updated biology/event schema docs for scaffold and planned event types.
- Phase 2 marked complete with validation coverage:
  - `frontend_state_is_separate_from_simulation`
  - `simulation_state_is_read_only_through_world_accessor`
  - command-only mutation path confirmed
  - test suite passing at time of completion
- Phase 2 implementation details:
  - `macroquad` frontend shell
  - overlay views, interaction controls, partition selection/tooltip panel
  - `--windowed` flag path in `polis-app`
- Added pre-vis references `Previs4.png` and `Previs5.png` and updated `docs/PrevisAssets.md`.
- Completed Phase 1 validation suite:
  - 17 resource dynamics tests in `crates/polis-sim/tests/resource_dynamics.rs`
  - convergence, diffusion, field dynamics, waste loop, stability checks
- Added minimal waste/byproduct substrate loop and `total_waste` metric.
- Added pre-vis prompt templates and indexed `Previs1.jpg`, `Previs2.png`, `Previs3.png`.
- Completed Phase 1 core implementation:
  - resource/field substrate and world/system/sim wiring
  - waste processing and partition validation
