use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SimulationSeed(pub u64);

impl SimulationSeed {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeterministicRng {
    state: u64,
}

impl DeterministicRng {
    pub const fn from_seed(seed: SimulationSeed) -> Self {
        Self { state: seed.0 }
    }

    pub const fn from_u64(seed: u64) -> Self {
        Self { state: seed }
    }

    pub fn next_u64(&mut self) -> u64 {
        // SplitMix64: small, fast, deterministic, and stable for reproducible streams.
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    pub fn next_bounded(&mut self, upper_exclusive: u64) -> u64 {
        if upper_exclusive == 0 {
            return 0;
        }
        self.next_u64() % upper_exclusive
    }
}

/// Model version from Cargo.toml
pub const fn model_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Workspace status indicator
pub const fn workspace_status() -> &'static str {
    "active"
}

/// Enabled subsystems bitflag
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnabledSubsystems {
    pub world_substrate: bool,     // Phase 1: Resources, fields, waste
    pub agents: bool,               // Phase 3: Individual agents
    pub social_fabric: bool,        // Phase 4: Social ties, cross-species
    pub collective_agency: bool,    // Phase 5: Collective actors
    pub discovery: bool,            // Phase 6: Discovery system
    pub biology: bool,             // Phase 6: Biology, domestication
    pub inference: bool,            // Phase 6 Pass 2: PLN inference
}

impl Default for EnabledSubsystems {
    fn default() -> Self {
        // All phases enabled by default
        Self {
            world_substrate: true,
            agents: true,
            social_fabric: true,
            collective_agency: true,
            discovery: true,
            biology: true,
            inference: true,
        }
    }
}

/// Complete run manifest with full provenance metadata per 08_ValidationAndExperiments.md
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunManifest {
    // Provenance metadata (Section 13.1)
    /// Model version from Cargo.toml
    pub model_version: String,
    /// Schema version for event/metric compatibility
    pub schema_version: String,
    /// Scenario name
    pub scenario_name: String,
    /// Random seed used
    pub seed: u64,
    /// Number of partitions
    pub partition_count: u64,
    /// Total ticks executed
    pub ticks: u64,
    /// Final state hash for reproducibility verification
    pub final_state_hash: u64,
    /// Full state hash series for reproducibility verification
    pub state_hash_series: Vec<u64>,
    /// Enabled subsystems
    pub enabled_subsystems: EnabledSubsystems,
    /// Execution mode
    pub execution_mode: String,
    /// Workspace status
    pub workspace_status: String,
    /// Run start timestamp (Unix epoch seconds)
    pub started_at: u64,
    /// Run end timestamp (Unix epoch seconds)
    pub ended_at: u64,
    /// Total events generated
    pub event_count: u64,
    /// Total metrics recorded
    pub metric_count: u64,
}

impl RunManifest {
    /// Create a new manifest with current timestamps
    pub fn new(
        scenario_name: String,
        seed: u64,
        partition_count: u64,
        ticks: u64,
        final_state_hash: u64,
        state_hash_series: Vec<u64>,
        execution_mode: String,
        event_count: u64,
        metric_count: u64,
    ) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Self {
            model_version: model_version().to_string(),
            schema_version: "0.3.0".to_string(),
            scenario_name,
            seed,
            partition_count,
            ticks,
            final_state_hash,
            state_hash_series,
            enabled_subsystems: EnabledSubsystems::default(),
            execution_mode,
            workspace_status: workspace_status().to_string(),
            started_at: now,
            ended_at: now,
            event_count,
            metric_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DeterministicRng, EnabledSubsystems, SimulationSeed, model_version, workspace_status};

    #[test]
    fn seed_round_trips() {
        let seed = SimulationSeed::new(42);
        assert_eq!(seed.0, 42);
    }

    #[test]
    fn reports_active_status() {
        assert_eq!(workspace_status(), "active");
    }

    #[test]
    fn reports_version() {
        let v = model_version();
        assert!(v.len() >= 5); // At least x.y.z
        assert!(v.contains('.'));
    }

    #[test]
    fn deterministic_rng_is_reproducible() {
        let mut a = DeterministicRng::from_u64(42);
        let mut b = DeterministicRng::from_u64(42);
        assert_eq!(a.next_u64(), b.next_u64());
        assert_eq!(a.next_u64(), b.next_u64());
        assert_eq!(a.next_bounded(100), b.next_bounded(100));
    }

    #[test]
    fn enabled_subsystems_default_all() {
        let subs = EnabledSubsystems::default();
        assert!(subs.world_substrate);
        assert!(subs.agents);
        assert!(subs.social_fabric);
        assert!(subs.collective_agency);
        assert!(subs.discovery);
        assert!(subs.biology);
        assert!(subs.inference);
    }
}
