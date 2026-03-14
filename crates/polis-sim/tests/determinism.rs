use polis_core::SimulationSeed;
use polis_sim::{ExecutionMode, SimEvent, Simulation};

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
    // Core scheduler events are always present:
    // Per tick: 1 start + 3 phases + 1 complete = 5 events.
    // Phase 4 may add additional social/cross-species events.
    let tick_started = sim
        .events()
        .iter()
        .filter(|e| matches!(e, SimEvent::TickStarted { .. }))
        .count() as u64;
    let phase_applied = sim
        .events()
        .iter()
        .filter(|e| matches!(e, SimEvent::PhaseApplied { .. }))
        .count() as u64;
    let tick_completed = sim
        .events()
        .iter()
        .filter(|e| matches!(e, SimEvent::TickCompleted { .. }))
        .count() as u64;

    assert_eq!(tick_started, 250, "Should have 250 TickStarted events");
    assert_eq!(
        phase_applied,
        250 * 3,
        "Should have 750 PhaseApplied events"
    );
    assert_eq!(tick_completed, 250, "Should have 250 TickCompleted events");
    assert!(
        run.event_count >= 250 * 5,
        "Should have at least 1250 total events (including social/cross-species)"
    );
    assert_eq!(
        run.metric_count, 250,
        "Should have 250 metrics (one per tick)"
    );
}
