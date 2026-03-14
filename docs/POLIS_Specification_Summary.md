# POLIS Specification Summary

**Version:** 0.1  
**Date:** 14 March 2026  
**Purpose:** Comprehensive overview of the POLIS civilization simulation specification suite

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Core Philosophy](#2-core-philosophy)
3. [Document Overview](#3-document-overview)
4. [World Model (01)](#4-world-model-01)
5. [State Model (02)](#5-state-model-02)
6. [Collective Agency (03)](#6-collective-agency-03)
7. [Resources and Materials (04)](#7-resources-and-materials-04)
8. [Discovery Heuristics (05)](#8-discovery-heuristics-05)
9. [Biology and Domestication (06)](#9-biology-and-domestication-06)
10. [Society and Institutions (07)](#10-society-and-institutions-07)
11. [Validation and Experiments (08)](#11-validation-and-experiments-08)
12. [Technical Architecture (10)](#12-technical-architecture-10)
13. [Cross-Cutting Concepts](#13-cross-cutting-concepts)
14. [Key Relationships](#14-key-relationships)

---

## 1. Introduction

POLIS is a multi-scale civilization simulator designed as a serious scientific instrument rather than a conventional game. It models the emergence and transformation of human societies from foraging bands to complex polities through explicit mechanisms rather than scripted progression.

The specification suite defines:
- The ontological foundations of the simulation
- How state is represented and evolves
- How collective actors emerge and function
- How resources, materials, and environments work
- How knowledge and technology develop
- How biological systems and agriculture function
- How societies, institutions, and conflict operate
- How the system is validated and experimented with
- The technical architecture supporting all of the above

---

## 2. Core Philosophy

### 2.1 Scientific Instrument Stance
POLIS is designed as a scientific instrument, not just a world generator. It must earn trust through:
- Explicit separation of verification, validation, uncertainty analysis, calibration, and interpretation
- Reproducibility and provenance tracking
- Structured pattern comparison rather than exact prediction

### 2.2 Mechanism Over Scripting
- No fixed tech trees or scripted invention ladders
- No vague faction labels as substitutes for social explanation
- No toy resource counters without physical depth
- No unmanageable item or chemistry explosions

### 2.3 Emergence Over Design
- Macro structures should emerge from micro mechanisms
- Civilization trajectories should arise from interactions, not be predetermined
- Historical plausibility without requiring exact reproduction

### 2.4 Multi-Scale Representation
- Individual actors at the micro level
- Households, groups, and organizations at meso levels
- Polities, institutions, and large-scale systems at macro levels
- Explicit scale-bridging mechanisms

---

## 3. Document Overview

| Document | Purpose | Key Dependencies |
|----------|---------|------------------|
| 01_WorldModel.md | Spatial, temporal, and environmental foundations | None (foundational) |
| 02_StateModel.md | Core state representation and semantics | 01 |
| 03_CollectiveAgency.md | How collective actors form and function | 01, 02 |
| 04_ResourcesAndMaterials.md | Resource ontology and material affordances | 01, 02 |
| 05_DiscoveryHeuristics.md | Knowledge, discovery, and technology | 01, 02, 04 |
| 06_BiologyAndDomestication.md | Biological systems and agriculture | 01, 02, 04, 05 |
| 07_SocietyAndInstitutions.md | Social structure and governance | 01, 02, 03, 04, 05, 06 |
| 08_ValidationAndExperiments.md | Scientific validation framework | All prior |
| 10_TechnicalArchitecture.md | Implementation architecture | All prior |

---

## 4. World Model (01)

### 4.1 Core Purpose
Defines the spatial, temporal, and environmental substrate on which all civilization processes operate.

### 4.2 Key Ontological Categories

#### Spatial Representation
- **World**: The global container for all simulation state
- **Region**: Large-scale geographic divisions (continents, climatic zones)
- **Patch**: The fundamental unit of local spatial representation
- **Settlement**: Concentrated human habitation (villages, towns, cities)
- **Route**: Connections between locations (roads, rivers, trade paths)
- **Territory**: Bounded jurisdictional space

#### Temporal Structure
- **Tick**: The smallest indivisible time unit
- **Phase**: Logical groupings of system updates within a tick
- **Epoch**: Longer periods defined by structural characteristics
- **Season/Year**: Calendar time for biological and agricultural cycles

#### Environmental Systems
- **ClimateSystem**: Temperature, precipitation, and their variability
- **Biome**: Characteristic ecological communities
- **Hydrology**: Water systems including surface water, groundwater, and drainage
- **Geology**: Soil, minerals, and geological processes

### 4.3 Key Design Principles
- Spatial representation must support multi-scale dynamics
- Time must accommodate processes from daily decisions to centuries-long transformations
- Environment must be dynamic and coupled to human activity
- World generation must produce historically plausible configurations

---

## 5. State Model (02)

### 5.1 Core Purpose
Defines how simulation state is represented, owned, and evolves over time.

### 5.2 Core State Categories

#### Actor State
- **Individual**: Single persons with attributes, skills, and social ties
- **Household**: Domestic units making collective decisions
- **Group**: Temporary or persistent collectives
- **Organization**: Formal institutions with explicit structure
- **Polity**: Territorial governance structures

#### Spatial State
- **PatchState**: Local conditions and contents
- **SettlementState**: Concentrated population and infrastructure
- **RouteState**: Connection status and capacity
- **TerritoryState**: Jurisdictional boundaries and claims

#### Resource State
- **ResourceStock**: Quantities of available resources
- **Infrastructure**: Built systems modifying affordances
- **EnvironmentalField**: Distributed conditions affecting productivity

#### Social State
- **SocialTie**: Relationships between actors
- **SocialNetwork**: Patterned topology of ties
- **Institution**: Rule bundles constraining action
- **Norm**: Shared expectations

### 5.3 State Ownership Rules
- Every piece of state has exactly one authoritative owner
- Ownership determines who can mutate and who must observe
- Cross-scale references must be explicit and tracked
- Derived/cached state must be marked and disposable

### 5.4 State Transitions
- **Events**: Discrete occurrences causing state changes
- **Processes**: Continuous or repeated dynamics
- **Decisions**: Agent choices affecting state
- **Shocks**: External perturbations

---

## 6. Collective Agency (03)

### 6.1 Core Purpose
Defines how individual actors form collective actors and how those collectives can have agency.

### 6.2 Core Ontology

#### Collective Formation
- **Collective**: Any group of actors treated as a unit
- **Membership**: Relationship between individual and collective
- **Boundary**: Definition of who is in and who is out
- **Identity**: Shared self-conception of the collective

#### Collective Decision-Making
- **DecisionProcedure**: Rules for reaching collective choices
- **PreferenceAggregation**: How individual preferences combine
- **AuthorityStructure**: Who has decision rights
- **Deliberation**: Process of reaching collective judgment

#### Collective Action
- **JointIntention**: Shared commitment to action
- **Coordination**: Aligning individual actions
- **DivisionOfLabor**: Role specialization within collectives
- **Monitoring**: Checking compliance with collective decisions

#### Collective State
- **CollectiveBelief**: Shared or common knowledge
- **CollectiveMemory**: Retained experience of the collective
- **CollectiveResource**: Assets owned by the collective
- **CollectiveObligation**: Commitments binding the collective

### 6.3 Scale Relations
- **Composition**: How lower-level units form higher-level units
- **Decomposition**: How higher-level units can be analyzed into parts
- **Emergence**: When collective properties are not simple sums
- **DownwardCausation**: How collectives constrain constituents

---

## 7. Resources and Materials (04)

### 7.1 Core Purpose
Defines the resource ontology, material classes, property model, and process templates.

### 7.2 Five Coupled Layers

#### ResourceSystem
Spatially bounded ecological, geological, hydrological, or managed systems that generate, store, regulate, or route resources.
- Examples: forests, river basins, aquifers, ore bodies, fisheries

#### ResourceStock
Localized or stored quantities of usable resource within a system, place, or structure.
- Examples: standing timber, grain in granary, ore in seam, water in reservoir

#### MaterialEnergyCarrier
Materials or usable energy forms represented by canonical classes and property vectors.
- Defined by what their properties allow them to do, not by item names
- Properties include: energy density, durability, workability, preservation, transportability

#### EnvironmentalField
Spatially distributed conditions shaping resource productivity and process feasibility.
- Examples: temperature, humidity, soil moisture, solar radiation, wind

#### InfrastructureLayer
Built or maintained systems changing effective access to resources and movement.
- Examples: roads, canals, reservoirs, kilns, furnaces, ports, mills

### 7.3 Canonical Material Categories

#### Raw Materials
- Stone, clay, sand, gravel
- Timber (various types)
- Ore and metal-bearing rock
- Fiber sources (plant and animal)

#### Processed Materials
- Lumber, charcoal, lime, ash
- Smelted metals, alloys
- Textiles, leather, pottery

#### Fuels and Energy Carriers
- Firewood, charcoal, dried dung
- Peat, coal (where appropriate)
- Animal and human muscle power
- Water and wind power

#### Nutrient-Bearing Matter
- Manure, compost, bone meal
- Ash, lime, marine deposits
- Fallow vegetation, legume residues

### 7.4 Process Templates
Generic operational sequences acting on property vectors:
- **Extraction**: Removing resources from systems
- **Transformation**: Converting materials between states
- **Transport**: Moving resources spatially
- **Storage**: Preserving resources over time
- **Construction**: Building infrastructure

---

## 8. Discovery Heuristics (05)

### 8.1 Core Purpose
Defines how affordance discovery, process learning, technique refinement, and knowledge transmission work without fixed tech trees.

### 8.2 Knowledge Ontology

#### Affordance
Realized action possibility linking actor capability, material state, tool state, environmental context, and expected effect.
- Recognition that "this can be done here with these materials"

#### Observation
Episodic record of an action and its outcome.
- What was attempted, under what conditions, what happened

#### ProcessSchema
Generalized operational sequence representing ordered steps, expected inputs, intermediate states, and outputs.

#### Technique
Process schema that has become purposeful and reproducible for intentional reuse.
- Includes parameter ranges, expected payoffs, risks, required inputs, required skills

#### Skill
Competence in executing techniques under varying conditions.
- Belongs to individuals, households, roles, or collective actors

#### CausalBelief
Explicit or semi-explicit proposition linking conditions, actions, and outcomes.
- May be true, false, partial, or overgeneralized

#### Routine
Recurrent, socially reproduced use of techniques across roles and time.
- Examples: annual planting cycles, seasonal fuel-making, kiln schedules

#### InstitutionalProcedure
Routine anchored in an institution with prescribed roles, permissions, obligations, and enforcement.

#### DesignSchema
Abstract pattern for tools, artifacts, or structures realizable through techniques.

#### CulturalTraitBundle
Transmissible package containing multiple linked knowledge elements.
- Examples: terrace agriculture package, metalworking package

### 8.3 Discovery Mechanisms
- **Latent Affordance Recognition**: Noticing possibilities in the world
- **Heuristic Search**: Bounded exploration of process space
- **Trial and Error**: Learning from success and failure
- **Social Learning**: Transmission across individuals and groups
- **Institutional Stabilization**: Successful practices becoming durable

---

## 9. Biology and Domestication (06)

### 9.1 Core Purpose
Defines biological ontology and dynamics for wild ecologies, crops, livestock, soils, disease, and managed niches.

### 9.2 Core Biological Ontology

#### WildEcologicalCommunity
Unmanaged assemblage of organisms providing foraged goods, hunting, ecosystem services, and disease reservoirs.

#### ManagedNicheSystem
Environment altered by human activity to favor certain species without full domestication.
- Examples: burned landscapes, protected groves, selectively weeded stands

#### Cultivar
Plant lineage shaped by recurrent human selection defining trait profiles for yield, harvestability, seasonality, stress tolerance.

#### Breed
Managed animal lineage shaped by selection defining traits for tameness, reproduction, feed conversion, traction suitability.

#### OrganismPopulation
Local population or managed biological group represented above the single-organism level.
- Useful for crops, herds, pest populations, forest stands

#### Agroecosystem
Managed social-ecological unit coupling human labor, knowledge, structures, soils, plants, animals, and water control.
- Examples: rainfed fields, irrigated terraces, mixed crop-livestock landscapes

#### SoilMatrix
Biologically active substrate supporting plant growth and water retention.
- Captures nutrient condition, organic matter, water-holding behavior, degradation/recovery

#### PathogenPressure
Disease and pest burden from host density, reservoirs, proximity, and environmental conditions.

### 9.3 Domestication Trajectory
1. **Wild**: Unmanaged or lightly disturbed
2. **Managed**: Intentionally biased by human action
3. **Domesticated**: Stable selection for human-dependent or human-favored traits

### 9.4 Biological Coupling to Society
- Food and surplus production
- Labor and traction availability
- Disease burden and mortality
- Fertilizer and soil recovery
- Mobility and transport capacity
- Settlement density constraints
- Taxation legibility
- Wealth inheritance patterns
- Long-run path dependence

---

## 10. Society and Institutions (07)

### 10.1 Core Purpose
Defines social variables, institutional mechanisms, exchange systems, coercive structures, and collapse dynamics.

### 10.2 Core Social Ontology

#### SocialTie
Directed or undirected relation between actors.
- Types: kinship, alliance, trade partnership, patronage, obedience, hostility, debt

#### SocialNetwork
Patterned topology of ties shaping information flow, reputation, coalition formation, practice diffusion.

#### Norm
Socially shared expectation about what actors should do.
- May be informal, weakly enforced, internalized, locally variable

#### Institution
Structured rule bundle constraining and enabling action in defined contexts.
- Defines: participants, permitted actions, obligations, sanctions, monitors

#### Organization
Collective actor with explicit roles, assets, and routines.
- Examples: guilds, militias, merchant coalitions, councils, temple administrations

#### ActionArena
Structured context for actor interaction under biophysical conditions, community attributes, and rules-in-use.
- Examples: markets, courts, irrigation allocation, elite councils

#### SettlementOrder
Patterned institutional and social arrangement of a settlement.
- Includes: local governance, market structures, defense, sanitation norms

#### PolityOrder
Layered institutional arrangement for territorial governance, extraction, force, and adjudication.
- Includes: office hierarchy, tax system, military chain, legal rules

### 10.3 Core Social Variables

#### Cooperation Pressures
- Collective action problems
- Trust and reputation
- Social dilemmas
- Coordination challenges

#### Conflict Pressures
- Resource competition
- Status rivalry
- Territorial disputes
- Succession crises

#### Hierarchy and Inequality
- Wealth concentration
- Status differentiation
- Power asymmetries
- Social stratification

#### Exchange Systems
- Reciprocal obligations
- Market exchange
- Redistribution
- Tribute and taxation

### 10.4 Institutional Dynamics
- **Emergence**: How institutions form from repeated interactions
- **Stabilization**: How institutions become durable
- **Transformation**: How institutions change
- **Collapse**: How institutions fail

---

## 11. Validation and Experiments (08)

### 11.1 Core Purpose
Defines how POLIS is validated, tested, calibrated, experimented with, and interpreted scientifically.

### 11.2 Five Validity Dimensions

#### Internal Validity
Whether the model's causal structure is coherent relative to its assumptions and research purpose.
- Ontology coherence, mechanism coherence, absence of contradictions

#### Numerical and Computational Validity
Whether algorithms and implementations behave correctly, stably, and predictably.
- Code correctness, numerical convergence, stability under resolution changes

#### Behavioural Validity
Whether emergent outputs match stylized facts, known qualitative regimes, theoretically expected patterns.

#### Experimental Validity
Whether studies are designed and analyzed in scientifically defensible ways.
- Proper controls, explicit hypotheses, suitable ensemble sizes, appropriate metrics

#### External and Historical Plausibility
Whether scenarios and results remain plausible given empirical evidence and comparative history.

### 11.3 Seven Validation Categories
1. **Conceptual and Structural Validation**: Model design coherence
2. **Code Verification and Numerical Analysis**: Implementation correctness
3. **Behavioural and Pattern Validation**: Output pattern matching
4. **Sensitivity and Uncertainty Analysis**: Robustness checking
5. **Calibration and History Matching**: Parameter constraint
6. **Scenario and Experiment Design Validation**: Study quality
7. **Reproducibility and Provenance**: Auditability

### 11.4 Experiment Requirements
- **Hypothesis-Driven**: Explicit questions and predictions
- **Controlled**: Appropriate comparison conditions
- **Ensemble-Based**: Multiple runs for stochastic processes
- **Metric-Rich**: Multiple measures of outcomes
- **Documented**: Full provenance of all decisions

---

## 12. Technical Architecture (10)

### 12.1 Core Purpose
Defines the architectural layers, runtime contracts, dataflow rules, performance strategy, and integration boundaries.

### 12.2 Seven Architectural Layers

#### 1. Ontology and Specification Layer
- Encodes formal ontology from specifications
- Defines entity classes, field types, resource categories, process families
- Provides machine-readable model descriptions
- **Must Not Own**: live instances, mutable state, scheduling, frontend

#### 2. State Model Layer
- Materializes conceptual ontology into authoritative runtime state
- Stores agent state, collective state, spatial fields, resource stocks
- Enforces ownership rules
- **Must Not Own**: domain behavior, global time, experiment logic, visualization

#### 3. Process and System Layer
- Implements all world dynamics as systems operating on state
- Covers ecological, demographic, economic, institutional, military, discovery processes
- Emits commands, events, deltas, diagnostics
- **Must Not Own**: authoritative state storage, persistence, UI, orchestration

#### 4. Scheduling and Time Layer
- Advances simulation time
- Coordinates fixed-step, variable-step, event-driven, multi-rate processes
- Resolves ordering between systems
- **Must Not Own**: domain semantics beyond contracts, persistence, presentation

#### 5. Persistence and Provenance Layer
- Records simulation lifecycle as snapshots, event logs, run metadata
- Supports replay, auditing, scientific provenance
- Captures model version, scenario definition, parameters, seeds
- **Must Not Own**: simulation decisions, scheduling authority, interpretation

#### 6. Experiment Orchestration Layer
- Runs parameter sweeps, ablations, calibration campaigns
- Allocates runs across compute resources
- Manages ensembles, seeds, stopping rules, output bundles
- **Must Not Own**: low-level world dynamics

#### 7. Presentation and Tooling Layer
- Provides visualization, inspection, and interaction tools
- Consumes snapshots, streams, and derived views
- **Must Not Own**: authoritative state, simulation logic, scheduling

### 12.3 Key Technical Principles
- **CPU-first** for correctness, reference behavior, broad portability
- **Data-oriented** in hot paths without collapsing conceptual ontology
- **Event-rich and provenance-aware** rather than opaque and state-only
- **Frontend-decoupled** with presentation consuming derived views
- **Batch-capable from the start** for scientific workflows

### 12.4 State Structure Recommendations
- Entity/component stores for heterogeneous actors
- Structure-of-arrays for hot homogeneous data
- Dense field grids for spatial variables (soils, water, climate)
- Graph/adjacency stores for social, political, trade links
- Aggregate caches only where marked as derived

### 12.5 Time Model
- Master scheduler with multi-rate execution
- Fast cadence: local actions, trade decisions, movement, disease contact
- Medium cadence: household allocation, market adjustment, institutional cycles
- Slow cadence: seasonal biology, infrastructure wear, soil change, demographic shifts

---

## 13. Cross-Cutting Concepts

### 13.1 Scale and Multi-Scale Dynamics
- **Micro**: Individual actors, single patches, immediate interactions
- **Meso**: Households, groups, local settlements, small organizations
- **Macro**: Polities, large institutions, regional systems, long-term trends
- **Scale-Bridging**: Explicit mechanisms connecting levels (composition, emergence, downward causation)

### 13.2 Coupling and Feedback
- **Environment-Society**: Climate, resources, and human activity
- **Biology-Society**: Agriculture, disease, labor, and social structure
- **Technology-Society**: Knowledge, techniques, and institutional forms
- **Conflict-Cooperation**: Competition and collaboration dynamics

### 13.3 Path Dependence
- Historical contingency in development trajectories
- Lock-in effects from early choices
- Critical junctures and branching points
- Irreversibilities and hysteresis

### 13.4 Uncertainty and Variability
- Stochastic processes in ecology, demography, discovery
- Parameter uncertainty and sensitivity
- Scenario uncertainty and robustness
- Epistemic uncertainty in knowledge systems

### 13.5 Representation Levels
- **Individual**: Specific actors with full detail
- **Aggregate**: Statistical summaries of populations
- **Typified**: Representative instances standing for categories
- **Hybrid**: Mixing levels where appropriate

---

## 14. Key Relationships

### 14.1 Ontology Dependencies
```
01_WorldModel (foundational)
    ↓
02_StateModel (state representation)
    ↓
    ├── 03_CollectiveAgency (collective actors)
    ├── 04_ResourcesAndMaterials (resources)
    │       ↓
    │   05_DiscoveryHeuristics (knowledge)
    │       ↓
    │   06_BiologyAndDomestication (biology)
    │       ↓
    └── 07_SocietyAndInstitutions (society)
            ↓
        08_ValidationAndExperiments (validation)
            ↓
        10_TechnicalArchitecture (implementation)
```

### 14.2 Core Feedback Loops

#### Resource-Institution Loop
```
Resources → Extraction → Surplus → Institutions → 
Resource Management → Resource Sustainability/Depletion
```

#### Knowledge-Practice Loop
```
Affordances → Observations → Techniques → Routines → 
InstitutionalProcedures → CulturalTransmission → NewAffordances
```

#### Social-Conflict Loop
```
SocialTies → Cooperation → CollectiveAction → 
Success/Failure → StatusChange → Conflict/Alliance → NewSocialTies
```

#### Agriculture-Society Loop
```
Domestication → Agriculture → Surplus → Settlement → 
PopulationGrowth → LaborSpecialization → InstitutionBuilding → 
AgriculturalIntensification
```

### 14.3 Critical Couplings

| Domain A | Domain B | Coupling Mechanism |
|----------|----------|-------------------|
| Climate | Agriculture | Growing conditions, water availability |
| Agriculture | Population | Food supply, carrying capacity |
| Population | Institutions | Scale effects, coordination needs |
| Institutions | Technology | Knowledge preservation, R&D support |
| Technology | Resources | Extraction efficiency, substitution |
| Resources | Conflict | Scarcity, competition, warfare |
| Conflict | Institutions | Military organization, state formation |
| Institutions | Economy | Property rights, markets, taxation |

---

## 15. Summary of Key Design Decisions

### 15.1 What POLIS Is
- A multi-scale civilization simulator
- A scientific instrument for studying social-ecological dynamics
- A mechanism-based rather than script-based model
- An emergence-focused rather than design-focused system

### 15.2 What POLIS Is Not
- A game with victory conditions or balanced gameplay
- A historical simulation requiring exact reproduction
- A closed-form mathematical model
- A toy model with no empirical grounding

### 15.3 Core Commitments
1. **Ontological explicitness**: Clear definitions of what exists in the model
2. **Mechanistic transparency**: How things happen is inspectable
3. **Multi-scale integrity**: Connections between levels are principled
4. **Scientific accountability**: Validation and reproducibility are first-class
5. **Historical plausibility**: Outcomes should be believable without being predetermined

### 15.4 Open Questions (for future development)
- Specific parameter values and calibration targets
- Implementation language and framework choices
- Frontend technology and visualization approaches
- Performance optimization strategies
- Integration with external data sources
- User interaction and scenario authoring tools

---

## 16. Document Cross-Reference Index

| Concept | Primary Document | Related Documents |
|---------|-----------------|-------------------|
| Actor | 02_StateModel | 03, 07 |
| Affordance | 05_DiscoveryHeuristics | 04, 06 |
| Agroecosystem | 06_BiologyAndDomestication | 04, 07 |
| Biome | 01_WorldModel | 04, 06 |
| Breed | 06_BiologyAndDomestication | 04, 05 |
| ClimateSystem | 01_WorldModel | 04, 06 |
| Collective | 03_CollectiveAgency | 02, 07 |
| Cultivar | 06_BiologyAndDomestication | 04, 05 |
| DecisionProcedure | 03_CollectiveAgency | 02, 07 |
| EnvironmentalField | 04_ResourcesAndMaterials | 01, 06 |
| Event | 02_StateModel | All |
| Household | 02_StateModel | 03, 06, 07 |
| Individual | 02_StateModel | 03, 05, 07 |
| InfrastructureLayer | 04_ResourcesAndMaterials | 02, 07 |
| Institution | 07_SocietyAndInstitutions | 03, 05 |
| MaterialEnergyCarrier | 04_ResourcesAndMaterials | 05, 06 |
| Norm | 07_SocietyAndInstitutions | 03, 05 |
| Organization | 07_SocietyAndInstitutions | 02, 03 |
| Patch | 01_WorldModel | 02, 04, 06 |
| PathogenPressure | 06_BiologyAndDomestication | 01, 07 |
| Polity | 02_StateModel | 03, 07 |
| ProcessSchema | 05_DiscoveryHeuristics | 04, 06 |
| ResourceStock | 04_ResourcesAndMaterials | 02, 06, 07 |
| ResourceSystem | 04_ResourcesAndMaterials | 01, 06 |
| Settlement | 01_WorldModel | 02, 06, 07 |
| Skill | 05_DiscoveryHeuristics | 02, 04 |
| SocialNetwork | 07_SocietyAndInstitutions | 02, 03 |
| SocialTie | 07_SocietyAndInstitutions | 02, 03 |
| SoilMatrix | 06_BiologyAndDomestication | 01, 04 |
| Technique | 05_DiscoveryHeuristics | 04, 06, 07 |
| Territory | 01_WorldModel | 02, 07 |
| Validation | 08_ValidationAndExperiments | All |

---

*This summary document provides a comprehensive overview of the POLIS specification suite. For detailed definitions, refer to the individual specification documents.*
