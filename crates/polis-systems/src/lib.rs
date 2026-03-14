pub struct SystemsModule;

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
