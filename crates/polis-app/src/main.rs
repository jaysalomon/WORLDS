use std::collections::BTreeMap;
use std::env;
use std::path::PathBuf;

use polis_export::{write_json_file, write_jsonl_file};
use polis_sim::{
    ExecutionMode, Simulation, build_run_manifest, load_scenario_file, run_seed_batch,
};

fn main() {
    let cli = Cli::from_env();

    // Windowed mode - run presentation shell
    if cli.windowed {
        run_windowed(cli);
        return;
    }

    // Headless mode - run simulation and export
    run_headless(cli);
}

fn run_headless(cli: Cli) {
    let scenario_path = PathBuf::from("scenarios/default.ron");
    let scenario = match load_scenario_file(&scenario_path) {
        Ok(scenario) => scenario,
        Err(err) => {
            eprintln!("failed to load scenario: {err}");
            std::process::exit(1);
        }
    };

    let mut sim = if let Some(checkpoint_path) = &cli.load_checkpoint {
        match Simulation::load_checkpoint(checkpoint_path) {
            Ok(sim) => sim,
            Err(err) => {
                eprintln!("failed to load checkpoint: {err}");
                std::process::exit(1);
            }
        }
    } else {
        Simulation::new_with_partition_count(
            polis_core::SimulationSeed::new(scenario.seed),
            scenario.partition_count,
        )
    };

    println!("Scenario: {}", scenario_path.display());
    println!("Scenario name: {}", scenario.name);
    println!("Partition count: {}", scenario.partition_count);

    if cli.batch <= 1 {
        let mode = if cli.parallel {
            ExecutionMode::Parallel
        } else {
            ExecutionMode::Serial
        };
        let summary = sim.run_for_with_mode(cli.ticks, mode);
        let manifest = build_run_manifest(&scenario, summary, mode);
        println!("Mode: single-run");
        println!("Execution: {}", manifest.execution_mode);
        println!("Seed: {}", manifest.seed);
        println!("Ticks: {}", manifest.ticks);
        println!("Final state hash: {}", manifest.final_state_hash);
        println!("Event count: {}", manifest.event_count);
        println!("Metric count: {}", manifest.metric_count);
        println!("Runtime status: {}", manifest.workspace_status);

        if let Some(checkpoint_path) = &cli.save_checkpoint {
            if let Err(err) = sim.save_checkpoint(checkpoint_path) {
                eprintln!("failed to save checkpoint: {err}");
                std::process::exit(1);
            }
            println!("Checkpoint saved: {}", checkpoint_path.display());
        }

        if let Some(export_dir) = &cli.export_dir {
            if let Err(err) = write_json_file(export_dir, "manifest.json", &manifest) {
                eprintln!("failed to export manifest: {err}");
                std::process::exit(1);
            }
            if let Err(err) = write_jsonl_file(export_dir, "events.jsonl", sim.events()) {
                eprintln!("failed to export events: {err}");
                std::process::exit(1);
            }
            if let Err(err) = write_jsonl_file(export_dir, "metrics.jsonl", sim.metrics()) {
                eprintln!("failed to export metrics: {err}");
                std::process::exit(1);
            }

            let checkpoint_file = export_dir.join("checkpoint.json");
            if let Err(err) = sim.save_checkpoint(&checkpoint_file) {
                eprintln!("failed to export checkpoint: {err}");
                std::process::exit(1);
            }

            let mut bundle_index = BTreeMap::new();
            bundle_index.insert("manifest".to_string(), "manifest.json".to_string());
            bundle_index.insert("events".to_string(), "events.jsonl".to_string());
            bundle_index.insert("metrics".to_string(), "metrics.jsonl".to_string());
            bundle_index.insert("checkpoint".to_string(), "checkpoint.json".to_string());
            if let Err(err) = write_json_file(export_dir, "bundle-index.json", &bundle_index) {
                eprintln!("failed to export bundle index: {err}");
                std::process::exit(1);
            }
            println!("Export dir: {}", export_dir.display());
        }
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
    println!(
        "Events per run: {}",
        runs.first().map_or(0, |run| run.event_count)
    );
    println!(
        "Metrics per run: {}",
        runs.first().map_or(0, |run| run.metric_count)
    );

    if let Some(export_dir) = &cli.export_dir {
        if let Err(err) = write_jsonl_file(export_dir, "batch-run-summaries.jsonl", &runs) {
            eprintln!("failed to export batch summaries: {err}");
            std::process::exit(1);
        }
        let mut bundle_index = BTreeMap::new();
        bundle_index.insert(
            "batch_summaries".to_string(),
            "batch-run-summaries.jsonl".to_string(),
        );
        if let Err(err) = write_json_file(export_dir, "bundle-index.json", &bundle_index) {
            eprintln!("failed to export bundle index: {err}");
            std::process::exit(1);
        }
        println!("Export dir: {}", export_dir.display());
    }
}

#[cfg(feature = "windowed")]
fn run_windowed(cli: Cli) {
    use polis_frontend::run_presentation_shell;
    use polis_sim::Simulation;

    let scenario_path = PathBuf::from("scenarios/default.ron");
    let scenario = match load_scenario_file(&scenario_path) {
        Ok(scenario) => scenario,
        Err(err) => {
            eprintln!("failed to load scenario: {err}");
            std::process::exit(1);
        }
    };

    let sim = if let Some(checkpoint_path) = &cli.load_checkpoint {
        match Simulation::load_checkpoint(checkpoint_path) {
            Ok(sim) => sim,
            Err(err) => {
                eprintln!("failed to load checkpoint: {err}");
                std::process::exit(1);
            }
        }
    } else {
        Simulation::new_with_partition_count(
            polis_core::SimulationSeed::new(scenario.seed),
            scenario.partition_count,
        )
    };

    println!("Starting POLIS Presentation Shell...");
    println!("Controls:");
    println!("  SPACE - Pause/Resume");
    println!("  S - Step single tick");
    println!("  1/2/3/4 - Set speed (1x/10x/60x/MAX)");
    println!("  R - Toggle resource overlay");
    println!("  F - Toggle field overlay");
    println!("  D - Toggle demand overlay");
    println!("  N - No overlay");
    println!("  Click - Select partition for details");
    println!("  ESC - Exit");

    // macroquad uses its own async runtime
    pollster::block_on(async {
        run_presentation_shell(sim).await;
    });
}

#[cfg(not(feature = "windowed"))]
fn run_windowed(_cli: Cli) {
    eprintln!("Windowed mode requires the 'windowed' feature to be enabled.");
    eprintln!("Build with: cargo run --features windowed -- --windowed");
    std::process::exit(1);
}

#[derive(Debug, Clone)]
struct Cli {
    ticks: u64,
    batch: u64,
    parallel: bool,
    export_dir: Option<PathBuf>,
    save_checkpoint: Option<PathBuf>,
    load_checkpoint: Option<PathBuf>,
    windowed: bool,
}

impl Cli {
    fn from_env() -> Self {
        let mut ticks = 1_000_u64;
        let mut batch = 1_u64;
        let mut parallel = false;
        let mut export_dir: Option<PathBuf> = None;
        let mut save_checkpoint: Option<PathBuf> = None;
        let mut load_checkpoint: Option<PathBuf> = None;
        let mut windowed = false;

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
                "--export-dir" => {
                    if let Some(value) = args.next() {
                        export_dir = Some(PathBuf::from(value));
                    }
                }
                "--save-checkpoint" => {
                    if let Some(value) = args.next() {
                        save_checkpoint = Some(PathBuf::from(value));
                    }
                }
                "--load-checkpoint" => {
                    if let Some(value) = args.next() {
                        load_checkpoint = Some(PathBuf::from(value));
                    }
                }
                "--windowed" => {
                    windowed = true;
                }
                _ => {}
            }
        }

        Self {
            ticks,
            batch,
            parallel,
            export_dir,
            save_checkpoint,
            load_checkpoint,
            windowed,
        }
    }
}
