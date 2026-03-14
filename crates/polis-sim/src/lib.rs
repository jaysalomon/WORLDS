use std::fs;
use std::path::Path;

use polis_core::SimulationSeed;
use rayon::prelude::*;
use serde::Deserialize;
use thiserror::Error;

pub struct SimModule;

impl SimModule {
    pub const fn name() -> &'static str {
        "polis-sim"
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct ScenarioConfig {
    pub name: String,
    pub seed: u64,
    pub notes: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimState {
    pub tick: u64,
    pub state_hash: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunSummary {
    pub seed: SimulationSeed,
    pub ticks: u64,
    pub final_state_hash: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
}

pub struct Simulation {
    seed: SimulationSeed,
    state: SimState,
}

impl Simulation {
    pub fn from_scenario_file(path: impl AsRef<Path>) -> Result<Self, SimError> {
        let path_ref = path.as_ref();
        let path_string = path_ref.display().to_string();
        let scenario_str =
            fs::read_to_string(path_ref).map_err(|source| SimError::ScenarioRead {
                path: path_string.clone(),
                source,
            })?;

        let scenario: ScenarioConfig =
            ron::from_str(&scenario_str).map_err(|source| SimError::ScenarioParse {
                path: path_string,
                source,
            })?;

        Ok(Self::new(SimulationSeed::new(scenario.seed)))
    }

    pub const fn new(seed: SimulationSeed) -> Self {
        Self {
            seed,
            state: SimState {
                tick: 0,
                state_hash: seed.0 ^ 0x9E37_79B9_7F4A_7C15,
            },
        }
    }

    pub const fn seed(&self) -> SimulationSeed {
        self.seed
    }

    pub const fn state(&self) -> SimState {
        self.state
    }

    pub fn step(&mut self) {
        self.state.tick = self.state.tick.wrapping_add(1);
        self.state.state_hash = step_hash(self.seed.0, self.state.tick, self.state.state_hash);
    }

    pub fn run_for(&mut self, ticks: u64) -> RunSummary {
        for _ in 0..ticks {
            self.step();
        }

        RunSummary {
            seed: self.seed,
            ticks: self.state.tick,
            final_state_hash: self.state.state_hash,
        }
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
            sim.run_for(ticks)
        })
        .collect()
}

pub fn run_seed_batch_parallel(base_seed: u64, batch_size: u64, ticks: u64) -> Vec<RunSummary> {
    let mut results: Vec<RunSummary> = (0..batch_size)
        .into_par_iter()
        .map(|offset| {
            let seed = SimulationSeed::new(base_seed.wrapping_add(offset));
            let mut sim = Simulation::new(seed);
            sim.run_for(ticks)
        })
        .collect();

    // Keep output order stable for deterministic research pipelines.
    results.sort_unstable_by_key(|summary| summary.seed.0);
    results
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

#[cfg(test)]
mod tests {
    use super::{
        ExecutionMode, ScenarioConfig, Simulation, run_seed_batch, run_seed_batch_parallel,
        run_seed_batch_serial,
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
                notes: "Placeholder scenario used while the runtime is being built.",
            )
        "#;

        let parsed: ScenarioConfig = ron::from_str(scenario_str).expect("valid scenario schema");
        assert_eq!(parsed.seed, 42);
        assert_eq!(parsed.name, "default");
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
}
