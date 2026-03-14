# POLIS API Contracts

**Version:** 0.1  
**Date:** 14 March 2026  
**Document:** API contracts and interfaces between POLIS crates  
**Purpose:** Define stable, versioned interfaces between crates to enable independent development and prevent breaking changes.

## 1. Overview

POLIS is organized into multiple crates with clear boundaries. This document defines the **public API contracts** between these crates, including:

- **Data structures** passed across crate boundaries
- **Function signatures** for public interfaces
- **Error types** for cross-crate error handling
- **Version compatibility** guarantees

### 1.1 Crate Dependency Graph

```
┌─────────────────────────────────────────────────────────────┐
│                        polis-cli                             │
│                    (CLI Application)                         │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│                      polis-runner                            │
│              (Simulation Orchestration)                      │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│                       polis-sim                              │
│                 (Core Simulation Engine)                     │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│                      polis-core                              │
│              (Shared Types and Utilities)                    │
└─────────────────────────────────────────────────────────────┘
```

### 1.2 API Stability Levels

| Level | Stability | Description |
|-------|-----------|-------------|
| **Stable** | Guaranteed | No breaking changes within major version |
| **Evolution** | Best effort | May change with deprecation period |
| **Experimental** | Unstable | May change without notice |
| **Internal** | Private | Not part of public API |

## 2. polis-core API

**Stability:** Stable  
**Version:** 0.1.0  
**Purpose:** Shared types used by all crates

### 2.1 Core Types

#### SimulationSeed

Unique identifier for a simulation run.

```rust
/// A unique identifier for a simulation run.
/// 
/// # Format
/// - 16 bytes (128 bits)
/// - Hex-encoded as 32-character string
/// - Generated from: timestamp + random + counter
/// 
/// # Example
/// ```
/// let seed = SimulationSeed::generate();
/// assert_eq!(seed.to_string().len(), 32);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SimulationSeed([u8; 16]);

impl SimulationSeed {
    /// Generate a new random seed
    pub fn generate() -> Self;
    
    /// Parse from hex string
    pub fn from_hex(hex: &str) -> Result<Self, SeedError>;
    
    /// Convert to hex string
    pub fn to_hex(&self) -> String;
}
```

#### AgentId

Unique identifier for an agent.

```rust
/// Unique identifier for an agent in the simulation.
/// 
/// # Format
/// - 8 bytes (64 bits)
/// - Sequential assignment starting from 1
/// - 0 reserved for "no agent"
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AgentId(u64);

impl AgentId {
    /// Create from raw value (internal use only)
    pub(crate) fn from_raw(id: u64) -> Self;
    
    /// Get the raw value
    pub fn as_u64(&self) -> u64;
    
    /// Check if this is a valid agent (not 0)
    pub fn is_valid(&self) -> bool;
}
```

#### Position

2D grid position.

```rust
/// A position on the simulation grid.
/// 
/// # Coordinates
/// - x: East-West axis (positive = East)
/// - y: North-South axis (positive = North)
/// - Origin (0, 0) is at the center of the world
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    /// Create a new position
    pub fn new(x: i32, y: i32) -> Self;
    
    /// Calculate Manhattan distance to another position
    pub fn manhattan_distance(&self, other: Position) -> u32;
    
    /// Calculate Euclidean distance squared (avoiding sqrt)
    pub fn distance_squared(&self, other: Position) -> u64;
    
    /// Get neighboring positions (4-connected)
    pub fn neighbors(&self) -> [Position; 4];
    
    /// Get neighboring positions (8-connected)
    pub fn neighbors8(&self) -> [Position; 8];
}
```

### 2.2 Error Types

#### CoreError

Base error type for core operations.

```rust
/// Errors that can occur in core operations.
#[derive(Debug, Error)]
pub enum CoreError {
    #[error("invalid seed format: {0}")]
    InvalidSeed(String),
    
    #[error("invalid position: ({x}, {y})")]
    InvalidPosition { x: i32, y: i32 },
    
    #[error("value out of range: {value} not in [{min}, {max}]")]
    OutOfRange { value: i64, min: i64, max: i64 },
}
```

### 2.3 Utilities

#### Grid Utilities

```rust
/// Calculate grid index from position and dimensions.
/// 
/// # Panics
/// Panics if position is outside grid bounds.
pub fn grid_index(pos: Position, width: u32, height: u32) -> usize;

/// Calculate position from grid index.
pub fn grid_position(index: usize, width: u32) -> Position;

/// Check if position is within grid bounds.
pub fn in_bounds(pos: Position, width: u32, height: u32) -> bool;
```

## 3. polis-sim API

**Stability:** Evolution  
**Version:** 0.1.0  
**Purpose:** Core simulation engine

### 3.1 Simulation Builder

#### SimulationBuilder

Fluent API for constructing simulations.

```rust
/// Builder for creating simulation instances.
/// 
/// # Example
/// ```
/// let sim = SimulationBuilder::new()
///     .with_seed(SimulationSeed::generate())
///     .with_world_size(256, 256)
///     .with_phases(vec![
///         Phase::Movement,
///         Phase::Interaction,
///         Phase::Update,
///     ])
///     .build()?;
/// ```
pub struct SimulationBuilder {
    // ...
}

impl SimulationBuilder {
    /// Create a new builder with defaults
    pub fn new() -> Self;
    
    /// Set the simulation seed
    pub fn with_seed(mut self, seed: SimulationSeed) -> Self;
    
    /// Set the world dimensions
    pub fn with_world_size(mut self, width: u32, height: u32) -> Self;
    
    /// Set the phase pipeline
    pub fn with_phases(mut self, phases: Vec<Phase>) -> Self;
    
    /// Set the partition count for parallel execution
    pub fn with_partitions(mut self, count: u32) -> Self;
    
    /// Set the execution mode
    pub fn with_execution_mode(mut self, mode: ExecutionMode) -> Self;
    
    /// Build the simulation
    pub fn build(self) -> Result<Simulation, SimError>;
}
```

### 3.2 Simulation Control

#### Simulation

Main simulation interface.

```rust
/// The core simulation engine.
/// 
/// # Thread Safety
/// Simulation is not Send + Sync. Use SimulationHandle for cross-thread access.
pub struct Simulation {
    // ...
}

impl Simulation {
    /// Execute a single tick
    pub fn tick(&mut self) -> Result<TickResult, SimError>;
    
    /// Execute multiple ticks
    pub fn run_ticks(&mut self, count: u64) -> Result<RunSummary, SimError>;
    
    /// Get the current tick number
    pub fn current_tick(&self) -> u64;
    
    /// Get the world state hash
    pub fn state_hash(&self) -> u64;
    
    /// Get the event stream
    pub fn events(&self) -> &[SimEvent];
    
    /// Clear the event buffer
    pub fn clear_events(&mut self);
    
    /// Get simulation metrics
    pub fn metrics(&self) -> &Metrics;
    
    /// Export events to JSONL
    pub fn export_events(&self, path: &Path) -> Result<(), SimError>;
    
    /// Export metrics to CSV
    pub fn export_metrics(&self, path: &Path) -> Result<(), SimError>;
}
```

#### SimulationHandle

Thread-safe handle for simulation control.

```rust
/// Thread-safe handle to a simulation.
/// 
/// # Usage
/// Used by polis-runner to control simulation from a separate thread.
#[derive(Clone)]
pub struct SimulationHandle {
    // ...
}

impl SimulationHandle {
    /// Request a single tick execution
    pub fn request_tick(&self) -> Result<(), SimError>;
    
    /// Request multiple ticks
    pub fn request_run(&self, tick_count: u64) -> Result<(), SimError>;
    
    /// Request simulation pause
    pub fn request_pause(&self);
    
    /// Request simulation stop
    pub fn request_stop(&self);
    
    /// Get current status
    pub fn status(&self) -> SimulationStatus;
    
    /// Subscribe to events
    pub fn subscribe_events(&self) -> mpsc::Receiver<SimEvent>;
}
```

### 3.3 Event Types

#### SimEvent

Events emitted during simulation.

```rust
/// Events emitted during simulation execution.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SimEvent {
    TickStarted {
        tick: u64,
    },
    PhaseApplied {
        tick: u64,
        phase_index: u8,
        partition_count: u64,
    },
    TickCompleted {
        tick: u64,
        state_hash: u64,
    },
}

impl SimEvent {
    /// Get the tick number for this event
    pub fn tick(&self) -> u64;
    
    /// Get the event type name
    pub fn event_type(&self) -> &'static str;
}
```

### 3.4 Result Types

#### TickResult

Result of a single tick execution.

```rust
/// Result of executing a single simulation tick.
#[derive(Debug, Clone)]
pub struct TickResult {
    pub tick: u64,
    pub state_hash: u64,
    pub events_emitted: u32,
    pub execution_time_ms: u64,
}
```

#### RunSummary

Summary of a simulation run.

```rust
/// Summary statistics for a simulation run.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct RunSummary {
    pub seed: SimulationSeed,
    pub partition_count: u64,
    pub ticks: u64,
    pub final_state_hash: u64,
    pub event_count: u64,
    pub metric_count: u64,
}
```

### 3.5 Error Types

#### SimError

Simulation errors.

```rust
/// Errors that can occur during simulation.
#[derive(Debug, Error)]
pub enum SimError {
    #[error("failed to read scenario file '{path}': {source}")]
    ScenarioRead {
        path: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    
    #[error("invalid scenario: {0}")]
    InvalidScenario(String),
    
    #[error("world size exceeds maximum: {0}x{1}")]
    WorldSizeExceeded(u32, u32),
    
    #[error("partition count {0} exceeds maximum {1}")]
    PartitionCountExceeded(u32, u32),
    
    #[error("simulation panicked: {0}")]
    SimulationPanicked(String),
    
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
```

### 3.6 Configuration Types

#### ExecutionMode

Execution mode for the simulation.

```rust
/// Execution mode for the simulation.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ExecutionMode {
    /// Single-threaded execution
    #[default]
    Serial,
    
    /// Multi-threaded execution with work-stealing
    Parallel,
}
```

#### SimulationStatus

Current simulation status.

```rust
/// Current status of the simulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationStatus {
    Idle,
    Running,
    Paused,
    Completed,
    Error(SimError),
}
```

## 4. polis-runner API

**Stability:** Evolution  
**Version:** 0.1.0  
**Purpose:** Simulation orchestration and run management

### 4.1 Run Management

#### RunManager

Manages simulation runs.

```rust
/// Manages simulation runs and their lifecycle.
pub struct RunManager {
    // ...
}

impl RunManager {
    /// Create a new run manager
    pub fn new() -> Self;
    
    /// Start a new run from a scenario file
    pub fn start_run(&mut self, scenario_path: &Path) -> Result<RunHandle, RunnerError>;
    
    /// Get a handle to an existing run
    pub fn get_run(&self, run_id: RunId) -> Option<RunHandle>;
    
    /// List all active runs
    pub fn active_runs(&self) -> Vec<RunId>;
    
    /// List all completed runs
    pub fn completed_runs(&self) -> Vec<RunId>;
    
    /// Cancel a running simulation
    pub fn cancel_run(&mut self, run_id: RunId) -> Result<(), RunnerError>;
}
```

#### RunHandle

Handle to a specific simulation run.

```rust
/// Handle to a specific simulation run.
#[derive(Clone)]
pub struct RunHandle {
    // ...
}

impl RunHandle {
    /// Get the run ID
    pub fn id(&self) -> RunId;
    
    /// Get the run status
    pub fn status(&self) -> RunStatus;
    
    /// Get the current progress (0.0 to 1.0)
    pub fn progress(&self) -> f64;
    
    /// Pause the run
    pub fn pause(&self) -> Result<(), RunnerError>;
    
    /// Resume the run
    pub fn resume(&self) -> Result<(), RunnerError>;
    
    /// Cancel the run
    pub fn cancel(&self) -> Result<(), RunnerError>;
    
    /// Wait for completion
    pub fn wait(&self) -> Result<RunResult, RunnerError>;
    
    /// Get the run result (if completed)
    pub fn result(&self) -> Option<RunResult>;
    
    /// Get the output directory
    pub fn output_dir(&self) -> &Path;
}
```

### 4.2 Result Types

#### RunResult

Result of a completed run.

```rust
/// Result of a completed simulation run.
#[derive(Debug, Clone)]
pub struct RunResult {
    pub run_id: RunId,
    pub seed: SimulationSeed,
    pub status: RunCompletionStatus,
    pub ticks_completed: u64,
    pub final_state_hash: u64,
    pub duration_ms: u64,
    pub output_dir: PathBuf,
}

/// Completion status for a run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunCompletionStatus {
    Success,
    Cancelled,
    Failed(RunnerError),
}
```

### 4.3 Error Types

#### RunnerError

Runner errors.

```rust
/// Errors that can occur in the run manager.
#[derive(Debug, Error)]
pub enum RunnerError {
    #[error("run not found: {0}")]
    RunNotFound(RunId),
    
    #[error("run already completed: {0}")]
    RunAlreadyCompleted(RunId),
    
    #[error("invalid scenario: {0}")]
    InvalidScenario(String),
    
    #[error("simulation error: {0}")]
    SimulationError(#[from] SimError),
    
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
```

## 5. polis-cli API

**Stability:** Experimental  
**Version:** 0.1.0  
**Purpose:** Command-line interface

### 5.1 CLI Commands

The CLI is the user-facing interface and does not expose a programmatic API. However, it provides:

#### Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Invalid arguments |
| 3 | Scenario error |
| 4 | Simulation error |
| 5 | I/O error |
| 130 | Interrupted (Ctrl+C) |

### 5.2 Output Formats

#### Console Output

```
POLIS Simulation Runner v0.1.0
==============================

Run ID:     a1b2c3d4e5f6...
Scenario:   scenarios/baseline.toml
Seed:       1234567890abcdef...

Starting simulation...
[████████████████████] 100% 1000/1000 ticks  ETA: 0s

Completed in 1.23s
Final state hash: 0x1234567890abcdef
Output:     runs/2026-03-14_001/
```

#### JSON Output (for scripting)

```bash
$ polis run scenarios/baseline.toml --json
```

```json
{
  "success": true,
  "run_id": "a1b2c3d4e5f6...",
  "seed": "1234567890abcdef...",
  "ticks_completed": 1000,
  "duration_ms": 1230,
  "final_state_hash": "1234567890abcdef",
  "output_dir": "runs/2026-03-14_001/"
}
```

## 6. Version Compatibility

### 6.1 Semantic Versioning

All crates follow [Semantic Versioning](https://semver.org/):

- **MAJOR:** Breaking API changes
- **MINOR:** New features, backward compatible
- **PATCH:** Bug fixes, backward compatible

### 6.2 Compatibility Matrix

| polis-core | polis-sim | polis-runner | polis-cli |
|------------|-----------|--------------|-----------|
| 0.1.x | 0.1.x | 0.1.x | 0.1.x |
| 0.2.x | 0.2.x | 0.2.x | 0.2.x |

**Rule:** All crates in a build must share the same minor version.

### 6.3 Breaking Change Policy

Breaking changes are allowed only in major version bumps:

1. **Deprecated** in version N (with warning)
2. **Removed** in version N+1 (major bump)

Minimum deprecation period: 3 months

## 7. FFI and External Interfaces

### 7.1 C API (Future)

A C API is planned for Phase 3 to enable bindings in other languages:

```c
// polis.h
#ifndef POLIS_H
#define POLIS_H

#include <stdint.h>

typedef struct polis_simulation polis_simulation;
typedef struct polis_error polis_error;

// Create a simulation from a scenario file
polis_simulation* polis_simulation_new(const char* scenario_path, 
                                       polis_error** error);

// Run the simulation for N ticks
int polis_simulation_run(polis_simulation* sim, uint64_t ticks,
                         polis_error** error);

// Get the current tick
uint64_t polis_simulation_get_tick(const polis_simulation* sim);

// Free the simulation
void polis_simulation_free(polis_simulation* sim);

// Get error message
const char* polis_error_message(const polis_error* error);

// Free error
void polis_error_free(polis_error* error);

#endif
```

### 7.2 Python Bindings (Future)

Python bindings are planned using PyO3:

```python
# polis.py (future)
import polis

# Create simulation
sim = polis.Simulation.from_scenario("scenarios/baseline.toml")

# Run simulation
result = sim.run(ticks=1000)

# Access results
print(f"Completed {result.ticks} ticks")
print(f"Final state hash: {result.state_hash}")

# Access events
for event in sim.events:
    print(event)
```

## 8. Testing API Contracts

### 8.1 Contract Tests

Each crate provides contract tests:

```rust
// In polis-sim/tests/contract_tests.rs

#[test]
fn test_simulation_builder_api() {
    // Test that SimulationBuilder API works as documented
    let sim = SimulationBuilder::new()
        .with_seed(SimulationSeed::generate())
        .with_world_size(100, 100)
        .build()
        .expect("builder should succeed");
    
    assert_eq!(sim.current_tick(), 0);
}

#[test]
fn test_event_stream_format() {
    // Test that events serialize to expected JSON format
    let event = SimEvent::TickStarted { tick: 42 };
    let json = serde_json::to_string(&event).unwrap();
    
    assert!(json.contains("\"tick\":42"));
    assert!(json.contains("\"TickStarted\""));
}
```

### 8.2 Integration Tests

Cross-crate integration tests verify API compatibility:

```rust
// In tests/integration/api_compatibility.rs

#[test]
fn test_runner_sim_integration() {
    // Test that polis-runner can use polis-sim correctly
    let mut manager = RunManager::new();
    let handle = manager.start_run(Path::new("test_scenario.toml"))
        .expect("should start run");
    
    let result = handle.wait().expect("should complete");
    assert!(result.ticks_completed > 0);
}
```

## 9. References

- [10_TechnicalArchitecture.md](10_TechnicalArchitecture.md) - Crate organization
- [ScenarioSchema.md](ScenarioSchema.md) - Scenario file format
- [EventSchema.md](EventSchema.md) - Event format
- [Plan_BuildOrder.md](Plan_BuildOrder.md) - Implementation phases

## 10. Appendix: Version History

| Version | Date | Changes |
|---------|------|---------|
| 0.1.0 | 2026-03-14 | Initial API contracts for Phase 0 |
| 0.2.0 | TBD | Added Agent API, Resource API |
| 0.3.0 | TBD | Added Trade API, Conflict API |
| 0.4.0 | TBD | Added Technology API, Settlement API |
| 1.0.0 | TBD | Stable API freeze |
