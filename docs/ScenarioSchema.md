# POLIS Scenario Schema Specification

**Version:** 0.1  
**Date:** 14 March 2026  
**Document:** Schema specification for POLIS scenario files  
**Purpose:** Define the formal schema for scenario configuration files, including field definitions, validation rules, and versioning policy.

## 1. Overview

Scenarios in POLIS are declarative configuration files that define the initial conditions and parameters for a simulation run. They are the primary interface for users to configure experiments and reproducible simulation setups.

### 1.1 Design Principles

- **Declarative**: Scenarios describe *what* the initial state should be, not *how* to construct it
- **Versioned**: Schema evolution is explicit and backward compatibility is maintained where possible
- **Validatable**: All scenarios can be validated against the schema before execution
- **Human-readable**: RON format chosen for clarity, comments, and type safety
- **Deterministic**: Same scenario + same seed = identical simulation (given identical code version)

### 1.2 File Format

Scenarios are stored as **RON** (Rusty Object Notation) files with extension `.ron`.

**Why RON:**
- Native Rust serialization (via `serde`)
- Supports comments (unlike JSON)
- Type-safe deserialization
- Clear distinction between strings, numbers, and identifiers
- Trailing commas allowed (unlike standard JSON)

## 2. Schema Definition

### 2.1 Root Structure

```rust
pub struct ScenarioConfig {
    /// Schema version for this scenario
    pub schema_version: Option<String>,
    
    /// Human-readable name for this scenario
    pub name: String,
    
    /// Random seed for deterministic reproduction
    pub seed: u64,
    
    /// Number of world partitions (must be power of 2, >= 1)
    #[serde(default = "default_partition_count")]
    pub partition_count: u64,
    
    /// Optional human-readable notes
    pub notes: Option<String>,
    
    /// World generation parameters
    pub world: Option<WorldConfig>,
    
    /// Initial population configuration
    pub population: Option<PopulationConfig>,
    
    /// Resource distribution configuration
    pub resources: Option<ResourcesConfig>,
    
    /// Simulation parameters
    pub simulation: Option<SimulationConfig>,
}
```

### 2.2 Field Specifications

#### `schema_version` (Optional, String)

- **Purpose:** Enables schema migration and validation
- **Format:** SemVer (e.g., "0.1.0")
- **Default:** If omitted, treated as version "0.1.0"
- **Validation:** Must be a valid semantic version string

#### `name` (Required, String)

- **Purpose:** Human-readable identifier for the scenario
- **Constraints:**
  - Minimum length: 1 character
  - Maximum length: 256 characters
  - Must be unique within a scenario set (enforced by tooling)
- **Usage:** Appears in run manifests, export directories, and UI displays

#### `seed` (Required, u64)

- **Purpose:** Deterministic random seed for reproducible runs
- **Range:** 0 to 2^64 - 1
- **Special values:**
  - `0`: May be treated as "random seed" in future versions (currently literal)
- **Determinism guarantee:** Same seed + same scenario + same code version = identical run

#### `partition_count` (Optional, u64)

- **Purpose:** Number of spatial partitions for parallel processing
- **Default:** 64 (defined in `polis-world`)
- **Constraints:**
  - Must be a power of 2 (1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024)
  - Maximum: 1024 (implementation limit)
- **Performance impact:** Higher values improve parallel scaling but increase overhead

#### `notes` (Optional, String)

- **Purpose:** Human-readable description of the scenario
- **Constraints:** Maximum 4096 characters
- **Usage:** Copied to run manifest, displayed in UI

### 2.3 Sub-configurations (Future)

The following sections are defined for future expansion. Current implementation (Phase 0) only requires the root fields above.

#### `WorldConfig` (Optional)

```rust
pub struct WorldConfig {
    /// World dimensions in cells (must be power of 2)
    pub dimensions: (u32, u32),
    
    /// Terrain generation parameters
    pub terrain: TerrainConfig,
    
    /// Climate parameters
    pub climate: ClimateConfig,
}
```

#### `PopulationConfig` (Optional)

```rust
pub struct PopulationConfig {
    /// Initial agent count
    pub initial_count: u64,
    
    /// Agent distribution strategy
    pub distribution: DistributionStrategy,
    
    /// Initial agent attributes
    pub attributes: AgentAttributes,
}
```

#### `ResourcesConfig` (Optional)

```rust
pub struct ResourcesConfig {
    /// Base resource abundance multiplier
    pub abundance: f64,
    
    /// Resource distribution noise parameters
    pub distribution: NoiseConfig,
    
    /// Specific resource overrides
    pub overrides: Vec<ResourceOverride>,
}
```

#### `SimulationConfig` (Optional)

```rust
pub struct SimulationConfig {
    /// Target ticks per second (for real-time modes)
    pub target_tps: Option<f64>,
    
    /// Maximum simulation ticks (0 = unlimited)
    pub max_ticks: Option<u64>,
    
    /// Checkpoint interval in ticks
    pub checkpoint_interval: Option<u64>,
}
```

## 3. Validation Rules

### 3.1 Schema Validation

All scenarios must pass the following validation before execution:

| Rule | Severity | Description |
|------|----------|-------------|
| RON_PARSE | Error | File must be valid RON syntax |
| SCHEMA_VERSION | Warning | Unknown schema version may indicate incompatibility |
| REQUIRED_FIELDS | Error | `name` and `seed` must be present |
| NAME_LENGTH | Error | Name must be 1-256 characters |
| PARTITION_POWER_OF_2 | Error | `partition_count` must be power of 2 |
| PARTITION_MAX | Error | `partition_count` must not exceed 1024 |
| SEED_RANGE | Error | `seed` must be valid u64 |

### 3.2 Semantic Validation

Additional validation performed at runtime:

| Rule | Severity | Description |
|------|----------|-------------|
| SCENARIO_NAME_UNIQUE | Warning | Duplicate names in scenario directory |
| COMPATIBLE_PARTITIONS | Error | Partition count must match world dimensions (future) |
| RESOURCE_ABUNDANCE_RANGE | Error | Abundance must be in [0.0, 10.0] (future) |

## 4. Versioning and Migration

### 4.1 Schema Version Policy

- **Major version:** Breaking changes (old scenarios won't load)
- **Minor version:** Additive changes (old scenarios load, new fields optional)
- **Patch version:** Documentation/clarification changes only

### 4.2 Migration Strategy

When schema versions diverge:

1. **Forward compatibility:** Newer code loads older schemas (with defaults)
2. **Migration tool:** `polis-app migrate-scenario <file>` upgrades old schemas
3. **Deprecation window:** Old schema support maintained for 2 minor versions

### 4.3 Version History

| Version | Date | Changes |
|---------|------|---------|
| 0.1.0 | 2026-03-14 | Initial schema - root fields only |
| 0.2.0 | TBD | Added WorldConfig, PopulationConfig |
| 0.3.0 | TBD | Added ResourcesConfig, SimulationConfig |

## 5. Example Scenarios

### 5.1 Minimal Valid Scenario

```ron
(
    name: "minimal",
    seed: 42,
)
```

### 5.2 Current Default Scenario

```ron
(
    name: "default",
    seed: 42,
    partition_count: 64,
    notes: "Placeholder scenario used while the runtime is being built.",
)
```

### 5.3 Future Full Scenario (Schema 0.3.0)

```ron
(
    schema_version: "0.3.0",
    name: "agricultural_revolution",
    seed: 12345,
    partition_count: 256,
    notes: "Simulates the emergence of agriculture with favorable climate",
    
    world: (
        dimensions: (1024, 1024),
        terrain: (
            base_elevation: 100.0,
            mountain_noise: (octaves: 4, persistence: 0.5),
        ),
        climate: (
            base_temperature: 15.0,
            rainfall: 800.0,
        ),
    ),
    
    population: (
        initial_count: 1000,
        distribution: Clustered(clusters: 10, radius: 50),
        attributes: (
            intelligence: (mean: 100.0, stddev: 15.0),
            aggression: (mean: 50.0, stddev: 20.0),
        ),
    ),
    
    resources: (
        abundance: 1.2,
        distribution: (scale: 100.0, octaves: 3),
        overrides: [
            (type: Water, abundance: 2.0, regions: [Rivers, Lakes]),
        ],
    ),
    
    simulation: (
        max_ticks: 100000,
        checkpoint_interval: 10000,
    ),
)
```

## 6. Tooling

### 6.1 Validation Command

```bash
# Validate a scenario file
polis-app validate-scenario scenarios/my_scenario.ron

# Validate all scenarios in directory
polis-app validate-scenario --all scenarios/
```

### 6.2 Schema Introspection

```bash
# Print current schema as JSON Schema
polis-app schema --format json > polis-schema.json

# Print current schema as RON
polis-app schema --format ron
```

## 7. Integration with Other Systems

### 7.1 Run Manifest

Scenario metadata is embedded in the run manifest:

```json
{
  "scenario": {
    "name": "default",
    "seed": 42,
    "schema_version": "0.1.0"
  },
  "run_id": "...",
  "started_at": "..."
}
```

### 7.2 Checkpoint Compatibility

Checkpoints include the scenario hash, not the full scenario. To resume:
- Checkpoint scenario hash must match loaded scenario hash
- Schema version must be compatible

### 7.3 Batch Experiments

Scenarios can be parameterized for batch runs:

```bash
# Run with seed sweep
polis-app batch --scenario scenarios/base.ron --seed-range 0..100

# Run with parameter override
polis-app batch --scenario scenarios/base.ron --override "partition_count=128"
```

## 8. References

- [02_StateModel.md](02_StateModel.md) - State categories and persistence
- [10_TechnicalArchitecture.md](10_TechnicalArchitecture.md) - Runtime architecture
- [Plan_BuildOrder.md](Plan_BuildOrder.md) - Implementation phases

## 9. Appendix: JSON Schema (Normative)

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "https://polis.dev/schema/scenario-0.1.0.json",
  "title": "POLIS Scenario Configuration",
  "type": "object",
  "required": ["name", "seed"],
  "properties": {
    "schema_version": {
      "type": "string",
      "pattern": "^\\d+\\.\\d+\\.\\d+$"
    },
    "name": {
      "type": "string",
      "minLength": 1,
      "maxLength": 256
    },
    "seed": {
      "type": "integer",
      "minimum": 0,
      "maximum": 18446744073709551615
    },
    "partition_count": {
      "type": "integer",
      "enum": [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024],
      "default": 64
    },
    "notes": {
      "type": ["string", "null"],
      "maxLength": 4096
    }
  },
  "additionalProperties": false
}
```
