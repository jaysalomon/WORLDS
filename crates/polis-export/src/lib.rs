use std::collections::HashMap;

// Re-export for convenience
pub use serde::{Deserialize, Serialize};

use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use thiserror::Error;

use polis_core::SimulationSeed;

// =============================================================================
// Module
// =============================================================================

pub struct ExportModule;

impl ExportModule {
    pub const fn name() -> &'static str {
        "polis-export"
    }
}

#[derive(Debug, Error)]
pub enum ExportError {
    #[error("failed to create export directory '{path}': {source}")]
    CreateDir {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to create export file '{path}': {source}")]
    CreateFile {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to write export file '{path}': {source}")]
    WriteFile {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to serialize JSON for '{path}': {source}")]
    Serialize {
        path: String,
        #[source]
        source: serde_json::Error,
    },
}

pub fn ensure_export_dir(dir: impl AsRef<Path>) -> Result<(), ExportError> {
    let dir_ref = dir.as_ref();
    fs::create_dir_all(dir_ref).map_err(|source| ExportError::CreateDir {
        path: dir_ref.display().to_string(),
        source,
    })
}

pub fn write_json_file<T: Serialize>(
    dir: impl AsRef<Path>,
    file_name: &str,
    value: &T,
) -> Result<(), ExportError> {
    ensure_export_dir(&dir)?;
    let path = dir.as_ref().join(file_name);
    let path_string = path.display().to_string();
    let file = File::create(&path).map_err(|source| ExportError::CreateFile {
        path: path_string.clone(),
        source,
    })?;
    let mut writer = BufWriter::new(file);
    let payload = serde_json::to_vec_pretty(value).map_err(|source| ExportError::Serialize {
        path: path_string.clone(),
        source,
    })?;
    writer
        .write_all(&payload)
        .map_err(|source| ExportError::WriteFile {
            path: path_string.clone(),
            source,
        })?;
    writer
        .write_all(b"\n")
        .map_err(|source| ExportError::WriteFile {
            path: path_string,
            source,
        })?;
    Ok(())
}

pub fn write_jsonl_file<T: Serialize>(
    dir: impl AsRef<Path>,
    file_name: &str,
    rows: &[T],
) -> Result<(), ExportError> {
    ensure_export_dir(&dir)?;
    let path = dir.as_ref().join(file_name);
    let path_string = path.display().to_string();
    let file = File::create(&path).map_err(|source| ExportError::CreateFile {
        path: path_string.clone(),
        source,
    })?;
    let mut writer = BufWriter::new(file);

    for row in rows {
        let line = serde_json::to_vec(row).map_err(|source| ExportError::Serialize {
            path: path_string.clone(),
            source,
        })?;
        writer
            .write_all(&line)
            .map_err(|source| ExportError::WriteFile {
                path: path_string.clone(),
                source,
            })?;
        writer
            .write_all(b"\n")
            .map_err(|source| ExportError::WriteFile {
                path: path_string.clone(),
                source,
            })?;
    }

    Ok(())
}

// =============================================================================
// Metrics Export
// =============================================================================

/// Time-series metrics recorded during a simulation run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunMetrics {
    pub tick: u64,
    pub state_hash: u64,
    pub phase_deltas: Vec<PhaseDelta>,
    pub custom_metrics: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseDelta {
    pub phase: String,
    pub delta: u64,
}

impl RunMetrics {
    pub fn new(tick: u64, state_hash: u64) -> Self {
        Self {
            tick,
            state_hash,
            phase_deltas: Vec::new(),
            custom_metrics: HashMap::new(),
        }
    }

    pub fn with_phase(mut self, phase: &str, delta: u64) -> Self {
        self.phase_deltas.push(PhaseDelta {
            phase: phase.to_string(),
            delta,
        });
        self
    }

    pub fn with_metric(mut self, name: &str, value: f64) -> Self {
        self.custom_metrics.insert(name.to_string(), value);
        self
    }
}

/// Bundle of all metrics from a run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsBundle {
    pub seed: u64,
    pub total_ticks: u64,
    pub final_state_hash: u64,
    pub state_hash_series: Vec<u64>,
    pub custom_series: HashMap<String, Vec<(u64, f64)>>,
}

impl MetricsBundle {
    pub fn new(seed: u64, total_ticks: u64, final_state_hash: u64) -> Self {
        Self {
            seed,
            total_ticks,
            final_state_hash,
            state_hash_series: Vec::new(),
            custom_series: HashMap::new(),
        }
    }

    pub fn record_state_hash(&mut self, tick: u64, hash: u64) {
        // Ensure we record in order
        if self.state_hash_series.len() == tick as usize {
            self.state_hash_series.push(hash);
        }
    }

    pub fn record_metric(&mut self, name: &str, tick: u64, value: f64) {
        self.custom_series
            .entry(name.to_string())
            .or_insert_with(Vec::new)
            .push((tick, value));
    }
}

// =============================================================================
// Snapshot / Checkpoint Export
// =============================================================================

/// Complete state snapshot for resumable execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub version: String,
    pub tick: u64,
    pub seed: u64,
    pub state_hash: u64,
    pub data: SnapshotData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SnapshotData {
    /// Placeholder until real state is implemented
    Placeholder {
        partition_count: u64,
        world_hash: u64,
    },
}

impl Snapshot {
    pub fn new(
        tick: u64,
        seed: SimulationSeed,
        state_hash: u64,
        partition_count: u64,
        world_hash: u64,
    ) -> Self {
        Self {
            version: "0.1.0".to_string(),
            tick,
            seed: seed.0,
            state_hash,
            data: SnapshotData::Placeholder {
                partition_count,
                world_hash,
            },
        }
    }
}

/// Export snapshot to JSON file
pub fn export_snapshot_json(
    snapshot: &Snapshot,
    path: impl AsRef<Path>,
) -> Result<(), ExportError> {
    ensure_export_dir(path.as_ref().parent().unwrap())?;
    write_json_file(
        path.as_ref().parent().unwrap(),
        path.as_ref().file_name().unwrap().to_str().unwrap(),
        snapshot,
    )
}

/// Load snapshot from JSON file
pub fn load_snapshot_json(path: impl AsRef<Path>) -> Result<Snapshot, ExportError> {
    let path_ref = path.as_ref();
    let content = fs::read_to_string(path_ref).map_err(|source| ExportError::WriteFile {
        path: path_ref.display().to_string(),
        source,
    })?;
    serde_json::from_str(&content).map_err(|source| ExportError::Serialize {
        path: path_ref.display().to_string(),
        source,
    })
}

// =============================================================================
// Experiment Bundle
// =============================================================================

/// Complete experiment output package
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentBundle {
    pub name: String,
    pub description: String,
    pub created_at: u64,
    pub seed: u64,
    pub ticks: u64,
    pub final_state_hash: u64,
    pub metrics: Option<MetricsBundle>,
    pub snapshots: Vec<Snapshot>,
}

impl ExperimentBundle {
    pub fn new(
        name: &str,
        description: &str,
        seed: u64,
        ticks: u64,
        final_state_hash: u64,
    ) -> Self {
        let created_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Self {
            name: name.to_string(),
            description: description.to_string(),
            created_at,
            seed,
            ticks,
            final_state_hash,
            metrics: None,
            snapshots: Vec::new(),
        }
    }

    pub fn with_metrics(mut self, metrics: MetricsBundle) -> Self {
        self.metrics = Some(metrics);
        self
    }

    pub fn add_snapshot(&mut self, snapshot: Snapshot) {
        self.snapshots.push(snapshot);
    }
}

/// Export complete experiment bundle
pub fn export_experiment_bundle(
    bundle: &ExperimentBundle,
    dir: impl AsRef<Path>,
) -> Result<PathBuf, ExportError> {
    let dir_ref = dir.as_ref();
    ensure_export_dir(dir_ref)?;

    // Export main bundle
    write_json_file(dir_ref, "experiment.json", bundle)?;

    Ok(dir_ref.to_path_buf())
}

// =============================================================================
// Reproducibility Verification
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReproducibilityReport {
    pub seed: u64,
    pub tick: u64,
    pub original_hash: u64,
    pub verification_hash: u64,
    pub is_verified: bool,
}

impl ReproducibilityReport {
    pub fn verify(original: u64, verification: u64, seed: u64, tick: u64) -> Self {
        let is_verified = original == verification;
        Self {
            seed,
            tick,
            original_hash: original,
            verification_hash: verification,
            is_verified,
        }
    }
}

/// Verify reproducibility by re-running and comparing hashes
pub fn verify_reproducibility(
    seed: u64,
    ticks: u64,
    original_hash: u64,
    revalidate: impl Fn(u64, u64) -> u64,
) -> ReproducibilityReport {
    let verification_hash = revalidate(seed, ticks);
    ReproducibilityReport::verify(original_hash, verification_hash, seed, ticks)
}

// =============================================================================
// Batch Run Export
// =============================================================================

/// Summary of a single run in a batch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchRunResult {
    pub seed: u64,
    pub ticks: u64,
    pub final_state_hash: u64,
}

/// Results from a batch sweep
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResults {
    pub base_seed: u64,
    pub batch_size: u64,
    pub ticks_per_run: u64,
    pub results: Vec<BatchRunResult>,
}

impl BatchResults {
    pub fn new(base_seed: u64, batch_size: u64, ticks_per_run: u64) -> Self {
        Self {
            base_seed,
            batch_size,
            ticks_per_run,
            results: Vec::new(),
        }
    }

    pub fn add_result(&mut self, result: BatchRunResult) {
        self.results.push(result);
    }

    pub fn all_hashes_match(&self) -> bool {
        if self.results.is_empty() {
            return true;
        }
        let first = self.results[0].final_state_hash;
        self.results.iter().all(|r| r.final_state_hash == first)
    }
}

/// Export batch results
pub fn export_batch_results(
    results: &BatchResults,
    dir: impl AsRef<Path>,
) -> Result<PathBuf, ExportError> {
    let dir_ref = dir.as_ref();
    ensure_export_dir(dir_ref)?;
    write_json_file(dir_ref, "batch_results.json", results)?;
    Ok(dir_ref.to_path_buf())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    use serde::{Deserialize, Serialize};

    use super::{write_json_file, write_jsonl_file};

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct Dummy {
        value: u64,
    }

    fn temp_dir() -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        PathBuf::from(format!("target/export-test-{nonce}"))
    }

    #[test]
    fn writes_json_file() {
        let dir = temp_dir();
        write_json_file(&dir, "manifest.json", &Dummy { value: 7 }).expect("json write");
        let payload = fs::read_to_string(dir.join("manifest.json")).expect("read");
        assert!(payload.contains("\"value\": 7"));
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn writes_jsonl_file() {
        let dir = temp_dir();
        let rows = vec![Dummy { value: 1 }, Dummy { value: 2 }];
        write_jsonl_file(&dir, "events.jsonl", &rows).expect("jsonl write");
        let payload = fs::read_to_string(dir.join("events.jsonl")).expect("read");
        assert!(payload.contains("{\"value\":1}\n"));
        assert!(payload.contains("{\"value\":2}\n"));
        let _ = fs::remove_dir_all(dir);
    }

    // Tests for new Phase 7 functionality

    #[test]
    fn metrics_bundle_records_state_hashes() {
        use super::MetricsBundle;

        let mut bundle = MetricsBundle::new(42, 100, 0x1234);
        bundle.record_state_hash(0, 0xABCD);
        bundle.record_state_hash(1, 0xBCDE);
        bundle.record_state_hash(2, 0xCDEF);

        assert_eq!(bundle.state_hash_series.len(), 3);
        assert_eq!(bundle.state_hash_series[0], 0xABCD);
    }

    #[test]
    fn snapshot_roundtrip() {
        use super::Snapshot;

        let snapshot = Snapshot::new(
            100,
            polis_core::SimulationSeed::new(42),
            0xDEADBEEF,
            64,
            0xCAFEBABE,
        );

        // Serialize to JSON
        let json = serde_json::to_string(&snapshot).expect("serialize");

        // Deserialize back
        let parsed: Snapshot = serde_json::from_str(&json).expect("deserialize");

        assert_eq!(parsed.tick, 100);
        assert_eq!(parsed.seed, 42);
        assert_eq!(parsed.state_hash, 0xDEADBEEF);
    }

    #[test]
    fn reproducibility_report_detects_match() {
        use super::ReproducibilityReport;

        let report = ReproducibilityReport::verify(0x1234, 0x1234, 42, 100);
        assert!(report.is_verified);
    }

    #[test]
    fn reproducibility_report_detects_mismatch() {
        use super::ReproducibilityReport;

        let report = ReproducibilityReport::verify(0x1234, 0x5678, 42, 100);
        assert!(!report.is_verified);
    }

    #[test]
    fn run_metrics_accumulates_data() {
        use super::RunMetrics;

        let metrics = RunMetrics::new(50, 0xABCD)
            .with_phase("Perception", 0x1111)
            .with_phase("Decision", 0x2222)
            .with_metric("population", 150.0);

        assert_eq!(metrics.phase_deltas.len(), 2);
        assert_eq!(metrics.custom_metrics.get("population"), Some(&150.0));
    }

    #[test]
    fn batch_results_all_hashes_match() {
        use super::{BatchResults, BatchRunResult};

        let mut results = BatchResults::new(100, 3, 1000);
        results.add_result(BatchRunResult {
            seed: 100,
            ticks: 1000,
            final_state_hash: 0xABCD,
        });
        results.add_result(BatchRunResult {
            seed: 101,
            ticks: 1000,
            final_state_hash: 0xABCD,
        });
        results.add_result(BatchRunResult {
            seed: 102,
            ticks: 1000,
            final_state_hash: 0xABCD,
        });

        assert!(results.all_hashes_match());
    }

    #[test]
    fn batch_results_detects_hash_mismatch() {
        use super::{BatchResults, BatchRunResult};

        let mut results = BatchResults::new(100, 2, 1000);
        results.add_result(BatchRunResult {
            seed: 100,
            ticks: 1000,
            final_state_hash: 0xABCD,
        });
        results.add_result(BatchRunResult {
            seed: 101,
            ticks: 1000,
            final_state_hash: 0x9876,
        }); // Different!

        assert!(!results.all_hashes_match());
    }

    #[test]
    fn experiment_bundle_creation() {
        use super::ExperimentBundle;

        let bundle =
            ExperimentBundle::new("test_experiment", "Test description", 42, 1000, 0xDEADBEEF);

        assert_eq!(bundle.name, "test_experiment");
        assert_eq!(bundle.seed, 42);
        assert_eq!(bundle.ticks, 1000);
    }
}
