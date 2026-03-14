use polis_core::SimulationSeed;
use polis_sim::{ExecutionMode, Simulation};

#[test]
fn serial_and_parallel_match_end_to_end() {
    let mut serial = Simulation::new(SimulationSeed::new(42));
    let mut parallel = Simulation::new(SimulationSeed::new(42));

    let a = serial.run_for_with_mode(1_000, ExecutionMode::Serial);
    let b = parallel.run_for_with_mode(1_000, ExecutionMode::Parallel);

    assert_eq!(a, b);
    assert_eq!(serial.events(), parallel.events());
    assert_eq!(serial.metrics(), parallel.metrics());
}

#[test]
fn event_and_metric_counts_scale_with_ticks() {
    let mut sim = Simulation::new(SimulationSeed::new(99));
    let run = sim.run_for_with_mode(250, ExecutionMode::Serial);
    // Per tick: 1 start + 3 phases + 1 complete = 5 events.
    assert_eq!(run.event_count, 250 * 5);
    assert_eq!(run.metric_count, 250);
}
