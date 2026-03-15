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

/// Experiment specification per 08_ValidationAndExperiments.md Section 11
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExperimentSpec {
    /// Research question or hypothesis
    pub research_question: Option<String>,
    /// Scenario family identifier
    pub scenario_family: Option<String>,
    /// Control case identifier
    pub control_case: Option<String>,
    /// Treatment variables applied
    pub treatment_variables: Vec<TreatmentVariable>,
    /// Parameter ranges explored
    pub parameter_ranges: Vec<ParameterRange>,
    /// Ensemble size
    pub ensemble_size: Option<u64>,
    /// Primary metrics for analysis
    pub primary_metrics: Vec<String>,
    /// Planned analysis method
    pub planned_analysis: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreatmentVariable {
    pub name: String,
    pub value: f64,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterRange {
    pub name: String,
    pub min: f64,
    pub max: f64,
    pub step: Option<f64>,
}

/// Complete experiment output package per 08_ValidationAndExperiments.md Section 14.4
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentBundle {
    // Provenance (Section 13.1, 14.4)
    pub name: String,
    pub description: String,
    pub created_at: u64,
    /// Model version
    pub model_version: String,
    /// Schema version
    pub schema_version: String,

    // Run parameters
    pub seed: u64,
    pub ticks: u64,
    pub partition_count: u64,
    pub execution_mode: String,
    pub final_state_hash: u64,
    pub state_hash_series: Vec<u64>,

    // Experiment specification
    pub experiment_spec: Option<ExperimentSpec>,

    // Outputs
    pub metrics: Option<MetricsBundle>,
    pub snapshots: Vec<Snapshot>,
    pub events_file: Option<String>,
    pub manifest: Option<ManifestBundle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestBundle {
    pub scenario_name: String,
    pub scenario_hash: String,
    pub enabled_subsystems: EnabledSubsystemsExport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnabledSubsystemsExport {
    pub world_substrate: bool,
    pub agents: bool,
    pub social_fabric: bool,
    pub collective_agency: bool,
    pub discovery: bool,
    pub biology: bool,
    pub inference: bool,
}

impl ExperimentBundle {
    pub fn new(
        name: &str,
        description: &str,
        model_version: &str,
        schema_version: &str,
    ) -> Self {
        let created_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Self {
            name: name.to_string(),
            description: description.to_string(),
            created_at,
            model_version: model_version.to_string(),
            schema_version: schema_version.to_string(),
            seed: 0,
            ticks: 0,
            partition_count: 0,
            execution_mode: "serial".to_string(),
            final_state_hash: 0,
            state_hash_series: Vec::new(),
            experiment_spec: None,
            metrics: None,
            snapshots: Vec::new(),
            events_file: None,
            manifest: None,
        }
    }

    pub fn with_run_params(
        mut self,
        seed: u64,
        ticks: u64,
        partition_count: u64,
        execution_mode: &str,
        final_state_hash: u64,
        state_hash_series: Vec<u64>,
    ) -> Self {
        self.seed = seed;
        self.ticks = ticks;
        self.partition_count = partition_count;
        self.execution_mode = execution_mode.to_string();
        self.final_state_hash = final_state_hash;
        self.state_hash_series = state_hash_series;
        self
    }

    pub fn with_experiment_spec(mut self, spec: ExperimentSpec) -> Self {
        self.experiment_spec = Some(spec);
        self
    }

    pub fn with_metrics(mut self, metrics: MetricsBundle) -> Self {
        self.metrics = Some(metrics);
        self
    }

    pub fn with_events_file(mut self, path: &str) -> Self {
        self.events_file = Some(path.to_string());
        self
    }

    pub fn with_manifest(mut self, manifest: ManifestBundle) -> Self {
        self.manifest = Some(manifest);
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

/// Detailed reproducibility report with series comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReproducibilityReport {
    pub seed: u64,
    pub ticks: u64,
    /// Original final state hash
    pub original_final_hash: u64,
    /// Verification final state hash
    pub verification_final_hash: u64,
    /// Whether final hash matches
    pub is_verified: bool,
    /// State hash series comparison (if available)
    pub series_comparison: Option<SeriesComparison>,
    /// First tick where hashes diverged (if any)
    pub first_divergence_tick: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeriesComparison {
    pub original_series: Vec<u64>,
    pub verification_series: Vec<u64>,
    pub match_count: u64,
    pub total_ticks: u64,
    pub all_match: bool,
}

impl ReproducibilityReport {
    /// Verify with just final hash (backward compatible)
    pub fn verify_single(
        original: u64,
        verification: u64,
        seed: u64,
        ticks: u64,
    ) -> Self {
        let is_verified = original == verification;
        Self {
            seed,
            ticks,
            original_final_hash: original,
            verification_final_hash: verification,
            is_verified,
            series_comparison: None,
            first_divergence_tick: None,
        }
    }

    /// Verify with full series comparison
    pub fn verify_series(
        original_series: &[u64],
        verification_series: &[u64],
        seed: u64,
        ticks: u64,
    ) -> Self {
        let original_final = original_series.last().copied().unwrap_or(0);
        let verification_final = verification_series.last().copied().unwrap_or(0);

        // Find first divergence point
        let mut first_divergence: Option<u64> = None;
        let matching: Vec<_> = original_series
            .iter()
            .zip(verification_series.iter())
            .take(ticks as usize)
            .enumerate()
            .filter_map(|(i, (a, b))| {
                if first_divergence.is_none() && a != b {
                    first_divergence = Some(i as u64);
                }
                if a == b { Some(i) } else { None }
            })
            .collect();
        let match_count = matching.len() as u64;

        let total_ticks = ticks.min(
            original_series.len().min(verification_series.len()) as u64
        );

        // For true reproducibility, series must match at all ticks (no divergence)
        let is_verified = first_divergence.is_none();

        let series_comparison = Some(SeriesComparison {
            original_series: original_series.to_vec(),
            verification_series: verification_series.to_vec(),
            match_count,
            total_ticks,
            all_match: first_divergence.is_none(),
        });

        Self {
            seed,
            ticks,
            original_final_hash: original_final,
            verification_final_hash: verification_final,
            is_verified,
            series_comparison,
            first_divergence_tick: first_divergence,
        }
    }

    /// Summary for display
    pub fn summary(&self) -> String {
        if self.is_verified {
            format!(
                "Reproducible: seed={}, ticks={}, verified={}",
                self.seed, self.ticks, self.is_verified
            )
        } else {
            format!(
                "NOT reproducible: seed={}, ticks={}, diverged at tick {:?}",
                self.seed, self.ticks, self.first_divergence_tick
            )
        }
    }
}

/// Verify reproducibility by re-running and comparing hashes (backward compatible)
pub fn verify_reproducibility(
    seed: u64,
    ticks: u64,
    original_hash: u64,
    revalidate: impl Fn(u64, u64) -> u64,
) -> ReproducibilityReport {
    let verification_hash = revalidate(seed, ticks);
    ReproducibilityReport::verify_single(original_hash, verification_hash, seed, ticks)
}

/// Verify with full series comparison
pub fn verify_reproducibility_series(
    seed: u64,
    original_series: &[u64],
    revalidate: impl Fn(u64, u64) -> Vec<u64>,
) -> ReproducibilityReport {
    let verification_series = revalidate(seed, original_series.len() as u64);
    ReproducibilityReport::verify_series(original_series, &verification_series, seed, original_series.len() as u64)
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

        let report = ReproducibilityReport::verify_single(0x1234, 0x1234, 42, 100);
        assert!(report.is_verified);
    }

    #[test]
    fn reproducibility_report_detects_mismatch() {
        use super::ReproducibilityReport;

        let report = ReproducibilityReport::verify_single(0x1234, 0x5678, 42, 100);
        assert!(!report.is_verified);
    }

    #[test]
    fn reproducibility_report_series_match() {
        use super::ReproducibilityReport;

        let original = vec![1, 2, 3, 4, 5];
        let verification = vec![1, 2, 3, 4, 5];
        let report = ReproducibilityReport::verify_series(&original, &verification, 42, 5);
        assert!(report.is_verified);
        assert!(report.first_divergence_tick.is_none());
        assert_eq!(report.series_comparison.as_ref().unwrap().all_match, true);
    }

    #[test]
    fn reproducibility_report_series_detects_divergence() {
        use super::ReproducibilityReport;

        let original = vec![1, 2, 3, 4, 5];
        let verification = vec![1, 2, 99, 4, 5];
        let report = ReproducibilityReport::verify_series(&original, &verification, 42, 5);
        assert!(!report.is_verified);
        assert_eq!(report.first_divergence_tick, Some(2));
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

        let bundle = ExperimentBundle::new("test_experiment", "Test description", "0.1.0", "0.3.0")
            .with_run_params(42, 1000, 64, "serial", 0xDEADBEEF, vec![1, 2, 3]);

        assert_eq!(bundle.name, "test_experiment");
        assert_eq!(bundle.seed, 42);
        assert_eq!(bundle.ticks, 1000);
        assert_eq!(bundle.model_version, "0.1.0");
        assert_eq!(bundle.schema_version, "0.3.0");
    }
}
