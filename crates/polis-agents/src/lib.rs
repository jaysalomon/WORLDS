//! Individual agents for POLIS Phase 3
//!
//! This module implements individuals with simple survival-driven behavior:
//! - Basic needs (food, water)
//! - Movement between partitions
//! - Consumption and metabolism
//! - Mortality and reproduction

use polis_core::DeterministicRng;
use serde::{Deserialize, Serialize};

pub struct AgentsModule;

impl AgentsModule {
    pub const fn name() -> &'static str {
        "polis-agents"
    }
}

/// Unique identifier for an individual agent
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AgentId(pub u64);

impl AgentId {
    pub fn next(&mut self) -> AgentId {
        let current = self.0;
        self.0 += 1;
        AgentId(current)
    }
}

/// The state of an individual agent
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Individual {
    pub id: AgentId,
    /// Current partition location
    pub partition_id: u64,
    /// Health (0-100)
    pub health: u8,
    /// Hunger (0-100, 100 is starving)
    pub hunger: u8,
    /// Thirst (0-100, 100 is dehydrated)
    pub thirst: u8,
    /// Age in ticks
    pub age: u64,
    /// Maximum lifespan
    pub max_lifespan: u64,
    /// Reproduction cooldown (ticks until can reproduce again)
    pub reproduction_cooldown: u32,
    /// Whether agent is alive
    pub is_alive: bool,
    /// Movement preference (0 = stays put, higher = more mobile)
    pub mobility: u8,
    /// Metabolic efficiency (affects consumption rates)
    pub metabolism: u8,
}

impl Individual {
    /// Create a new individual with randomized but bounded attributes
    pub fn new(id: AgentId, partition_id: u64, rng: &mut DeterministicRng) -> Self {
        Self {
            id,
            partition_id,
            health: 80 + rng.next_bounded(20) as u8,
            hunger: rng.next_bounded(30) as u8,
            thirst: rng.next_bounded(30) as u8,
            age: 0,
            max_lifespan: 5000 + rng.next_bounded(3000),
            reproduction_cooldown: 100 + rng.next_bounded(200) as u32,
            is_alive: true,
            mobility: 30 + rng.next_bounded(70) as u8,
            metabolism: 80 + rng.next_bounded(40) as u8,
        }
    }

    /// Create a newborn individual (from reproduction)
    pub fn newborn(
        id: AgentId,
        partition_id: u64,
        parent_metabolism: u8,
        rng: &mut DeterministicRng,
    ) -> Self {
        let mut individual = Self::new(id, partition_id, rng);
        // Newborns start with higher needs and no reproduction cooldown
        individual.hunger = 20 + rng.next_bounded(20) as u8;
        individual.thirst = 20 + rng.next_bounded(20) as u8;
        individual.reproduction_cooldown = 500 + rng.next_bounded(500) as u32; // Longer for newborns
        // Inherit metabolism with slight variation
        let variation = rng.next_bounded(10) as i8 - 5;
        individual.metabolism = (parent_metabolism as i16 + variation as i16).clamp(50, 150) as u8;
        individual
    }

    /// Update needs based on metabolism
    pub fn update_needs(&mut self) {
        if !self.is_alive {
            return;
        }

        // Hunger increases based on metabolism
        let hunger_increase = (self.metabolism / 20).max(1);
        self.hunger = self.hunger.saturating_add(hunger_increase as u8);

        // Thirst increases faster
        let thirst_increase = (self.metabolism / 15).max(1);
        self.thirst = self.thirst.saturating_add(thirst_increase as u8);

        // Age increments
        self.age += 1;

        // Reproduction cooldown decrements
        if self.reproduction_cooldown > 0 {
            self.reproduction_cooldown -= 1;
        }

        // Health degradation from needs
        if self.hunger >= 80 {
            self.health = self.health.saturating_sub(2);
        }
        if self.thirst >= 80 {
            self.health = self.health.saturating_sub(3);
        }

        // Age-related health decline
        if self.age > self.max_lifespan * 3 / 4 {
            self.health = self.health.saturating_sub(1);
        }

        // Death check
        if self.health == 0 || self.age >= self.max_lifespan {
            self.is_alive = false;
        }
    }

    /// Consume food and water from partition
    /// Returns (food_consumed, water_consumed)
    pub fn consume(&mut self, available_food: u64, available_water: u64) -> (u64, u64) {
        if !self.is_alive {
            return (0, 0);
        }

        // Calculate consumption needs based on metabolism
        let food_need = (self.metabolism / 10).max(1) as u64;
        let water_need = (self.metabolism / 8).max(1) as u64;

        // Consume food
        let food_consumed = food_need.min(available_food);
        self.hunger = self
            .hunger
            .saturating_sub((food_consumed * 10).min(100) as u8);

        // Consume water
        let water_consumed = water_need.min(available_water);
        self.thirst = self
            .thirst
            .saturating_sub((water_consumed * 10).min(100) as u8);

        // Health recovery if well-fed
        if self.hunger < 30 && self.thirst < 30 {
            self.health = (self.health + 1).min(100);
        }

        (food_consumed, water_consumed)
    }

    /// Check if agent wants to move (based on needs and mobility)
    pub fn wants_to_move(&self, current_food: u64, current_water: u64) -> bool {
        if !self.is_alive {
            return false;
        }

        // High mobility agents move more readily
        let move_threshold = 100 - self.mobility;

        // Move if hungry/thirsty and current partition is low on resources
        let hungry = self.hunger > 50;
        let thirsty = self.thirst > 50;

        if (hungry && current_food < 50) || (thirsty && current_water < 50) {
            // Higher chance to move when needs aren't met
            return true;
        }

        // Random movement based on mobility
        self.id.0 % 100 < move_threshold as u64
    }

    /// Move to a new partition
    pub fn move_to(&mut self, new_partition_id: u64) {
        self.partition_id = new_partition_id;
    }

    /// Check if agent can reproduce
    pub fn can_reproduce(&self) -> bool {
        self.is_alive
            && self.age > 1000 // Minimum age
            && self.reproduction_cooldown == 0
            && self.health > 60
            && self.hunger < 50
            && self.thirst < 50
    }

    /// Perform reproduction (resets cooldown)
    pub fn reproduce(&mut self) {
        self.reproduction_cooldown = 500 + 500; // Base + variation added by caller
    }

    /// Get effective population contribution (alive = 1, dead = 0)
    pub fn population_contribution(&self) -> u64 {
        if self.is_alive { 1 } else { 0 }
    }
}

/// Collection of all agents in the simulation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentPopulation {
    next_id: AgentId,
    agents: Vec<Individual>,
}

impl AgentPopulation {
    pub fn new() -> Self {
        Self {
            next_id: AgentId(0),
            agents: Vec::new(),
        }
    }

    /// Initialize population with starting agents
    pub fn initialize(&mut self, count: usize, partition_count: u64, seed: u64) {
        let mut rng = DeterministicRng::from_u64(seed);

        for _ in 0..count {
            let partition_id = rng.next_bounded(partition_count);
            let agent = Individual::new(self.next_id.next(), partition_id, &mut rng);
            self.agents.push(agent);
        }
    }

    /// Get all agents
    pub fn agents(&self) -> &[Individual] {
        &self.agents
    }

    /// Get mutable agents
    pub fn agents_mut(&mut self) -> &mut [Individual] {
        &mut self.agents
    }

    /// Get agents in a specific partition
    pub fn agents_in_partition(&self, partition_id: u64) -> impl Iterator<Item = &Individual> {
        self.agents
            .iter()
            .filter(move |a| a.partition_id == partition_id)
    }

    /// Get count of living agents
    pub fn living_count(&self) -> usize {
        self.agents.iter().filter(|a| a.is_alive).count()
    }

    /// Get count of living agents in partition
    pub fn living_in_partition(&self, partition_id: u64) -> usize {
        self.agents
            .iter()
            .filter(|a| a.is_alive && a.partition_id == partition_id)
            .count()
    }

    /// Add a new agent (birth)
    pub fn add_agent(&mut self, agent: Individual) {
        self.agents.push(agent);
    }

    /// Spawn a newborn from parents
    pub fn spawn_newborn(
        &mut self,
        partition_id: u64,
        parent_metabolism: u8,
        seed: u64,
    ) -> Individual {
        let mut rng = DeterministicRng::from_u64(seed);
        let agent = Individual::newborn(
            self.next_id.next(),
            partition_id,
            parent_metabolism,
            &mut rng,
        );
        self.agents.push(agent.clone());
        agent
    }

    /// Remove dead agents (cleanup)
    pub fn cleanup_dead(&mut self) {
        self.agents.retain(|a| a.is_alive);
    }

    /// Get population statistics
    pub fn statistics(&self) -> PopulationStatistics {
        let living: Vec<_> = self.agents.iter().filter(|a| a.is_alive).collect();
        let count = living.len();

        if count == 0 {
            return PopulationStatistics::default();
        }

        let total_age: u64 = living.iter().map(|a| a.age).sum();
        let total_health: u32 = living.iter().map(|a| a.health as u32).sum();
        let total_hunger: u32 = living.iter().map(|a| a.hunger as u32).sum();
        let total_thirst: u32 = living.iter().map(|a| a.thirst as u32).sum();

        PopulationStatistics {
            total_population: count as u64,
            average_age: total_age / count as u64,
            average_health: (total_health / count as u32) as u8,
            average_hunger: (total_hunger / count as u32) as u8,
            average_thirst: (total_thirst / count as u32) as u8,
        }
    }

    /// Deterministic digest of the full agent population state.
    /// Used to include agent dynamics in simulation state hashing.
    pub fn digest(&self) -> u64 {
        let mut h = 0x9E37_79B9_7F4A_7C15_u64 ^ self.next_id.0;
        for a in &self.agents {
            h = mix64(h ^ a.id.0);
            h = mix64(h ^ a.partition_id.rotate_left(7));
            h = mix64(h ^ (a.health as u64));
            h = mix64(h ^ ((a.hunger as u64) << 8));
            h = mix64(h ^ ((a.thirst as u64) << 16));
            h = mix64(h ^ a.age.rotate_left(11));
            h = mix64(h ^ a.max_lifespan.rotate_left(13));
            h = mix64(h ^ ((a.reproduction_cooldown as u64) << 21));
            h = mix64(h ^ (a.is_alive as u64));
            h = mix64(h ^ ((a.mobility as u64) << 29));
            h = mix64(h ^ ((a.metabolism as u64) << 37));
        }
        h
    }
}

impl Default for AgentPopulation {
    fn default() -> Self {
        Self::new()
    }
}

fn mix64(mut x: u64) -> u64 {
    x ^= x >> 33;
    x = x.wrapping_mul(0xff51_afd7_ed55_8ccd);
    x ^= x >> 33;
    x = x.wrapping_mul(0xc4ce_b9fe_1a85_ec53);
    x ^= x >> 33;
    x
}

/// Statistics about the agent population
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct PopulationStatistics {
    pub total_population: u64,
    pub average_age: u64,
    pub average_health: u8,
    pub average_hunger: u8,
    pub average_thirst: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agent_id_increments() {
        let mut id = AgentId(0);
        assert_eq!(id.next().0, 0);
        assert_eq!(id.next().0, 1);
        assert_eq!(id.next().0, 2);
    }

    #[test]
    fn individual_creation() {
        let mut rng = DeterministicRng::from_u64(42);
        let agent = Individual::new(AgentId(1), 0, &mut rng);

        assert_eq!(agent.id.0, 1);
        assert_eq!(agent.partition_id, 0);
        assert!(agent.is_alive);
        assert!(agent.health > 0);
    }

    #[test]
    fn needs_increase_over_time() {
        let mut rng = DeterministicRng::from_u64(42);
        let mut agent = Individual::new(AgentId(1), 0, &mut rng);
        let initial_hunger = agent.hunger;
        let initial_thirst = agent.thirst;

        agent.update_needs();

        assert!(agent.hunger >= initial_hunger);
        assert!(agent.thirst >= initial_thirst);
        assert_eq!(agent.age, 1);
    }

    #[test]
    fn consumption_reduces_needs() {
        let mut rng = DeterministicRng::from_u64(42);
        let mut agent = Individual::new(AgentId(1), 0, &mut rng);
        agent.hunger = 50;
        agent.thirst = 50;

        let (food, water) = agent.consume(100, 100);

        assert!(food > 0);
        assert!(water > 0);
        assert!(agent.hunger < 50);
        assert!(agent.thirst < 50);
    }

    #[test]
    fn starvation_reduces_health() {
        let mut rng = DeterministicRng::from_u64(42);
        let mut agent = Individual::new(AgentId(1), 0, &mut rng);
        agent.hunger = 80;
        agent.thirst = 80;
        let initial_health = agent.health;

        agent.update_needs();

        assert!(agent.health < initial_health);
    }

    #[test]
    fn death_occurs_at_zero_health() {
        let mut rng = DeterministicRng::from_u64(42);
        let mut agent = Individual::new(AgentId(1), 0, &mut rng);
        agent.health = 1;
        agent.hunger = 100; // Force starvation damage

        agent.update_needs();

        assert!(!agent.is_alive);
    }

    #[test]
    fn reproduction_requirements() {
        let mut rng = DeterministicRng::from_u64(42);
        let mut agent = Individual::new(AgentId(1), 0, &mut rng);

        // New agent cannot reproduce
        assert!(!agent.can_reproduce());

        // Age the agent
        agent.age = 2000;
        agent.reproduction_cooldown = 0;
        agent.health = 80;
        agent.hunger = 20;
        agent.thirst = 20;

        assert!(agent.can_reproduce());
    }

    #[test]
    fn population_initialization() {
        let mut population = AgentPopulation::new();
        population.initialize(100, 10, 42);

        assert_eq!(population.living_count(), 100);
        assert_eq!(population.agents().len(), 100);
    }

    #[test]
    fn population_cleanup() {
        let mut population = AgentPopulation::new();
        population.initialize(10, 1, 42);

        // Kill some agents
        for agent in population.agents_mut().iter_mut().take(5) {
            agent.is_alive = false;
        }

        assert_eq!(population.living_count(), 5);
        assert_eq!(population.agents().len(), 10);

        population.cleanup_dead();

        assert_eq!(population.living_count(), 5);
        assert_eq!(population.agents().len(), 5);
    }

    #[test]
    fn newborn_inherits_metabolism() {
        let mut rng = DeterministicRng::from_u64(42);
        let parent = Individual::new(AgentId(1), 0, &mut rng);
        let parent_metabolism = parent.metabolism;

        let newborn = Individual::newborn(AgentId(2), 0, parent_metabolism, &mut rng);

        // Metabolism should be similar but not identical
        assert!((newborn.metabolism as i16 - parent_metabolism as i16).abs() <= 10);
    }

    #[test]
    fn movement_based_on_needs() {
        let mut rng = DeterministicRng::from_u64(42);
        let mut agent = Individual::new(AgentId(1), 0, &mut rng);
        agent.mobility = 100; // High mobility

        // Low needs, plenty of resources - unlikely to move
        agent.hunger = 10;
        agent.thirst = 10;
        let wants_move_low_needs = agent.wants_to_move(1000, 1000);

        // High needs, low resources - likely to move
        agent.hunger = 80;
        agent.thirst = 80;
        let wants_move_high_needs = agent.wants_to_move(10, 10);

        // High needs should trigger movement more than low needs
        assert!(!wants_move_low_needs || wants_move_high_needs);
    }
}
