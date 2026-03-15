pub struct WorldModule;

use polis_core::DeterministicRng;
use polis_compute::{ComputeConfig, ComputeEngine};
use serde::{Deserialize, Serialize};

impl WorldModule {
    pub const fn name() -> &'static str {
        "polis-world"
    }
}

// =============================================================================
// Constants
// =============================================================================

pub const DEFAULT_PARTITION_COUNT: u64 = 64;

/// Maximum allowed resource values to prevent overflow
pub const MAX_RESOURCE_VALUE: u64 = 100_000;
/// Minimum allowed resource values (can go negative only with explicit allowance)
pub const MIN_RESOURCE_VALUE: i64 = -10_000;

// =============================================================================
// Resource Types
// =============================================================================

/// Canonical resource kinds in the simulation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceKind {
    /// Food/nutrition-bearing matter
    Food,
    /// Fuel/energy carrier
    Fuel,
    /// Building material
    Material,
    /// Water/hydration source
    Water,
    /// Ore/mineral resource
    Ore,
    /// Waste/byproduct stock from consumption and processing
    Waste,
    /// Knowledge or information resource
    Knowledge,
}

impl ResourceKind {
    /// Get the default regeneration rate for this resource kind (per tick)
    pub fn base_regen_rate(&self) -> u64 {
        match self {
            ResourceKind::Food => 5,
            ResourceKind::Fuel => 2,
            ResourceKind::Material => 3,
            ResourceKind::Water => 8,
            ResourceKind::Ore => 1,
            ResourceKind::Waste => 0,
            ResourceKind::Knowledge => 0, // Knowledge doesn't regenerate naturally
        }
    }

    /// Get the maximum carrying capacity multiplier
    pub fn carrying_capacity_multiplier(&self) -> u64 {
        match self {
            ResourceKind::Food => 1000,
            ResourceKind::Fuel => 500,
            ResourceKind::Material => 800,
            ResourceKind::Water => 1200,
            ResourceKind::Ore => 300,
            ResourceKind::Waste => 2000,
            ResourceKind::Knowledge => u64::MAX, // No natural cap
        }
    }
}

/// A localized stock of a specific resource kind
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct ResourceStock {
    pub kind: ResourceKind,
    /// Quantity in abstract units
    pub quantity: i64,
    /// Quality modifier (0.0 to 1.0)
    pub quality: f32,
}

impl ResourceStock {
    pub fn new(kind: ResourceKind, quantity: u64) -> Self {
        Self {
            kind,
            quantity: quantity as i64,
            quality: 1.0,
        }
    }

    pub fn with_quality(mut self, quality: f32) -> Self {
        self.quality = quality.clamp(0.0, 1.0);
        self
    }

    /// Get effective quantity (quantity * quality)
    pub fn effective_quantity(&self) -> i64 {
        (self.quantity as f32 * self.quality).round() as i64
    }

    /// Check if stock has positive quantity
    pub fn has_stock(&self) -> bool {
        self.quantity > 0
    }

    /// Extract quantity (cannot go below zero without explicit allowance)
    pub fn extract(&mut self, amount: u64) -> i64 {
        let extracted = amount.min(self.quantity as u64);
        self.quantity -= extracted as i64;
        extracted as i64
    }

    /// Add quantity to stock (capped at MAX_RESOURCE_VALUE)
    pub fn deposit(&mut self, amount: u64) {
        let new_qty = (self.quantity as u64)
            .saturating_add(amount)
            .min(MAX_RESOURCE_VALUE);
        self.quantity = new_qty as i64;
    }
}

// =============================================================================
// Environmental Fields
// =============================================================================

/// Environmental field types that affect resource productivity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FieldKind {
    Temperature,
    Moisture,
    Fertility,
    SolarRadiation,
    BioticPressure,
}

impl FieldKind {
    pub fn default_value(&self) -> f64 {
        match self {
            FieldKind::Temperature => 20.0,   // Celsius
            FieldKind::Moisture => 0.5,       // 0-1 scale
            FieldKind::Fertility => 0.5,      // 0-1 scale
            FieldKind::SolarRadiation => 0.7, // 0-1 scale
            FieldKind::BioticPressure => 0.3, // 0-1 scale
        }
    }

    pub fn min_value(&self) -> f64 {
        match self {
            FieldKind::Temperature => -30.0,
            FieldKind::Moisture => 0.0,
            FieldKind::Fertility => 0.0,
            FieldKind::SolarRadiation => 0.0,
            FieldKind::BioticPressure => 0.0,
        }
    }

    pub fn max_value(&self) -> f64 {
        match self {
            FieldKind::Temperature => 50.0,
            FieldKind::Moisture => 1.0,
            FieldKind::Fertility => 1.0,
            FieldKind::SolarRadiation => 1.0,
            FieldKind::BioticPressure => 1.0,
        }
    }
}

/// A spatial field value with diffusion support
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct FieldCell {
    pub value: f64,
}

impl FieldCell {
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    pub fn clamp(&mut self, min: f64, max: f64) {
        self.value = self.value.clamp(min, max);
    }

    /// Apply diffusion influence from a neighbor
    pub fn diffuse_from(&mut self, neighbor_value: f64, rate: f64) {
        let diff = neighbor_value - self.value;
        self.value += diff * rate;
    }
}

// =============================================================================
// Partition State (World Substrate)
// =============================================================================

/// State for a single spatial partition/patch
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PartitionState {
    // Core resources
    pub food: ResourceStock,
    pub water: ResourceStock,
    pub material: ResourceStock,
    pub fuel: ResourceStock,
    pub ore: ResourceStock,
    pub waste: ResourceStock,

    // Environmental fields
    pub temperature: FieldCell,
    pub moisture: FieldCell,
    pub fertility: FieldCell,
    pub solar_radiation: FieldCell,
    pub biotic_pressure: FieldCell,

    // Social/demographic
    pub demand: u64,
    pub cohesion: u64,

    // Animal co-evolution scaffold
    pub herbivore_population: u64,
    pub predator_population: u64,
    pub proto_domestic_population: u64,
    /// 0.0 (fully wild) to 1.0 (highly domesticated-tame lineage tendency)
    pub domestication_tameness: f64,

    // Phase 4: Cross-species interaction state (human-animal)
    /// Average familiarity of animals with humans in this partition
    pub animal_familiarity: u8,
    /// Average fear level of animals toward humans
    pub animal_fear: u8,
    /// Average aggression of animals toward humans
    pub animal_aggression: u8,
    /// Average tolerance of animals for human proximity
    pub animal_human_tolerance: u8,
    /// Cumulative positive human-animal interactions
    pub positive_human_animal_interactions: u32,
    /// Cumulative negative human-animal interactions
    pub negative_human_animal_interactions: u32,

    // Carrying capacity for each resource
    pub carrying_capacity_food: u64,
    pub carrying_capacity_water: u64,
}

impl PartitionState {
    /// Create a new partition from a seed
    pub fn from_seed(seed: u64, partition_id: u64) -> Self {
        let mut rng = DeterministicRng::from_u64(seed ^ partition_id.rotate_left(9));

        // Initialize resources with some variation
        let base_food = 100 + rng.next_bounded(900);
        let base_water = 200 + rng.next_bounded(800);
        let base_material = 50 + rng.next_bounded(450);
        let base_fuel = 30 + rng.next_bounded(270);
        let base_ore = 20 + rng.next_bounded(180);

        // Pre-calculate environmental field values
        let temp = 20.0 + (rng.next_u64() as f64 / (u64::MAX as f64 + 1.0) - 0.5) * 20.0;
        let moist = 0.5 + (rng.next_u64() as f64 / (u64::MAX as f64 + 1.0) - 0.5) * 0.3;
        let fert = 0.5 + (rng.next_u64() as f64 / (u64::MAX as f64 + 1.0) - 0.5) * 0.3;
        let solar = 0.7 + (rng.next_u64() as f64 / (u64::MAX as f64 + 1.0) - 0.5) * 0.2;
        let biotic = 0.3 + (rng.next_u64() as f64 / (u64::MAX as f64 + 1.0) - 0.5) * 0.2;

        Self {
            food: ResourceStock::new(ResourceKind::Food, base_food),
            water: ResourceStock::new(ResourceKind::Water, base_water),
            material: ResourceStock::new(ResourceKind::Material, base_material),
            fuel: ResourceStock::new(ResourceKind::Fuel, base_fuel),
            ore: ResourceStock::new(ResourceKind::Ore, base_ore),
            waste: ResourceStock::new(ResourceKind::Waste, 0),

            // Initialize environmental fields with defaults + variation
            temperature: FieldCell::new(temp),
            moisture: FieldCell::new(moist),
            fertility: FieldCell::new(fert),
            solar_radiation: FieldCell::new(solar),
            biotic_pressure: FieldCell::new(biotic),

            // Social state
            demand: 10 + rng.next_bounded(90),
            cohesion: 20 + rng.next_bounded(80),

            // Animal populations and early domestication state
            herbivore_population: 30 + rng.next_bounded(120),
            predator_population: 5 + rng.next_bounded(25),
            proto_domestic_population: rng.next_bounded(10),
            domestication_tameness: 0.05 + (rng.next_u64() as f64 / (u64::MAX as f64 + 1.0)) * 0.1,

            // Cross-species interaction state (Phase 4)
            animal_familiarity: 5 + rng.next_bounded(15) as u8,
            animal_fear: 70 + rng.next_bounded(30) as u8,
            animal_aggression: 20 + rng.next_bounded(40) as u8,
            animal_human_tolerance: 5 + rng.next_bounded(20) as u8,
            positive_human_animal_interactions: 0,
            negative_human_animal_interactions: 0,

            // Carrying capacities (can vary by partition)
            carrying_capacity_food: 1000 + rng.next_bounded(2000),
            carrying_capacity_water: 1500 + rng.next_bounded(1500),
        }
    }

    /// Total resource value across all stocks
    pub fn total_resources(&self) -> i64 {
        self.food.effective_quantity()
            + self.water.effective_quantity()
            + self.material.effective_quantity()
            + self.fuel.effective_quantity()
            + self.ore.effective_quantity()
            + self.waste.effective_quantity()
    }

    /// Check if any resource is depleted
    pub fn has_any_resource(&self) -> bool {
        self.food.has_stock() || self.water.has_stock() || self.material.has_stock()
    }

    /// Apply environmental modifiers to resource regeneration
    pub fn regen_multiplier(&self) -> f64 {
        // Temperature: optimal around 20C, drops toward extremes
        let temp_mod = 1.0 - ((self.temperature.value - 20.0).abs() / 40.0).min(1.0);
        // Moisture: optimal around 0.6
        let moisture_mod = 1.0 - ((self.moisture.value - 0.6).abs() / 0.8).min(1.0);
        // Fertility directly multiplies
        let fertility_mod = self.fertility.value;
        // Biotic pressure reduces effective growth
        let pressure_mod = 1.0 - self.biotic_pressure.value * 0.5;

        (temp_mod * moisture_mod * fertility_mod * pressure_mod).max(0.0)
    }
}

// =============================================================================
// World State
// =============================================================================

/// Complete world state with all partitions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorldState {
    tick: u64,
    partitions: Vec<PartitionState>,
}

impl WorldState {
    pub fn new(seed: u64, partition_count: u64) -> Self {
        let partitions = (0..partition_count)
            .map(|partition_id| PartitionState::from_seed(seed, partition_id))
            .collect();

        Self {
            tick: 0,
            partitions,
        }
    }

    pub const fn tick(&self) -> u64 {
        self.tick
    }

    pub fn set_tick(&mut self, tick: u64) {
        self.tick = tick;
    }

    pub fn partition_count(&self) -> u64 {
        self.partitions.len() as u64
    }

    pub fn partitions(&self) -> &[PartitionState] {
        &self.partitions
    }

    pub fn partitions_mut(&mut self) -> &mut [PartitionState] {
        &mut self.partitions
    }

    pub fn digest(&self) -> u64 {
        self.partitions
            .iter()
            .enumerate()
            .fold(self.tick, |acc, (idx, partition)| {
                let idx_u64 = idx as u64;
                let value = partition.total_resources() as u64
                    ^ (partition.demand as u64).rotate_left(7)
                    ^ (partition.cohesion as u64).rotate_right(11)
                    ^ idx_u64.rotate_left(3);
                mix_hash(acc, idx_u64, value)
            })
    }
}

// =============================================================================
// Resource Dynamics (CPU Kernels)
// =============================================================================

/// Apply natural resource regeneration to a partition
pub fn regenerate_resources(partition: &mut PartitionState) {
    let mult = partition.regen_multiplier();

    // Food regenerates based on fertility and moisture
    let food_rate = (ResourceKind::Food.base_regen_rate() as f64 * mult) as u64;
    let food_cap = partition.carrying_capacity_food;
    if partition.food.quantity < food_cap as i64 {
        let room = food_cap.saturating_sub(partition.food.quantity as u64);
        partition.food.deposit(food_rate.min(room));
    }

    // Water regenerates (simulating rain/groundwater)
    let water_rate = (ResourceKind::Water.base_regen_rate() as f64 * mult) as u64;
    let water_cap = partition.carrying_capacity_water;
    if partition.water.quantity < water_cap as i64 {
        let room = water_cap.saturating_sub(partition.water.quantity as u64);
        partition.water.deposit(water_rate.min(room));
    }

    // Other resources have base regeneration
    let mat_rate = (ResourceKind::Material.base_regen_rate() as f64 * mult) as u64;
    partition.material.deposit(mat_rate);

    let fuel_rate = (ResourceKind::Fuel.base_regen_rate() as f64 * mult) as u64;
    partition.fuel.deposit(fuel_rate);

    // Ore regenerates very slowly
    let ore_rate = ResourceKind::Ore.base_regen_rate();
    partition.ore.deposit(ore_rate);

    // Waste naturally breaks down and feeds back weakly into fertility.
    process_waste(partition);
}

/// Process accumulated waste into partial fertility feedback while retaining pollution pressure.
pub fn process_waste(partition: &mut PartitionState) {
    let current_waste = partition.waste.quantity.max(0) as u64;
    if current_waste == 0 {
        return;
    }

    // Natural decomposition is modest and fertility-dependent.
    let fertility_factor = (partition.fertility.value.clamp(0.0, 1.0) * 4.0).round() as u64;
    let processed = (2 + fertility_factor).min(current_waste);
    partition.waste.quantity = (current_waste - processed) as i64;

    // A fraction of processed waste improves fertility.
    let fertility_gain = processed as f64 / 10_000.0;
    partition.fertility.value = (partition.fertility.value + fertility_gain).clamp(
        FieldKind::Fertility.min_value(),
        FieldKind::Fertility.max_value(),
    );

    // Residual waste contributes to biotic pressure.
    let residual_waste = partition.waste.quantity.max(0) as f64;
    let pressure_increase = (residual_waste / 200_000.0).min(0.01);
    partition.biotic_pressure.value = (partition.biotic_pressure.value + pressure_increase).clamp(
        FieldKind::BioticPressure.min_value(),
        FieldKind::BioticPressure.max_value(),
    );
}

/// Evolve animal populations and early domestication dynamics.
///
/// This is a Phase-1 scaffold for parallel co-evolution:
/// - humans hunt herbivores (predator role)
/// - predators pressure human demand/cohesion (humans as prey/vulnerable hosts)
/// - managed capture nudges proto-domestic populations and tameness over time
pub fn evolve_animal_populations(partition: &mut PartitionState) {
    let herb = partition.herbivore_population;
    let pred = partition.predator_population;
    let managed = partition.proto_domestic_population;

    // Herbivore reproduction depends on ecological productivity and food availability.
    let eco_factor = partition.regen_multiplier().clamp(0.0, 1.0);
    let herb_births = ((herb as f64 * 0.04 * eco_factor).round() as u64).max(1);
    let forage_limit = partition.food.quantity.max(0) as u64 / 10;
    let herb_growth = herb_births.min(forage_limit.saturating_add(1));

    // Predator reproduction depends on available herbivores.
    let pred_births = ((pred as f64 * 0.02).round() as u64).min((herb / 20).max(1));

    // Predator pressure on herbivores.
    let predation_on_herbivores = ((pred / 2).max(1)).min(herb.saturating_add(herb_growth));

    // Human hunting pressure on herbivores scales with demand proxy.
    let human_hunt = ((partition.demand / 12).max(1)).min(
        herb.saturating_add(herb_growth)
            .saturating_sub(predation_on_herbivores),
    );

    // Capture pressure into proto-domestic pool: small subset of hunted herbivores.
    let captured_for_management = (human_hunt / 8).max(1).min(human_hunt);

    // Humans as prey/vulnerable hosts proxy: predators reduce demand/cohesion.
    let human_losses = ((pred / 6).max(1)).min(partition.demand);
    partition.demand = partition.demand.saturating_sub(human_losses);
    if human_losses > 0 {
        partition.cohesion = partition.cohesion.saturating_sub((human_losses / 2).max(1));
    }

    // Apply population transitions.
    partition.herbivore_population = herb
        .saturating_add(herb_growth)
        .saturating_sub(predation_on_herbivores)
        .saturating_sub(human_hunt)
        .max(1);

    partition.predator_population = pred.saturating_add(pred_births).max(1);
    partition.proto_domestic_population = managed
        .saturating_add(captured_for_management)
        .min(MAX_RESOURCE_VALUE);

    // Hunting and predation create food and waste byproducts.
    partition.food.deposit((human_hunt * 2).max(1));
    partition
        .waste
        .deposit((human_hunt + predation_on_herbivores).max(1));

    // Managed interaction shifts tameness slowly over time.
    let tame_gain = (partition.proto_domestic_population as f64 / 20000.0)
        * (partition.cohesion as f64 / 100.0).clamp(0.1, 1.0);
    partition.domestication_tameness =
        (partition.domestication_tameness + tame_gain).clamp(0.0, 1.0);
}

/// Apply diffusion between adjacent partitions
pub fn diffuse_resources(partitions: &mut [PartitionState], diffusion_rate: f64) {
    // Simple 1D ring diffusion for now
    let n = partitions.len();
    if n < 2 {
        return;
    }

    let engine = ComputeEngine::new(ComputeConfig::default());
    let rate = diffusion_rate.clamp(0.0, 1.0) as f32;

    // Food diffusion via compute backend.
    let food_input: Vec<f32> = partitions.iter().map(|p| p.food.quantity as f32).collect();
    let mut food_output = vec![0.0_f32; n];
    engine.diffuse_ring_f32(&food_input, rate, &mut food_output);
    for (i, partition) in partitions.iter_mut().enumerate() {
        partition.food.quantity = (food_output[i].round() as i64).clamp(0, MAX_RESOURCE_VALUE as i64);
    }

    // Water diffusion via compute backend.
    let water_input: Vec<f32> = partitions.iter().map(|p| p.water.quantity as f32).collect();
    let mut water_output = vec![0.0_f32; n];
    engine.diffuse_ring_f32(&water_input, rate, &mut water_output);
    for (i, partition) in partitions.iter_mut().enumerate() {
        partition.water.quantity =
            (water_output[i].round() as i64).clamp(0, MAX_RESOURCE_VALUE as i64);
    }
}

/// Apply environmental field evolution
pub fn evolve_fields(partition: &mut PartitionState, tick: u64) {
    // Temperature cycles with tick (simplified seasonal)
    let seasonal = (tick as f64 * 0.01).sin() * 5.0;
    partition.temperature.value = (20.0 + seasonal).clamp(
        FieldKind::Temperature.min_value(),
        FieldKind::Temperature.max_value(),
    );

    // Moisture decreases slightly each tick (evaporation)
    partition.moisture.value = (partition.moisture.value - 0.001).clamp(
        FieldKind::Moisture.min_value(),
        FieldKind::Moisture.max_value(),
    );

    // Biotic pressure increases with population (simplified)
    let pressure = partition.demand as f64 / 1000.0;
    partition.biotic_pressure.value = pressure.clamp(
        FieldKind::BioticPressure.min_value(),
        FieldKind::BioticPressure.max_value(),
    );

    // Fertility slowly degrades with use
    partition.fertility.value = (partition.fertility.value - 0.0001).clamp(
        FieldKind::Fertility.min_value(),
        FieldKind::Fertility.max_value(),
    );
}

// =============================================================================
// Validation Helpers
// =============================================================================

/// Check that all resource values are within valid bounds
pub fn validate_partition(partition: &PartitionState) -> Result<(), String> {
    // Check all resource stocks for negative values
    for (stock, name) in [
        (&partition.food, "food"),
        (&partition.water, "water"),
        (&partition.material, "material"),
        (&partition.fuel, "fuel"),
        (&partition.ore, "ore"),
        (&partition.waste, "waste"),
    ] {
        if stock.quantity < 0 {
            return Err(format!("{} stock negative: {}", name, stock.quantity));
        }
        if stock.quantity > MAX_RESOURCE_VALUE as i64 {
            return Err(format!("{} stock exceeds max: {}", name, stock.quantity));
        }
    }

    if partition.domestication_tameness.is_nan() || partition.domestication_tameness.is_infinite() {
        return Err("domestication_tameness has NaN/Inf".to_string());
    }
    if !(0.0..=1.0).contains(&partition.domestication_tameness) {
        return Err(format!(
            "domestication_tameness out of bounds: {}",
            partition.domestication_tameness
        ));
    }

    // Check field bounds using FieldKind limits
    let field_checks = [
        (
            &partition.temperature,
            FieldKind::Temperature,
            "temperature",
        ),
        (&partition.moisture, FieldKind::Moisture, "moisture"),
        (&partition.fertility, FieldKind::Fertility, "fertility"),
        (
            &partition.solar_radiation,
            FieldKind::SolarRadiation,
            "solar_radiation",
        ),
        (
            &partition.biotic_pressure,
            FieldKind::BioticPressure,
            "biotic_pressure",
        ),
    ];

    for (field, kind, name) in field_checks {
        if field.value.is_nan() || field.value.is_infinite() {
            return Err(format!("{} has NaN/Inf", name));
        }
        if field.value < kind.min_value() {
            return Err(format!(
                "{} below min: {} < {}",
                name,
                field.value,
                kind.min_value()
            ));
        }
        if field.value > kind.max_value() {
            return Err(format!(
                "{} above max: {} > {}",
                name,
                field.value,
                kind.max_value()
            ));
        }
    }

    Ok(())
}

/// Check that resource approaches carrying capacity without harvest
pub fn check_carrying_capacity_convergence(partition: &PartitionState) -> bool {
    let food_ratio = partition.food.quantity as f64 / partition.carrying_capacity_food as f64;
    let water_ratio = partition.water.quantity as f64 / partition.carrying_capacity_water as f64;

    // Should approach ~80%+ of capacity over time without extraction
    food_ratio > 0.8 && water_ratio > 0.8
}

// =============================================================================
// Helpers
// =============================================================================

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
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_stock_extract_and_deposit() {
        let mut stock = ResourceStock::new(ResourceKind::Food, 100);
        assert_eq!(stock.quantity, 100);

        let extracted = stock.extract(30);
        assert_eq!(extracted, 30);
        assert_eq!(stock.quantity, 70);

        stock.deposit(50);
        assert_eq!(stock.quantity, 120);
    }

    #[test]
    fn resource_stock_effective_quantity() {
        let stock = ResourceStock::new(ResourceKind::Food, 100).with_quality(0.5);
        assert_eq!(stock.effective_quantity(), 50);
    }

    #[test]
    fn resource_kinds_have_regen_rates() {
        assert!(ResourceKind::Food.base_regen_rate() > 0);
        assert_eq!(ResourceKind::Waste.base_regen_rate(), 0);
        assert!(ResourceKind::Knowledge.base_regen_rate() == 0);
    }

    #[test]
    fn partition_has_resources() {
        let partition = PartitionState::from_seed(42, 0);
        assert!(partition.has_any_resource());
        assert!(partition.total_resources() > 0);
    }

    #[test]
    fn world_state_is_deterministic_for_same_seed() {
        let a = WorldState::new(42, DEFAULT_PARTITION_COUNT);
        let b = WorldState::new(42, DEFAULT_PARTITION_COUNT);
        assert_eq!(a.digest(), b.digest());
    }

    #[test]
    fn world_state_differs_for_different_seeds() {
        let a = WorldState::new(42, DEFAULT_PARTITION_COUNT);
        let b = WorldState::new(43, DEFAULT_PARTITION_COUNT);
        assert_ne!(a.digest(), b.digest());
    }

    #[test]
    fn field_clamp_works() {
        let mut field = FieldCell::new(100.0);
        field.clamp(0.0, 1.0);
        assert_eq!(field.value, 1.0);
    }

    #[test]
    fn validate_partition_accepts_valid() {
        let partition = PartitionState::from_seed(42, 0);
        assert!(validate_partition(&partition).is_ok());
    }

    #[test]
    fn partition_regen_multiplier_in_valid_range() {
        let partition = PartitionState::from_seed(42, 0);
        let mult = partition.regen_multiplier();
        assert!(mult >= 0.0 && mult <= 1.0);
    }

    #[test]
    fn resource_extraction_cannot_go_negative() {
        let mut stock = ResourceStock::new(ResourceKind::Food, 10);
        let extracted = stock.extract(100); // Try to extract more than available
        assert_eq!(extracted, 10);
        assert_eq!(stock.quantity, 0);
    }

    #[test]
    fn carrying_capacity_limit() {
        let mut stock = ResourceStock::new(ResourceKind::Food, MAX_RESOURCE_VALUE - 10);
        stock.deposit(100); // Would exceed max
        assert!(stock.quantity <= MAX_RESOURCE_VALUE as i64);
    }

    #[test]
    fn waste_processing_reduces_waste() {
        let mut partition = PartitionState::from_seed(42, 0);
        partition.waste.quantity = 100;
        let before = partition.waste.quantity;
        process_waste(&mut partition);
        assert!(partition.waste.quantity < before);
    }

    #[test]
    fn animal_populations_evolve_and_remain_valid() {
        let mut partition = PartitionState::from_seed(42, 0);
        let herb_before = partition.herbivore_population;
        evolve_animal_populations(&mut partition);
        assert!(partition.herbivore_population > 0);
        assert!(partition.predator_population > 0);
        assert!(partition.domestication_tameness >= 0.0);
        assert!(partition.domestication_tameness <= 1.0);
        assert!(
            partition.herbivore_population != herb_before
                || partition.proto_domestic_population > 0
        );
        assert!(validate_partition(&partition).is_ok());
    }
}
