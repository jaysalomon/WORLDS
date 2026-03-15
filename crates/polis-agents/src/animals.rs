//! Animal species archetypes for Phase 6
//!
//! This module implements hard-coded species archetypes through a shared trait+capability model.
//! Task outputs are trait-derived, not species-switched.
//!
//! Required archetypes: Horse, Ox/Cattle, Dog, Sheep, Goat, Pig, Poultry, Waterfowl

use serde::{Deserialize, Serialize};

/// Unique identifier for an animal archetype
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SpeciesId(pub u64);

/// Core animal traits that define capabilities
/// All values are normalized 0-100 where applicable
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct AnimalTraits {
    /// Physical strength (affects traction, carrying)
    pub strength: u8,
    /// Stamina/endurance (affects duration of work)
    pub stamina: u8,
    /// Movement speed
    pub speed: u8,
    /// Behavioral temperament (higher = calmer/more manageable)
    pub temperament: u8,
    /// Ability to learn and respond to training
    pub trainability: u8,
    /// Food/energy requirements (higher = needs more feed)
    pub feed_requirement: u8,
    /// Susceptibility to disease (higher = more vulnerable)
    pub disease_susceptibility: u8,
    /// Reproductive rate (higher = breeds faster)
    pub reproduction_rate: u8,
    /// Typical lifespan in ticks (base value)
    pub lifespan_profile: u32,
    /// Waste output (affects manure production)
    pub waste_output: u8,
}

impl AnimalTraits {
    /// Create traits with default balanced values
    pub fn balanced() -> Self {
        Self {
            strength: 50,
            stamina: 50,
            speed: 50,
            temperament: 50,
            trainability: 50,
            feed_requirement: 50,
            disease_susceptibility: 50,
            reproduction_rate: 50,
            lifespan_profile: 2000,
            waste_output: 50,
        }
    }

    /// Calculate effective transport capacity
    /// Formula: (strength * 0.4 + stamina * 0.4 + speed * 0.2) * health_factor
    pub fn transport_capacity(&self, health: u8, training: u8) -> f32 {
        let base = (self.strength as f32 * 0.4
            + self.stamina as f32 * 0.4
            + self.speed as f32 * 0.2)
            / 100.0;
        let health_factor = health as f32 / 100.0;
        let training_factor = 0.5 + (training as f32 / 200.0); // 0.5 to 1.0
        base * health_factor * training_factor * 100.0
    }

    /// Calculate effective traction power
    /// Formula: (strength * 0.6 + stamina * 0.3 + temperament * 0.1) * harness_training_factor
    pub fn traction_power(&self, health: u8, harness_training: u8) -> f32 {
        let base = (self.strength as f32 * 0.6
            + self.stamina as f32 * 0.3
            + self.temperament as f32 * 0.1)
            / 100.0;
        let health_factor = health as f32 / 100.0;
        let harness_factor = 0.3 + (harness_training as f32 / 142.0); // 0.3 to 1.0
        base * health_factor * harness_factor * 100.0
    }

    /// Calculate hunting support effectiveness
    /// Formula: (speed * 0.5 + trainability * 0.3 + temperament * 0.2) * handler_bond_factor
    pub fn hunting_effectiveness(&self, health: u8, handler_bond: u8) -> f32 {
        let base = (self.speed as f32 * 0.5
            + self.trainability as f32 * 0.3
            + self.temperament as f32 * 0.2)
            / 100.0;
        let health_factor = health as f32 / 100.0;
        let bond_factor = 0.5 + (handler_bond as f32 / 200.0);
        base * health_factor * bond_factor * 100.0
    }

    /// Calculate guard effectiveness
    /// Formula: (temperament * 0.4 + trainability * 0.3 + strength * 0.3) * alertness_factor
    pub fn guard_effectiveness(&self, health: u8, alertness: u8) -> f32 {
        // Temperament matters: needs balance of aggression and control
        let base = (self.temperament as f32 * 0.4
            + self.trainability as f32 * 0.3
            + self.strength as f32 * 0.3)
            / 100.0;
        let health_factor = health as f32 / 100.0;
        let alertness_factor = alertness as f32 / 100.0;
        base * health_factor * alertness_factor * 100.0
    }

    /// Calculate milk production potential
    /// Formula: species_base * (strength * 0.3 + stamina * 0.4 + temperament * 0.3) / 100 * nutrition_factor
    pub fn milk_production(&self, nutrition: u8, health: u8) -> f32 {
        let base = (self.strength as f32 * 0.3
            + self.stamina as f32 * 0.4
            + self.temperament as f32 * 0.3)
            / 100.0;
        let nutrition_factor = nutrition as f32 / 100.0;
        let health_factor = health as f32 / 100.0;
        base * nutrition_factor * health_factor * 100.0
    }

    /// Calculate egg production potential (for poultry/waterfowl)
    /// Formula: reproduction_rate * stamina * nutrition / 10000
    pub fn egg_production(&self, nutrition: u8, health: u8) -> f32 {
        let base = (self.reproduction_rate as f32 * self.stamina as f32) / 100.0;
        let nutrition_factor = nutrition as f32 / 100.0;
        let health_factor = health as f32 / 100.0;
        base * nutrition_factor * health_factor
    }

    /// Calculate fiber/wool production
    /// Formula: reproduction_rate * 0.5 + stamina * 0.3 + temperament * 0.2
    pub fn fiber_production(&self, nutrition: u8, health: u8) -> f32 {
        let base = (self.reproduction_rate as f32 * 0.5
            + self.stamina as f32 * 0.3
            + self.temperament as f32 * 0.2)
            / 100.0;
        let nutrition_factor = nutrition as f32 / 100.0;
        let health_factor = health as f32 / 100.0;
        base * nutrition_factor * health_factor * 100.0
    }

    /// Calculate manure/fertilizer output
    /// Based on feed consumption and waste_output trait
    pub fn manure_output(&self, feed_consumed: f32) -> f32 {
        let conversion_rate = self.waste_output as f32 / 200.0; // 0.0 to 0.5
        feed_consumed * conversion_rate
    }
}

/// Capability flags for what an animal can do
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct AnimalCapabilities {
    /// Can be ridden
    pub can_ride: bool,
    /// Can carry loads (pack animal)
    pub can_carry_load: bool,
    /// Can pull traction (plow, cart)
    pub can_pull_traction: bool,
    /// Can assist in hunting
    pub can_hunt_assist: bool,
    /// Can guard/protect
    pub can_guard: bool,
    /// Can assist in herding other animals
    pub can_herd_assist: bool,
    /// Produces milk
    pub produces_milk: bool,
    /// Produces fiber/wool
    pub produces_fiber: bool,
    /// Produces eggs
    pub produces_eggs: bool,
    /// Produces high-value manure
    pub high_manure_value: bool,
}

impl AnimalCapabilities {
    /// No capabilities (baseline)
    pub fn none() -> Self {
        Self {
            can_ride: false,
            can_carry_load: false,
            can_pull_traction: false,
            can_hunt_assist: false,
            can_guard: false,
            can_herd_assist: false,
            produces_milk: false,
            produces_fiber: false,
            produces_eggs: false,
            high_manure_value: false,
        }
    }

    /// All capabilities (for testing)
    pub fn all() -> Self {
        Self {
            can_ride: true,
            can_carry_load: true,
            can_pull_traction: true,
            can_hunt_assist: true,
            can_guard: true,
            can_herd_assist: true,
            produces_milk: true,
            produces_fiber: true,
            produces_eggs: true,
            high_manure_value: true,
        }
    }
}

/// A species archetype definition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpeciesArchetype {
    pub id: SpeciesId,
    pub name: &'static str,
    pub traits: AnimalTraits,
    pub capabilities: AnimalCapabilities,
    /// Base meat yield when slaughtered
    pub base_meat_yield: u32,
    /// Whether this is a domesticated species
    pub is_domesticated: bool,
}

/// Predefined species archetypes
pub struct SpeciesArchetypes;

impl SpeciesArchetypes {
    /// Horse - transport, riding, light traction
    pub fn horse() -> SpeciesArchetype {
        SpeciesArchetype {
            id: SpeciesId(1),
            name: "Horse",
            traits: AnimalTraits {
                strength: 75,
                stamina: 80,
                speed: 90,
                temperament: 60,
                trainability: 75,
                feed_requirement: 70,
                disease_susceptibility: 60,
                reproduction_rate: 40,
                lifespan_profile: 2500,
                waste_output: 60,
            },
            capabilities: AnimalCapabilities {
                can_ride: true,
                can_carry_load: true,
                can_pull_traction: true,
                can_hunt_assist: false,
                can_guard: false,
                can_herd_assist: false,
                produces_milk: false,
                produces_fiber: false,
                produces_eggs: false,
                high_manure_value: true,
            },
            base_meat_yield: 300,
            is_domesticated: true,
        }
    }

    /// Ox/Cattle - heavy traction, meat, milk, manure
    pub fn ox_cattle() -> SpeciesArchetype {
        SpeciesArchetype {
            id: SpeciesId(2),
            name: "Ox/Cattle",
            traits: AnimalTraits {
                strength: 95,
                stamina: 70,
                speed: 40,
                temperament: 65,
                trainability: 60,
                feed_requirement: 85,
                disease_susceptibility: 55,
                reproduction_rate: 35,
                lifespan_profile: 2200,
                waste_output: 90,
            },
            capabilities: AnimalCapabilities {
                can_ride: false,
                can_carry_load: true,
                can_pull_traction: true,
                can_hunt_assist: false,
                can_guard: false,
                can_herd_assist: false,
                produces_milk: true,
                produces_fiber: false,
                produces_eggs: false,
                high_manure_value: true,
            },
            base_meat_yield: 400,
            is_domesticated: true,
        }
    }

    /// Dog - companionship, hunting support, guarding, herding
    pub fn dog() -> SpeciesArchetype {
        SpeciesArchetype {
            id: SpeciesId(3),
            name: "Dog",
            traits: AnimalTraits {
                strength: 40,
                stamina: 75,
                speed: 80,
                temperament: 70,
                trainability: 90,
                feed_requirement: 40,
                disease_susceptibility: 70,
                reproduction_rate: 80,
                lifespan_profile: 1500,
                waste_output: 30,
            },
            capabilities: AnimalCapabilities {
                can_ride: false,
                can_carry_load: false,
                can_pull_traction: false,
                can_hunt_assist: true,
                can_guard: true,
                can_herd_assist: true,
                produces_milk: false,
                produces_fiber: false,
                produces_eggs: false,
                high_manure_value: false,
            },
            base_meat_yield: 50,
            is_domesticated: true,
        }
    }

    /// Sheep - wool, meat, milk (light), manure
    pub fn sheep() -> SpeciesArchetype {
        SpeciesArchetype {
            id: SpeciesId(4),
            name: "Sheep",
            traits: AnimalTraits {
                strength: 35,
                stamina: 60,
                speed: 50,
                temperament: 55,
                trainability: 50,
                feed_requirement: 50,
                disease_susceptibility: 65,
                reproduction_rate: 70,
                lifespan_profile: 1200,
                waste_output: 50,
            },
            capabilities: AnimalCapabilities {
                can_ride: false,
                can_carry_load: false,
                can_pull_traction: false,
                can_hunt_assist: false,
                can_guard: false,
                can_herd_assist: false,
                produces_milk: true,
                produces_fiber: true,
                produces_eggs: false,
                high_manure_value: true,
            },
            base_meat_yield: 80,
            is_domesticated: true,
        }
    }

    /// Goat - milk, meat, rough terrain browsing, manure
    pub fn goat() -> SpeciesArchetype {
        SpeciesArchetype {
            id: SpeciesId(5),
            name: "Goat",
            traits: AnimalTraits {
                strength: 40,
                stamina: 70,
                speed: 55,
                temperament: 50,
                trainability: 55,
                feed_requirement: 45,
                disease_susceptibility: 60,
                reproduction_rate: 75,
                lifespan_profile: 1400,
                waste_output: 45,
            },
            capabilities: AnimalCapabilities {
                can_ride: false,
                can_carry_load: true,
                can_pull_traction: false,
                can_hunt_assist: false,
                can_guard: false,
                can_herd_assist: false,
                produces_milk: true,
                produces_fiber: true, // Mohair/cashmere
                produces_eggs: false,
                high_manure_value: true,
            },
            base_meat_yield: 60,
            is_domesticated: true,
        }
    }

    /// Pig - meat, waste conversion, high feed demand
    pub fn pig() -> SpeciesArchetype {
        SpeciesArchetype {
            id: SpeciesId(6),
            name: "Pig",
            traits: AnimalTraits {
                strength: 50,
                stamina: 45,
                speed: 45,
                temperament: 45,
                trainability: 65,
                feed_requirement: 90,
                disease_susceptibility: 75,
                reproduction_rate: 95,
                lifespan_profile: 1000,
                waste_output: 70,
            },
            capabilities: AnimalCapabilities {
                can_ride: false,
                can_carry_load: false,
                can_pull_traction: false,
                can_hunt_assist: false,
                can_guard: true, // Can be territorial
                can_herd_assist: false,
                produces_milk: false,
                produces_fiber: false,
                produces_eggs: false,
                high_manure_value: true,
            },
            base_meat_yield: 150,
            is_domesticated: true,
        }
    }

    /// Poultry - eggs, meat, pest pressure coupling
    pub fn poultry() -> SpeciesArchetype {
        SpeciesArchetype {
            id: SpeciesId(7),
            name: "Poultry",
            traits: AnimalTraits {
                strength: 10,
                stamina: 40,
                speed: 30,
                temperament: 40,
                trainability: 30,
                feed_requirement: 25,
                disease_susceptibility: 80,
                reproduction_rate: 90,
                lifespan_profile: 400,
                waste_output: 35,
            },
            capabilities: AnimalCapabilities {
                can_ride: false,
                can_carry_load: false,
                can_pull_traction: false,
                can_hunt_assist: false,
                can_guard: false,
                can_herd_assist: false,
                produces_milk: false,
                produces_fiber: false,
                produces_eggs: true,
                high_manure_value: false,
            },
            base_meat_yield: 20,
            is_domesticated: true,
        }
    }

    /// Waterfowl - eggs/meat in wet systems, moderate pest coupling
    pub fn waterfowl() -> SpeciesArchetype {
        SpeciesArchetype {
            id: SpeciesId(8),
            name: "Waterfowl",
            traits: AnimalTraits {
                strength: 15,
                stamina: 50,
                speed: 45,
                temperament: 45,
                trainability: 35,
                feed_requirement: 35,
                disease_susceptibility: 70,
                reproduction_rate: 70,
                lifespan_profile: 600,
                waste_output: 40,
            },
            capabilities: AnimalCapabilities {
                can_ride: false,
                can_carry_load: false,
                can_pull_traction: false,
                can_hunt_assist: false,
                can_guard: false,
                can_herd_assist: false,
                produces_milk: false,
                produces_fiber: false,
                produces_eggs: true,
                high_manure_value: false,
            },
            base_meat_yield: 30,
            is_domesticated: true,
        }
    }

    /// Deer - wild prey baseline
    pub fn deer() -> SpeciesArchetype {
        SpeciesArchetype {
            id: SpeciesId(9),
            name: "Deer",
            traits: AnimalTraits {
                strength: 50,
                stamina: 70,
                speed: 85,
                temperament: 20,
                trainability: 15,
                feed_requirement: 60,
                disease_susceptibility: 50,
                reproduction_rate: 50,
                lifespan_profile: 1200,
                waste_output: 50,
            },
            capabilities: AnimalCapabilities::none(),
            base_meat_yield: 100,
            is_domesticated: false,
        }
    }

    /// Wild Boar - wild risk and prey
    pub fn wild_boar() -> SpeciesArchetype {
        SpeciesArchetype {
            id: SpeciesId(10),
            name: "Wild Boar",
            traits: AnimalTraits {
                strength: 70,
                stamina: 65,
                speed: 60,
                temperament: 25,
                trainability: 20,
                feed_requirement: 70,
                disease_susceptibility: 70,
                reproduction_rate: 70,
                lifespan_profile: 1000,
                waste_output: 60,
            },
            capabilities: AnimalCapabilities::none(),
            base_meat_yield: 120,
            is_domesticated: false,
        }
    }

    /// Get all domesticated species
    pub fn domesticated() -> Vec<SpeciesArchetype> {
        vec![
            Self::horse(),
            Self::ox_cattle(),
            Self::dog(),
            Self::sheep(),
            Self::goat(),
            Self::pig(),
            Self::poultry(),
            Self::waterfowl(),
        ]
    }

    /// Get all wild species
    pub fn wild() -> Vec<SpeciesArchetype> {
        vec![Self::deer(), Self::wild_boar()]
    }

    /// Get all species
    pub fn all() -> Vec<SpeciesArchetype> {
        let mut all = Self::domesticated();
        all.extend(Self::wild());
        all
    }

    /// Get species by ID
    pub fn by_id(id: SpeciesId) -> Option<SpeciesArchetype> {
        Self::all().into_iter().find(|s| s.id == id)
    }
}

/// An individual animal instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Animal {
    pub id: u64,
    pub species_id: SpeciesId,
    pub partition_id: u64,
    /// Current health (0-100)
    pub health: u8,
    /// Training level for specific tasks (0-100)
    pub training: u8,
    /// Harness training for traction (0-100)
    pub harness_training: u8,
    /// Bond with handler for hunting/guarding (0-100)
    pub handler_bond: u8,
    /// Alertness for guarding (0-100)
    pub alertness: u8,
    /// Current nutrition level (0-100)
    pub nutrition: u8,
    /// Age in ticks
    pub age: u32,
    /// Whether animal is alive
    pub is_alive: bool,
    /// Whether animal is owned/tamed
    pub is_domesticated: bool,
    /// Owner agent ID (if domesticated)
    pub owner_id: Option<super::AgentId>,
}

impl Animal {
    /// Create a new animal
    pub fn new(id: u64, species_id: SpeciesId, partition_id: u64) -> Self {
        Self {
            id,
            species_id,
            partition_id,
            health: 100,
            training: 0,
            harness_training: 0,
            handler_bond: 0,
            alertness: 50,
            nutrition: 80,
            age: 0,
            is_alive: true,
            is_domesticated: false,
            owner_id: None,
        }
    }

    /// Get the species archetype
    pub fn species(&self) -> Option<SpeciesArchetype> {
        SpeciesArchetypes::by_id(self.species_id)
    }

    /// Calculate effective transport capacity (trait-derived)
    pub fn effective_transport_capacity(&self) -> f32 {
        if let Some(species) = self.species() {
            if species.capabilities.can_carry_load || species.capabilities.can_ride {
                return species.traits.transport_capacity(self.health, self.training);
            }
        }
        0.0
    }

    /// Calculate effective traction power (trait-derived)
    pub fn effective_traction_power(&self) -> f32 {
        if let Some(species) = self.species() {
            if species.capabilities.can_pull_traction {
                return species.traits.traction_power(self.health, self.harness_training);
            }
        }
        0.0
    }

    /// Calculate hunting effectiveness (trait-derived)
    pub fn effective_hunting_support(&self) -> f32 {
        if let Some(species) = self.species() {
            if species.capabilities.can_hunt_assist {
                return species.traits.hunting_effectiveness(self.health, self.handler_bond);
            }
        }
        0.0
    }

    /// Calculate guard effectiveness (trait-derived)
    pub fn effective_guard_utility(&self) -> f32 {
        if let Some(species) = self.species() {
            if species.capabilities.can_guard {
                return species.traits.guard_effectiveness(self.health, self.alertness);
            }
        }
        0.0
    }

    /// Calculate milk production (trait-derived)
    pub fn effective_milk_production(&self) -> f32 {
        if let Some(species) = self.species() {
            if species.capabilities.produces_milk {
                return species.traits.milk_production(self.nutrition, self.health);
            }
        }
        0.0
    }

    /// Calculate egg production (trait-derived)
    pub fn effective_egg_production(&self) -> f32 {
        if let Some(species) = self.species() {
            if species.capabilities.produces_eggs {
                return species.traits.egg_production(self.nutrition, self.health);
            }
        }
        0.0
    }

    /// Calculate fiber/wool production (trait-derived)
    pub fn effective_fiber_production(&self) -> f32 {
        if let Some(species) = self.species() {
            if species.capabilities.produces_fiber {
                return species.traits.fiber_production(self.nutrition, self.health);
            }
        }
        0.0
    }

    /// Calculate manure output (trait-derived)
    pub fn manure_output(&self, feed_consumed: f32) -> f32 {
        if let Some(species) = self.species() {
            return species.traits.manure_output(feed_consumed);
        }
        0.0
    }

    /// Update animal state
    pub fn update(&mut self) {
        if !self.is_alive {
            return;
        }

        // Age increment
        self.age += 1;

        // Health decline from poor nutrition
        if self.nutrition < 30 {
            self.health = self.health.saturating_sub(1);
        }

        // Nutrition decay
        self.nutrition = self.nutrition.saturating_sub(2);

        // Death check
        if let Some(species) = self.species() {
            if self.age >= species.traits.lifespan_profile || self.health == 0 {
                self.is_alive = false;
            }
        }
    }

    /// Feed the animal
    pub fn feed(&mut self, amount: u8) {
        self.nutrition = (self.nutrition + amount).min(100);
    }
}

/// Population of animals per partition
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnimalPopulation {
    next_id: u64,
    animals: Vec<Animal>,
}

impl AnimalPopulation {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            animals: Vec::new(),
        }
    }

    /// Add an animal
    pub fn add(&mut self, animal: Animal) {
        self.animals.push(animal);
    }

    /// Spawn a new animal
    pub fn spawn(&mut self, species_id: SpeciesId, partition_id: u64) -> Animal {
        let animal = Animal::new(self.next_id, species_id, partition_id);
        self.next_id += 1;
        self.animals.push(animal.clone());
        animal
    }

    /// Get all animals
    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }

    /// Get mutable animals
    pub fn animals_mut(&mut self) -> &mut [Animal] {
        &mut self.animals
    }

    /// Get animals in partition
    pub fn animals_in_partition(&self, partition_id: u64) -> impl Iterator<Item = &Animal> {
        self.animals.iter().filter(move |a| a.partition_id == partition_id)
    }

    /// Get living count
    pub fn living_count(&self) -> usize {
        self.animals.iter().filter(|a| a.is_alive).count()
    }

    /// Get count by species
    pub fn count_by_species(&self, species_id: SpeciesId) -> usize {
        self.animals
            .iter()
            .filter(|a| a.is_alive && a.species_id == species_id)
            .count()
    }

    /// Calculate total transport capacity in partition
    pub fn total_transport_capacity(&self, partition_id: u64) -> f32 {
        self.animals_in_partition(partition_id)
            .filter(|a| a.is_alive)
            .map(|a| a.effective_transport_capacity())
            .sum()
    }

    /// Calculate total traction power in partition
    pub fn total_traction_power(&self, partition_id: u64) -> f32 {
        self.animals_in_partition(partition_id)
            .filter(|a| a.is_alive)
            .map(|a| a.effective_traction_power())
            .sum()
    }

    /// Calculate total secondary product outputs
    pub fn total_secondary_products(&self, partition_id: u64) -> SecondaryProducts {
        let mut products = SecondaryProducts::default();
        for animal in self.animals_in_partition(partition_id).filter(|a| a.is_alive) {
            products.milk += animal.effective_milk_production();
            products.eggs += animal.effective_egg_production();
            products.fiber += animal.effective_fiber_production();
        }
        products
    }

    /// Update all animals
    pub fn update_all(&mut self) {
        for animal in &mut self.animals {
            animal.update();
        }
    }

    /// Remove dead animals
    pub fn cleanup_dead(&mut self) {
        self.animals.retain(|a| a.is_alive);
    }

    /// Get statistics
    pub fn statistics(&self) -> AnimalPopulationStatistics {
        let living: Vec<_> = self.animals.iter().filter(|a| a.is_alive).collect();
        let count = living.len();

        if count == 0 {
            return AnimalPopulationStatistics::default();
        }

        let total_health: u32 = living.iter().map(|a| a.health as u32).sum();
        let total_nutrition: u32 = living.iter().map(|a| a.nutrition as u32).sum();

        AnimalPopulationStatistics {
            total_animals: count as u64,
            average_health: (total_health / count as u32) as u8,
            average_nutrition: (total_nutrition / count as u32) as u8,
        }
    }
}

/// Secondary product outputs
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct SecondaryProducts {
    pub milk: f32,
    pub eggs: f32,
    pub fiber: f32,
}

/// Animal population statistics
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct AnimalPopulationStatistics {
    pub total_animals: u64,
    pub average_health: u8,
    pub average_nutrition: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn species_archetypes_have_unique_ids() {
        let all = SpeciesArchetypes::all();
        let mut ids: Vec<_> = all.iter().map(|s| s.id.0).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), all.len());
    }

    #[test]
    fn horse_has_transport_capabilities() {
        let horse = SpeciesArchetypes::horse();
        assert!(horse.capabilities.can_ride);
        assert!(horse.capabilities.can_carry_load);
        assert!(horse.capabilities.high_manure_value);
        assert!(!horse.capabilities.produces_milk);
    }

    #[test]
    fn cattle_have_traction_and_milk() {
        let cattle = SpeciesArchetypes::ox_cattle();
        assert!(cattle.capabilities.can_pull_traction);
        assert!(cattle.capabilities.produces_milk);
        assert!(cattle.capabilities.high_manure_value);
        assert!(!cattle.capabilities.can_ride);
    }

    #[test]
    fn dog_has_hunting_and_guard() {
        let dog = SpeciesArchetypes::dog();
        assert!(dog.capabilities.can_hunt_assist);
        assert!(dog.capabilities.can_guard);
        assert!(dog.capabilities.can_herd_assist);
        assert!(!dog.capabilities.produces_milk);
    }

    #[test]
    fn poultry_produces_eggs() {
        let poultry = SpeciesArchetypes::poultry();
        assert!(poultry.capabilities.produces_eggs);
        assert!(!poultry.capabilities.produces_milk);
        assert!(!poultry.capabilities.can_pull_traction);
    }

    #[test]
    fn sheep_produces_fiber() {
        let sheep = SpeciesArchetypes::sheep();
        assert!(sheep.capabilities.produces_fiber);
        assert!(sheep.capabilities.produces_milk);
    }

    #[test]
    fn transport_capacity_is_trait_derived() {
        let horse = SpeciesArchetypes::horse();
        let cattle = SpeciesArchetypes::ox_cattle();

        // Horse should have higher transport capacity due to speed
        let horse_capacity = horse.traits.transport_capacity(100, 50);
        let cattle_capacity = cattle.traits.transport_capacity(100, 50);

        assert!(horse_capacity > cattle_capacity);
    }

    #[test]
    fn traction_power_is_trait_derived() {
        let horse = SpeciesArchetypes::horse();
        let cattle = SpeciesArchetypes::ox_cattle();

        // Cattle should have higher traction due to strength
        let horse_traction = horse.traits.traction_power(100, 50);
        let cattle_traction = cattle.traits.traction_power(100, 50);

        assert!(cattle_traction > horse_traction);
    }

    #[test]
    fn animal_effective_outputs_use_traits() {
        let mut animal = Animal::new(1, SpeciesId(1), 0); // Horse
        animal.health = 100;
        animal.training = 50;

        let capacity = animal.effective_transport_capacity();
        assert!(capacity > 0.0);

        // Non-capable animal should have 0 output
        let mut chicken = Animal::new(2, SpeciesId(7), 0); // Poultry
        chicken.health = 100;
        chicken.training = 50;
        let traction = chicken.effective_traction_power();
        assert_eq!(traction, 0.0);
    }

    #[test]
    fn secondary_products_calculated() {
        let cattle = SpeciesArchetypes::ox_cattle();
        let milk = cattle.traits.milk_production(100, 100);
        assert!(milk > 0.0);

        let poultry = SpeciesArchetypes::poultry();
        let eggs = poultry.traits.egg_production(100, 100);
        assert!(eggs > 0.0);

        let sheep = SpeciesArchetypes::sheep();
        let fiber = sheep.traits.fiber_production(100, 100);
        assert!(fiber > 0.0);
    }

    #[test]
    fn manure_output_trait_derived() {
        let cattle = SpeciesArchetypes::ox_cattle();
        let poultry = SpeciesArchetypes::poultry();

        // Cattle should produce more manure due to higher waste_output trait
        let cattle_manure = cattle.traits.manure_output(100.0);
        let poultry_manure = poultry.traits.manure_output(100.0);

        assert!(cattle_manure > poultry_manure);
    }

    #[test]
    fn animal_population_tracks_species() {
        let mut pop = AnimalPopulation::new();
        pop.spawn(SpeciesId(1), 0); // Horse
        pop.spawn(SpeciesId(1), 0); // Horse
        pop.spawn(SpeciesId(7), 0); // Poultry

        assert_eq!(pop.count_by_species(SpeciesId(1)), 2);
        assert_eq!(pop.count_by_species(SpeciesId(7)), 1);
    }

    #[test]
    fn wild_species_not_domesticated() {
        let deer = SpeciesArchetypes::deer();
        let boar = SpeciesArchetypes::wild_boar();

        assert!(!deer.is_domesticated);
        assert!(!boar.is_domesticated);
        assert_eq!(deer.capabilities.can_ride, false);
        assert_eq!(boar.capabilities.can_pull_traction, false);
    }
}
