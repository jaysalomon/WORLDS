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

pub const fn workspace_status() -> &'static str {
    "scaffold"
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunManifest {
    pub scenario_name: String,
    pub seed: u64,
    pub partition_count: u64,
    pub ticks: u64,
    pub final_state_hash: u64,
    pub execution_mode: String,
    pub workspace_status: String,
}

#[cfg(test)]
mod tests {
    use super::{DeterministicRng, SimulationSeed, workspace_status};

    #[test]
    fn seed_round_trips() {
        let seed = SimulationSeed::new(42);
        assert_eq!(seed.0, 42);
    }

    #[test]
    fn reports_scaffold_status() {
        assert_eq!(workspace_status(), "scaffold");
    }

    #[test]
    fn deterministic_rng_is_reproducible() {
        let mut a = DeterministicRng::from_u64(42);
        let mut b = DeterministicRng::from_u64(42);
        assert_eq!(a.next_u64(), b.next_u64());
        assert_eq!(a.next_u64(), b.next_u64());
        assert_eq!(a.next_bounded(100), b.next_bounded(100));
    }
}
