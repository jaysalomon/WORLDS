# POLIS Build Tracking

Last updated: **2026-03-15**
Current status: **Phase 7 In Progress** - Reproducibility audit and experiment pipeline implementation

## Dashboard

| Area | Status | Notes |
|---|---|---|
| Phase 0 - Core Runtime | Done | Deterministic runtime foundation complete |
| Phase 1 - World Substrate | Done | Includes waste loop and biology extension scaffold |
| Phase 2 - Presentation Shell | Done | Windowed shell and read-only state contract validated |
| Phase 3 - Individuals and Demography | Done | Agents with needs, movement, consumption, mortality, reproduction |
| Phase 4 - Social Fabric | Done | Social ties, cooperation/conflict, cross-species primitives, frontend overlays complete |
| Phase 5 - Collective Agency | Done | Core collective actors + frontend collective overlay + event/metrics doc sync |
| Phase 6 - Discovery, Biology, Institutions | Done* | Implementation complete; long-run verification suite queued as deferred gate |
| Phase 7 - Reproducibility Audit | In Progress | Full provenance, reproducibility verification, experiment bundles |
| Phase 8 - Performance And Selective Acceleration | In Progress | Backend split and compute scaffold started |

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

- [ ] Phase 5: Collective Agency (Institutions and Factions)

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

- [ ] Phase 5: Collective Agency (Institutions and Factions)

#### Blocked

- [ ] None

---

### Phase 4 - Social Fabric

#### Completed

- [x] Social ties graph: trust, grievance, interaction_count, last_interaction_tick
- [x] Deterministic trust/grievance updates from repeated interactions
- [x] Cooperation rules (help/share) gated by trust
- [x] Conflict rules (local disputes) increasing with scarcity + grievance
- [x] Event outputs: TrustShifted, CooperationOccurred, ConflictOccurred
- [x] Cross-species interaction primitives (human-animal):
  - animal-side state: familiarity, fear, aggression, human_tolerance
  - repeated encounters update these deterministically
  - early domestication progression depends on tolerance states
  - harsh contact increases fear/aggression; stable low-threat contact increases tolerance
- [x] Social metrics in TickMetrics: total_social_ties, average_trust, average_grievance, cooperation_count, conflict_count, social_tension
- [x] Cross-species metrics: average_animal_familiarity, average_animal_fear, average_animal_tolerance
- [x] Deterministic serial/parallel parity for social systems
- [x] Phase 4 validation:
  - Repeated interactions measurably shift trust/grievance
  - Scarcity increases conflict hazard
  - Higher trust increases cooperation frequency
  - Human-animal repeated low-threat contact increases tolerance over time
  - Determinism parity holds under fixed seed
- [x] Frontend social overlay mode (tension/conflict hotspots and tie-strength visibility)
- [x] Spec/doc sync for new runtime social/cross-species events and metrics

#### In Progress

- [ ] None

#### Next

- [ ] Phase 5: Collective Agency (Institutions and Factions)

#### Blocked

- [ ] None

---

### Phase 5 - Collective Agency

#### Completed

- [x] Collective actor types: CoordinationCluster, StableGroup, CollectiveActor, HouseholdActor, OrganizationActor
- [x] Group life-cycle states: ephemeral → proto-group → unstable → stabilized → fragmenting → dissolved
- [x] Promotion criteria based on 03_CollectiveAgency.md section 6.1:
  - boundary clarity, membership rules, shared resources, decision procedure, external capacity
- [x] Internal structure: membership, roles, influence weights, pooled assets, legitimacy, factionalism
- [x] Constitution (decision procedures): MajorityVote, WeightedCouncil, Consensus, CommandHierarchy, PatriarchalDominance, OligarchicDominance
- [x] Merge/split rules with hysteresis to prevent thrashing
- [x] Downward causation through constraints/incentives (NOT direct overwriting of individual beliefs/preferences)
- [x] Collective events: CollectiveLifecycleTransition, CollectiveMerged, CollectiveSplit
- [x] Collective metrics: total_collectives, total_collective_members, average_collective_size, average_collective_legitimacy, average_collective_factionalism
- [x] Integration with simulation step and metrics collection
- [x] Determinism parity maintained
- [x] Frontend collective overlay visualization
- [x] Documentation sync for Phase 5 events and metrics (`docs/EventSchema.md`)
- [x] Review fix: removed periodic hard cleanup of dead agents from sim loop (carry-forward corpse lifecycle in Phase 6)
- [x] Review fix: corrected per-partition social tension mapping (partition index, not pointer cast)

#### In Progress

- [ ] None

#### Next

- [ ] Phase 6: Discovery, Biology, And Institutions

#### Blocked

- [ ] None

---

### Phase 6 - Discovery, Biology, And Institutions

#### Completed

- [x] Hard-coded species archetypes with shared trait+capability model:
  - Horse, Ox/Cattle, Dog, Sheep, Goat, Pig, Poultry, Waterfowl
  - Trait-derived task outputs (transport, traction, hunting, guarding, secondary products)
  - No species-only branching logic - all outputs computed from traits
- [x] Discovery lifecycle (6 stages):
  - AccidentalObservation → AffordanceCandidate → ProcessSchema → Technique → CodifiedKnowledge → InstitutionalizedPractice
  - Stage transitions with bounded rationality
  - Knowledge diffusion with social transmission
- [x] Corpse lifecycle coupling to waste/disease:
  - Dead agents persist as corpses
  - Corpses contribute to waste accumulation
  - Disease pressure from unprocessed corpses
  - Decomposition over ~500 ticks before removal
- [x] Secondary products system:
  - Milk, eggs, wool/fiber, manure production
  - Trait-derived output formulas
  - Zoonotic pressure coupling (livestock density → disease)
- [x] Phase 6 events:
  - DiscoveryStageTransition, CorpseCreated, CorpseDecomposed
  - AnimalCapabilityUtilized, SecondaryProductProduced, ZoonoticPressureChange
- [x] Phase 6 metrics:
  - discoveries_this_tick, total_knowledge_items, average_discovery_stage
  - total_domestic_animals, transport_capacity, traction_capacity
  - milk_produced, eggs_produced, wool_produced, manure_produced
  - zoonotic_pressure, corpse_count
- [x] Determinism parity maintained
- [x] Validation tests for all Phase 6 criteria

#### In Progress (Pass 2)

- [x] PLN-style probabilistic inference layer scaffold (`polis-agents::inference`)
- [x] Truth values with strength/confidence representation
- [x] Belief nodes and inference engine with slower cadence (every 100 ticks)
- [x] Zoonotic spillover risk inference (livestock density + corpse load + sanitation)
- [x] Trade cheating/default risk inference (scarcity + trust deficit + enforcement)
- [x] Institution enforcement-failure risk inference (factionalism + legitimacy + strain)
- [x] Collective fracture/escalation risk inference (grievance + tension + cooperation)
- [x] Famine/crisis early-warning risk inference (food + health + disease)
- [x] Deterministic realization of high-risk incidents
- [x] Validation tests for directional response and determinism
- [x] Fix: Factor beliefs now update on reuse (no frozen factor nodes)
- [x] Fix: Risk belief nodes reuse + configured retention cleanup prevents unbounded growth
- [x] Fix: Deterministic containers for inference/knowledge stores (ordered maps/sets)
- [x] Fix: Removed hardcoded inference retention literal from simulation loop
- [x] Fix: Serial/parallel + checkpoint determinism restored after pass-2 wiring
- [x] Fix: Corpse lifecycle events now emit concrete per-corpse payloads
- [x] Fix: Discovery metrics no longer hardcoded placeholders (event-derived)
- [x] DiscoveryStageTransition surface emission wired
- [x] AnimalCapabilityUtilized surface emission wired
- [x] SecondaryProductProduced surface emission wired
- [x] ZoonoticPressureChange surface emission wired
- [x] Discovery metrics calculation (discoveries_this_tick, total_knowledge_items, average_discovery_stage)

#### Next

- [ ] Spec 08 implementation pass (validation/experimentation plumbing)

#### Deferred Verification Queue

- [ ] Re-run full long-running deterministic suite before release tag:
  - `cargo test -p polis-sim serial_and_parallel_batch_outputs_match`
  - `cargo test --workspace`

#### Blocked

- [ ] None

---

### Phase 2 - Minimal Presentation Shell

#### Completed

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

- [ ] Phase 5: Collective Agency (Institutions and Factions)

#### Blocked

- [ ] None

---

### Phase 7 - Reproducibility Audit and Experiment Pipeline

#### Completed

- [x] Headless execution path
- [x] Run manifests with full provenance:
  - model_version, schema_version from Cargo.toml
  - state_hash_series for reproducibility verification
  - enabled_subsystems bitflag
  - event_count, metric_count
  - started_at, ended_at timestamps
- [x] Snapshot and metric export
- [x] Batch sweep support
- [x] ReproducibilityReport full implementation:
  - single hash verification (backward compatible)
  - series comparison with divergence detection
  - first_divergence_tick tracking
- [x] ExperimentBundle full implementation:
  - ExperimentSpec for formal experiment design per 08_Section 11
  - TreatmentVariable, ParameterRange for experiment parameters
  - ManifestBundle for scenario + subsystem tracking
  - model_version, schema_version, state_hash_series
- [x] RunSummary expanded with state_hash_series
- [x] Simulation tracks state_hash_series through execution
- [x] Tests for reproducibility verification

---

### Phase 8 - Performance And Selective Acceleration

#### Completed

- [x] Phase 8 policy locked: CPU reference is authoritative, GPU is selective acceleration
- [x] `polis-compute` backend scaffold:
  - `ComputeBackend::{CpuReference, GpuAccelerated}`
  - `ComputeConfig` with determinism/tolerance controls
  - `ComputeEngine` facade
- [x] First kernel candidates scaffolded:
  - 1D ring diffusion
  - deterministic reduction (`sum_u64`)
- [x] CPU/GPU parity tests added for scaffolded kernels
- [x] Integrated compute path into production substrate:
  - `polis-world::diffuse_resources` now routes through `polis-compute::ComputeEngine`

#### In Progress

- [ ] Integrate `ComputeEngine` into additional world/subsystem hot paths
- [ ] Add profiling harness for representative scenarios
- [ ] Add acceptance thresholds (minimum speedup + parity tolerance gates)

#### Next

- [ ] Wire diffusion/regeneration system calls through `polis-compute`
- [ ] Add benchmark outputs to experiment/provenance bundle

## Active Work Queue

1. Phase 6 Pass 2: Probabilistic risk inference (PLN-style) integration
   - ✅ Zoonotic spillover risk inference (COMPLETED)
   - ✅ Trade cheating/default risk (COMPLETED)
   - ✅ Institution enforcement-failure risk (COMPLETED)
   - ✅ Collective fracture/escalation risk (COMPLETED)
   - ✅ Famine/crisis early warning risk (COMPLETED)
2. Phase close-out gate (mandatory):
   - code review pass
   - docs sync
   - commit + push before confirming move-on

## Update Log

### 2026-03-14

- Phase 6 Pass 2 COMPLETED:
  - PLN-style probabilistic inference layer (`polis-agents::inference`)
  - TruthValue with strength/confidence, belief nodes, inference engine
  - Slower cadence inference (every 100 ticks) with deterministic realization
  - All 5 risk inference domains implemented:
    - Zoonotic spillover (livestock density + corpse load + sanitation)
    - Trade cheating/default (scarcity + trust deficit + enforcement)
    - Institution enforcement-failure (factionalism + legitimacy + strain)
    - Collective fracture/escalation (grievance + tension + cooperation)
    - Famine/crisis early-warning (food + health + disease)
  - RiskUpdated and IncidentRealized events with contributing factors
  - All 150 tests passing, compilation clean
  - Ready for phase close-out: code review + docs sync + commit/push

### 2026-03-14

- Phase 6 implementation completed:
  - Hard-coded species archetypes (Horse, Ox/Cattle, Dog, Sheep, Goat, Pig, Poultry, Waterfowl)
  - Shared trait+capability model with trait-derived outputs (no species branching)
  - Discovery lifecycle: 6 stages from AccidentalObservation to InstitutionalizedPractice
  - Corpse lifecycle coupling: dead agents persist → waste/disease → decomposition
  - Secondary products: milk, eggs, wool, manure with trait-derived production
  - Phase 6 events: DiscoveryStageTransition, CorpseCreated, AnimalCapabilityUtilized, SecondaryProductProduced, ZoonoticPressureChange
  - Phase 6 metrics: discovery rates, animal capacities, secondary product yields, zoonotic pressure
  - All 51 tests passing, determinism parity maintained

### 2026-03-14

- Phase 5 close-out completed:
  - Added frontend Phase 5 overlay (`Collectives`) with keyboard toggle `C`
  - Added collective event schema section and Phase 5 metric-correlation notes in `docs/EventSchema.md`
  - Fixed review blockers:
    - Removed periodic dead-agent hard cleanup from simulation loop
    - Corrected partition social-tension calculation to use partition index mapping
  - Full workspace tests passing after fixes

### 2026-03-14

- Implemented Phase 5 core: Collective Agency
  - `polis-agents::collective` module with `CollectiveActor`, `CollectiveRegistry`, `CollectiveLifecycleState`
  - Collective actor types: CoordinationCluster, StableGroup, CollectiveActor, HouseholdActor, OrganizationActor
  - Group life-cycle: ephemeral → proto-group → unstable → stabilized → fragmenting → dissolved
  - Promotion criteria based on 03_CollectiveAgency.md: boundary clarity, membership rules, shared resources, decision procedure, external capacity
  - Internal structure: membership, roles, influence weights, pooled assets, legitimacy, factionalism
  - Constitution types: MajorityVote, WeightedCouncil, Consensus, CommandHierarchy, PatriarchalDominance, OligarchicDominance
  - Merge/split rules with hysteresis to prevent thrashing
  - Disciplined downward causation: constraints/incentives, NOT direct overwriting of individual beliefs/preferences
  - Collective events: CollectiveLifecycleTransition, CollectiveMerged, CollectiveSplit
  - Collective metrics: total_collectives, total_collective_members, average_collective_size, average_collective_legitimacy, average_collective_factionalism
  - Integration with simulation step in `polis-sim`
  - All 87 tests passing, determinism parity maintained

### 2026-03-14

- Completed Phase 4 close-out:
  - Frontend social overlays: SocialTension (red/blue cohesion gradient) and CrossSpecies (green/red tolerance gradient)
  - Keyboard controls: T for SocialTension, A for CrossSpecies overlays
  - Social metrics in tooltips: fear/tolerance display
  - Cross-species metrics in detail panel: fear, tolerance, familiarity
  - Updated EventSchema.md with Phase 4 events (TrustShifted, CooperationOccurred, ConflictOccurred, HumanAnimalContact)
  - Added supporting enums to documentation (TrustShiftReason, CooperationKind, ConflictReason, HumanAnimalContactType, HumanAnimalOutcome)
  - Updated version history to 0.2.0
  - Updated Rust implementation appendix with complete SimEvent enum
  - All 109 tests passing, determinism parity verified

### 2026-03-14

- Implemented Phase 4 backend/runtime: Social Fabric with cross-species domestication primitives:
  - `polis-agents::social` module with `SocialTie`, `SocialNetwork`, `CrossSpeciesState`
  - Social ties graph: trust (-100 to +100), grievance (0-100), interaction history
  - Deterministic trust/grievance updates from cooperation/conflict events
  - Cooperation rules gated by trust levels
  - Conflict rules based on scarcity stress and accumulated grievance
  - Cross-species interaction: familiarity, fear, aggression, human_tolerance
  - Human-animal contact types: Hunting, Feeding, Proximity, Handling
  - Early domestication progression based on tolerance (not capture count)
  - Social events: TrustShifted, CooperationOccurred, ConflictOccurred, HumanAnimalContact
  - Social metrics: total_social_ties, average_trust, average_grievance, cooperation_count, conflict_count, social_tension
  - Cross-species metrics: average_animal_familiarity, average_animal_fear, average_animal_tolerance
  - Deterministic serial/parallel parity restored after deterministic ordering fixes
  - Full workspace tests passing locally

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
