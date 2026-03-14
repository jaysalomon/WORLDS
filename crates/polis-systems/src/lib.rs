pub struct SystemsModule;

use polis_agents::collective::CollectiveId;
use polis_agents::{AgentPopulation, Individual};
use polis_core::DeterministicRng;
use polis_world::{PartitionState, evolve_animal_populations, evolve_fields, regenerate_resources};

impl SystemsModule {
    pub const fn name() -> &'static str {
        "polis-systems"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemPhase {
    Perception,
    Decision,
    Commit,
}

const REGISTERED_PHASES: [SystemPhase; 3] = [
    SystemPhase::Perception,
    SystemPhase::Decision,
    SystemPhase::Commit,
];

pub const fn registered_phases() -> &'static [SystemPhase] {
    &REGISTERED_PHASES
}

pub fn apply_phase_to_partition(
    phase: SystemPhase,
    tick: u64,
    partition_id: u64,
    partition: &mut PartitionState,
) {
    let influence = phase_partition_delta(0xD1CE_CAFE_F00D_BAAD, tick, phase, partition_id);
    match phase {
        SystemPhase::Perception => {
            // Demand increases based on population pressure from food/water
            let pressure =
                ((partition.food.quantity + partition.water.quantity) / 25).max(1) as u64;
            partition.demand = partition
                .demand
                .wrapping_add((influence & 0x0F).wrapping_add(pressure))
                % 10_000;
        }
        SystemPhase::Decision => {
            // Consume resources based on demand
            let spend = (partition.demand / 8).max(1) as i64;

            // Extract food and water
            let food_before = partition.food.quantity;
            let water_before = partition.water.quantity;
            partition.food.quantity = (partition.food.quantity - spend).max(0);
            partition.water.quantity = (partition.water.quantity - spend).max(0);

            // Consumption produces waste byproduct.
            let consumed_food = (food_before - partition.food.quantity).max(0) as u64;
            let consumed_water = (water_before - partition.water.quantity).max(0) as u64;
            let consumed_total = consumed_food + consumed_water;
            if consumed_total > 0 {
                let waste_created = (consumed_total / 3).max(1);
                partition.waste.deposit(waste_created);
            }

            // Cohesion affected by resource availability
            let resource_availability =
                if partition.food.quantity > 0 && partition.water.quantity > 0 {
                    (influence >> 8) & 0x07
                } else {
                    0
                };
            partition.cohesion = partition
                .cohesion
                .wrapping_add(resource_availability)
                .saturating_sub((partition.demand / 20).min(partition.cohesion))
                as u64
                % 10_000;
        }
        SystemPhase::Commit => {
            // Evolve substrate fields and regenerate stocks each commit.
            evolve_fields(partition, tick);
            evolve_animal_populations(partition);
            regenerate_resources(partition);

            // Small deterministic extra recovery to keep early scaffolds active.
            let recovery = 1 + ((influence >> 16) & 0x0F);
            partition.food.quantity = (partition.food.quantity + recovery as i64).min(100000);
            partition.demand = partition.demand.saturating_sub((recovery / 2).max(1));
        }
    }
}

pub fn phase_partition_delta(seed: u64, tick: u64, phase: SystemPhase, partition_id: u64) -> u64 {
    let phase_tag = match phase {
        SystemPhase::Perception => 0xA5A5_A5A5_A5A5_A5A5_u64,
        SystemPhase::Decision => 0xC3C3_C3C3_C3C3_C3C3_u64,
        SystemPhase::Commit => 0xF0F0_F0F0_F0F0_F0F0_u64,
    };

    mix_hash(
        seed ^ phase_tag,
        tick ^ partition_id.rotate_left(3),
        partition_id.wrapping_mul(0x9E37_79B9_7F4A_7C15),
    )
}

fn mix_hash(seed: u64, tick: u64, current: u64) -> u64 {
    let mut x = current ^ seed.rotate_left(13) ^ tick.rotate_right(7);
    x ^= x >> 33;
    x = x.wrapping_mul(0xff51_afd7_ed55_8ccd);
    x ^= x >> 33;
    x = x.wrapping_mul(0xc4ce_b9fe_1a85_ec53);
    x ^= x >> 33;
    x
}

// =============================================================================
// Agent Systems (Phase 3)
// =============================================================================

/// Process agent perception phase
/// Agents assess their environment and decide if they want to move
pub fn agent_perception_phase(
    agents: &mut [Individual],
    partitions: &[PartitionState],
    tick: u64,
    seed: u64,
) {
    let mut rng = DeterministicRng::from_u64(seed ^ tick);

    for agent in agents.iter_mut().filter(|a| a.is_alive) {
        if let Some(partition) = partitions.get(agent.partition_id as usize) {
            // Check if agent wants to move based on needs
            let food_available = partition.food.quantity.max(0) as u64;
            let water_available = partition.water.quantity.max(0) as u64;

            if agent.wants_to_move(food_available, water_available) {
                // Find a neighboring partition with better resources
                let new_partition = find_better_partition(
                    agent.partition_id,
                    partitions,
                    &mut rng,
                    agent.hunger > agent.thirst, // Prioritize food if hungrier
                );

                if new_partition != agent.partition_id {
                    agent.move_to(new_partition);
                }
            }
        }
    }
}

/// Process agent decision phase
/// Agents consume resources and decide on reproduction
/// Returns number of newborns created (caller must handle spawning)
pub fn agent_decision_phase(
    agents: &mut [Individual],
    partitions: &mut [PartitionState],
    _tick: u64,
    _seed: u64,
) -> Vec<(u64, u8)> {
    // First pass: consumption
    for agent in agents.iter_mut().filter(|a| a.is_alive) {
        if let Some(partition) = partitions.get_mut(agent.partition_id as usize) {
            // Consume resources from partition
            let food_available = partition.food.quantity.max(0) as u64;
            let water_available = partition.water.quantity.max(0) as u64;

            let (food_consumed, water_consumed) = agent.consume(food_available, water_available);

            // Deduct from partition
            partition.food.quantity = partition.food.quantity.saturating_sub(food_consumed as i64);
            partition.water.quantity = partition
                .water
                .quantity
                .saturating_sub(water_consumed as i64);

            // Consumption produces waste
            let total_consumed = food_consumed + water_consumed;
            if total_consumed > 0 {
                let waste_created = (total_consumed / 4).max(1);
                partition.waste.deposit(waste_created);
            }
        }
    }

    // Second pass: reproduction - collect newborns
    let mut newborns: Vec<(u64, u8)> = Vec::new();
    let mut local_counts: std::collections::HashMap<u64, usize> = agents
        .iter()
        .filter(|a| a.is_alive)
        .map(|a| a.partition_id)
        .fold(std::collections::HashMap::new(), |mut acc, pid| {
            *acc.entry(pid).or_insert(0) += 1;
            acc
        });

    for agent in agents.iter_mut().filter(|a| a.is_alive) {
        if agent.can_reproduce() {
            // Check if there's space and resources
            if let Some(partition) = partitions.get(agent.partition_id as usize) {
                let local_population = local_counts.get(&agent.partition_id).copied().unwrap_or(0);
                let carrying_capacity = 50; // Per partition agent limit

                if local_population < carrying_capacity
                    && partition.food.quantity > 200
                    && partition.water.quantity > 200
                {
                    // Reproduction successful - reset parent cooldown
                    agent.reproduce();
                    newborns.push((agent.partition_id, agent.metabolism));
                    // Reserve capacity for this newborn in the current tick.
                    *local_counts.entry(agent.partition_id).or_insert(0) += 1;
                }
            }
        }
    }

    newborns
}

/// Process agent commit phase
/// Update agent needs and check mortality
pub fn agent_commit_phase(agents: &mut [Individual], partitions: &mut [PartitionState]) {
    for agent in agents.iter_mut().filter(|a| a.is_alive) {
        // Update needs (hunger, thirst, age)
        agent.update_needs();

        // Update partition demand based on agent population
        if let Some(partition) = partitions.get_mut(agent.partition_id as usize) {
            // Each living agent contributes to demand
            partition.demand = partition.demand.saturating_add(1).min(10_000);
        }
    }
}

/// Find a better partition for an agent to move to
fn find_better_partition(
    current_partition_id: u64,
    partitions: &[PartitionState],
    _rng: &mut DeterministicRng,
    prioritize_food: bool,
) -> u64 {
    let n = partitions.len() as u64;
    if n <= 1 {
        return current_partition_id;
    }

    // Check neighbors (simplified 1D ring topology)
    let prev = (current_partition_id + n - 1) % n;
    let next = (current_partition_id + 1) % n;

    let current = &partitions[current_partition_id as usize];
    let prev_partition = &partitions[prev as usize];
    let next_partition = &partitions[next as usize];

    // Score partitions based on resources
    let score = |p: &PartitionState| -> i64 {
        let food_score = p.food.quantity.max(0);
        let water_score = p.water.quantity.max(0);
        if prioritize_food {
            food_score * 2 + water_score
        } else {
            food_score + water_score * 2
        }
    };

    let current_score = score(current);
    let prev_score = score(prev_partition);
    let next_score = score(next_partition);

    // Move to best option
    if prev_score > current_score && prev_score >= next_score {
        prev
    } else if next_score > current_score {
        next
    } else {
        current_partition_id
    }
}

/// Cleanup dead agents from population
pub fn cleanup_dead_agents(population: &mut AgentPopulation) {
    population.cleanup_dead();
}

// =============================================================================
// Social Fabric Systems (Phase 4)
// =============================================================================

/// Process social interaction phase
/// Agents in the same partition may interact, building trust or conflict
pub fn social_interaction_phase(
    agents: &[Individual],
    partitions: &[PartitionState],
    social_network: &mut polis_agents::social::SocialNetwork,
    tick: u64,
    seed: u64,
) -> Vec<SocialEvent> {
    let mut rng = DeterministicRng::from_u64(seed ^ tick);
    let mut events = Vec::new();

    // Group agents by partition
    let mut partition_agents: std::collections::HashMap<u64, Vec<usize>> =
        std::collections::HashMap::new();
    for (idx, agent) in agents.iter().enumerate() {
        if agent.is_alive {
            partition_agents
                .entry(agent.partition_id)
                .or_default()
                .push(idx);
        }
    }

    // Sort agent indices within each partition for determinism
    for indices in partition_agents.values_mut() {
        indices.sort();
    }

    // Process interactions within each partition (sorted for determinism)
    let mut partition_ids: Vec<u64> = partition_agents.keys().copied().collect();
    partition_ids.sort();
    for partition_id in partition_ids {
        let agent_indices = partition_agents.get(&partition_id).unwrap();
        if agent_indices.len() < 2 {
            continue;
        }

        let partition = match partitions.get(partition_id as usize) {
            Some(p) => p,
            None => continue,
        };

        // Calculate scarcity stress for this partition
        let scarcity_stress = calculate_scarcity_stress(partition);

        // Randomly select pairs for interaction (not everyone interacts every tick)
        let num_interactions = (agent_indices.len() / 2).max(1);
        for _ in 0..num_interactions {
            if agent_indices.len() < 2 {
                break;
            }

            let idx_a = rng.next_bounded(agent_indices.len() as u64) as usize;
            let agent_a_idx = agent_indices[idx_a];
            let agent_a = &agents[agent_a_idx];

            // Find a different agent
            let mut idx_b = rng.next_bounded(agent_indices.len() as u64) as usize;
            let mut attempts = 0;
            while idx_b == idx_a && attempts < 10 {
                idx_b = rng.next_bounded(agent_indices.len() as u64) as usize;
                attempts += 1;
            }
            if idx_b == idx_a {
                continue;
            }
            let agent_b_idx = agent_indices[idx_b];

            // Get current tie state
            let tie = social_network.get_or_create_tie(agent_a.id, agents[agent_b_idx].id, tick);
            let current_trust = tie.trust;
            let current_grievance = tie.grievance;

            // Determine interaction outcome
            let interaction =
                determine_interaction(current_trust, current_grievance, scarcity_stress, &mut rng);

            match interaction {
                InteractionOutcome::Cooperation => {
                    social_network.record_cooperation(agent_a.id, agents[agent_b_idx].id, tick);
                    if let Some(tie) = social_network.get_tie(agent_a.id, agents[agent_b_idx].id) {
                        events.push(SocialEvent::TrustShifted {
                            agent_a: agent_a.id.0,
                            agent_b: agents[agent_b_idx].id.0,
                            new_trust: tie.trust,
                            reason: TrustShiftReason::Cooperation,
                        });
                    }
                    events.push(SocialEvent::Cooperation {
                        agent_a: agent_a.id.0,
                        agent_b: agents[agent_b_idx].id.0,
                        kind: CooperationKind::ResourceSharing,
                    });
                }
                InteractionOutcome::Conflict { severity } => {
                    social_network.record_conflict(
                        agent_a.id,
                        agents[agent_b_idx].id,
                        severity,
                        tick,
                    );
                    if let Some(tie) = social_network.get_tie(agent_a.id, agents[agent_b_idx].id) {
                        events.push(SocialEvent::TrustShifted {
                            agent_a: agent_a.id.0,
                            agent_b: agents[agent_b_idx].id.0,
                            new_trust: tie.trust,
                            reason: TrustShiftReason::Conflict,
                        });
                    }
                    events.push(SocialEvent::Conflict {
                        agent_a: agent_a.id.0,
                        agent_b: agents[agent_b_idx].id.0,
                        severity,
                        reason: ConflictReason::ResourceScarcity,
                    });
                }
                InteractionOutcome::Neutral => {
                    social_network.record_neutral(agent_a.id, agents[agent_b_idx].id, tick);
                }
            }
        }
    }

    // Apply time decay to all ties
    social_network.apply_decay(tick);

    events
}

/// Calculate scarcity stress for a partition (0-100)
fn calculate_scarcity_stress(partition: &PartitionState) -> u8 {
    let food_ratio = (partition.food.quantity.max(0) as f64)
        / (partition.carrying_capacity_food as f64).max(1.0);
    let water_ratio = (partition.water.quantity.max(0) as f64)
        / (partition.carrying_capacity_water as f64).max(1.0);

    // Higher stress when resources are low
    let avg_ratio = (food_ratio + water_ratio) / 2.0;
    let stress = ((1.0 - avg_ratio.min(1.0)) * 100.0) as u8;
    stress.min(100)
}

/// Possible interaction outcomes
enum InteractionOutcome {
    Cooperation,
    Conflict { severity: u8 },
    Neutral,
}

/// Determine what happens in an interaction
fn determine_interaction(
    trust: i8,
    grievance: u8,
    scarcity_stress: u8,
    rng: &mut DeterministicRng,
) -> InteractionOutcome {
    // Base probabilities modified by trust and grievance
    let cooperation_threshold = 30 + (trust as i64 * 50 / 100); // 30% at neutral, higher with trust
    let conflict_threshold =
        20 + (grievance as i64 * 40 / 100) + (scarcity_stress as i64 * 30 / 100);

    let roll = rng.next_bounded(100) as i64;

    if roll < cooperation_threshold {
        InteractionOutcome::Cooperation
    } else if roll < cooperation_threshold.saturating_add(conflict_threshold) {
        let severity = ((grievance as u16 + scarcity_stress as u16) / 2).max(1) as u8;
        InteractionOutcome::Conflict { severity }
    } else {
        InteractionOutcome::Neutral
    }
}

/// Events generated by social interactions
#[derive(Debug, Clone)]
pub enum SocialEvent {
    TrustShifted {
        agent_a: u64,
        agent_b: u64,
        new_trust: i8,
        reason: TrustShiftReason,
    },
    Cooperation {
        agent_a: u64,
        agent_b: u64,
        kind: CooperationKind,
    },
    Conflict {
        agent_a: u64,
        agent_b: u64,
        severity: u8,
        reason: ConflictReason,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum TrustShiftReason {
    Cooperation,
    Conflict,
    TimeDecay,
}

#[derive(Debug, Clone, Copy)]
pub enum CooperationKind {
    ResourceSharing,
    MutualAid,
    Information,
}

#[derive(Debug, Clone, Copy)]
pub enum ConflictReason {
    ResourceScarcity,
    Grievance,
    Territorial,
}

// =============================================================================
// Cross-Species Interaction Systems (Phase 4)
// =============================================================================

/// Process human-animal interactions
/// Updates cross-species state based on agent proximity and actions
pub fn cross_species_interaction_phase(
    agents: &[Individual],
    partitions: &mut [PartitionState],
    tick: u64,
    seed: u64,
) -> Vec<CrossSpeciesEvent> {
    let mut rng = DeterministicRng::from_u64(seed ^ tick);
    let mut events = Vec::new();

    // Group agents by partition
    let mut partition_agent_counts: std::collections::HashMap<u64, usize> =
        std::collections::HashMap::new();
    for agent in agents.iter().filter(|a| a.is_alive) {
        *partition_agent_counts
            .entry(agent.partition_id)
            .or_insert(0) += 1;
    }

    // Process each partition with both agents and animals in deterministic order.
    let mut partition_ids: Vec<u64> = partition_agent_counts.keys().copied().collect();
    partition_ids.sort_unstable();
    for partition_id in partition_ids {
        if partition_id >= partitions.len() as u64 {
            continue;
        }
        let partition = &mut partitions[partition_id as usize];

        // Skip if no animals present
        let total_animals = partition.herbivore_population
            + partition.predator_population
            + partition.proto_domestic_population;
        if total_animals == 0 {
            continue;
        }

        // Determine contact type based on agent behavior
        // (simplified: more agents = more chance of various contact types)
        let agent_count = partition_agent_counts
            .get(&partition_id)
            .copied()
            .unwrap_or(0);
        let activity_bias = (agent_count as u64).min(20);
        let contact_roll = (rng.next_bounded(100) + activity_bias) % 100;
        let (contact_type, severity) = if contact_roll < 10 {
            // Hunting attempt
            (HumanAnimalContactType::Hunting, -15)
        } else if contact_roll < 30 {
            // Feeding/provisioning
            (HumanAnimalContactType::Feeding, 10)
        } else if contact_roll < 60 {
            // Just proximity
            (HumanAnimalContactType::Proximity, 2)
        } else {
            // Handling/capture attempt
            (HumanAnimalContactType::Handling, -5)
        };

        // Update cross-species state
        update_cross_species_state(partition, severity, 20, tick);

        // Determine outcome
        let outcome = if severity > 0 {
            HumanAnimalOutcome::Positive
        } else if severity < -10 {
            HumanAnimalOutcome::Negative
        } else {
            HumanAnimalOutcome::Neutral
        };

        // Track interactions
        match outcome {
            HumanAnimalOutcome::Positive => {
                partition.positive_human_animal_interactions += 1;
            }
            HumanAnimalOutcome::Negative => {
                partition.negative_human_animal_interactions += 1;
            }
            _ => {}
        }

        events.push(CrossSpeciesEvent {
            partition_id,
            contact_type,
            outcome,
        });
    }

    events
}

/// Update cross-species state for a partition
fn update_cross_species_state(
    partition: &mut PartitionState,
    contact_severity: i8,
    proximity: u8,
    _tick: u64,
) {
    // Update all cross-species metrics based on contact
    // This is a simplified model - in reality, individual animals would have states

    if contact_severity < 0 {
        // Negative contact
        let severity = (-contact_severity) as u8;
        partition.animal_fear = partition.animal_fear.saturating_add(severity * 2);
        partition.animal_aggression = partition.animal_aggression.saturating_add(severity);
        partition.animal_human_tolerance =
            partition.animal_human_tolerance.saturating_sub(severity);
    } else {
        // Positive contact
        let benefit = contact_severity as u8;
        if proximity <= partition.animal_human_tolerance {
            partition.animal_human_tolerance =
                (partition.animal_human_tolerance + benefit).min(100);
            partition.animal_fear = partition.animal_fear.saturating_sub(benefit * 2);
        }
        partition.animal_aggression = partition.animal_aggression.saturating_sub(benefit);
    }

    // Always increase familiarity with contact
    let familiarity_increase = if contact_severity < 0 {
        ((-contact_severity) / 2) as u8
    } else {
        (contact_severity as u8) / 2
    };
    partition.animal_familiarity = (partition.animal_familiarity + familiarity_increase).min(100);

    // Bounds enforcement
    partition.animal_fear = partition.animal_fear.min(100);
    partition.animal_aggression = partition.animal_aggression.min(100);
}

/// Events generated by cross-species interactions
#[derive(Debug, Clone)]
pub struct CrossSpeciesEvent {
    pub partition_id: u64,
    pub contact_type: HumanAnimalContactType,
    pub outcome: HumanAnimalOutcome,
}

#[derive(Debug, Clone, Copy)]
pub enum HumanAnimalContactType {
    Hunting,
    Feeding,
    Proximity,
    Handling,
}

#[derive(Debug, Clone, Copy)]
pub enum HumanAnimalOutcome {
    Positive,
    Negative,
    Neutral,
}

// =============================================================================
// Collective Agency Systems (Phase 5)
// =============================================================================

/// Process collective lifecycle phase
/// Updates lifecycle states, handles merge/split detection, applies downward causation
/// Returns events for lifecycle transitions
pub fn collective_lifecycle_phase(
    agents: &mut [Individual],
    collective_registry: &mut polis_agents::collective::CollectiveRegistry,
    tick: u64,
    seed: u64,
) -> Vec<CollectiveEvent> {
    let mut rng = DeterministicRng::from_u64(seed ^ tick);
    let mut events = Vec::new();

    // Update lifecycle states for all collectives
    let transitions = collective_registry.update_lifecycle_states(tick);
    for (collective_id, old_state) in transitions {
        if let Some(collective) = collective_registry.get(collective_id) {
            events.push(CollectiveEvent::LifecycleTransition {
                collective_id: collective_id.0,
                old_state: lifecycle_to_string(old_state),
                new_state: lifecycle_to_string(collective.lifecycle_state),
            });
        }
    }

    // Apply downward causation for each collective
    // This modifies agent contexts through constraints, NOT direct overwriting
    let collective_ids: Vec<_> = collective_registry
        .active_collectives()
        .iter()
        .map(|c| c.id)
        .collect();

    for collective_id in collective_ids {
        if let Some(collective) = collective_registry.get(collective_id) {
            for agent in agents.iter_mut().filter(|a| a.is_alive) {
                if collective.is_member(agent.id) {
                    collective.apply_downward_causation(agent, &mut rng);
                }
            }
        }
    }

    // Check for potential merges between compatible collectives
    check_potential_merges(collective_registry, tick, &mut events, &mut rng);

    // Check for potential splits in fragmenting collectives
    check_potential_splits(collective_registry, tick, &mut events, &mut rng);

    events
}

/// Check for potential merges between compatible collectives
fn check_potential_merges(
    collective_registry: &mut polis_agents::collective::CollectiveRegistry,
    tick: u64,
    events: &mut Vec<CollectiveEvent>,
    _rng: &mut DeterministicRng,
) {
    // Get active collectives
    let active: Vec<_> = collective_registry.active_collectives();
    if active.len() < 2 {
        return;
    }

    // Check pairs for merge compatibility
    // Use indices to avoid borrow issues
    let mut merges_to_perform: Vec<(CollectiveId, CollectiveId)> = Vec::new();

    for i in 0..active.len() {
        for j in (i + 1)..active.len() {
            let c1 = &active[i];
            let c2 = &active[j];

            // Check merge criteria
            let merge_criteria = c1.can_merge_with(c2);

            // Merge if all criteria are strong enough (threshold: 60/100)
            if merge_criteria.compatible_institutions >= 60
                && merge_criteria.coordination_benefit >= 60
                && merge_criteria.manageable_factional_distance >= 60
                && merge_criteria.asset_integration_possible >= 60
            {
                merges_to_perform.push((c1.id, c2.id));
            }
        }
    }

    // Perform merges
    for (primary_id, secondary_id) in merges_to_perform {
        if let Some(merged_id) =
            collective_registry.merge_collectives(primary_id, secondary_id, tick)
        {
            events.push(CollectiveEvent::Merged {
                primary_id: primary_id.0,
                secondary_id: secondary_id.0,
                merged_id: merged_id.0,
            });
        }
    }
}

/// Check for potential splits in fragmenting collectives
fn check_potential_splits(
    collective_registry: &mut polis_agents::collective::CollectiveRegistry,
    tick: u64,
    events: &mut Vec<CollectiveEvent>,
    rng: &mut DeterministicRng,
) {
    // Get collectives that might split
    let candidates: Vec<_> = collective_registry
        .active_collectives()
        .iter()
        .filter(|c| {
            c.lifecycle_state
                == polis_agents::collective::CollectiveLifecycleState::FragmentingCollective
                || c.factionalism > 70
        })
        .map(|c| c.id)
        .collect();

    for collective_id in candidates {
        if let Some(collective) = collective_registry.get(collective_id) {
            let split_criteria = collective.should_split();

            // Split if all criteria are strong enough
            if split_criteria.identifiable_subgroups >= 60
                && split_criteria.independent_cohesion >= 60
                && split_criteria.severe_disagreement >= 60
                && split_criteria.independent_action_path >= 60
            {
                // Identify a subgroup to split off
                // For simplicity, split based on factions or random half
                let members: Vec<_> = collective.members.keys().copied().collect();
                if members.len() >= 6 {
                    let split_point = members.len() / 2;
                    let subgroup = members[..split_point].to_vec();

                    if let Some(new_id) =
                        collective_registry.split_collective(collective_id, subgroup, tick)
                    {
                        events.push(CollectiveEvent::Split {
                            original_id: collective_id.0,
                            new_id: new_id.0,
                        });
                    }
                }
            }
        }
    }
}

/// Convert lifecycle state to string for events
fn lifecycle_to_string(state: polis_agents::collective::CollectiveLifecycleState) -> String {
    use polis_agents::collective::CollectiveLifecycleState::*;
    match state {
        EphemeralCoordination => "ephemeral_coordination".to_string(),
        ProtoGroup => "proto_group".to_string(),
        UnstableCollective => "unstable_collective".to_string(),
        StabilizedCollective => "stabilized_collective".to_string(),
        FragmentingCollective => "fragmenting_collective".to_string(),
        Dissolved => "dissolved".to_string(),
    }
}

/// Events generated by collective systems
#[derive(Debug, Clone)]
pub enum CollectiveEvent {
    LifecycleTransition {
        collective_id: u64,
        old_state: String,
        new_state: String,
    },
    Merged {
        primary_id: u64,
        secondary_id: u64,
        merged_id: u64,
    },
    Split {
        original_id: u64,
        new_id: u64,
    },
}

#[cfg(test)]
mod tests {
    use super::{SystemPhase, apply_phase_to_partition, phase_partition_delta, registered_phases};
    use polis_world::PartitionState;

    #[test]
    fn registry_order_is_stable() {
        assert_eq!(
            registered_phases(),
            &[
                SystemPhase::Perception,
                SystemPhase::Decision,
                SystemPhase::Commit
            ]
        );
    }

    #[test]
    fn partition_delta_is_deterministic() {
        let a = phase_partition_delta(42, 99, SystemPhase::Decision, 7);
        let b = phase_partition_delta(42, 99, SystemPhase::Decision, 7);
        assert_eq!(a, b);
    }

    #[test]
    fn phase_application_is_deterministic() {
        let mut a = PartitionState::from_seed(42, 3);
        let mut b = PartitionState::from_seed(42, 3);
        apply_phase_to_partition(SystemPhase::Perception, 12, 3, &mut a);
        apply_phase_to_partition(SystemPhase::Perception, 12, 3, &mut b);
        assert_eq!(a, b);
    }

    #[test]
    fn decision_phase_produces_waste() {
        let mut partition = PartitionState::from_seed(42, 1);
        let before = partition.waste.quantity;
        apply_phase_to_partition(SystemPhase::Decision, 10, 1, &mut partition);
        assert!(partition.waste.quantity >= before);
    }
}
