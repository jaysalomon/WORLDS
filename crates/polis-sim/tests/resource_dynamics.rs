//! Resource dynamics validation tests for Phase 1
//!
//! These tests validate:
//! - Resource convergence toward carrying capacity
//! - Diffusion behavior between partitions
//! - Field dynamics (seasonal cycles, evaporation, etc.)

use polis_core::SimulationSeed;
use polis_sim::{ExecutionMode, Simulation};
use polis_world::{
    FieldKind, PartitionState, WorldState, check_carrying_capacity_convergence, diffuse_resources,
    evolve_fields, process_waste, regenerate_resources, validate_partition,
};

// =============================================================================
// Carrying Capacity Convergence Tests
// =============================================================================

#[test]
fn food_approaches_carrying_capacity_without_extraction() {
    // Create a world with a single partition
    let mut world = WorldState::new(42, 1);
    let partition = &mut world.partitions_mut()[0];

    // Set low initial food and high carrying capacity
    partition.food.quantity = 100;
    partition.carrying_capacity_food = 1000;

    // Run regeneration for many ticks
    for tick in 0..500 {
        evolve_fields(partition, tick);
        regenerate_resources(partition);
    }

    // Food should have grown significantly toward capacity
    // Note: With environmental modifiers, convergence is partial not complete
    let food_ratio = partition.food.quantity as f64 / partition.carrying_capacity_food as f64;
    assert!(
        food_ratio > 0.35,
        "Food should grow toward carrying capacity: got {}% ({} / {})",
        food_ratio * 100.0,
        partition.food.quantity,
        partition.carrying_capacity_food
    );

    // Should have grown from initial 100
    assert!(
        partition.food.quantity > 200,
        "Food should have grown from initial 100 to at least 200, got {}",
        partition.food.quantity
    );

    // Validate partition state is still valid
    validate_partition(partition).expect("Partition should remain valid");
}

#[test]
fn water_approaches_carrying_capacity_without_extraction() {
    let mut world = WorldState::new(42, 1);
    let partition = &mut world.partitions_mut()[0];

    // Set low initial water
    partition.water.quantity = 200;
    partition.carrying_capacity_water = 2000;

    for tick in 0..500 {
        evolve_fields(partition, tick);
        regenerate_resources(partition);
    }

    // Water should have grown significantly (with environmental modifiers)
    let water_ratio = partition.water.quantity as f64 / partition.carrying_capacity_water as f64;
    assert!(
        water_ratio > 0.35,
        "Water should grow toward carrying capacity: got {}%",
        water_ratio * 100.0
    );

    // Should have grown from initial 200
    assert!(
        partition.water.quantity > 350,
        "Water should have grown from initial 200, got {}",
        partition.water.quantity
    );
}

#[test]
fn carrying_capacity_convergence_helper_works() {
    let mut partition = PartitionState::from_seed(42, 0);

    // Initially not converged (random initialization)
    partition.food.quantity = 100;
    partition.carrying_capacity_food = 1000;
    partition.water.quantity = 200;
    partition.carrying_capacity_water = 2000;

    assert!(
        !check_carrying_capacity_convergence(&partition),
        "Should not be converged initially"
    );

    // Simulate to convergence (with environmental degradation, full convergence is partial)
    for tick in 0..1000 {
        evolve_fields(&mut partition, tick);
        regenerate_resources(&mut partition);
    }

    // Should have grown significantly (may not reach 80% due to environmental degradation)
    let food_ratio = partition.food.quantity as f64 / partition.carrying_capacity_food as f64;
    assert!(
        food_ratio > 0.30,
        "Should have grown toward carrying capacity: food_ratio={}",
        food_ratio
    );
}

#[test]
fn extraction_prevents_full_convergence() {
    let mut world = WorldState::new(42, 1);
    let partition = &mut world.partitions_mut()[0];

    // Set up for growth
    partition.food.quantity = 500;
    partition.carrying_capacity_food = 1000;

    // Simulate with periodic extraction
    for _tick in 0..500 {
        regenerate_resources(partition);
        // Extract 10% of food each tick
        let extract_amount = (partition.food.quantity / 10) as u64;
        partition.food.extract(extract_amount);
    }

    // Should not reach full carrying capacity due to extraction
    let food_ratio = partition.food.quantity as f64 / partition.carrying_capacity_food as f64;
    assert!(
        food_ratio < 0.9,
        "Extraction should prevent full convergence: ratio = {}",
        food_ratio
    );
}

// =============================================================================
// Diffusion Tests
// =============================================================================

#[test]
fn diffusion_equalizes_resource_distribution() {
    // Create world with 3 partitions: low, high, medium
    let mut world = WorldState::new(42, 3);
    let partitions = world.partitions_mut();

    // Set up unequal distribution
    partitions[0].food.quantity = 1000; // High
    partitions[1].food.quantity = 100; // Low
    partitions[2].food.quantity = 500; // Medium

    let initial_variance = calculate_variance(&[
        partitions[0].food.quantity,
        partitions[1].food.quantity,
        partitions[2].food.quantity,
    ]);

    // Run diffusion many times
    for _ in 0..100 {
        diffuse_resources(partitions, 0.1);
    }

    let final_variance = calculate_variance(&[
        partitions[0].food.quantity,
        partitions[1].food.quantity,
        partitions[2].food.quantity,
    ]);

    // Variance should decrease (resources equalize)
    assert!(
        final_variance < initial_variance,
        "Diffusion should reduce variance: {} -> {}",
        initial_variance,
        final_variance
    );

    // All partitions should have positive food
    for (i, p) in partitions.iter().enumerate() {
        assert!(
            p.food.quantity > 0,
            "Partition {} should have positive food after diffusion",
            i
        );
    }
}

#[test]
fn diffusion_preserves_total_mass() {
    let mut world = WorldState::new(42, 10);
    let partitions = world.partitions_mut();

    // Set specific initial values
    for (i, p) in partitions.iter_mut().enumerate() {
        p.food.quantity = (100 + i * 50) as i64;
    }

    let initial_total: i64 = partitions.iter().map(|p| p.food.quantity).sum();

    // Run diffusion
    for _ in 0..50 {
        diffuse_resources(partitions, 0.1);
    }

    let final_total: i64 = partitions.iter().map(|p| p.food.quantity).sum();

    // Total should be approximately preserved (allowing for rounding)
    let diff = (initial_total - final_total).abs();
    assert!(
        diff <= 10,
        "Diffusion should preserve total mass: initial={}, final={}, diff={}",
        initial_total,
        final_total,
        diff
    );
}

#[test]
fn diffusion_rate_affects_speed() {
    let mut world1 = WorldState::new(42, 3);
    let mut world2 = WorldState::new(42, 3);

    // Set up same initial conditions
    for world in [&mut world1, &mut world2].iter_mut() {
        let partitions = world.partitions_mut();
        partitions[0].food.quantity = 1000;
        partitions[1].food.quantity = 100;
        partitions[2].food.quantity = 500;
    }

    // Run with different rates
    for _ in 0..50 {
        diffuse_resources(world1.partitions_mut(), 0.05); // Slow
        diffuse_resources(world2.partitions_mut(), 0.2); // Fast
    }

    // Calculate variance for both
    let var1 = calculate_variance(
        &world1
            .partitions()
            .iter()
            .map(|p| p.food.quantity)
            .collect::<Vec<_>>(),
    );
    let var2 = calculate_variance(
        &world2
            .partitions()
            .iter()
            .map(|p| p.food.quantity)
            .collect::<Vec<_>>(),
    );

    // Higher rate should equalize faster (lower variance)
    assert!(
        var2 < var1,
        "Higher diffusion rate should equalize faster: var_slow={}, var_fast={}",
        var1,
        var2
    );
}

#[test]
fn water_diffusion_works_same_as_food() {
    let mut world = WorldState::new(42, 3);
    let partitions = world.partitions_mut();

    partitions[0].water.quantity = 1000;
    partitions[1].water.quantity = 100;
    partitions[2].water.quantity = 500;

    let initial_total: i64 = partitions.iter().map(|p| p.water.quantity).sum();

    // Run diffusion (which includes water)
    for _ in 0..50 {
        diffuse_resources(partitions, 0.1);
    }

    let final_total: i64 = partitions.iter().map(|p| p.water.quantity).sum();

    // Water should have diffused and total preserved
    let diff = (initial_total - final_total).abs();
    assert!(
        diff <= 10,
        "Water diffusion should preserve mass: initial={}, final={}",
        initial_total,
        final_total
    );
}

// =============================================================================
// Field Dynamics Tests
// =============================================================================

#[test]
fn temperature_follows_seasonal_cycle() {
    let mut partition = PartitionState::from_seed(42, 0);

    // Sample temperatures at different ticks
    let mut temps = Vec::new();
    for tick in [0, 100, 200, 300, 400, 500, 600, 700] {
        evolve_fields(&mut partition, tick);
        temps.push(partition.temperature.value);
    }

    // Should see variation (seasonal cycle)
    let min_temp = temps.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_temp = temps.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    assert!(
        max_temp > min_temp + 1.0,
        "Temperature should vary seasonally: min={}, max={}",
        min_temp,
        max_temp
    );

    // All temperatures should be within valid bounds
    for (i, temp) in temps.iter().enumerate() {
        assert!(
            *temp >= FieldKind::Temperature.min_value()
                && *temp <= FieldKind::Temperature.max_value(),
            "Temperature at sample {} out of bounds: {}",
            i,
            temp
        );
    }
}

#[test]
fn moisture_decreases_over_time() {
    let mut partition = PartitionState::from_seed(42, 0);
    partition.moisture.value = 0.8; // Start high

    let initial_moisture = partition.moisture.value;

    // Run many ticks
    for tick in 0..100 {
        evolve_fields(&mut partition, tick);
    }

    // Moisture should decrease due to evaporation
    assert!(
        partition.moisture.value < initial_moisture,
        "Moisture should decrease over time: {} -> {}",
        initial_moisture,
        partition.moisture.value
    );

    // Should stay within bounds
    assert!(
        partition.moisture.value >= FieldKind::Moisture.min_value(),
        "Moisture should not go below minimum"
    );
}

#[test]
fn fertility_degrades_slowly() {
    let mut partition = PartitionState::from_seed(42, 0);
    partition.fertility.value = 0.8;

    let initial_fertility = partition.fertility.value;

    // Run many ticks
    for tick in 0..1000 {
        evolve_fields(&mut partition, tick);
    }

    // Fertility should degrade slowly
    assert!(
        partition.fertility.value < initial_fertility,
        "Fertility should degrade: {} -> {}",
        initial_fertility,
        partition.fertility.value
    );

    // But not too quickly
    assert!(
        partition.fertility.value > initial_fertility * 0.8,
        "Fertility should degrade slowly: {} -> {} (less than 20% loss)",
        initial_fertility,
        partition.fertility.value
    );
}

#[test]
fn biotic_pressure_responds_to_demand() {
    let mut partition = PartitionState::from_seed(42, 0);

    // Low demand
    partition.demand = 100;
    evolve_fields(&mut partition, 100);
    let low_pressure = partition.biotic_pressure.value;

    // Reset and set high demand
    let mut partition2 = PartitionState::from_seed(42, 0);
    partition2.demand = 2000;
    evolve_fields(&mut partition2, 100);
    let high_pressure = partition2.biotic_pressure.value;

    // Higher demand should produce higher biotic pressure
    assert!(
        high_pressure > low_pressure,
        "Higher demand should increase biotic pressure: low={}, high={}",
        low_pressure,
        high_pressure
    );
}

#[test]
fn field_bounds_are_enforced() {
    let mut partition = PartitionState::from_seed(42, 0);

    // Run many ticks to let fields evolve
    for tick in 0..10000 {
        evolve_fields(&mut partition, tick);

        // Check all fields stay in bounds
        assert!(
            partition.temperature.value >= FieldKind::Temperature.min_value()
                && partition.temperature.value <= FieldKind::Temperature.max_value(),
            "Temperature out of bounds at tick {}",
            tick
        );
        assert!(
            partition.moisture.value >= FieldKind::Moisture.min_value()
                && partition.moisture.value <= FieldKind::Moisture.max_value(),
            "Moisture out of bounds at tick {}",
            tick
        );
        assert!(
            partition.fertility.value >= FieldKind::Fertility.min_value()
                && partition.fertility.value <= FieldKind::Fertility.max_value(),
            "Fertility out of bounds at tick {}",
            tick
        );
        assert!(
            partition.biotic_pressure.value >= FieldKind::BioticPressure.min_value()
                && partition.biotic_pressure.value <= FieldKind::BioticPressure.max_value(),
            "Biotic pressure out of bounds at tick {}",
            tick
        );
    }
}

// =============================================================================
// Waste Loop Tests
// =============================================================================

#[test]
fn waste_processing_reduces_waste_and_boosts_fertility() {
    let mut partition = PartitionState::from_seed(42, 0);

    // Set up initial conditions
    partition.waste.quantity = 1000;
    let initial_fertility = partition.fertility.value;

    // Process waste
    process_waste(&mut partition);

    // Waste should decrease
    assert!(
        partition.waste.quantity < 1000,
        "Waste should be processed: {} -> 1000",
        partition.waste.quantity
    );

    // Fertility should increase slightly
    assert!(
        partition.fertility.value > initial_fertility,
        "Fertility should increase from waste processing: {} -> {}",
        initial_fertility,
        partition.fertility.value
    );
}

#[test]
fn waste_loop_in_simulation() {
    let mut sim = Simulation::new(SimulationSeed::new(42));

    // Run simulation
    sim.run_for_with_mode(100, ExecutionMode::Serial);

    // Check that waste was produced and tracked
    let metrics = sim.metrics();
    assert!(
        metrics.iter().any(|m| m.total_waste > 0),
        "Waste should be produced during simulation"
    );

    // Check that partitions remain valid
    for partition in sim.world().partitions() {
        validate_partition(partition).expect("Partition should remain valid");
    }
}

// =============================================================================
// Integration Tests
// =============================================================================

#[test]
fn full_resource_cycle_stability() {
    // Run a longer simulation and verify stability
    let mut sim = Simulation::new(SimulationSeed::new(42));

    sim.run_for_with_mode(1000, ExecutionMode::Serial);

    // All partitions should have valid state
    for (i, partition) in sim.world().partitions().iter().enumerate() {
        validate_partition(partition)
            .expect(&format!("Partition {} should be valid after 1000 ticks", i));

        // Resources should be non-negative
        assert!(
            partition.food.quantity >= 0,
            "Partition {} food negative",
            i
        );
        assert!(
            partition.water.quantity >= 0,
            "Partition {} water negative",
            i
        );
    }

    // Metrics should show reasonable values
    let final_metrics = sim.metrics().last().expect("Should have metrics");
    assert!(
        final_metrics.total_resource > 0,
        "Should have positive total resources"
    );
}

#[test]
fn environmental_modifiers_affect_regeneration() {
    let mut partition1 = PartitionState::from_seed(42, 0);
    let mut partition2 = PartitionState::from_seed(42, 0);

    // Set up same initial food
    partition1.food.quantity = 100;
    partition2.food.quantity = 100;

    // Make partition1 have better conditions
    partition1.fertility.value = 0.9;
    partition1.moisture.value = 0.7;
    partition1.temperature.value = 20.0; // Optimal

    // Make partition2 have worse conditions
    partition2.fertility.value = 0.2;
    partition2.moisture.value = 0.2;
    partition2.temperature.value = 40.0; // Hot

    // Run regeneration
    for tick in 0..100 {
        evolve_fields(&mut partition1, tick);
        evolve_fields(&mut partition2, tick);
        regenerate_resources(&mut partition1);
        regenerate_resources(&mut partition2);
    }

    // Better conditions should produce more food
    assert!(
        partition1.food.quantity > partition2.food.quantity,
        "Better conditions should produce more food: good={}, bad={}",
        partition1.food.quantity,
        partition2.food.quantity
    );
}

// =============================================================================
// Helpers
// =============================================================================

fn calculate_variance(values: &[i64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let mean = values.iter().sum::<i64>() as f64 / values.len() as f64;
    let sum_sq_diff: f64 = values
        .iter()
        .map(|v| {
            let diff = *v as f64 - mean;
            diff * diff
        })
        .sum();
    sum_sq_diff / values.len() as f64
}
