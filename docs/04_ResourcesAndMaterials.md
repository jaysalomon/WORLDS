# POLIS Resources, Materials, And Physical Affordances

**Version:** 0.1  
**Date:** 14 March 2026  
**Document:** 04 of the POLIS spec suite  
**Purpose:** Define the resource ontology, material classes, property model, environmental coupling, and process-template logic that underpin extraction, production, storage, transport, ecology, and discovery in POLIS.

## 1. Scope

This document defines how POLIS represents:

- resource systems
- resource stocks and flows
- material and energy carriers
- environmental fields relevant to materials and productivity
- infrastructure-mediated resource access and transformation
- generic process templates operating over the above

This document does **not** define:

- detailed governance systems
- full biological or domestication rules
- discovery heuristics in full
- implementation-specific data layout

Institutions are referenced here only where resource access and process feasibility depend on them.

## 2. Dependencies

This document depends on:

- [01_WorldModel.md](/abs/path/e:/Drive/WORLDS/01_WorldModel.md)
- [02_StateModel.md](/abs/path/e:/Drive/WORLDS/02_StateModel.md)

It provides foundational concepts for:

- `05_DiscoveryHeuristics.md`
- `06_BiologyAndDomestication.md`
- `07_SocietyAndInstitutions.md`
- `08_ValidationAndExperiments.md`

## 3. Guiding Principle

POLIS should use a mesoscopic, property-based resource ontology.

The system must avoid both:

- toy resource counters with no physical depth
- unmanageable item or chemistry explosions

The model should therefore represent:

- a limited set of canonical resource and material classes
- compact but expressive property vectors
- process templates that act on those properties
- environmental and infrastructural constraints on feasibility

Specific named technologies should emerge from these combinations rather than from a fixed tech tree.

## 4. Core Ontology Layers

POLIS resource logic is built around five coupled layers.

### 4.1 ResourceSystem

A `ResourceSystem` is a spatially bounded ecological, geological, hydrological, or managed system that generates, stores, regulates, or routes resources over time.

Examples:

- forest
- pasture
- river basin
- aquifer
- fishery
- ore body
- agricultural zone
- managed irrigation network

`ResourceSystem` is the large-scale context for productivity, renewal, depletion, and ecological feedback.

### 4.2 ResourceStock

A `ResourceStock` is a localized or stored quantity of usable resource within a system, place, or structure.

Examples:

- standing timber in a patch
- grain in a granary
- fuel in storage
- ore in an exposed seam
- water in a reservoir
- nutrient content in soil

`ResourceStock` is the operational quantity that actors extract, move, preserve, consume, transform, or contest.

### 4.3 MaterialEnergyCarrier

A `MaterialEnergyCarrier` is a material or usable energy form represented by a canonical class and a property vector.

It may represent:

- raw materials
- processed materials
- fuels
- energy carriers
- nutrient-bearing matter
- biologically useful matter

The key design rule is:

materials are not defined primarily by item names, but by what their property vectors allow them to do.

### 4.4 EnvironmentalField

An `EnvironmentalField` is a spatially distributed condition that shapes:

- resource productivity
- material stability
- process feasibility
- transport cost
- preservation potential

Examples:

- temperature
- humidity
- soil moisture
- nutrient availability
- salinity
- solar radiation
- wind
- elevation
- biotic pressure

### 4.5 InfrastructureLayer

An `InfrastructureLayer` is a built or maintained system that changes effective access to resources, movement, storage, energy conversion, or process feasibility.

Examples:

- roads
- canals
- reservoirs
- storage facilities
- kilns
- furnaces
- ports
- mills
- power transmission systems

Institutions are not part of the material ontology itself, but they constrain:

- who may access a resource system
- who may withdraw from a stock
- who may run a process template
- who controls infrastructure

## 5. Category Boundaries

The boundaries between these layers must remain explicit.

### 5.1 ResourceSystem vs ResourceStock

- `ResourceSystem` is the larger regenerative or structural context
- `ResourceStock` is the local or stored quantity within that context

Examples:

- forest versus harvestable timber
- river basin versus available irrigation water
- ore body versus extractable high-grade ore in a shaft

### 5.2 ResourceStock vs MaterialEnergyCarrier

- `ResourceStock` answers how much and where
- `MaterialEnergyCarrier` answers what kind of matter or energy this is and what it can do

### 5.3 EnvironmentalField vs Resource

Environmental fields are not identical to resource quantities.

For example:

- temperature is a field
- water volume is a stock
- nutrient concentration is often a field or stock depending on representation

### 5.4 Infrastructure vs Institution

- `InfrastructureLayer` changes material and logistical affordances physically
- institutions change permissions, obligations, monitoring, and rights socially

Both matter, but they are not the same ontological object.

## 6. Canonical Resource And Material Categories

POLIS should begin with a compact but extensible catalog of material-energy classes.

### 6.1 Raw material classes

Suggested initial classes:

- hard lithics
- soft lithics
- clay-rich earth
- sand and gravel
- low-grade ore
- high-grade ore
- native metal
- light biomass
- dense biomass
- fibrous biomass
- fats and oils
- fresh water
- brine or saline water
- atmospheric gases
- fertile soil matrix
- nutrient-rich sediment

### 6.2 Processed material classes

Suggested initial classes:

- worked stone
- fired ceramic
- timber structural stock
- structural composite earth
- charcoal
- preserved food
- textile fiber
- paper-like or record medium
- simple metal
- metal alloy
- refined fuel
- lime, ash, or simple reagent

### 6.3 Energy carriers and fuels

Suggested initial energy classes:

- human labor
- animal labor
- direct biomass fuel
- charcoal-like dense fuel
- flowing water power
- wind power
- peat-like fuel
- coal-like fuel
- oil-like fuel
- gas-like fuel
- pressurized steam
- electrical power

These should remain generic enough to span eras without overcommitting to modern industrial categories.

### 6.4 Biological resource classes

Biological resources should be representable as recurring sources of:

- biomass
- labor
- traction
- fertilizer
- fiber
- protein
- disease pressure
- pollination

Detailed biological dynamics belong later, but the material ontology must support these use modes now.

## 7. Material Property Model

Every material-energy class should be represented by a compact property vector.

The vector should be coarse-grained enough to scale and rich enough to drive discovery and transformation.

### 7.1 Mechanical properties

- compressive strength
- tensile strength
- stiffness
- toughness
- density
- brittleness
- flexibility

### 7.2 Thermal properties

- ignition ease
- burn rate
- heat output class
- heat retention
- heat capacity
- thermal conductivity
- softening or melting range
- insulation value

### 7.3 Chemical properties

- reactivity
- reducibility
- corrosion or rot tendency
- moisture sensitivity
- spoilage tendency
- toxicity
- smoke or by-product profile

### 7.4 Energetic properties

- usable energy density by mass
- usable energy density by volume
- conversion loss class
- storage loss class

POLIS may use a generalized usable-energy or exergy-like metric for comparing resource quality across fuels and process chains, but this should remain a modelling convenience rather than a full thermodynamic commitment in v1.

### 7.5 Biological properties

- caloric value
- digestibility
- nutrient profile class
- palatability
- pathogen susceptibility
- fertility contribution

### 7.6 Handling and fabrication properties

- granularity
- ductility
- plasticity
- workability
- porosity
- permeability
- modularity

### 7.7 Durability properties

- weather resistance
- shelf stability
- pest resistance
- structural longevity
- environmental half-life class

### 7.8 Property resolution

Property values should usually be stored as bins or coarse ordinal ranges, for example:

- very low
- low
- medium
- high
- very high

This preserves relative ordering and supports threshold logic without implying laboratory precision the model does not truly possess.

## 8. Environmental Fields And Constraints

Environmental fields constrain what resources exist, what materials persist, and which process templates are feasible.

### 8.1 Core field families

POLIS should start with at least these field families:

- temperature
- humidity
- precipitation and water availability
- soil moisture
- soil nutrient status
- solar radiation
- wind exposure
- salinity
- elevation and slope
- biotic pressure

### 8.2 Resource productivity coupling

Resource-system productivity should be driven by local field state.

Examples:

- forest growth depends on moisture, temperature, and soil quality
- crop yield depends on water, nutrients, and seasonality
- fisheries depend on local ecological conditions
- soil fertility depends on nutrient state, organic return, and erosion

### 8.3 Process feasibility coupling

Process templates must check environmental constraints as part of feasibility.

Examples:

- drying depends on humidity and airflow
- freezing depends on available cold conditions
- irrigation depends on water source, elevation, and soil infiltration
- combustion quality depends on fuel moisture and oxygen availability
- metallurgy depends on achievable temperatures and local fuel conditions

### 8.4 Shock coupling

Field changes may produce system shocks such as:

- drought
- cold snap
- flood
- nutrient collapse
- pest outbreak

These shocks should propagate through resource systems and process viability rather than appearing as disconnected scripted events.

## 9. Affordance Model

POLIS should define affordances relationally.

A material or stock affords an action only when:

- its property vector is compatible
- the relevant environmental conditions are present
- the required infrastructure exists
- the actor has sufficient capability
- institutional constraints do not forbid or heavily penalize the action

### 9.1 Affordance principle

Affordances are not attached to materials as static labels.

They arise from the relation between:

- material properties
- environmental fields
- process knowledge
- actor capability
- infrastructure access

### 9.2 Latent affordances

Many affordances should remain latent until discovered or institutionalized.

Examples:

- that clay can become durable ceramic under sufficient heat
- that cold can preserve food
- that charcoal enables higher-temperature processes than raw wood
- that some animals can convert low-quality biomass into labor and transport

This is essential for later discovery documents.

## 10. Process Templates

The main mechanism connecting resources, materials, and technological development is the `ProcessTemplate`.

### 10.1 ProcessTemplate definition

A `ProcessTemplate` is a generic transformation schema that specifies:

- required inputs
- allowed input classes
- property thresholds
- environmental constraints
- infrastructure requirements
- institutional access constraints
- expected outputs
- losses and side effects

### 10.2 Template structure

Each template should define:

- input resource stocks
- input material-energy classes
- actor capability requirements
- required environmental ranges
- required structures or infrastructure
- transformation rules
- waste outputs
- usable-energy gain or loss class
- side effects on fields or resource systems

### 10.3 Process families

Suggested initial families:

- extraction
- harvesting
- cutting and shaping
- grinding
- drying
- preserving
- storing
- transporting
- combusting
- refining
- smelting
- building
- irrigating
- breeding
- cooling
- pumping
- signalling

### 10.4 Example: combustion

Inputs:

- combustible fuel class
- oxygen availability
- ignition method

Checks:

- ignition threshold
- moisture penalty
- infrastructure such as hearth or kiln if needed

Outputs:

- heat
- ash
- smoke or toxic by-product class
- reduced fuel stock

Side effects:

- local air-quality impact
- fire risk
- pressure on biomass systems or fuel stocks

### 10.5 Example: preservation

Inputs:

- perishable food
- container or storage structure
- cold, drying, smoking, salting, or fermentation pathway

Checks:

- spoilage properties
- environmental suitability
- required material or structure access

Outputs:

- preserved food class with altered shelf stability
- process loss
- possible waste by-products

### 10.6 Example: smelting

Inputs:

- ore class
- fuel class
- flux or supporting material if needed
- furnace-like infrastructure

Checks:

- reducibility
- temperature feasibility
- fuel quality
- labor and skill thresholds

Outputs:

- simple metal
- slag or waste
- depleted ore stock
- heavy fuel consumption

### 10.7 Example: transport

Inputs:

- material to be moved
- carrier type
- path or route

Checks:

- weight and bulk
- fragility
- route quality
- transport infrastructure

Outputs:

- relocated stock
- time cost
- transport loss or breakage risk

## 11. Stocks, Flows, And Transformation Chains

POLIS should treat many social and technological systems as chains of stock-flow transformation.

### 11.1 Stock-flow principle

Resources should move through chains such as:

- extraction
- transport
- processing
- storage
- use
- waste or recycling

### 11.2 Transformation principle

Transformation should usually:

- conserve mass approximately at the class level where relevant
- degrade useful energy or usable quality over time
- create side products or waste
- alter future affordances

### 11.3 Waste and degradation

Not all outputs should remain equally useful.

Examples:

- heat dissipates
- ore becomes slag
- food rots
- timber decays
- soil loses nutrients

This degradation is central to realism and to later institutional/resource-crisis dynamics.

## 12. Key Domain Requirements

This section identifies the minimum material logic needed to support core POLIS themes.

### 12.1 Fire and heat use

Requires:

- fuel classes with moisture and energy distinctions
- ignition feasibility
- heat transfer properties
- structure fire risk
- smoke and by-product effects

### 12.2 Preservation and refrigeration

Requires:

- spoilage functions
- insulation and permeability distinctions
- cold-source or evaporative-cooling logic
- seasonal or spatial access to cold gradients

### 12.3 Storage

Requires:

- container-like structures
- pest resistance
- durability
- spoilage modification
- theft and accessibility considerations

### 12.4 Farming

Requires:

- soil moisture and nutrient logic
- plant biomass and yield classes
- seasonal productivity
- water-control capability

### 12.5 Domestication and animal use

Requires:

- biological resources as recurring material and labor systems
- feed conversion logic
- traction and transport affordances
- fertilizer return pathways

### 12.6 Construction

Requires:

- competing structural materials
- strength, durability, and workability differences
- local availability constraints

### 12.7 Metallurgy

Requires:

- ore grades
- fuel-temperature limits
- reducibility classes
- refining and waste outputs

### 12.8 Fossil and concentrated fuel transitions

Requires:

- concentrated fuel deposits
- extraction difficulty
- superior energy-density classes
- transport and storage feasibility
- pollution and landscape side effects

## 13. Spatial Structure And Path Dependence

Resource and material systems must remain spatial.

### 13.1 Spatial heterogeneity

Different regions should differ in:

- productivity
- fuel quality
- ore availability
- water access
- soil fertility
- transport difficulty

### 13.2 Trade and specialization

Trade, specialization, and conflict should emerge partly from:

- unequal material distributions
- environmental gradients
- infrastructure bottlenecks
- differing local process feasibility

### 13.3 Path dependence

Early access to particular materials or energy carriers should influence:

- settlement patterns
- institutional pressures
- military advantages
- discovery pathways
- long-run development trajectories

## 14. Governance Constraints

Governance is not the primary topic of this document, but resource logic must remain compatible with it.

### 14.1 Resource access constraints

Resource systems and key stocks should be able to reference:

- ownership mode
- access restrictions
- monitoring intensity
- extraction rights
- sanction risk

### 14.2 Infrastructure control

Control over infrastructure should matter because it changes:

- transport capacity
- storage security
- irrigation access
- energy conversion capacity

### 14.3 Constraint rule

No process template should be assumed universally available merely because the material and field conditions exist.

Rights, control, and governance may still block or shape use.

## 15. Scalability And Extensibility Constraints

POLIS must stay extensible without becoming an ontology landfill.

### 15.1 Initial scope rule

The initial number of canonical material-energy classes and environmental fields should remain deliberately limited.

### 15.2 Extension rule

New material classes may be added only if they define:

- a distinct property profile
- distinct process compatibility
- distinct simulation relevance

### 15.3 Anti-item-sprawl rule

No ad hoc named item should be added unless it either:

- maps cleanly onto an existing property-based class
- or justifies a new class by enabling genuinely different affordances

## 16. Risks And Anti-Patterns

The following design failures must be avoided.

### 16.1 Toy resource counters

Reducing the world to generic counters such as `food`, `wood`, `stone`, and `money` with no important property differences.

### 16.2 Item explosion

Enumerating huge material lists with no disciplined ontology.

### 16.3 Hard-coded tech trees

Treating technologies as scripted unlocks disconnected from material environment and process feasibility.

### 16.4 Governance blindness

Ignoring rights, exclusion, monitoring, and control over resources.

### 16.5 Spatial flattening

Assuming the same resource and process conditions everywhere.

### 16.6 Modern-category leakage

Projecting modern industrial sector categories too rigidly into early societies instead of using more general physical and ecological classes.

## 17. Open Questions

These questions remain for later documents:

- How many nutrient dimensions are worth tracking in v1 for soil and food systems?
- Should usable-energy quality be represented as a single scalar, several coarse dimensions, or only domain-specific properties?
- Which biological materials should be first-class material-energy carriers versus represented through organism systems alone?
- At what point should waste streams become reusable resource stocks?

These do not block the ontology, but they matter for later refinement.

## 18. Summary

POLIS should represent resources and materials through a mesoscopic system built from:

- resource systems
- resource stocks and flows
- material and energy carriers with compact property vectors
- environmental fields
- infrastructure-mediated access and transformation
- generic process templates

This gives the simulator enough physical and ecological depth to support emergent fire use, farming, storage, refrigeration, metallurgy, fuel transitions, trade, conflict, and discovery without collapsing into either a toy inventory model or an unmanageable chemistry simulation.
