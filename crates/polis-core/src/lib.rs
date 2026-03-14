use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SimulationSeed(pub u64);

impl SimulationSeed {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

pub const fn workspace_status() -> &'static str {
    "scaffold"
}

#[cfg(test)]
mod tests {
    use super::{SimulationSeed, workspace_status};

    #[test]
    fn seed_round_trips() {
        let seed = SimulationSeed::new(42);
        assert_eq!(seed.0, 42);
    }

    #[test]
    fn reports_scaffold_status() {
        assert_eq!(workspace_status(), "scaffold");
    }
}
