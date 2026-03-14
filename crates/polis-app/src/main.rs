use std::env;
use std::path::PathBuf;

use polis_sim::{ExecutionMode, Simulation, run_seed_batch};

fn main() {
    let cli = Cli::from_env();
    let scenario_path = PathBuf::from("scenarios/default.ron");

    let mut sim = match Simulation::from_scenario_file(&scenario_path) {
        Ok(sim) => sim,
        Err(err) => {
            eprintln!("failed to start simulation: {err}");
            std::process::exit(1);
        }
    };

    println!("POLIS runtime status: {}", polis_core::workspace_status());
    println!("Scenario: {}", scenario_path.display());

    if cli.batch <= 1 {
        let summary = sim.run_for(cli.ticks);
        println!("Mode: single-run");
        println!("Seed: {}", summary.seed.0);
        println!("Ticks: {}", summary.ticks);
        println!("Final state hash: {}", summary.final_state_hash);
        return;
    }

    let mode = if cli.parallel {
        ExecutionMode::Parallel
    } else {
        ExecutionMode::Serial
    };
    let base_seed = sim.seed().0;
    let runs = run_seed_batch(base_seed, cli.batch, cli.ticks, mode);

    println!("Mode: batch");
    println!("Execution: {:?}", mode);
    println!("Base seed: {}", base_seed);
    println!("Batch size: {}", cli.batch);
    println!("Ticks per run: {}", cli.ticks);
    println!(
        "First run hash: {}",
        runs.first().map_or(0, |run| run.final_state_hash)
    );
    println!(
        "Last run hash: {}",
        runs.last().map_or(0, |run| run.final_state_hash)
    );
}

#[derive(Debug, Clone, Copy)]
struct Cli {
    ticks: u64,
    batch: u64,
    parallel: bool,
}

impl Cli {
    fn from_env() -> Self {
        let mut ticks = 1_000_u64;
        let mut batch = 1_u64;
        let mut parallel = false;

        let mut args = env::args().skip(1);
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--ticks" => {
                    if let Some(value) = args.next() {
                        if let Ok(parsed) = value.parse::<u64>() {
                            ticks = parsed.max(1);
                        }
                    }
                }
                "--batch" => {
                    if let Some(value) = args.next() {
                        if let Ok(parsed) = value.parse::<u64>() {
                            batch = parsed.max(1);
                        }
                    }
                }
                "--parallel" => {
                    parallel = true;
                }
                _ => {}
            }
        }

        Self {
            ticks,
            batch,
            parallel,
        }
    }
}
