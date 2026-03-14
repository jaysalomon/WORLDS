# Frontend Data Contract Specification

**Document ID:** SPEC-FDC-001  
**Version:** 0.1.0  
**Status:** Draft  
**Date:** 2025-01-14  
**Depends on:** SpecSuite.md, 09_FrontendAndPresentation.md

---

## 1. Purpose

This document defines the data contract between the POLIS simulation backend and the frontend presentation layer. It specifies:

- The **Snapshot API** for world state queries
- The **Event Stream** for real-time updates
- The **Query Protocol** for frontend data requests
- The **Serialization Format** for all frontend-bound data

---

## 2. Design Principles

### 2.1 Decoupling
- Frontend never accesses simulation internals directly
- All data flows through well-defined contract boundaries
- Backend can evolve without breaking frontend contracts

### 2.2 Efficiency
- Differential updates minimize bandwidth
- Spatial indexing supports viewport queries
- Level-of-detail reduces data volume for distant objects

### 2.3 Flexibility
- Contract supports multiple frontend implementations
- Query parameters allow selective data fetching
- Extensible schema supports future visualization needs

---

## 3. Snapshot API

### 3.1 World Snapshot Structure

```rust
/// Complete world state snapshot for frontend consumption
pub struct WorldSnapshot {
    /// Snapshot metadata
    pub meta: SnapshotMeta,
    
    /// Spatial data organized by region
    pub regions: Vec<RegionSnapshot>,
    
    /// Agent populations by region
    pub populations: Vec<PopulationSnapshot>,
    
    /// Active institutions
    pub institutions: Vec<InstitutionSnapshot>,
    
    /// Resource distributions
    pub resources: Vec<ResourceSnapshot>,
    
    /// Discovery state
    pub discoveries: Vec<DiscoverySnapshot>,
    
    /// Biological state
    pub biology: BiologySnapshot,
}

pub struct SnapshotMeta {
    /// Simulation tick this snapshot represents
    pub tick: u64,
    
    /// Real-world timestamp
    pub timestamp: u64,
    
    /// Snapshot version for schema compatibility
    pub version: (u16, u16, u16), // major, minor, patch
    
    /// Spatial bounds of this snapshot
    pub bounds: BoundingBox,
}
```

### 3.2 Region Snapshot

```rust
pub struct RegionSnapshot {
    /// Unique region identifier
    pub id: RegionId,
    
    /// Region boundaries
    pub bounds: BoundingBox,
    
    /// Terrain data (heightmap, features)
    pub terrain: TerrainData,
    
    /// Climate summary
    pub climate: ClimateSummary,
    
    /// Settlement locations
    pub settlements: Vec<SettlementSummary>,
    
    /// Resource deposits
    pub deposits: Vec<ResourceDeposit>,
    
    /// Biological populations
    pub species: Vec<SpeciesPresence>,
}

pub struct TerrainData {
    /// Simplified heightmap (LOD-dependent resolution)
    pub elevation: Vec<f32>,
    
    /// Terrain type classification
    pub terrain_type: TerrainType,
    
    /// Water features (rivers, lakes)
    pub water_features: Vec<WaterFeature>,
    
    /// Vegetation coverage
    pub vegetation: VegetationSummary,
}

pub struct ClimateSummary {
    /// Current season
    pub season: Season,
    
    /// Temperature range (min, max, avg)
    pub temperature: (f32, f32, f32),
    
    /// Precipitation level
    pub precipitation: PrecipitationLevel,
    
    /// Weather events active
    pub weather_events: Vec<WeatherEventType>,
}
```

### 3.3 Population Snapshot

```rust
pub struct PopulationSnapshot {
    /// Region where population resides
    pub region_id: RegionId,
    
    /// Settlement (if applicable)
    pub settlement_id: Option<SettlementId>,
    
    /// Population size
    pub size: u32,
    
    /// Demographic breakdown
    pub demographics: Demographics,
    
    /// Cultural markers
    pub culture: CultureSummary,
    
    /// Economic activity
    pub economy: EconomicSummary,
    
    /// Social indicators
    pub social: SocialIndicators,
}

pub struct Demographics {
    /// Age distribution (buckets)
    pub age_distribution: Vec<(AgeBracket, u32)>,
    
    /// Gender distribution
    pub gender_ratio: f32, // male/female
    
    /// Health status
    pub health_average: f32, // 0.0 - 1.0
    
    /// Mortality rate
    pub mortality_rate: f32,
}

pub struct CultureSummary {
    /// Primary language family
    pub language_family: LanguageFamilyId,
    
    /// Religious traditions
    pub religious_traditions: Vec<ReligiousTraditionId>,
    
    /// Cultural practices (simplified)
    pub practices: Vec<CulturalPracticeType>,
    
    /// Material culture level
    pub material_culture: TechnologyLevel,
}

pub struct EconomicSummary {
    /// Primary subsistence mode
    pub subsistence: SubsistenceMode,
    
    /// Secondary activities
    pub secondary: Vec<EconomicActivity>,
    
    /// Trade connections
    pub trade_connections: u32,
    
    /// Resource stress level
    pub resource_stress: StressLevel,
}
```

### 3.4 Institution Snapshot

```rust
pub struct InstitutionSnapshot {
    /// Institution identifier
    pub id: InstitutionId,
    
    /// Institution type
    pub institution_type: InstitutionType,
    
    /// Geographic scope
    pub scope: GeographicScope,
    
    /// Member count
    pub member_count: u32,
    
    /// Influence level
    pub influence: InfluenceLevel,
    
    /// Current activities
    pub activities: Vec<InstitutionActivity>,
    
    /// Relationships with other institutions
    pub relationships: Vec<InstitutionRelation>,
}

pub enum InstitutionType {
    Kinship,      // Extended family networks
    Religious,    // Temples, shrines, priesthoods
    Economic,     // Guilds, trade associations
    Political,    // Councils, chieftaincies
    Military,     // War bands, defense leagues
    Knowledge,    // Schools, libraries, archives
}
```

---

## 4. Event Stream

### 4.1 Event Structure

```rust
/// Events emitted by simulation for frontend consumption
pub enum FrontendEvent {
    /// World state changes
    World(WorldEvent),
    
    /// Agent population changes
    Population(PopulationEvent),
    
    /// Institution lifecycle
    Institution(InstitutionEvent),
    
    /// Discovery events
    Discovery(DiscoveryEvent),
    
    /// Biological events
    Biology(BiologyEvent),
    
    /// Narrative events
    Narrative(NarrativeEvent),
}

pub struct EventHeader {
    /// Event type discriminator
    pub event_type: EventType,
    
    /// Simulation tick when event occurred
    pub tick: u64,
    
    /// Event timestamp
    pub timestamp: u64,
    
    /// Event priority (affects delivery guarantees)
    pub priority: EventPriority,
    
    /// Spatial location (if applicable)
    pub location: Option<WorldLocation>,
}

pub enum EventPriority {
    Critical,  // Guaranteed delivery, immediate processing
    High,      // Likely delivery, prompt processing
    Normal,    // Best effort delivery
    Low,       // May be dropped under load
}
```

### 4.2 World Events

```rust
pub enum WorldEvent {
    /// Climate change in region
    ClimateShift {
        region_id: RegionId,
        change: ClimateChange,
    },
    
    /// Natural disaster
    NaturalDisaster {
        disaster_type: DisasterType,
        location: WorldLocation,
        severity: Severity,
        affected_regions: Vec<RegionId>,
    },
    
    /// Resource depletion
    ResourceDepleted {
        resource_type: ResourceType,
        location: WorldLocation,
        depletion_rate: f32,
    },
    
    /// New resource discovery
    ResourceDiscovered {
        resource_type: ResourceType,
        location: WorldLocation,
        quantity: ResourceQuantity,
    },
}

pub enum DisasterType {
    Flood,
    Drought,
    Earthquake,
    VolcanicEruption,
    Plague,
    Famine,
    Wildfire,
}

pub enum Severity {
    Minor,
    Moderate,
    Major,
    Catastrophic,
}
```

### 4.3 Population Events

```rust
pub enum PopulationEvent {
    /// Population migration
    Migration {
        population_id: PopulationId,
        from_region: RegionId,
        to_region: RegionId,
        size: u32,
        reason: MigrationReason,
    },
    
    /// Population growth milestone
    GrowthMilestone {
        population_id: PopulationId,
        milestone: GrowthMilestone,
        new_size: u32,
    },
    
    /// Conflict outbreak
    Conflict {
        conflict_type: ConflictType,
        parties: Vec<PopulationId>,
        location: WorldLocation,
        intensity: ConflictIntensity,
    },
    
    /// Cultural shift
    CulturalShift {
        population_id: PopulationId,
        shift_type: CulturalShiftType,
        magnitude: f32,
    },
    
    /// Technological adoption
    TechAdoption {
        population_id: PopulationId,
        technology: TechnologyId,
        adoption_rate: f32,
    },
}

pub enum MigrationReason {
    ResourceScarcity,
    ClimateChange,
    Conflict,
    TradeOpportunity,
    Exploration,
    Forced, // slavery, displacement
}

pub enum ConflictType {
    Raid,
    War,
    Feud,
    TradeDispute,
    TerritorialDispute,
    SuccessionCrisis,
}
```

### 4.4 Discovery Events

```rust
pub enum DiscoveryEvent {
    /// New technology discovered
    TechnologyDiscovered {
        discoverer: PopulationId,
        technology: TechnologyId,
        method: DiscoveryMethod,
    },
    
    /// New region explored
    RegionExplored {
        explorer: PopulationId,
        region_id: RegionId,
        exploration_depth: ExplorationDepth,
    },
    
    /// Species domesticated
    SpeciesDomesticated {
        domesticator: PopulationId,
        species: SpeciesId,
        domestication_level: DomesticationLevel,
    },
    
    /// New material processed
    MaterialProcessed {
        processor: PopulationId,
        material: MaterialId,
        process: ProcessId,
    },
}

pub enum DiscoveryMethod {
    Experimentation,
    Observation,
    TradeTransfer,
    Conquest,
    IndependentInvention,
}
```

---

## 5. Query Protocol

### 5.1 Query Types

```rust
/// Frontend queries for simulation data
pub enum DataQuery {
    /// Request world snapshot
    WorldSnapshot(WorldSnapshotQuery),
    
    /// Request region data
    RegionQuery(RegionQuery),
    
    /// Request population data
    PopulationQuery(PopulationQuery),
    
    /// Request historical data
    HistoryQuery(HistoryQuery),
    
    /// Request entity details
    EntityQuery(EntityQuery),
}

pub struct WorldSnapshotQuery {
    /// Spatial bounds (None = entire world)
    pub bounds: Option<BoundingBox>,
    
    /// Detail level
    pub detail_level: DetailLevel,
    
    /// Specific tick (None = current)
    pub at_tick: Option<u64>,
    
    /// Data categories to include
    pub categories: Vec<DataCategory>,
}

pub enum DetailLevel {
    /// Minimal data (overview)
    Low,
    /// Standard detail
    Medium,
    /// Full detail
    High,
    /// Maximum detail (debug)
    Maximum,
}

pub enum DataCategory {
    Terrain,
    Climate,
    Population,
    Institutions,
    Resources,
    Discoveries,
    Biology,
    All,
}
```

### 5.2 Spatial Queries

```rust
pub struct RegionQuery {
    /// Region identifier
    pub region_id: RegionId,
    
    /// Detail level for response
    pub detail_level: DetailLevel,
    
    /// Data categories
    pub categories: Vec<DataCategory>,
}

pub struct SpatialQuery {
    /// Center point
    pub center: WorldLocation,
    
    /// Radius in world units
    pub radius: f32,
    
    /// Entity types to include
    pub entity_types: Vec<EntityType>,
    
    /// Maximum results
    pub limit: Option<usize>,
}

pub struct ViewportQuery {
    /// Viewport bounds
    pub viewport: BoundingBox,
    
    /// Zoom level (affects LOD)
    pub zoom: ZoomLevel,
    
    /// Data layers to include
    pub layers: Vec<MapLayer>,
}

pub enum MapLayer {
    Terrain,
    Climate,
    PopulationDensity,
    ResourceDistribution,
    TradeRoutes,
    PoliticalBoundaries,
    CulturalRegions,
    Institutions,
}
```

### 5.3 Historical Queries

```rust
pub struct HistoryQuery {
    /// Time range
    pub time_range: TimeRange,
    
    /// Entity to query history for
    pub subject: HistorySubject,
    
    /// Metric to track
    pub metric: HistoryMetric,
    
    /// Sampling resolution
    pub resolution: TimeResolution,
}

pub enum HistorySubject {
    World,
    Region(RegionId),
    Population(PopulationId),
    Institution(InstitutionId),
}

pub enum HistoryMetric {
    PopulationSize,
    ResourceAvailability,
    TechnologyLevel,
    ConflictIntensity,
    TradeVolume,
    CulturalDiversity,
    InstitutionCount,
}

pub enum TimeResolution {
    /// Per-tick data
    Tick,
    /// Daily aggregates
    Day,
    /// Monthly aggregates
    Month,
    /// Yearly aggregates
    Year,
    /// Decade aggregates
    Decade,
}
```

---

## 6. Serialization Format

### 6.1 Wire Format

```rust
/// Binary serialization for efficient transport
pub mod binary {
    use serde::{Serialize, Deserialize};
    
    /// Message envelope
    #[derive(Serialize, Deserialize)]
    pub struct Message {
        /// Message type
        pub msg_type: MessageType,
        
        /// Payload version
        pub version: u16,
        
        /// Compression flag
        pub compressed: bool,
        
        /// Serialized payload
        pub payload: Vec<u8>,
    }
    
    #[derive(Serialize, Deserialize)]
    pub enum MessageType {
        Snapshot,
        Event,
        Query,
        Response,
        Error,
    }
}

/// JSON serialization for debugging and web APIs
pub mod json {
    /// All contract types implement Serialize for JSON output
    /// Use #[serde(rename_all = "camelCase")] for JS compatibility
    /// Use #[serde(skip_serializing_if = "Option::is_none")] for optional fields
}
```

### 6.2 Compression Strategy

```rust
pub enum CompressionStrategy {
    /// No compression (small messages)
    None,
    /// Deflate for general compression
    Deflate,
    /// LZ4 for speed-critical paths
    Lz4,
    /// Zstd for large snapshots
    Zstd,
}

/// Apply compression based on payload size
pub fn auto_compress(payload: &[u8]) -> (Vec<u8>, CompressionStrategy) {
    const COMPRESS_THRESHOLD: usize = 1024;
    
    if payload.len() < COMPRESS_THRESHOLD {
        (payload.to_vec(), CompressionStrategy::None)
    } else {
        // Use zstd for large payloads
        let compressed = zstd::encode_all(payload, 3)
            .expect("zstd compression failed");
        (compressed, CompressionStrategy::Zstd)
    }
}
```

### 6.3 Schema Versioning

```rust
/// Schema version for compatibility checking
pub const SCHEMA_VERSION: (u16, u16, u16) = (0, 1, 0);

/// Version compatibility check
pub fn check_compatibility(client_version: (u16, u16, u16)) -> Compatibility {
    let (major, minor, patch) = SCHEMA_VERSION;
    let (c_major, c_minor, _c_patch) = client_version;
    
    if c_major != major {
        Compatibility::Incompatible
    } else if c_minor > minor {
        Compatibility::ClientAhead
    } else {
        Compatibility::Compatible
    }
}

pub enum Compatibility {
    /// Fully compatible
    Compatible,
    /// Client newer than server (may have unsupported features)
    ClientAhead,
    /// Major version mismatch
    Incompatible,
}
```

---

## 7. Error Handling

### 7.1 Error Types

```rust
/// Errors that can occur in frontend data contract
pub enum DataContractError {
    /// Query validation failed
    InvalidQuery { reason: String },
    
    /// Requested data not available
    DataUnavailable { data_type: String },
    
    /// Rate limit exceeded
    RateLimited { retry_after: u64 },
    
    /// Schema version mismatch
    VersionMismatch { 
        client: (u16, u16, u16), 
        server: (u16, u16, u16) 
    },
    
    /// Serialization error
    SerializationError { details: String },
    
    /// Spatial query out of bounds
    OutOfBounds { 
        requested: BoundingBox, 
        available: BoundingBox 
    },
}

impl DataContractError {
    /// HTTP-like status code for error classification
    pub fn status_code(&self) -> u16 {
        match self {
            Self::InvalidQuery { .. } => 400,
            Self::DataUnavailable { .. } => 404,
            Self::RateLimited { .. } => 429,
            Self::VersionMismatch { .. } => 426, // Upgrade Required
            Self::SerializationError { .. } => 500,
            Self::OutOfBounds { .. } => 416, // Range Not Satisfiable
        }
    }
}
```

---

## 8. Implementation Notes

### 8.1 Backend Responsibilities

1. **Snapshot Generation**: Efficiently serialize world state
2. **Event Filtering**: Route events to interested subscribers
3. **Query Processing**: Execute spatial and temporal queries
4. **Rate Limiting**: Prevent frontend from overwhelming simulation
5. **Caching**: Cache frequently-requested snapshots

### 8.2 Frontend Responsibilities

1. **Subscription Management**: Subscribe to relevant event streams
2. **Viewport Tracking**: Request data for visible regions only
3. **Caching**: Cache received data to minimize re-queries
4. **Interpolation**: Smooth visual transitions between updates
5. **Error Recovery**: Handle disconnections and version mismatches

### 8.3 Performance Targets

| Metric | Target | Notes |
|--------|--------|-------|
| Snapshot latency | < 100ms | For 1000 regions |
| Event delivery | < 50ms | Critical events |
| Query response | < 200ms | Complex spatial query |
| Serialization | < 50ms | Full world snapshot |
| Bandwidth | < 1MB/s | Typical frontend load |

---

## 9. Future Extensions

### 9.1 Planned Features

- **WebSocket Support**: Real-time bidirectional communication
- **GraphQL Interface**: Flexible query composition
- **Delta Compression**: Send only changed fields
- **Predictive Fetching**: Pre-fetch based on viewport movement
- **Multiplayer Sync**: Synchronize multiple frontend clients

### 9.2 Version History

| Version | Date | Changes |
|---------|------|---------|
| 0.1.0 | 2025-01-14 | Initial specification |

---

## 10. References

- [SpecSuite.md](SpecSuite.md) - Master specification
- [09_FrontendAndPresentation.md](09_FrontendAndPresentation.md) - Frontend architecture
- [Spec_ScenarioSchema.md](Spec_ScenarioSchema.md) - Scenario data format
- [Spec_EventSchema.md](Spec_EventSchema.md) - Event serialization
