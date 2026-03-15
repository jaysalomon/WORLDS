use std::fs;
use std::path::Path;

use polis_agents::AgentPopulation;
use polis_core::{DeterministicRng, RunManifest, SimulationSeed, workspace_status};
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
    // Phase 6: Discovery metrics
    pub discoveries_this_tick: u64,
    pub total_knowledge_items: u64,
    pub average_discovery_stage: u64,
    // Phase 6: Animal metrics
    pub total_domestic_animals: u64,
    pub transport_capacity: u64,
    pub traction_capacity: u64,
    // Phase 6: Secondary product metrics
    pub milk_produced: u64,
    pub eggs_produced: u64,
    pub wool_produced: u64,
    pub manure_produced: u64,
    // Phase 6: Disease metrics
    pub zoonotic_pressure: u64,
    pub corpse_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    // Phase 6: Discovery events
    DiscoveryStageTransition {
        tick: u64,
        agent_id: u64,
        knowledge_id: u64,
        old_stage: DiscoveryStage,
        new_stage: DiscoveryStage,
        method: DiscoveryMethod,
    },
    // Phase 6: Corpse lifecycle events
    CorpseCreated {
        tick: u64,
        agent_id: u64,
        partition_id: u64,
        biomass: u32,
    },
    CorpseDecomposed {
        tick: u64,
        agent_id: u64,
        partition_id: u64,
        waste_produced: u32,
    },
    // Phase 6: Animal capability utilization
    AnimalCapabilityUtilized {
        tick: u64,
        animal_id: u64,
        species_id: u64,
        partition_id: u64,
        capability: AnimalCapability,
        effectiveness: u8, // 0-100
    },
    // Phase 6: Secondary product outputs
    SecondaryProductProduced {
        tick: u64,
        partition_id: u64,
        product_type: SecondaryProductType,
        amount: u32, // Changed from f32 to u32 for Eq
    },
    // Phase 6: Zoonotic disease events
    ZoonoticPressureChange {
        tick: u64,
        partition_id: u64,
        livestock_density: u32,
        disease_pressure: u32,
        spillover_risk: u8, // 0-100
    },
    // Phase 6 Pass 2: Inference events
    RiskUpdated {
        tick: u64,
        partition_id: u64,
        risk_type: RiskType,
        strength: f32,
        confidence: f32,
        top_factors: Vec<(String, f32)>,
    },
    IncidentRealized {
        tick: u64,
        partition_id: u64,
        risk_type: RiskType,
        severity: u8, // 0-100
        contributing_factors: Vec<String>,
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

/// Discovery lifecycle stages (from 05_DiscoveryHeuristics.md)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DiscoveryStage {
    AccidentalObservation,
    AffordanceCandidate,
    ProcessSchema,
    Technique,
    CodifiedKnowledge,
    InstitutionalizedPractice,
}

impl DiscoveryStage {
    pub const fn as_u8(self) -> u8 {
        match self {
            DiscoveryStage::AccidentalObservation => 1,
            DiscoveryStage::AffordanceCandidate => 2,
            DiscoveryStage::ProcessSchema => 3,
            DiscoveryStage::Technique => 4,
            DiscoveryStage::CodifiedKnowledge => 5,
            DiscoveryStage::InstitutionalizedPractice => 6,
        }
    }
}

/// Discovery methods
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DiscoveryMethod {
    Repetition,
    Search,
    Accident,
    SocialTransmission,
}

/// Animal capabilities that can be utilized
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AnimalCapability {
    Transport,
    Traction,
    HuntingSupport,
    Guarding,
    Herding,
    MilkProduction,
    EggProduction,
    FiberProduction,
}

/// Types of secondary products from animals
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SecondaryProductType {
    Milk,
    Eggs,
    Wool,
    Manure,
}

// RiskType is re-exported from polis_agents::inference
pub use polis_agents::inference::RiskType;

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

        // Phase 6: Corpse lifecycle - process dead agents into corpses
        let new_corpses = self.agents.process_dead_into_corpses(self.state.tick);
        for corpse in &new_corpses {
            self.events.push(SimEvent::CorpseCreated {
                tick: self.state.tick,
                agent_id: corpse.agent_id.0,
                partition_id: corpse.partition_id,
                biomass: corpse.biomass,
            });
        }

        // Phase 6: Cleanup decomposed corpses
        let removed_corpses = self.agents.cleanup_decomposed_corpses(self.state.tick);
        for corpse in &removed_corpses {
            self.events.push(SimEvent::CorpseDecomposed {
                tick: self.state.tick,
                agent_id: corpse.agent_id.0,
                partition_id: corpse.partition_id,
                waste_produced: corpse.decomposition_waste(self.state.tick),
            });
        }

        // Phase 6: Update animal populations and generate secondary products
        self.agents.animal_population.update_all();

        // Phase 6: Cleanup dead animals
        self.agents.animal_population.cleanup_dead();

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

        // Phase 6 Pass 2: Probabilistic inference layer (slower cadence)
        if self.agents.inference_engine.should_run(self.state.tick) {
            self.run_inference_phase();
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
            .push(compute_tick_metrics(&self.world, &self.agents, &self.events));
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

    /// Phase 6 Pass 2: Run probabilistic inference for risk assessment
    fn run_inference_phase(&mut self) {
        use polis_core::DeterministicRng;

        let mut rng = DeterministicRng::from_u64(self.seed.0.wrapping_add(self.state.tick));
        let partition_count = self.world.partition_count();

        // Run all risk inference types for each partition
        for partition_id in 0..partition_count {
            // 1. Zoonotic spillover risk
            self.run_zoonotic_risk_inference(partition_id, &mut rng);

            // 2. Trade cheating/default risk
            self.run_trade_cheating_risk_inference(partition_id, &mut rng);

            // 3. Institution enforcement-failure risk
            self.run_enforcement_failure_risk_inference(partition_id, &mut rng);

            // 4. Collective fracture/escalation risk
            self.run_collective_fracture_risk_inference(partition_id, &mut rng);

            // 5. Famine/crisis early-warning risk
            self.run_famine_crisis_risk_inference(partition_id, &mut rng);
        }

        // Clear old beliefs (older than 500 ticks) to prevent unbounded growth
        self.agents.inference_engine.clear_expired_beliefs(self.state.tick);

        // Update last inference tick
        self.agents.inference_engine.last_inference_tick = self.state.tick;
    }

    /// Run zoonotic spillover risk inference for a partition
    fn run_zoonotic_risk_inference(
        &mut self,
        partition_id: u64,
        rng: &mut DeterministicRng,
    ) {
        use polis_agents::inference::RiskType;

        let livestock_density = self.calculate_livestock_density(partition_id);
        let corpse_load = self.calculate_corpse_load(partition_id);
        let sanitation_level = self.calculate_sanitation_level(partition_id);

        let assessment = self.agents.inference_engine.infer_zoonotic_risk(
            partition_id,
            livestock_density,
            corpse_load,
            sanitation_level,
            self.state.tick,
            rng,
        );

        let factors: Vec<(String, f32)> = assessment
            .top_factors
            .iter()
            .map(|(_, name, weight)| (name.clone(), *weight))
            .collect();

        self.events.push(SimEvent::RiskUpdated {
            tick: self.state.tick,
            partition_id: assessment.partition_id,
            risk_type: RiskType::ZoonoticSpillover,
            strength: assessment.truth_value.strength,
            confidence: assessment.truth_value.confidence,
            top_factors: factors.clone(),
        });

        // Check for incident realization
        if assessment.truth_value.is_likely(0.7) {
            let realization_threshold = 0.8;
            if assessment.truth_value.strength > realization_threshold {
                self.events.push(SimEvent::IncidentRealized {
                    tick: self.state.tick,
                    partition_id: assessment.partition_id,
                    risk_type: RiskType::ZoonoticSpillover,
                    severity: ((assessment.truth_value.strength - 0.7) * 333.0) as u8,
                    contributing_factors: factors.iter().map(|(n, _)| n.clone()).collect(),
                });
            }
        }
    }

    /// Run trade cheating/default risk inference for a partition
    fn run_trade_cheating_risk_inference(
        &mut self,
        partition_id: u64,
        rng: &mut DeterministicRng,
    ) {
        use polis_agents::inference::RiskType;

        let scarcity_stress = self.calculate_scarcity_stress(partition_id);
        let trust_level = self.calculate_trust_level(partition_id);
        let enforcement_coverage = self.calculate_enforcement_coverage(partition_id);

        let assessment = self.agents.inference_engine.infer_trade_cheating_risk(
            partition_id,
            scarcity_stress,
            trust_level,
            enforcement_coverage,
            self.state.tick,
            rng,
        );

        let factors: Vec<(String, f32)> = assessment
            .top_factors
            .iter()
            .map(|(_, name, weight)| (name.clone(), *weight))
            .collect();

        self.events.push(SimEvent::RiskUpdated {
            tick: self.state.tick,
            partition_id: assessment.partition_id,
            risk_type: RiskType::TradeCheating,
            strength: assessment.truth_value.strength,
            confidence: assessment.truth_value.confidence,
            top_factors: factors.clone(),
        });

        if assessment.truth_value.is_likely(0.7) {
            let realization_threshold = 0.8;
            if assessment.truth_value.strength > realization_threshold {
                self.events.push(SimEvent::IncidentRealized {
                    tick: self.state.tick,
                    partition_id: assessment.partition_id,
                    risk_type: RiskType::TradeCheating,
                    severity: ((assessment.truth_value.strength - 0.7) * 333.0) as u8,
                    contributing_factors: factors.iter().map(|(n, _)| n.clone()).collect(),
                });
            }
        }
    }

    /// Run enforcement-failure risk inference for a partition
    fn run_enforcement_failure_risk_inference(
        &mut self,
        partition_id: u64,
        rng: &mut DeterministicRng,
    ) {
        use polis_agents::inference::RiskType;

        let factionalism = self.calculate_factionalism(partition_id);
        let legitimacy = self.calculate_legitimacy(partition_id);
        let resource_strain = self.calculate_resource_strain(partition_id);

        let assessment = self.agents.inference_engine.infer_enforcement_failure_risk(
            partition_id,
            factionalism,
            legitimacy,
            resource_strain,
            self.state.tick,
            rng,
        );

        let factors: Vec<(String, f32)> = assessment
            .top_factors
            .iter()
            .map(|(_, name, weight)| (name.clone(), *weight))
            .collect();

        self.events.push(SimEvent::RiskUpdated {
            tick: self.state.tick,
            partition_id: assessment.partition_id,
            risk_type: RiskType::EnforcementFailure,
            strength: assessment.truth_value.strength,
            confidence: assessment.truth_value.confidence,
            top_factors: factors.clone(),
        });

        if assessment.truth_value.is_likely(0.7) {
            let realization_threshold = 0.8;
            if assessment.truth_value.strength > realization_threshold {
                self.events.push(SimEvent::IncidentRealized {
                    tick: self.state.tick,
                    partition_id: assessment.partition_id,
                    risk_type: RiskType::EnforcementFailure,
                    severity: ((assessment.truth_value.strength - 0.7) * 333.0) as u8,
                    contributing_factors: factors.iter().map(|(n, _)| n.clone()).collect(),
                });
            }
        }
    }

    /// Run collective fracture risk inference for a partition
    fn run_collective_fracture_risk_inference(
        &mut self,
        partition_id: u64,
        rng: &mut DeterministicRng,
    ) {
        use polis_agents::inference::RiskType;

        let grievance_level = self.calculate_grievance_level(partition_id);
        let social_tension = self.calculate_social_tension(partition_id);
        let cooperation_rate = self.calculate_cooperation_rate(partition_id);

        let assessment = self.agents.inference_engine.infer_collective_fracture_risk(
            partition_id,
            grievance_level,
            social_tension,
            cooperation_rate,
            self.state.tick,
            rng,
        );

        let factors: Vec<(String, f32)> = assessment
            .top_factors
            .iter()
            .map(|(_, name, weight)| (name.clone(), *weight))
            .collect();

        self.events.push(SimEvent::RiskUpdated {
            tick: self.state.tick,
            partition_id: assessment.partition_id,
            risk_type: RiskType::CollectiveFracture,
            strength: assessment.truth_value.strength,
            confidence: assessment.truth_value.confidence,
            top_factors: factors.clone(),
        });

        if assessment.truth_value.is_likely(0.7) {
            let realization_threshold = 0.8;
            if assessment.truth_value.strength > realization_threshold {
                self.events.push(SimEvent::IncidentRealized {
                    tick: self.state.tick,
                    partition_id: assessment.partition_id,
                    risk_type: RiskType::CollectiveFracture,
                    severity: ((assessment.truth_value.strength - 0.7) * 333.0) as u8,
                    contributing_factors: factors.iter().map(|(n, _)| n.clone()).collect(),
                });
            }
        }
    }

    /// Run famine/crisis risk inference for a partition
    fn run_famine_crisis_risk_inference(
        &mut self,
        partition_id: u64,
        rng: &mut DeterministicRng,
    ) {
        use polis_agents::inference::RiskType;

        let food_scarcity = self.calculate_food_scarcity(partition_id);
        let health_decline = self.calculate_health_decline(partition_id);
        let disease_pressure = self.calculate_disease_pressure(partition_id);

        let assessment = self.agents.inference_engine.infer_famine_crisis_risk(
            partition_id,
            food_scarcity,
            health_decline,
            disease_pressure,
            self.state.tick,
            rng,
        );

        let factors: Vec<(String, f32)> = assessment
            .top_factors
            .iter()
            .map(|(_, name, weight)| (name.clone(), *weight))
            .collect();

        self.events.push(SimEvent::RiskUpdated {
            tick: self.state.tick,
            partition_id: assessment.partition_id,
            risk_type: RiskType::FamineCrisis,
            strength: assessment.truth_value.strength,
            confidence: assessment.truth_value.confidence,
            top_factors: factors.clone(),
        });

        if assessment.truth_value.is_likely(0.7) {
            let realization_threshold = 0.8;
            if assessment.truth_value.strength > realization_threshold {
                self.events.push(SimEvent::IncidentRealized {
                    tick: self.state.tick,
                    partition_id: assessment.partition_id,
                    risk_type: RiskType::FamineCrisis,
                    severity: ((assessment.truth_value.strength - 0.7) * 333.0) as u8,
                    contributing_factors: factors.iter().map(|(n, _)| n.clone()).collect(),
                });
            }
        }
    }

    /// Calculate livestock density for a partition (0.0 to 1.0)
    fn calculate_livestock_density(&self, partition_id: u64) -> f32 {
        let domestic_count = self
            .agents
            .animal_population
            .animals_in_partition(partition_id)
            .filter(|a| a.is_alive && a.is_domesticated)
            .count() as f32;
        // Normalize: assume 10+ animals is high density
        (domestic_count / 10.0).min(1.0)
    }

    /// Calculate corpse load for a partition (0.0 to 1.0)
    fn calculate_corpse_load(&self, partition_id: u64) -> f32 {
        let corpse_count = self
            .agents
            .corpses_in_partition(partition_id)
            .count() as f32;
        // Normalize: assume 5+ corpses is high load
        (corpse_count / 5.0).min(1.0)
    }

    /// Calculate sanitation level for a partition (0.0 to 1.0)
    fn calculate_sanitation_level(&self, partition_id: u64) -> f32 {
        // Simplified: based on waste level in partition
        if let Some(partition) = self.world.partitions().get(partition_id as usize) {
            let waste_ratio = partition.waste.quantity as f32
                / (partition.total_resources() as f32 + 1.0);
            // Higher waste = lower sanitation
            (1.0 - waste_ratio).max(0.0)
        } else {
            0.5 // Default medium sanitation
        }
    }

    /// Calculate scarcity stress for trade cheating risk (0.0 to 1.0)
    fn calculate_scarcity_stress(&self, partition_id: u64) -> f32 {
        // Based on food availability vs population needs
        if let Some(partition) = self.world.partitions().get(partition_id as usize) {
            let agent_count = self.agents.living_in_partition(partition_id) as f32;
            let food_available = partition.food.quantity as f32;
            // Normalize: less than 10 food per agent is stressful
            let stress = 1.0 - (food_available / (agent_count * 10.0 + 1.0)).min(1.0);
            stress.max(0.0).min(1.0)
        } else {
            0.5
        }
    }

    /// Calculate trust level for trade cheating risk (-1.0 to 1.0)
    fn calculate_trust_level(&self, partition_id: u64) -> f32 {
        // Average trust from social network in this partition
        let agents_in_partition: Vec<_> = self.agents
            .agents_in_partition(partition_id)
            .map(|a| a.id)
            .collect();

        if agents_in_partition.is_empty() {
            return 0.0;
        }

        let mut total_trust = 0_i64;
        let mut count = 0_u64;
        for agent_id in &agents_in_partition {
            let mut ties = self.agents.social_network.get_agent_ties(*agent_id);
            ties.sort_unstable_by_key(|(other, _)| other.0);
            for (_, tie) in ties {
                total_trust += tie.trust as i64;
                count += 1;
            }
        }

        if count == 0_u64 {
            0.0
        } else {
            ((total_trust as f32) / (count as f32) / 100.0).clamp(-1.0, 1.0)
        }
    }

    /// Calculate enforcement coverage for trade cheating risk (0.0 to 1.0)
    fn calculate_enforcement_coverage(&self, _partition_id: u64) -> f32 {
        // Based on presence of collectives with enforcement capability
        let active_collectives = self.agents.collective_registry.active_collectives();

        if active_collectives.is_empty() {
            return 0.2; // Low baseline enforcement
        }

        let total_legitimacy: f32 = active_collectives
            .iter()
            .map(|c| c.legitimacy as f32 / 100.0)
            .sum::<f32>();

        (total_legitimacy / active_collectives.len() as f32).min(1.0)
    }

    /// Calculate factionalism for enforcement-failure risk (0.0 to 1.0)
    fn calculate_factionalism(&self, _partition_id: u64) -> f32 {
        let active_collectives = self.agents.collective_registry.active_collectives();

        if active_collectives.is_empty() {
            return 0.0;
        }

        let avg_factionalism: f32 = active_collectives
            .iter()
            .map(|c| c.factionalism as f32 / 100.0)
            .sum::<f32>()
            / active_collectives.len() as f32;

        avg_factionalism.min(1.0)
    }

    /// Calculate legitimacy for enforcement-failure risk (0.0 to 1.0)
    fn calculate_legitimacy(&self, _partition_id: u64) -> f32 {
        let active_collectives = self.agents.collective_registry.active_collectives();

        if active_collectives.is_empty() {
            return 0.5; // Neutral baseline
        }

        let avg_legitimacy: f32 = active_collectives
            .iter()
            .map(|c| c.legitimacy as f32 / 100.0)
            .sum::<f32>()
            / active_collectives.len() as f32;

        avg_legitimacy.min(1.0)
    }

    /// Calculate resource strain for enforcement-failure risk (0.0 to 1.0)
    fn calculate_resource_strain(&self, _partition_id: u64) -> f32 {
        let active_collectives = self.agents.collective_registry.active_collectives();

        if active_collectives.is_empty() {
            return 0.0;
        }

        // Strain = low pooled resources relative to member count
        let mut total_strain = 0.0;
        for collective in &active_collectives {
            let member_count = collective.members.len() as f32;
            let total_resources: u64 = collective.pooled_resources.values().sum();
            let strain = 1.0 - ((total_resources as f32) / (member_count * 10.0 + 1.0)).min(1.0);
            total_strain += strain;
        }

        (total_strain / active_collectives.len() as f32).min(1.0)
    }

    /// Calculate grievance level for collective fracture risk (0.0 to 1.0)
    fn calculate_grievance_level(&self, partition_id: u64) -> f32 {
        let agents_in_partition: Vec<_> = self.agents
            .agents_in_partition(partition_id)
            .map(|a| a.id)
            .collect();

        if agents_in_partition.is_empty() {
            return 0.0;
        }

        let mut total_grievance = 0_u64;
        let mut count = 0_u64;
        for agent_id in &agents_in_partition {
            let mut ties = self.agents.social_network.get_agent_ties(*agent_id);
            ties.sort_unstable_by_key(|(other, _)| other.0);
            for (_, tie) in ties {
                total_grievance += tie.grievance as u64;
                count += 1;
            }
        }

        if count == 0_u64 {
            0.0
        } else {
            ((total_grievance as f32) / (count as f32) / 100.0).min(1.0)
        }
    }

    /// Calculate social tension for collective fracture risk (0.0 to 1.0)
    fn calculate_social_tension(&self, partition_id: u64) -> f32 {
        // Get agents in partition and calculate tension
        let agents_in_partition: Vec<_> = self.agents
            .agents_in_partition(partition_id)
            .map(|a| a.id)
            .collect();

        if agents_in_partition.is_empty() {
            return 0.0;
        }

        let tension = self.agents.social_network.partition_tension(&agents_in_partition);
        tension as f32 / 100.0 // Normalize to 0-1
    }

    /// Calculate cooperation rate for collective fracture risk (0.0 to 1.0)
    fn calculate_cooperation_rate(&self, _partition_id: u64) -> f32 {
        // Based on recent cooperation events in partition
        let recent_events = self.state.tick.saturating_sub(100);
        let cooperation_count = self
            .events
            .iter()
            .filter(|e| {
                matches!(e, SimEvent::CooperationOccurred { tick, .. } if *tick >= recent_events)
            })
            .count() as f32;

        let conflict_count = self
            .events
            .iter()
            .filter(|e| {
                matches!(e, SimEvent::ConflictOccurred { tick, .. } if *tick >= recent_events)
            })
            .count() as f32;

        let total = cooperation_count + conflict_count;
        if total == 0.0 {
            0.5 // Neutral baseline
        } else {
            (cooperation_count / total).min(1.0)
        }
    }

    /// Calculate food scarcity for famine/crisis risk (0.0 to 1.0)
    fn calculate_food_scarcity(&self, partition_id: u64) -> f32 {
        // Inverse of food availability
        if let Some(partition) = self.world.partitions().get(partition_id as usize) {
            let agent_count = self.agents.living_in_partition(partition_id) as f32;
            let food_available = partition.food.quantity as f32;
            // Scarcity = 1 - (food / needs)
            let scarcity = 1.0 - (food_available / (agent_count * 5.0 + 1.0)).min(1.0);
            scarcity.max(0.0).min(1.0)
        } else {
            0.5
        }
    }

    /// Calculate health decline for famine/crisis risk (0.0 to 1.0)
    fn calculate_health_decline(&self, partition_id: u64) -> f32 {
        let agents_in_partition: Vec<_> = self.agents
            .agents_in_partition(partition_id)
            .collect();

        if agents_in_partition.is_empty() {
            return 0.0;
        }

        let avg_health: f32 = agents_in_partition
            .iter()
            .map(|a| a.health as f32 / 100.0)
            .sum::<f32>()
            / agents_in_partition.len() as f32;

        // Decline = inverse of health
        (1.0 - avg_health).min(1.0)
    }

    /// Calculate disease pressure for famine/crisis risk (0.0 to 1.0)
    fn calculate_disease_pressure(&self, partition_id: u64) -> f32 {
        // Combine zoonotic pressure and waste levels
        let zoonotic = self.calculate_livestock_density(partition_id) * 0.5
            + self.calculate_corpse_load(partition_id) * 0.5;

        let waste_pressure = if let Some(partition) = self.world.partitions().get(partition_id as usize) {
            (partition.waste.quantity as f32 / 100.0).min(1.0)
        } else {
            0.0
        };

        (zoonotic * 0.6 + waste_pressure * 0.4).min(1.0)
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

// Phase 6: Helper functions for animal-related metrics

/// Calculate total transport capacity across all partitions
fn calculate_total_transport_capacity(agents: &AgentPopulation, partition_count: u64) -> u64 {
    (0..partition_count)
        .map(|p| agents.animal_population.total_transport_capacity(p) as u64)
        .sum()
}

/// Calculate total traction capacity across all partitions
fn calculate_total_traction_capacity(agents: &AgentPopulation, partition_count: u64) -> u64 {
    (0..partition_count)
        .map(|p| agents.animal_population.total_traction_power(p) as u64)
        .sum()
}

/// Calculate milk production from all animals
fn calculate_milk_production(agents: &AgentPopulation) -> f32 {
    agents
        .animal_population
        .animals()
        .iter()
        .filter(|a| a.is_alive)
        .map(|a| a.effective_milk_production())
        .sum()
}

/// Calculate egg production from all animals
fn calculate_egg_production(agents: &AgentPopulation) -> f32 {
    agents
        .animal_population
        .animals()
        .iter()
        .filter(|a| a.is_alive)
        .map(|a| a.effective_egg_production())
        .sum()
}

/// Calculate wool/fiber production from all animals
fn calculate_wool_production(agents: &AgentPopulation) -> f32 {
    agents
        .animal_population
        .animals()
        .iter()
        .filter(|a| a.is_alive)
        .map(|a| a.effective_fiber_production())
        .sum()
}

/// Calculate manure production from all animals
fn calculate_manure_production(agents: &AgentPopulation) -> f32 {
    // Estimate based on feed consumption (simplified)
    agents
        .animal_population
        .animals()
        .iter()
        .filter(|a| a.is_alive)
        .map(|a| {
            // Estimate feed consumed based on nutrition level
            let feed_consumed = (100 - a.nutrition) as f32 * 0.5;
            a.manure_output(feed_consumed)
        })
        .sum()
}

/// Calculate zoonotic disease pressure from livestock density
fn calculate_zoonotic_pressure(agents: &AgentPopulation, partition_count: u64) -> u64 {
    if partition_count == 0 {
        return 0;
    }

    // Calculate per-partition livestock density and aggregate pressure
    (0..partition_count)
        .map(|partition_id| {
            let livestock_count = agents
                .animal_population
                .animals_in_partition(partition_id)
                .filter(|a| a.is_alive && a.is_domesticated)
                .count() as u64;
            let density_factor = livestock_count * livestock_count; // Quadratic scaling
            let corpse_pressure = agents.corpse_disease_pressure(partition_id) as u64;
            density_factor + corpse_pressure
        })
        .sum::<u64>()
        / partition_count
}

fn compute_tick_metrics(
    world: &WorldState,
    agents: &AgentPopulation,
    events: &[SimEvent],
) -> TickMetrics {
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

    // Phase 6: Discovery metrics derived from emitted discovery events.
    let current_tick = world.tick();
    let discoveries_this_tick = events
        .iter()
        .filter(|e| {
            matches!(
                e,
                SimEvent::DiscoveryStageTransition { tick, new_stage, .. }
                    if *tick == current_tick && *new_stage == DiscoveryStage::Technique
            )
        })
        .count() as u64;

    let mut discovered_pairs: std::collections::BTreeSet<(u64, u64)> =
        std::collections::BTreeSet::new();
    let mut cumulative_stage_sum = 0_u64;
    let mut cumulative_stage_count = 0_u64;
    for event in events {
        if let SimEvent::DiscoveryStageTransition {
            agent_id,
            knowledge_id,
            new_stage,
            ..
        } = event
        {
            if new_stage.as_u8() >= DiscoveryStage::Technique.as_u8() {
                discovered_pairs.insert((*agent_id, *knowledge_id));
            }
            cumulative_stage_sum += new_stage.as_u8() as u64;
            cumulative_stage_count += 1;
        }
    }

    let total_knowledge_items = discovered_pairs.len() as u64;
    let average_discovery_stage = if cumulative_stage_count == 0 {
        0
    } else {
        cumulative_stage_sum / cumulative_stage_count
    };

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
        // Phase 6: Discovery metrics
        discoveries_this_tick,
        total_knowledge_items,
        average_discovery_stage,
        // Phase 6: Animal metrics (calculated from animal population)
        total_domestic_animals: agents.animal_population.living_count() as u64,
        transport_capacity: calculate_total_transport_capacity(agents, partition_count),
        traction_capacity: calculate_total_traction_capacity(agents, partition_count),
        // Phase 6: Secondary product metrics (calculated from animal population)
        milk_produced: calculate_milk_production(agents) as u64,
        eggs_produced: calculate_egg_production(agents) as u64,
        wool_produced: calculate_wool_production(agents) as u64,
        manure_produced: calculate_manure_production(agents) as u64,
        // Phase 6: Disease metrics
        zoonotic_pressure: calculate_zoonotic_pressure(agents, partition_count),
        corpse_count: agents.corpse_count() as u64,
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
