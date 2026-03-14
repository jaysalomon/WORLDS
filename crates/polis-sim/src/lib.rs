use std::fs;
use std::path::Path;

use polis_agents::AgentPopulation;
use polis_core::{RunManifest, SimulationSeed, workspace_status};
use polis_systems::{apply_phase_to_partition, registered_phases};
use polis_world::{DEFAULT_PARTITION_COUNT, WorldState};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub struct SimModule;

impl SimModule {
    pub const fn name() -> &'static str {
        "polis-sim"
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ScenarioConfig {
    pub name: String,
    pub seed: u64,
    #[serde(default = "default_partition_count")]
    pub partition_count: u64,
    pub notes: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimState {
    pub tick: u64,
    pub state_hash: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct TickMetrics {
    pub tick: u64,
    pub total_resource: u64,
    pub total_waste: u64,
    pub total_herbivores: u64,
    pub total_predators: u64,
    pub total_proto_domestic: u64,
    pub average_tameness_ppm: u64,
    pub total_demand: u64,
    pub average_cohesion: u64,
    // Phase 3: Agent population metrics
    pub total_agents: u64,
    pub average_agent_health: u64,
    pub average_agent_hunger: u64,
    pub average_agent_thirst: u64,
    // Phase 4: Social fabric metrics
    pub total_social_ties: u64,
    pub average_trust: i64, // Signed for negative values
    pub average_grievance: u64,
    pub cooperation_count: u64,
    pub conflict_count: u64,
    pub social_tension: u64,
    // Phase 4: Cross-species metrics
    pub average_animal_familiarity: u64,
    pub average_animal_fear: u64,
    pub average_animal_tolerance: u64,
    // Phase 5: Collective agency metrics
    pub total_collectives: u64,
    pub total_collective_members: u64,
    pub average_collective_size: u64,
    pub average_collective_legitimacy: u64,
    pub average_collective_factionalism: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SimEvent {
    TickStarted {
        tick: u64,
    },
    PhaseApplied {
        tick: u64,
        phase_index: u8,
        partition_count: u64,
    },
    TickCompleted {
        tick: u64,
        state_hash: u64,
    },
    // Phase 4: Social fabric events
    TrustShifted {
        tick: u64,
        agent_a: u64,
        agent_b: u64,
        new_trust: i8,
        reason: TrustShiftReason,
    },
    CooperationOccurred {
        tick: u64,
        agent_a: u64,
        agent_b: u64,
        kind: CooperationKind,
    },
    ConflictOccurred {
        tick: u64,
        agent_a: u64,
        agent_b: u64,
        severity: u8,
        reason: ConflictReason,
    },
    // Phase 4: Cross-species events
    HumanAnimalContact {
        tick: u64,
        partition_id: u64,
        contact_type: HumanAnimalContactType,
        outcome: HumanAnimalOutcome,
    },
    // Phase 5: Collective agency events
    CollectiveLifecycleTransition {
        tick: u64,
        collective_id: u64,
        old_state: String,
        new_state: String,
    },
    CollectiveMerged {
        tick: u64,
        primary_id: u64,
        secondary_id: u64,
        merged_id: u64,
    },
    CollectiveSplit {
        tick: u64,
        original_id: u64,
        new_id: u64,
    },
}

/// Reason for trust shift
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TrustShiftReason {
    Cooperation,
    Conflict,
    TimeDecay,
}

/// Types of cooperation
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CooperationKind {
    ResourceSharing,
    MutualAid,
    Information,
}

/// Reason for conflict
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConflictReason {
    ResourceScarcity,
    Grievance,
    Territorial,
}

/// Type of human-animal contact
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum HumanAnimalContactType {
    Hunting,   // Negative: harsh
    Feeding,   // Positive: gentle
    Proximity, // Neutral: just nearby
    Handling,  // Could be positive or negative
}

/// Outcome of human-animal contact
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum HumanAnimalOutcome {
    Positive,
    Negative,
    Neutral,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct RunSummary {
    pub seed: SimulationSeed,
    pub partition_count: u64,
    pub ticks: u64,
    pub final_state_hash: u64,
    pub event_count: u64,
    pub metric_count: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExecutionMode {
    Serial,
    Parallel,
}

#[derive(Debug, Error)]
pub enum SimError {
    #[error("failed to read scenario file '{path}': {source}")]
    ScenarioRead {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to parse scenario RON '{path}': {source}")]
    ScenarioParse {
        path: String,
        #[source]
        source: ron::error::SpannedError,
    },
    #[error("failed to read checkpoint file '{path}': {source}")]
    CheckpointRead {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to parse checkpoint JSON '{path}': {source}")]
    CheckpointParse {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("failed to serialize checkpoint JSON '{path}': {source}")]
    CheckpointSerialize {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("failed to write checkpoint file '{path}': {source}")]
    CheckpointWrite {
        path: String,
        #[source]
        source: std::io::Error,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationCheckpoint {
    pub seed: SimulationSeed,
    pub state: SimState,
    pub world: WorldState,
    pub agents: AgentPopulation,
    pub events: Vec<SimEvent>,
    pub metrics: Vec<TickMetrics>,
}

pub struct Simulation {
    seed: SimulationSeed,
    state: SimState,
    world: WorldState,
    agents: AgentPopulation,
    events: Vec<SimEvent>,
    metrics: Vec<TickMetrics>,
}

impl Simulation {
    pub fn from_scenario_file(path: impl AsRef<Path>) -> Result<Self, SimError> {
        let scenario = load_scenario_file(path)?;
        Ok(Self::new_with_partition_count(
            SimulationSeed::new(scenario.seed),
            scenario.partition_count,
        ))
    }

    pub fn new(seed: SimulationSeed) -> Self {
        Self::new_with_partition_count(seed, DEFAULT_PARTITION_COUNT)
    }

    pub fn new_with_partition_count(seed: SimulationSeed, partition_count: u64) -> Self {
        let world = WorldState::new(seed.0, partition_count.max(1));
        let mut agents = AgentPopulation::new();
        // Initialize with starting population (10 agents per partition)
        agents.initialize(partition_count as usize * 10, partition_count, seed.0);

        Self {
            seed,
            state: SimState {
                tick: 0,
                state_hash: step_hash(seed.0, 0, world.digest() ^ agents.digest().rotate_left(1)),
            },
            world,
            agents,
            events: Vec::new(),
            metrics: Vec::new(),
        }
    }

    pub fn from_checkpoint(checkpoint: SimulationCheckpoint) -> Self {
        Self {
            seed: checkpoint.seed,
            state: checkpoint.state,
            world: checkpoint.world,
            agents: checkpoint.agents,
            events: checkpoint.events,
            metrics: checkpoint.metrics,
        }
    }

    pub const fn seed(&self) -> SimulationSeed {
        self.seed
    }

    pub const fn state(&self) -> SimState {
        self.state
    }

    pub fn world(&self) -> &WorldState {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut WorldState {
        &mut self.world
    }

    pub fn agents(&self) -> &AgentPopulation {
        &self.agents
    }

    pub fn agents_mut(&mut self) -> &mut AgentPopulation {
        &mut self.agents
    }

    pub fn events(&self) -> &[SimEvent] {
        &self.events
    }

    pub fn metrics(&self) -> &[TickMetrics] {
        &self.metrics
    }

    pub fn checkpoint(&self) -> SimulationCheckpoint {
        SimulationCheckpoint {
            seed: self.seed,
            state: self.state,
            world: self.world.clone(),
            agents: self.agents.clone(),
            events: self.events.clone(),
            metrics: self.metrics.clone(),
        }
    }

    pub fn save_checkpoint(&self, path: impl AsRef<Path>) -> Result<(), SimError> {
        let path_ref = path.as_ref();
        let path_string = path_ref.display().to_string();
        if let Some(parent) = path_ref.parent() {
            fs::create_dir_all(parent).map_err(|source| SimError::CheckpointWrite {
                path: path_string.clone(),
                source,
            })?;
        }

        let bytes = serde_json::to_vec_pretty(&self.checkpoint()).map_err(|source| {
            SimError::CheckpointSerialize {
                path: path_string.clone(),
                source,
            }
        })?;

        fs::write(path_ref, bytes).map_err(|source| SimError::CheckpointWrite {
            path: path_string,
            source,
        })
    }

    pub fn load_checkpoint(path: impl AsRef<Path>) -> Result<Self, SimError> {
        let path_ref = path.as_ref();
        let path_string = path_ref.display().to_string();
        let bytes = fs::read(path_ref).map_err(|source| SimError::CheckpointRead {
            path: path_string.clone(),
            source,
        })?;
        let checkpoint: SimulationCheckpoint =
            serde_json::from_slice(&bytes).map_err(|source| SimError::CheckpointParse {
                path: path_string,
                source,
            })?;
        Ok(Self::from_checkpoint(checkpoint))
    }

    pub fn step(&mut self) {
        self.step_with_mode(ExecutionMode::Serial);
    }

    pub fn step_with_mode(&mut self, mode: ExecutionMode) {
        use polis_systems::{
            agent_commit_phase, agent_decision_phase, agent_perception_phase,
            cross_species_interaction_phase, social_interaction_phase,
        };

        self.state.tick = self.state.tick.wrapping_add(1);
        self.world.set_tick(self.state.tick);
        self.events.push(SimEvent::TickStarted {
            tick: self.state.tick,
        });

        // Agent perception phase (movement decisions)
        agent_perception_phase(
            self.agents.agents_mut(),
            self.world.partitions(),
            self.state.tick,
            self.seed.0,
        );

        for (phase_index, phase) in registered_phases().iter().enumerate() {
            match mode {
                ExecutionMode::Serial => {
                    for (partition_id, partition) in
                        self.world.partitions_mut().iter_mut().enumerate()
                    {
                        apply_phase_to_partition(
                            *phase,
                            self.state.tick,
                            partition_id as u64,
                            partition,
                        );
                    }
                }
                ExecutionMode::Parallel => {
                    self.world
                        .partitions_mut()
                        .par_iter_mut()
                        .enumerate()
                        .for_each(|(partition_id, partition)| {
                            apply_phase_to_partition(
                                *phase,
                                self.state.tick,
                                partition_id as u64,
                                partition,
                            );
                        });
                }
            }
            self.events.push(SimEvent::PhaseApplied {
                tick: self.state.tick,
                phase_index: phase_index as u8,
                partition_count: self.world.partition_count(),
            });
        }

        // Agent decision phase (consumption and reproduction)
        let newborns = {
            let agents = self.agents.agents_mut();
            let partitions = self.world.partitions_mut();
            agent_decision_phase(agents, partitions, self.state.tick, self.seed.0)
        };

        // Spawn newborns outside the borrow
        for (partition_id, parent_metabolism) in newborns {
            let seed = self.seed.0 ^ self.state.tick ^ partition_id;
            self.agents
                .spawn_newborn(partition_id, parent_metabolism, seed);
        }

        // Agent commit phase (needs update and mortality)
        agent_commit_phase(self.agents.agents_mut(), self.world.partitions_mut());

        // Phase 4: Social interaction phase
        // Process social interactions and get events
        // We need to temporarily take the social network to avoid borrow issues
        let mut social_network = std::mem::take(&mut self.agents.social_network);
        let social_events = social_interaction_phase(
            self.agents.agents(),
            self.world.partitions(),
            &mut social_network,
            self.state.tick,
            self.seed.0,
        );
        self.agents.social_network = social_network;

        // Convert and add social events
        for event in social_events {
            let sim_event = match event {
                polis_systems::SocialEvent::TrustShifted {
                    agent_a,
                    agent_b,
                    new_trust,
                    reason,
                } => SimEvent::TrustShifted {
                    tick: self.state.tick,
                    agent_a,
                    agent_b,
                    new_trust,
                    reason: match reason {
                        polis_systems::TrustShiftReason::Cooperation => {
                            TrustShiftReason::Cooperation
                        }
                        polis_systems::TrustShiftReason::Conflict => TrustShiftReason::Conflict,
                        polis_systems::TrustShiftReason::TimeDecay => TrustShiftReason::TimeDecay,
                    },
                },
                polis_systems::SocialEvent::Cooperation {
                    agent_a,
                    agent_b,
                    kind,
                } => SimEvent::CooperationOccurred {
                    tick: self.state.tick,
                    agent_a,
                    agent_b,
                    kind: match kind {
                        polis_systems::CooperationKind::ResourceSharing => {
                            CooperationKind::ResourceSharing
                        }
                        polis_systems::CooperationKind::MutualAid => CooperationKind::MutualAid,
                        polis_systems::CooperationKind::Information => CooperationKind::Information,
                    },
                },
                polis_systems::SocialEvent::Conflict {
                    agent_a,
                    agent_b,
                    severity,
                    reason,
                } => SimEvent::ConflictOccurred {
                    tick: self.state.tick,
                    agent_a,
                    agent_b,
                    severity,
                    reason: match reason {
                        polis_systems::ConflictReason::ResourceScarcity => {
                            ConflictReason::ResourceScarcity
                        }
                        polis_systems::ConflictReason::Grievance => ConflictReason::Grievance,
                        polis_systems::ConflictReason::Territorial => ConflictReason::Territorial,
                    },
                },
            };
            self.events.push(sim_event);
        }

        // Phase 4: Cross-species interaction phase
        let cross_species_events = cross_species_interaction_phase(
            self.agents.agents(),
            self.world.partitions_mut(),
            self.state.tick,
            self.seed.0,
        );

        // Convert and add cross-species events
        for event in cross_species_events {
            let contact_type = match event.contact_type {
                polis_systems::HumanAnimalContactType::Hunting => HumanAnimalContactType::Hunting,
                polis_systems::HumanAnimalContactType::Feeding => HumanAnimalContactType::Feeding,
                polis_systems::HumanAnimalContactType::Proximity => {
                    HumanAnimalContactType::Proximity
                }
                polis_systems::HumanAnimalContactType::Handling => HumanAnimalContactType::Handling,
            };
            let outcome = match event.outcome {
                polis_systems::HumanAnimalOutcome::Positive => HumanAnimalOutcome::Positive,
                polis_systems::HumanAnimalOutcome::Negative => HumanAnimalOutcome::Negative,
                polis_systems::HumanAnimalOutcome::Neutral => HumanAnimalOutcome::Neutral,
            };
            self.events.push(SimEvent::HumanAnimalContact {
                tick: self.state.tick,
                partition_id: event.partition_id,
                contact_type,
                outcome,
            });
        }

        // Phase 5: Collective lifecycle phase
        // Process collective lifecycle, merge/split detection, downward causation
        let mut collective_registry = std::mem::take(&mut self.agents.collective_registry);
        let collective_events = polis_systems::collective_lifecycle_phase(
            self.agents.agents_mut(),
            &mut collective_registry,
            self.state.tick,
            self.seed.0,
        );
        self.agents.collective_registry = collective_registry;

        // Convert and add collective events
        for event in collective_events {
            let sim_event = match event {
                polis_systems::CollectiveEvent::LifecycleTransition {
                    collective_id,
                    old_state,
                    new_state,
                } => SimEvent::CollectiveLifecycleTransition {
                    tick: self.state.tick,
                    collective_id,
                    old_state,
                    new_state,
                },
                polis_systems::CollectiveEvent::Merged {
                    primary_id,
                    secondary_id,
                    merged_id,
                } => SimEvent::CollectiveMerged {
                    tick: self.state.tick,
                    primary_id,
                    secondary_id,
                    merged_id,
                },
                polis_systems::CollectiveEvent::Split {
                    original_id,
                    new_id,
                } => SimEvent::CollectiveSplit {
                    tick: self.state.tick,
                    original_id,
                    new_id,
                },
            };
            self.events.push(sim_event);
        }

        self.state.state_hash = step_hash(
            self.seed.0,
            self.state.tick,
            self.world.digest() ^ self.agents.digest().rotate_left(1),
        );
        self.events.push(SimEvent::TickCompleted {
            tick: self.state.tick,
            state_hash: self.state.state_hash,
        });
        self.metrics
            .push(compute_tick_metrics(&self.world, &self.agents));
    }

    pub fn run_for(&mut self, ticks: u64) -> RunSummary {
        self.run_for_with_mode(ticks, ExecutionMode::Serial)
    }

    pub fn run_for_with_mode(&mut self, ticks: u64, mode: ExecutionMode) -> RunSummary {
        for _ in 0..ticks {
            self.step_with_mode(mode);
        }

        RunSummary {
            seed: self.seed,
            partition_count: self.world.partition_count(),
            ticks: self.state.tick,
            final_state_hash: self.state.state_hash,
            event_count: self.events.len() as u64,
            metric_count: self.metrics.len() as u64,
        }
    }
}

pub fn load_scenario_file(path: impl AsRef<Path>) -> Result<ScenarioConfig, SimError> {
    let path_ref = path.as_ref();
    let path_string = path_ref.display().to_string();
    let scenario_str = fs::read_to_string(path_ref).map_err(|source| SimError::ScenarioRead {
        path: path_string.clone(),
        source,
    })?;

    ron::from_str(&scenario_str).map_err(|source| SimError::ScenarioParse {
        path: path_string,
        source,
    })
}

pub fn build_run_manifest(
    scenario: &ScenarioConfig,
    summary: RunSummary,
    mode: ExecutionMode,
) -> RunManifest {
    RunManifest {
        scenario_name: scenario.name.clone(),
        seed: summary.seed.0,
        partition_count: summary.partition_count,
        ticks: summary.ticks,
        final_state_hash: summary.final_state_hash,
        execution_mode: match mode {
            ExecutionMode::Serial => "serial".to_string(),
            ExecutionMode::Parallel => "parallel".to_string(),
        },
        workspace_status: workspace_status().to_string(),
    }
}

pub fn run_seed_batch(
    base_seed: u64,
    batch_size: u64,
    ticks: u64,
    mode: ExecutionMode,
) -> Vec<RunSummary> {
    match mode {
        ExecutionMode::Serial => run_seed_batch_serial(base_seed, batch_size, ticks),
        ExecutionMode::Parallel => run_seed_batch_parallel(base_seed, batch_size, ticks),
    }
}

pub fn run_seed_batch_serial(base_seed: u64, batch_size: u64, ticks: u64) -> Vec<RunSummary> {
    (0..batch_size)
        .map(|offset| {
            let seed = SimulationSeed::new(base_seed.wrapping_add(offset));
            let mut sim = Simulation::new(seed);
            sim.run_for_with_mode(ticks, ExecutionMode::Serial)
        })
        .collect()
}

pub fn run_seed_batch_parallel(base_seed: u64, batch_size: u64, ticks: u64) -> Vec<RunSummary> {
    let mut results: Vec<RunSummary> = (0..batch_size)
        .into_par_iter()
        .map(|offset| {
            let seed = SimulationSeed::new(base_seed.wrapping_add(offset));
            let mut sim = Simulation::new(seed);
            // Batch fan-out is parallelized at the run level; each run stays
            // serial internally to avoid nested parallel oversubscription.
            sim.run_for_with_mode(ticks, ExecutionMode::Serial)
        })
        .collect();

    // Keep output order stable for deterministic research pipelines.
    results.sort_unstable_by_key(|summary| summary.seed.0);
    results
}

const fn default_partition_count() -> u64 {
    DEFAULT_PARTITION_COUNT
}

fn step_hash(seed: u64, tick: u64, current: u64) -> u64 {
    // Small deterministic mix function for scaffold-level reproducibility checks.
    let mut x = current ^ seed.rotate_left(13) ^ tick.rotate_right(7);
    x ^= x >> 33;
    x = x.wrapping_mul(0xff51_afd7_ed55_8ccd);
    x ^= x >> 33;
    x = x.wrapping_mul(0xc4ce_b9fe_1a85_ec53);
    x ^= x >> 33;
    x
}

fn compute_tick_metrics(world: &WorldState, agents: &AgentPopulation) -> TickMetrics {
    let partition_count = world.partition_count().max(1);
    let (
        total_resource,
        total_waste,
        total_herbivores,
        total_predators,
        total_proto_domestic,
        total_tameness_ppm,
        total_demand,
        total_cohesion,
        // Phase 4: Cross-species metrics
        total_animal_familiarity,
        total_animal_fear,
        total_animal_tolerance,
    ) = world.partitions().iter().fold(
        (
            0_u64, 0_u64, 0_u64, 0_u64, 0_u64, 0_u64, 0_u64, 0_u64, 0_u64, 0_u64, 0_u64,
        ),
        |(r, w, h, p, pd, tppm, d, c, afam, afe, at), partition| {
            (
                r.wrapping_add(partition.total_resources().max(0) as u64),
                w.wrapping_add(partition.waste.quantity.max(0) as u64),
                h.wrapping_add(partition.herbivore_population),
                p.wrapping_add(partition.predator_population),
                pd.wrapping_add(partition.proto_domestic_population),
                tppm.wrapping_add((partition.domestication_tameness * 1_000_000.0) as u64),
                d.wrapping_add(partition.demand),
                c.wrapping_add(partition.cohesion),
                afam.wrapping_add(partition.animal_familiarity as u64),
                afe.wrapping_add(partition.animal_fear as u64),
                at.wrapping_add(partition.animal_human_tolerance as u64),
            )
        },
    );

    // Agent population metrics
    let agent_stats = agents.statistics();

    // Social network metrics
    let social_stats = agents.social_statistics();

    // Calculate social tension across partitions
    let social_tension: u64 = world
        .partitions()
        .iter()
        .enumerate()
        .map(|(partition_index, _)| {
            let agents_in_partition: Vec<_> = agents
                .agents()
                .iter()
                .filter(|a| a.is_alive && a.partition_id == partition_index as u64)
                .map(|a| a.id)
                .collect();
            agents
                .social_network
                .partition_tension(&agents_in_partition) as u64
        })
        .sum();

    // Phase 5: Collective metrics
    let collective_stats = agents.collective_statistics();

    TickMetrics {
        tick: world.tick(),
        total_resource,
        total_waste,
        total_herbivores,
        total_predators,
        total_proto_domestic,
        average_tameness_ppm: total_tameness_ppm / partition_count,
        total_demand,
        average_cohesion: total_cohesion / partition_count,
        // Phase 3 agent metrics
        total_agents: agent_stats.total_population,
        average_agent_health: agent_stats.average_health as u64,
        average_agent_hunger: agent_stats.average_hunger as u64,
        average_agent_thirst: agent_stats.average_thirst as u64,
        // Phase 4 social metrics
        total_social_ties: social_stats.total_ties,
        average_trust: social_stats.average_trust as i64,
        average_grievance: social_stats.average_grievance as u64,
        cooperation_count: social_stats.total_cooperation,
        conflict_count: social_stats.total_conflict,
        social_tension: social_tension / partition_count.max(1),
        // Phase 4 cross-species metrics
        average_animal_familiarity: total_animal_familiarity / partition_count,
        average_animal_fear: total_animal_fear / partition_count,
        average_animal_tolerance: total_animal_tolerance / partition_count,
        // Phase 5 collective metrics
        total_collectives: collective_stats.total_collectives,
        total_collective_members: collective_stats.total_members,
        average_collective_size: collective_stats.average_size,
        average_collective_legitimacy: collective_stats.average_legitimacy as u64,
        average_collective_factionalism: collective_stats.average_factionalism as u64,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ExecutionMode, ScenarioConfig, SimEvent, Simulation, build_run_manifest, run_seed_batch,
        run_seed_batch_parallel, run_seed_batch_serial,
    };
    use polis_core::SimulationSeed;

    #[test]
    fn deterministic_run_matches_for_same_seed() {
        let mut a = Simulation::new(SimulationSeed::new(42));
        let mut b = Simulation::new(SimulationSeed::new(42));

        let run_a = a.run_for(1_000);
        let run_b = b.run_for(1_000);

        assert_eq!(run_a, run_b);
    }

    #[test]
    fn run_diverges_for_different_seeds() {
        let mut a = Simulation::new(SimulationSeed::new(42));
        let mut b = Simulation::new(SimulationSeed::new(43));

        let run_a = a.run_for(1_000);
        let run_b = b.run_for(1_000);

        assert_ne!(run_a.final_state_hash, run_b.final_state_hash);
    }

    #[test]
    fn parse_default_scenario_schema() {
        let scenario_str = r#"
            (
                name: "default",
                seed: 42,
                partition_count: 64,
                notes: "Placeholder scenario used while the runtime is being built.",
            )
        "#;

        let parsed: ScenarioConfig = ron::from_str(scenario_str).expect("valid scenario schema");
        assert_eq!(parsed.seed, 42);
        assert_eq!(parsed.name, "default");
        assert_eq!(parsed.partition_count, 64);
    }

    #[test]
    fn scenario_defaults_partition_count() {
        let scenario_str = r#"
            (
                name: "default",
                seed: 42,
                notes: "Placeholder scenario used while the runtime is being built.",
            )
        "#;

        let parsed: ScenarioConfig = ron::from_str(scenario_str).expect("valid scenario schema");
        assert_eq!(parsed.partition_count, super::default_partition_count());
    }

    #[test]
    fn missing_scenario_file_returns_clear_error() {
        let err = super::load_scenario_file("scenarios/does-not-exist.ron").expect_err("missing");
        match err {
            super::SimError::ScenarioRead { path, .. } => {
                assert!(path.ends_with("does-not-exist.ron"));
            }
            other => panic!("unexpected error: {other}"),
        }
    }

    #[test]
    fn invalid_scenario_ron_returns_parse_error() {
        use std::fs;
        use std::path::PathBuf;
        use std::time::{SystemTime, UNIX_EPOCH};

        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let path = PathBuf::from(format!("target/tmp-invalid-scenario-{nonce}.ron"));
        fs::create_dir_all("target").expect("target dir");
        fs::write(&path, "(name: \"broken\", seed: , notes: \"bad\")").expect("write");

        let err = super::load_scenario_file(&path).expect_err("parse error");
        match err {
            super::SimError::ScenarioParse { path, .. } => {
                assert!(path.contains("tmp-invalid-scenario-"));
            }
            other => panic!("unexpected error: {other}"),
        }

        let _ = fs::remove_file(path);
    }

    #[test]
    fn serial_and_parallel_batch_outputs_match() {
        let serial = run_seed_batch_serial(100, 128, 1_000);
        let parallel = run_seed_batch_parallel(100, 128, 1_000);
        assert_eq!(serial, parallel);
    }

    #[test]
    fn mode_dispatch_matches_specialized_paths() {
        let via_mode = run_seed_batch(500, 64, 500, ExecutionMode::Parallel);
        let direct = run_seed_batch_parallel(500, 64, 500);
        assert_eq!(via_mode, direct);
    }

    #[test]
    fn single_run_serial_and_parallel_match() {
        let mut serial = Simulation::new(SimulationSeed::new(123));
        let mut parallel = Simulation::new(SimulationSeed::new(123));

        let run_serial = serial.run_for_with_mode(2_000, ExecutionMode::Serial);
        let run_parallel = parallel.run_for_with_mode(2_000, ExecutionMode::Parallel);

        assert_eq!(run_serial, run_parallel);
    }

    #[test]
    fn serial_and_parallel_event_streams_match() {
        let mut serial = Simulation::new(SimulationSeed::new(777));
        let mut parallel = Simulation::new(SimulationSeed::new(777));

        serial.run_for_with_mode(128, ExecutionMode::Serial);
        parallel.run_for_with_mode(128, ExecutionMode::Parallel);

        assert_eq!(serial.events(), parallel.events());
        assert_eq!(serial.metrics(), parallel.metrics());
    }

    #[test]
    fn event_order_is_tick_then_phases_then_complete() {
        let mut sim = Simulation::new(SimulationSeed::new(123));
        sim.step();
        let events = sim.events();

        // Core events: TickStarted, 3 PhaseApplied, TickCompleted
        // Plus optional social events (TrustShifted, CooperationOccurred, ConflictOccurred, HumanAnimalContact)
        assert!(
            events.len() >= 5,
            "Should have at least 5 core events, got {}",
            events.len()
        );

        // First event should be TickStarted
        assert_eq!(events[0], SimEvent::TickStarted { tick: 1 });

        // Find PhaseApplied events
        let phase_events: Vec<&SimEvent> = events
            .iter()
            .filter(|e| matches!(e, SimEvent::PhaseApplied { .. }))
            .collect();
        assert_eq!(phase_events.len(), 3, "Should have 3 phase applied events");

        // Check phase indices are in order
        for (i, phase_event) in phase_events.iter().enumerate() {
            if let SimEvent::PhaseApplied { phase_index, .. } = phase_event {
                assert_eq!(*phase_index, i as u8, "Phase indices should be in order");
            }
        }

        // Last event should be TickCompleted
        assert!(
            matches!(
                events.last(),
                Some(SimEvent::TickCompleted {
                    tick: 1,
                    state_hash: _
                })
            ),
            "Last event should be TickCompleted"
        );

        assert_eq!(sim.metrics().len(), 1);
    }

    #[test]
    fn manifest_contains_expected_fields() {
        let scenario = ScenarioConfig {
            name: "default".to_string(),
            seed: 42,
            partition_count: 64,
            notes: "test".to_string(),
        };
        let summary = super::RunSummary {
            seed: SimulationSeed::new(42),
            partition_count: 64,
            ticks: 10,
            final_state_hash: 99,
            event_count: 40,
            metric_count: 10,
        };

        let manifest = build_run_manifest(&scenario, summary, ExecutionMode::Parallel);
        assert_eq!(manifest.scenario_name, "default");
        assert_eq!(manifest.seed, 42);
        assert_eq!(manifest.partition_count, 64);
        assert_eq!(manifest.ticks, 10);
        assert_eq!(manifest.final_state_hash, 99);
        assert_eq!(manifest.execution_mode, "parallel");
    }

    #[test]
    fn checkpoint_round_trip_restores_state() {
        let mut sim = Simulation::new(SimulationSeed::new(321));
        sim.run_for_with_mode(200, ExecutionMode::Parallel);
        let checkpoint = sim.checkpoint();
        let restored = Simulation::from_checkpoint(checkpoint.clone());
        assert_eq!(restored.seed(), checkpoint.seed);
        assert_eq!(restored.state(), checkpoint.state);
        assert_eq!(restored.events(), checkpoint.events.as_slice());
        assert_eq!(restored.metrics(), checkpoint.metrics.as_slice());
    }

    #[test]
    fn checkpoint_resume_matches_continuous_run() {
        let mut continuous = Simulation::new(SimulationSeed::new(555));
        let first = continuous.run_for_with_mode(300, ExecutionMode::Serial);
        assert_eq!(first.ticks, 300);
        let checkpoint = continuous.checkpoint();

        let mut resumed = Simulation::from_checkpoint(checkpoint);
        let resumed_summary = resumed.run_for_with_mode(700, ExecutionMode::Serial);

        let mut baseline = Simulation::new(SimulationSeed::new(555));
        let baseline_summary = baseline.run_for_with_mode(1_000, ExecutionMode::Serial);

        assert_eq!(
            resumed_summary.final_state_hash,
            baseline_summary.final_state_hash
        );
        assert_eq!(resumed.events(), baseline.events());
        assert_eq!(resumed.metrics(), baseline.metrics());
    }

    #[test]
    fn metrics_track_waste_flow() {
        let mut sim = Simulation::new(SimulationSeed::new(42));
        sim.run_for_with_mode(25, ExecutionMode::Serial);
        let metrics = sim.metrics();
        assert!(!metrics.is_empty());
        assert!(metrics.iter().any(|m| m.total_waste > 0));
        assert!(metrics.iter().all(|m| m.total_herbivores > 0));
        assert!(metrics.iter().all(|m| m.total_predators > 0));
    }
}
