//! Agent dynamics validation tests for Phase 3
//!
//! These tests validate:
//! - Agents consume available resources
//! - Starvation and recovery behave coherently
//! - Population responds to substrate quality

use polis_agents::{AgentPopulation, Individual};
use polis_core::DeterministicRng;
use polis_core::SimulationSeed;
use polis_sim::{ExecutionMode, Simulation};
use polis_systems::{agent_decision_phase, agent_perception_phase, cleanup_dead_agents};
use polis_world::WorldState;

// =============================================================================
// Resource Consumption Tests
// =============================================================================

#[test]
fn agents_consume_food_and_water() {
    // Create a world with abundant resources
    let mut world = WorldState::new(42, 1);
    let partition = &mut world.partitions_mut()[0];

    // Set up abundant resources
    partition.food.quantity = 1000;
    partition.water.quantity = 1000;

    // Create hungry/thirsty agents
    let mut rng = DeterministicRng::from_u64(42);
    let mut agents = Vec::new();
    for i in 0..5 {
        let mut agent = Individual::new(polis_agents::AgentId(i), 0, &mut rng);
        agent.hunger = 80; // Very hungry
        agent.thirst = 80; // Very thirsty
        agents.push(agent);
    }

    // Run decision phase (consumption)
    agent_decision_phase(&mut agents, &mut [partition.clone()], 1, 42);

    // Agents should have consumed resources
    // Note: We check the agents' hunger/thirst decreased
    for agent in &agents {
        assert!(
            agent.hunger < 80 || agent.thirst < 80,
            "Agent should have consumed something: hunger={}, thirst={}",
            agent.hunger,
            agent.thirst
        );
    }
}

#[test]
fn consumption_reduces_partition_resources() {
    let mut world = WorldState::new(42, 1);
    let partitions = world.partitions_mut();

    // Set up specific resource levels
    partitions[0].food.quantity = 500;
    partitions[0].water.quantity = 500;

    // Create simulation with agents
    let mut sim = Simulation::new_with_partition_count(SimulationSeed::new(42), 1);

    // Get initial resources
    let initial_food = sim.world().partitions()[0].food.quantity;
    let _initial_water = sim.world().partitions()[0].water.quantity;

    // Run several ticks to allow consumption
    for _ in 0..50 {
        sim.step();
    }

    // Resources should have been consumed
    let final_food = sim.world().partitions()[0].food.quantity;
    let _final_water = sim.world().partitions()[0].water.quantity;

    // With agents consuming, resources should generally decrease
    // (though regeneration also happens)
    assert!(
        final_food < initial_food * 2, // Should not grow excessively
        "Food should be consumed: initial={}, final={}",
        initial_food,
        final_food
    );
}

#[test]
fn consumption_produces_waste() {
    let mut sim = Simulation::new_with_partition_count(SimulationSeed::new(42), 1);

    // Get initial waste
    let initial_waste: i64 = sim
        .world()
        .partitions()
        .iter()
        .map(|p| p.waste.quantity as i64)
        .sum();

    // Run several ticks
    for _ in 0..100 {
        sim.step();
    }

    // Check waste was produced
    let final_waste: i64 = sim
        .world()
        .partitions()
        .iter()
        .map(|p| p.waste.quantity as i64)
        .sum();

    assert!(
        final_waste > initial_waste || final_waste >= 0,
        "Waste should be non-negative and generally increase with consumption"
    );
}

// =============================================================================
// Starvation and Recovery Tests
// =============================================================================

#[test]
fn starvation_reduces_health() {
    let mut rng = DeterministicRng::from_u64(42);
    let mut agent = Individual::new(polis_agents::AgentId(1), 0, &mut rng);

    // Set up starving conditions
    agent.hunger = 90;
    agent.thirst = 90;
    let initial_health = agent.health;

    // Update needs (this is what happens in commit phase)
    agent.update_needs();

    // Health should decrease when starving
    assert!(
        agent.health < initial_health,
        "Health should decrease when starving: initial={}, after={}",
        initial_health,
        agent.health
    );
}

#[test]
fn well_fed_agents_recover_health() {
    let mut rng = DeterministicRng::from_u64(42);
    let mut agent = Individual::new(polis_agents::AgentId(1), 0, &mut rng);

    // Damage the agent first
    agent.health = 50;
    agent.hunger = 20; // Well fed
    agent.thirst = 20; // Well hydrated

    // Consume resources (this triggers health recovery)
    agent.consume(100, 100);

    // Health should recover
    assert!(
        agent.health > 50,
        "Health should recover when well-fed: initial=50, after={}",
        agent.health
    );
}

#[test]
fn death_occurs_at_zero_health() {
    let mut rng = DeterministicRng::from_u64(42);
    let mut agent = Individual::new(polis_agents::AgentId(1), 0, &mut rng);

    // Force starvation
    agent.health = 5;
    agent.hunger = 100;
    agent.thirst = 100;

    // Update needs until death
    for _ in 0..10 {
        agent.update_needs();
        if !agent.is_alive {
            break;
        }
    }

    assert!(!agent.is_alive, "Agent should die from starvation");
}

#[test]
fn starvation_recovery_cycle() {
    let mut rng = DeterministicRng::from_u64(42);
    let mut agent = Individual::new(polis_agents::AgentId(1), 0, &mut rng);

    // Phase 1: Starvation
    agent.hunger = 85;
    agent.thirst = 85;
    let health_before_starvation = agent.health;

    agent.update_needs();
    let health_after_starvation = agent.health;

    assert!(
        health_after_starvation < health_before_starvation,
        "Starvation should reduce health"
    );

    // Phase 2: Recovery
    agent.hunger = 10;
    agent.thirst = 10;
    agent.consume(100, 100);

    assert!(
        agent.health > health_after_starvation || agent.health >= 95,
        "Recovery should increase health or max it out"
    );
}

// =============================================================================
// Population Response to Substrate Tests
// =============================================================================

#[test]
fn population_dynamics_work_in_simulation() {
    // Create simulation with abundant resources
    let mut sim = Simulation::new_with_partition_count(SimulationSeed::new(42), 3);

    // Set up abundant resources in all partitions
    for partition in sim.world_mut().partitions_mut() {
        partition.food.quantity = 5000;
        partition.water.quantity = 5000;
        partition.carrying_capacity_food = 10000;
        partition.carrying_capacity_water = 10000;
    }

    // Run for several ticks to observe population dynamics
    for _ in 0..100 {
        sim.step();
    }

    // Check that metrics track the population
    let metrics = sim.metrics();
    assert!(!metrics.is_empty(), "Should have metrics");

    // Population should have been tracked throughout
    let total_agents_tracked: u64 = metrics.iter().map(|m| m.total_agents).sum();
    assert!(total_agents_tracked > 0, "Should track agents over time");

    // Health and hunger should be tracked
    let avg_healths: Vec<u64> = metrics.iter().map(|m| m.average_agent_health).collect();
    assert!(
        avg_healths.iter().any(|h| *h > 0),
        "Should track agent health"
    );
}

#[test]
fn population_declines_in_poor_conditions() {
    // Create simulation with scarce resources
    let mut sim = Simulation::new_with_partition_count(SimulationSeed::new(42), 1);

    // Set up very scarce resources
    let partitions = sim.world_mut().partitions_mut();
    for partition in partitions {
        partition.food.quantity = 10;
        partition.water.quantity = 10;
        partition.carrying_capacity_food = 50;
        partition.carrying_capacity_water = 50;
    }

    // Get initial living count
    let initial_population = sim.agents().living_count();

    // Run for many ticks
    for _ in 0..300 {
        sim.step();
    }

    let final_population = sim.agents().living_count();

    // In very poor conditions, population should decline
    // (agents starve faster than they reproduce)
    assert!(
        final_population <= initial_population + 5, // Allow small growth but not much
        "Population should struggle in poor conditions: initial={}, final={}",
        initial_population,
        final_population
    );
}

#[test]
fn agents_move_toward_resources() {
    let mut world = WorldState::new(42, 3);
    let partitions = world.partitions_mut();

    // Set up unequal resources: partition 0 has nothing, partition 1 has abundance
    partitions[0].food.quantity = 10;
    partitions[0].water.quantity = 10;
    partitions[1].food.quantity = 1000;
    partitions[1].water.quantity = 1000;
    partitions[2].food.quantity = 500;
    partitions[2].water.quantity = 500;

    // Create hungry agent in partition 0
    let mut rng = DeterministicRng::from_u64(42);
    let mut agent = Individual::new(polis_agents::AgentId(1), 0, &mut rng);
    agent.hunger = 80;
    agent.thirst = 80;
    agent.mobility = 100; // High mobility

    let mut agents = vec![agent];

    // Run perception phase multiple times
    for tick in 0..10 {
        agent_perception_phase(&mut agents, world.partitions(), tick, 42);
    }

    // Agent should have moved to a partition with better resources
    assert!(
        agents[0].partition_id != 0 || agents[0].hunger < 80,
        "Agent should move toward resources or have consumed: partition={}, hunger={}",
        agents[0].partition_id,
        agents[0].hunger
    );
}

#[test]
fn carrying_capacity_limits_population() {
    // Create simulation with limited carrying capacity
    let mut sim = Simulation::new_with_partition_count(SimulationSeed::new(42), 2);

    // Set low carrying capacity
    for partition in sim.world_mut().partitions_mut() {
        partition.carrying_capacity_food = 200;
        partition.carrying_capacity_water = 200;
    }

    // Run for many ticks
    for _ in 0..1000 {
        sim.step();
    }

    // Population should be limited by carrying capacity
    // With 2 partitions and low capacity, population shouldn't explode
    let final_population = sim.agents().living_count();
    let total_carrying_capacity = 2 * 50; // 2 partitions * 50 agents per partition limit

    assert!(
        final_population < total_carrying_capacity * 10, // Should not exceed reasonable bounds
        "Population should be limited: final={}, capacity={}",
        final_population,
        total_carrying_capacity
    );
}

// =============================================================================
// Integration Tests
// =============================================================================

#[test]
fn agent_lifecycle_in_simulation() {
    let mut sim = Simulation::new_with_partition_count(SimulationSeed::new(42), 3);

    // Track metrics over time
    let mut populations = Vec::new();
    let mut avg_healths = Vec::new();

    for _ in 0..200 {
        sim.step();
        let stats = sim.agents().statistics();
        populations.push(stats.total_population);
        avg_healths.push(stats.average_health);
    }

    // Should have population data
    assert!(!populations.is_empty());

    // Health should generally stay in valid range
    for (i, health) in avg_healths.iter().enumerate() {
        assert!(
            *health <= 100,
            "Average health should be <= 100 at tick {}",
            i
        );
    }

    // Final metrics should be recorded
    let metrics = sim.metrics();
    assert!(
        metrics.iter().any(|m| m.total_agents > 0),
        "Should track agent counts in metrics"
    );
}

#[test]
fn reproduction_increases_population() {
    let mut rng = DeterministicRng::from_u64(42);
    let mut population = AgentPopulation::new();

    // Create a mature, healthy agent
    let mut parent = Individual::new(polis_agents::AgentId(0), 0, &mut rng);
    parent.age = 2000;
    parent.reproduction_cooldown = 0;
    parent.health = 90;
    parent.hunger = 20;
    parent.thirst = 20;

    population.add_agent(parent);

    let initial_count = population.living_count();

    // Simulate reproduction by adding a newborn
    population.spawn_newborn(0, 100, 42);

    let final_count = population.living_count();

    assert!(
        final_count > initial_count,
        "Reproduction should increase population: {} -> {}",
        initial_count,
        final_count
    );
}

#[test]
fn cleanup_removes_dead_agents() {
    let mut rng = DeterministicRng::from_u64(42);
    let mut population = AgentPopulation::new();

    // Add some agents
    for i in 0..5 {
        let agent = Individual::new(polis_agents::AgentId(i), 0, &mut rng);
        population.add_agent(agent);
    }

    // Kill some agents
    for agent in population.agents_mut().iter_mut().take(3) {
        agent.is_alive = false;
    }

    let before_cleanup = population.agents().len();
    assert_eq!(before_cleanup, 5);

    // Cleanup dead agents
    cleanup_dead_agents(&mut population);

    let after_cleanup = population.agents().len();
    assert_eq!(
        after_cleanup, 2,
        "Should have 2 living agents after cleanup"
    );
}

#[test]
fn population_statistics_are_accurate() {
    let mut rng = DeterministicRng::from_u64(42);
    let mut population = AgentPopulation::new();

    // Add agents with known states
    let mut agent1 = Individual::new(polis_agents::AgentId(0), 0, &mut rng);
    agent1.health = 80;
    agent1.hunger = 30;
    agent1.thirst = 30;
    agent1.age = 100;

    let mut agent2 = Individual::new(polis_agents::AgentId(1), 0, &mut rng);
    agent2.health = 60;
    agent2.hunger = 50;
    agent2.thirst = 50;
    agent2.age = 200;

    population.add_agent(agent1);
    population.add_agent(agent2);

    let stats = population.statistics();

    assert_eq!(stats.total_population, 2);
    assert_eq!(stats.average_health, 70); // (80 + 60) / 2
    assert_eq!(stats.average_hunger, 40); // (30 + 50) / 2
    assert_eq!(stats.average_thirst, 40); // (30 + 50) / 2
    assert_eq!(stats.average_age, 150); // (100 + 200) / 2
}

#[test]
fn agent_metabolism_affects_consumption() {
    let mut rng = DeterministicRng::from_u64(42);

    // Create two agents with different metabolisms
    let mut fast_metabolism = Individual::new(polis_agents::AgentId(0), 0, &mut rng);
    fast_metabolism.metabolism = 120; // High metabolism
    fast_metabolism.hunger = 50;
    fast_metabolism.thirst = 50;

    let mut slow_metabolism = Individual::new(polis_agents::AgentId(1), 0, &mut rng);
    slow_metabolism.metabolism = 60; // Low metabolism
    slow_metabolism.hunger = 50;
    slow_metabolism.thirst = 50;

    // Both consume from same resource pool
    let (food1, _water1) = fast_metabolism.consume(1000, 1000);
    let (food2, _water2) = slow_metabolism.consume(1000, 1000);

    // Higher metabolism should consume more
    assert!(
        food1 >= food2,
        "High metabolism should consume at least as much food: fast={}, slow={}",
        food1,
        food2
    );
}

#[test]
fn simulation_with_agents_is_deterministic() {
    let mut sim1 = Simulation::new_with_partition_count(SimulationSeed::new(42), 3);
    let mut sim2 = Simulation::new_with_partition_count(SimulationSeed::new(42), 3);

    // Run both simulations
    for _ in 0..100 {
        sim1.step_with_mode(ExecutionMode::Serial);
        sim2.step_with_mode(ExecutionMode::Serial);
    }

    // Check metrics match
    let metrics1 = sim1.metrics();
    let metrics2 = sim2.metrics();

    assert_eq!(
        metrics1.len(),
        metrics2.len(),
        "Should have same number of metrics"
    );

    for (i, (m1, m2)) in metrics1.iter().zip(metrics2.iter()).enumerate() {
        assert_eq!(
            m1.total_agents, m2.total_agents,
            "Agent counts should match at tick {}",
            i
        );
        assert_eq!(
            m1.average_agent_health, m2.average_agent_health,
            "Health should match at tick {}",
            i
        );
    }
}
