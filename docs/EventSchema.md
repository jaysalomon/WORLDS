# POLIS Event Schema Specification

**Version:** 0.3.0
**Date:** 14 March 2026  
**Document:** Schema specification for POLIS simulation events  
**Purpose:** Define the formal schema for events emitted during simulation runs, including event types, fields, and export formats.

## 1. Overview

Events in POLIS are immutable records of significant occurrences during a simulation run. They form the primary audit trail and enable post-hoc analysis, debugging, and visualization of simulation behavior.

### 1.1 Design Principles

- **Immutable:** Events are never modified after creation
- **Ordered:** Events have a strict total order (by tick, then sequence)
- **Serializable:** All events can be serialized to JSON/JSONL for export
- **Deterministic:** Same simulation run produces identical event sequences
- **Compact:** Events are lightweight; heavy data stored elsewhere
- **Typed:** Strongly typed in code, with schema validation for exports

### 1.2 Event Stream Model

The simulation produces a **stream of events** during execution:

```
TickStarted → PhaseApplied → PhaseApplied → PhaseApplied → TickCompleted → TickStarted → ...
```

Events are:
- Generated during simulation execution
- Buffered in memory (configurable limit)
- Exported to JSONL files on run completion
- Never stored in checkpoints (reproducible from replay)

## 2. Schema Definition

### 2.1 Event Structure

All events share a common envelope structure:

```rust
pub struct Event {
    /// Event type discriminator
    pub event_type: String,
    
    /// Simulation tick when event occurred
    pub tick: u64,
    
    /// Monotonic sequence number within tick
    pub sequence: u32,
    
    /// Event timestamp (simulation time, not wall clock)
    pub timestamp: u64,
    
    /// Event-specific payload
    pub payload: EventPayload,
}
```

### 2.1.1 Runtime vs Export Representation

POLIS currently uses two compatible representations:

- **Runtime core (`SimEvent`)**: strongly typed enum variants containing domain payload fields (tick always present).
- **Export envelope (`events.jsonl`)**: flattened records with `event_type`, `tick`, `payload`, and optional export metadata such as `sequence` and `timestamp`.

`sequence` and `timestamp` are export-layer fields, not required as fields on every runtime enum variant.

### 2.2 Event Types (Current)

The following event types are defined in **Phase 0**:

#### `TickStarted`

Marks the beginning of a simulation tick.

```rust
pub struct TickStarted {
    /// The tick number being started
    pub tick: u64,
}
```

**JSONL Example:**
```json
{"event_type":"TickStarted","tick":0,"sequence":0,"timestamp":0,"payload":{"tick":0}}
```

#### `PhaseApplied`

Marks completion of a phase execution across all partitions.

```rust
pub struct PhaseApplied {
    /// The tick number
    pub tick: u64,
    
    /// Index of the phase in the phase pipeline (0-based)
    pub phase_index: u8,
    
    /// Number of partitions processed
    pub partition_count: u64,
}
```

**JSONL Example:**
```json
{"event_type":"PhaseApplied","tick":0,"sequence":1,"timestamp":0,"payload":{"tick":0,"phase_index":0,"partition_count":64}}
```

#### `TickCompleted`

Marks the end of a simulation tick with state verification.

```rust
pub struct TickCompleted {
    /// The tick number that completed
    pub tick: u64,
    
    /// Hash of the world state after this tick
    pub state_hash: u64,
}
```

**JSONL Example:**
```json
{"event_type":"TickCompleted","tick":0,"sequence":5,"timestamp":0,"payload":{"tick":0,"state_hash":12345678901234567890}}
```

### 2.3 Event Types (Phase 4 - Social Fabric)

The following event types are defined in **Phase 4** for social dynamics and cross-species interactions:

#### `TrustShifted`

Records a change in trust relationship between two agents.

```rust
pub struct TrustShifted {
    pub tick: u64,
    pub agent_a: u64,
    pub agent_b: u64,
    pub new_trust: i8,        // -100 to +100
    pub reason: TrustShiftReason,
}

pub enum TrustShiftReason {
    Cooperation,
    Conflict,
    TimeDecay,
}
```

Note: `TimeDecay` is reserved for decay-driven trust-shift emission and may not appear in all runs until enabled in runtime event emission.

**JSONL Example:**
```json
{"event_type":"TrustShifted","tick":100,"sequence":12,"timestamp":100,"payload":{"tick":100,"agent_a":42,"agent_b":57,"new_trust":25,"reason":"Cooperation"}}
```

#### `CooperationOccurred`

Records a successful cooperative interaction between two agents.

```rust
pub struct CooperationOccurred {
    pub tick: u64,
    pub agent_a: u64,
    pub agent_b: u64,
    pub kind: CooperationKind,
}

pub enum CooperationKind {
    ResourceSharing,
    MutualAid,
    Information,
}
```

**JSONL Example:**
```json
{"event_type":"CooperationOccurred","tick":100,"sequence":13,"timestamp":100,"payload":{"tick":100,"agent_a":42,"agent_b":57,"kind":"ResourceSharing"}}
```

#### `ConflictOccurred`

Records a conflict or negative interaction between two agents.

```rust
pub struct ConflictOccurred {
    pub tick: u64,
    pub agent_a: u64,
    pub agent_b: u64,
    pub severity: u8,         // 0-100
    pub reason: ConflictReason,
}

pub enum ConflictReason {
    ResourceScarcity,
    Grievance,
    Territorial,
}
```

**JSONL Example:**
```json
{"event_type":"ConflictOccurred","tick":100,"sequence":14,"timestamp":100,"payload":{"tick":100,"agent_a":42,"agent_b":57,"severity":30,"reason":"ResourceScarcity"}}
```

#### `HumanAnimalContact`

Records interaction between humans and animals in a partition (cross-species domestication primitives).

```rust
pub struct HumanAnimalContact {
    pub tick: u64,
    pub partition_id: u64,
    pub contact_type: HumanAnimalContactType,
    pub outcome: HumanAnimalOutcome,
}

pub enum HumanAnimalContactType {
    Hunting,    // Negative: harsh contact
    Feeding,    // Positive: gentle contact
    Proximity,  // Neutral: just nearby
    Handling,   // Could be positive or negative
}

pub enum HumanAnimalOutcome {
    Positive,
    Negative,
    Neutral,
}
```

**JSONL Example:**
```json
{"event_type":"HumanAnimalContact","tick":100,"sequence":15,"timestamp":100,"payload":{"tick":100,"partition_id":12,"contact_type":"Feeding","outcome":"Positive"}}
```

### 2.4 Event Types (Phase 5 - Collective Agency)

The following event types are defined in **Phase 5** for collective actor lifecycle and structure transitions:

#### `CollectiveLifecycleTransition`

Records a collective actor lifecycle state transition.

```rust
pub struct CollectiveLifecycleTransition {
    pub tick: u64,
    pub collective_id: u64,
    pub old_state: String,
    pub new_state: String,
}
```

**JSONL Example:**
```json
{"event_type":"CollectiveLifecycleTransition","tick":240,"sequence":7,"timestamp":240,"payload":{"tick":240,"collective_id":12,"old_state":"CollectiveActorUnstable","new_state":"CollectiveActorStabilized"}}
```

#### `CollectiveMerged`

Records a merge between two collectives into one merged actor.

```rust
pub struct CollectiveMerged {
    pub tick: u64,
    pub primary_id: u64,
    pub secondary_id: u64,
    pub merged_id: u64,
}
```

**JSONL Example:**
```json
{"event_type":"CollectiveMerged","tick":320,"sequence":4,"timestamp":320,"payload":{"tick":320,"primary_id":4,"secondary_id":9,"merged_id":4}}
```

#### `CollectiveSplit`

Records a split where part of a collective forms a new collective.

```rust
pub struct CollectiveSplit {
    pub tick: u64,
    pub original_id: u64,
    pub new_id: u64,
}
```

**JSONL Example:**
```json
{"event_type":"CollectiveSplit","tick":412,"sequence":9,"timestamp":412,"payload":{"tick":412,"original_id":7,"new_id":15}}
```

### 2.5 Event Types (Future)

The following event types are planned for future phases:

#### `AgentSpawned` (Phase 1)

```rust
pub struct AgentSpawned {
    pub tick: u64,
    pub agent_id: AgentId,
    pub position: (i32, i32),
    pub initial_attributes: AgentAttributes,
}
```

#### `AgentDied` (Phase 1)

```rust
pub struct AgentDied {
    pub tick: u64,
    pub agent_id: AgentId,
    pub cause: DeathCause,
}
```

#### `ResourceDiscovered` (Phase 1)

```rust
pub struct ResourceDiscovered {
    pub tick: u64,
    pub agent_id: AgentId,
    pub resource_type: ResourceType,
    pub quantity: u64,
    pub position: (i32, i32),
}
```

#### `TradeOccurred` (Phase 2)

```rust
pub struct TradeOccurred {
    pub tick: u64,
    pub buyer_id: AgentId,
    pub seller_id: AgentId,
    pub resource_type: ResourceType,
    pub quantity: u64,
    pub price: u64,
}
```

#### `ConflictStarted` (Phase 2)

```rust
pub struct ConflictStarted {
    pub tick: u64,
    pub conflict_id: ConflictId,
    pub aggressor_id: AgentId,
    pub defender_id: AgentId,
    pub cause: ConflictCause,
}
```

#### `TechnologyDiscovered` (Phase 3)

```rust
pub struct TechnologyDiscovered {
    pub tick: u64,
    pub agent_id: AgentId,
    pub technology_id: TechnologyId,
    pub prerequisites: Vec<TechnologyId>,
}
```

#### `SettlementFounded` (Phase 3)

```rust
pub struct SettlementFounded {
    pub tick: u64,
    pub settlement_id: SettlementId,
    pub founder_id: AgentId,
    pub position: (i32, i32),
    pub population: u64,
}
```

#### `PredationEncountered` (Planned)

```rust
pub struct PredationEncountered {
    pub tick: u64,
    pub partition_id: u64,
    pub predator_population: u64,
    pub herbivore_losses: u64,
    pub human_loss_proxy: u64,
}
```

#### `DomesticationShift` (Planned)

```rust
pub struct DomesticationShift {
    pub tick: u64,
    pub partition_id: u64,
    pub proto_domestic_population: u64,
    pub tameness_before_ppm: u64,
    pub tameness_after_ppm: u64,
}
```

#### `BattleResolved` (Planned)

```rust
pub struct BattleResolved {
    pub tick: u64,
    pub partition_id: u64,
    pub attacker_id: u64,
    pub defender_id: u64,
    pub attacker_power: u32,
    pub defender_power: u32,
    pub attacker_win_probability: u8, // 0-100
    pub attacker_won: bool,
    pub attacker_losses: u32,
    pub defender_losses: u32,
}
```

#### `TerritoryControlChanged` (Planned)

```rust
pub struct TerritoryControlChanged {
    pub tick: u64,
    pub partition_id: u64,
    pub old_controller: Option<u64>,
    pub new_controller: Option<u64>,
    pub old_state: String, // Claimed/Contested/Controlled/Occupied
    pub new_state: String,
}
```

#### `OccupationStabilized` (Planned)

```rust
pub struct OccupationStabilized {
    pub tick: u64,
    pub partition_id: u64,
    pub occupier_id: u64,
    pub control_score: u32,
    pub resistance_score: u32,
}
```

## 3. Event Stream Properties

### 3.1 Event Count Formula

For a simulation run with `T` ticks and `P` phases per tick:

```
Total Events = T × (1 + P + 1) = T × (P + 2)
```

**Phase 0:** 3 phases per tick → 5 events per tick (minimum)

**Phase 4-5:** Additional social/cross-species/collective events variable per tick based on interactions and lifecycle transitions

| Ticks | Phase 0 Events | Phase 4-5 Events (typical) | JSONL Size (est.) |
|-------|---------------|--------------------------|-------------------|
| 100 | 500 | 500-2,000 | ~25-100 KB |
| 1,000 | 5,000 | 5,000-20,000 | ~250 KB-1 MB |
| 10,000 | 50,000 | 50,000-200,000 | ~2.5-10 MB |
| 100,000 | 500,000 | 500,000-2,000,000 | ~25-100 MB |
| 1,000,000 | 5,000,000 | 5,000,000-20,000,000 | ~250 MB-1 GB |

### 3.2 Ordering Guarantees

Events are strictly ordered by:
1. **Tick number** (ascending)
2. **Sequence number** (ascending within tick)

This provides a **total order** across all events in a run.

### 3.3 Determinism

Given:
- Same scenario configuration
- Same random seed
- Same code version

Then:
- Event sequence is identical (bit-for-bit)
- State hashes match at each tick
- Event counts match exactly

## 4. Export Format

### 4.1 JSONL (JSON Lines)

Events are exported as **JSONL** (JSON Lines) format:

- One JSON object per line
- No outer array wrapper
- UTF-8 encoding
- LF line endings (even on Windows)

**Example `events.jsonl`:**
```jsonl
{"event_type":"TickStarted","tick":0,"sequence":0,"timestamp":0,"payload":{"tick":0}}
{"event_type":"PhaseApplied","tick":0,"sequence":1,"timestamp":0,"payload":{"tick":0,"phase_index":0,"partition_count":64}}
{"event_type":"PhaseApplied","tick":0,"sequence":2,"timestamp":0,"payload":{"tick":0,"phase_index":1,"partition_count":64}}
{"event_type":"PhaseApplied","tick":0,"sequence":3,"timestamp":0,"payload":{"tick":0,"phase_index":2,"partition_count":64}}
{"event_type":"TickCompleted","tick":0,"sequence":4,"timestamp":0,"payload":{"tick":0,"state_hash":12345678901234567890}}
```

### 4.2 Schema Validation

Exported events validate against JSON Schema:

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "https://polis.dev/schema/event-0.1.0.json",
  "type": "object",
  "required": ["event_type", "tick", "sequence", "timestamp", "payload"],
  "properties": {
    "event_type": {
      "type": "string",
      "enum": ["TickStarted", "PhaseApplied", "TickCompleted", "TrustShifted", "CooperationOccurred", "ConflictOccurred", "HumanAnimalContact", "CollectiveLifecycleTransition", "CollectiveMerged", "CollectiveSplit"]
    },
    "tick": { "type": "integer", "minimum": 0 },
    "sequence": { "type": "integer", "minimum": 0 },
    "timestamp": { "type": "integer", "minimum": 0 },
    "payload": {
      "type": "object",
      "required": ["tick"],
      "properties": {
        "tick": { "type": "integer", "minimum": 0 }
      }
    }
  }
}
```

### 4.3 Compression

Large event files may be gzip-compressed:
- Extension: `.jsonl.gz`
- Compression level: 6 (default)
- Transparently handled by export/import tools

## 5. Event Consumption

### 5.1 Streaming API

```rust
use polis_sim::EventStream;

// Create event stream from file
let stream = EventStream::from_file("events.jsonl")?;

// Iterate over events
for event in stream {
    match event.payload {
        EventPayload::TickStarted { tick } => {
            println!("Tick {} started", tick);
        }
        EventPayload::TickCompleted { tick, state_hash } => {
            println!("Tick {} completed with hash {}", tick, state_hash);
        }
        _ => {}
    }
}
```

### 5.2 Filtering

```rust
// Filter events by type
let tick_events: Vec<_> = stream
    .filter(|e| matches!(e.payload, EventPayload::TickCompleted { .. }))
    .collect();

// Filter by tick range
let tick_100_events: Vec<_> = stream
    .filter(|e| e.tick == 100)
    .collect();
```

### 5.3 Aggregation

```rust
// Count events by type
let counts = stream
    .map(|e| e.event_type)
    .counts();

// Calculate state hash progression
let hashes: Vec<u64> = stream
    .filter_map(|e| match e.payload {
        EventPayload::TickCompleted { state_hash, .. } => Some(state_hash),
        _ => None,
    })
    .collect();
```

## 6. Integration with Other Systems

### 6.1 Run Manifest

The run manifest references the event file:

```json
{
  "exports": {
    "events": "events.jsonl",
    "metrics": "metrics.csv"
  },
  "statistics": {
    "event_count": 5000000,
    "event_types": {
      "TickStarted": 1000000,
      "PhaseApplied": 3000000,
      "TickCompleted": 1000000,
      "TrustShifted": 50000,
      "CooperationOccurred": 30000,
      "ConflictOccurred": 20000,
      "HumanAnimalContact": 15000,
      "CollectiveLifecycleTransition": 5000,
      "CollectiveMerged": 400,
      "CollectiveSplit": 550
    }
  }
}
```

### 6.2 Metrics Correlation

Events and metrics can be correlated by tick:

```python
import pandas as pd

# Load events
events = pd.read_json("events.jsonl", lines=True)

# Load metrics
metrics = pd.read_csv("metrics.csv")

# Join on tick
combined = events.merge(metrics, on="tick", how="outer")
```

Phase 5 correlation fields commonly used with collective events:
- `total_collectives`
- `total_collective_members`
- `average_collective_size`
- `average_collective_legitimacy`
- `average_collective_factionalism`

Typical analysis pattern:
- `CollectiveLifecycleTransition` frequency vs `average_collective_legitimacy`
- merge/split rates vs `average_collective_factionalism`
- collective membership growth vs `social_tension`

### 6.3 Visualization

Events can be visualized as:
- **Timeline:** Event sequence over time
- **Heatmap:** Event density by tick and type
- **State graph:** State hash transitions

## 7. Performance Considerations

### 7.1 Memory Management

- Events are buffered in a `Vec<SimEvent>` during simulation
- Buffer size: Unlimited (constrained by system memory)
- For long runs, consider:
  - Streaming export (write events periodically)
  - Event sampling (export every Nth tick)
  - Event filtering (only export specific types)

### 7.2 Disk Usage

| Run Duration | Events | Uncompressed | Gzipped |
|--------------|--------|--------------|---------|
| 1K ticks | 5K | 250 KB | 35 KB |
| 100K ticks | 500K | 25 MB | 3.5 MB |
| 1M ticks | 5M | 250 MB | 35 MB |
| 10M ticks | 50M | 2.5 GB | 350 MB |

### 7.3 I/O Optimization

- Events written in batches (default: 1000 events)
- Async I/O for non-blocking export
- Optional memory-mapped files for large exports

## 8. Versioning

### 8.1 Event Schema Version

Event schema version is embedded in export metadata:

```json
{
  "schema_version": "0.3.0",
  "event_count": 5000000,
  "events": [...]
}
```

### 8.2 Backward Compatibility

- New event types: Added to enum (old consumers ignore unknown types)
- New fields: Added to payload (old consumers ignore unknown fields)
- Removed events: Deprecated for 2 versions, then removed

### 8.3 Version History

| Version | Date | Changes |
|---------|------|---------|
| 0.1.0 | 2026-03-14 | Initial schema - TickStarted, PhaseApplied, TickCompleted |
| 0.2.0 | 2026-03-14 | Added Phase 4 social fabric events (TrustShifted, CooperationOccurred, ConflictOccurred, HumanAnimalContact) |
| 0.3.0 | 2026-03-14 | Added Phase 5 collective events (CollectiveLifecycleTransition, CollectiveMerged, CollectiveSplit) |
| 0.4.0 | 2026-03-15 | Added Phase 6 event set (DiscoveryStageTransition, CorpseCreated, CorpseDecomposed, AnimalCapabilityUtilized, SecondaryProductProduced, ZoonoticPressureChange) |
| 0.5.0 | 2026-03-15 | Added Phase 6 Pass 2 inference events (RiskUpdated, IncidentRealized) |
| 0.6.0 | TBD | Added AgentSpawned, AgentDied, ResourceDiscovered |
| 0.7.0 | TBD | Added TradeOccurred, ConflictStarted |
| 0.8.0 | TBD | Added TechnologyDiscovered, SettlementFounded |
| 0.9.0 | TBD | Additional biology interaction events (PredationEncountered, DomesticationShift) |

## 8.4 Phase 6 Additions (Current Runtime)

The runtime `SimEvent` enum now includes the following additional variants beyond Phase 5:

- `DiscoveryStageTransition`
- `CorpseCreated`
- `CorpseDecomposed`
- `AnimalCapabilityUtilized`
- `SecondaryProductProduced`
- `ZoonoticPressureChange`
- `RiskUpdated`
- `IncidentRealized`

Notes:
- `CorpseCreated` and `CorpseDecomposed` are actively emitted.
- `RiskUpdated` and `IncidentRealized` are actively emitted on inference cadence.
- `DiscoveryStageTransition`, `AnimalCapabilityUtilized`, `SecondaryProductProduced`, and `ZoonoticPressureChange` remain schema-defined and are pending full emission wiring.

## 9. References

- [02_StateModel.md](02_StateModel.md) - State categories and persistence
- [ScenarioSchema.md](ScenarioSchema.md) - Scenario configuration schema
- [10_TechnicalArchitecture.md](10_TechnicalArchitecture.md) - Runtime architecture
- [Plan_BuildOrder.md](Plan_BuildOrder.md) - Implementation phases

## 10. Appendix: Rust Implementation

Current Rust implementation in `polis-sim`:

```rust
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
    // Phase 4: Social fabric events
    TrustShifted {
        tick: u64,
        agent_a: u64,
        agent_b: u64,
        new_trust: i8,
        reason: TrustShiftReason,
    },
    CooperationOccurred {
        tick: u64,
        agent_a: u64,
        agent_b: u64,
        kind: CooperationKind,
    },
    ConflictOccurred {
        tick: u64,
        agent_a: u64,
        agent_b: u64,
        severity: u8,
        reason: ConflictReason,
    },
    // Phase 4: Cross-species events
    HumanAnimalContact {
        tick: u64,
        partition_id: u64,
        contact_type: HumanAnimalContactType,
        outcome: HumanAnimalOutcome,
    },
    // Phase 5: Collective agency events
    CollectiveLifecycleTransition {
        tick: u64,
        collective_id: u64,
        old_state: String,
        new_state: String,
    },
    CollectiveMerged {
        tick: u64,
        primary_id: u64,
        secondary_id: u64,
        merged_id: u64,
    },
    CollectiveSplit {
        tick: u64,
        original_id: u64,
        new_id: u64,
    },
}

/// Reason for trust shift
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TrustShiftReason {
    Cooperation,
    Conflict,
    TimeDecay,
}

/// Types of cooperation
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CooperationKind {
    ResourceSharing,
    MutualAid,
    Information,
}

/// Reason for conflict
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConflictReason {
    ResourceScarcity,
    Grievance,
    Territorial,
}

/// Type of human-animal contact
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum HumanAnimalContactType {
    Hunting,    // Negative: harsh contact
    Feeding,    // Positive: gentle contact
    Proximity,  // Neutral: just nearby
    Handling,   // Could be positive or negative
}

/// Outcome of human-animal contact
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum HumanAnimalOutcome {
    Positive,
    Negative,
    Neutral,
}
```

```rust
impl SimEvent {
    /// Returns the tick number for this event
    pub fn tick(&self) -> u64 {
        match self {
            SimEvent::TickStarted { tick } => *tick,
            SimEvent::PhaseApplied { tick, .. } => *tick,
            SimEvent::TickCompleted { tick, .. } => *tick,
            SimEvent::TrustShifted { tick, .. } => *tick,
            SimEvent::CooperationOccurred { tick, .. } => *tick,
            SimEvent::ConflictOccurred { tick, .. } => *tick,
            SimEvent::HumanAnimalContact { tick, .. } => *tick,
            SimEvent::CollectiveLifecycleTransition { tick, .. } => *tick,
            SimEvent::CollectiveMerged { tick, .. } => *tick,
            SimEvent::CollectiveSplit { tick, .. } => *tick,
        }
    }

    /// Returns the event type name
    pub fn event_type(&self) -> &'static str {
        match self {
            SimEvent::TickStarted { .. } => "TickStarted",
            SimEvent::PhaseApplied { .. } => "PhaseApplied",
            SimEvent::TickCompleted { .. } => "TickCompleted",
            SimEvent::TrustShifted { .. } => "TrustShifted",
            SimEvent::CooperationOccurred { .. } => "CooperationOccurred",
            SimEvent::ConflictOccurred { .. } => "ConflictOccurred",
            SimEvent::HumanAnimalContact { .. } => "HumanAnimalContact",
            SimEvent::CollectiveLifecycleTransition { .. } => "CollectiveLifecycleTransition",
            SimEvent::CollectiveMerged { .. } => "CollectiveMerged",
            SimEvent::CollectiveSplit { .. } => "CollectiveSplit",
        }
    }
}
```
