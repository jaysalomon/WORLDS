# POLIS Discovery Heuristics Spec

**Version:** 0.1  
**Date:** 14 March 2026  
**Project:** POLIS  
**Purpose:** Define how the simulation discovers useful processes, precursor systems, and generic structures without relying on a brittle hand-authored tech tree.

**Status:** Legacy discovery draft. The authoritative design baseline is [05_DiscoveryHeuristics.md](/abs/path/e:/Drive/WORLDS/05_DiscoveryHeuristics.md) within the numbered suite indexed by [SpecSuite.md](/abs/path/e:/Drive/WORLDS/SpecSuite.md). This file is retained as source history.

## 1. Goal

POLIS should not rely on a fixed scripted sequence such as:

- discover fire
- then pottery
- then farming
- then metallurgy

That approach does not scale and does not support serious experimentation.

Instead, the system should discover useful regularities in the world through interaction with:

- resource properties
- material processes
- biological systems
- environmental gradients
- collective coordination needs

The simulation therefore needs a **latent discovery space**.

Agents and groups do not invent from infinite possibility. They uncover combinations that are already possible within the world model.

## 2. Core Principle

The system should be able to discover practical laws of the world without requiring those laws to be narrated in advance.

This does **not** mean agents derive formal thermodynamics equations on a chalkboard unless later systems support that.

It means they can discover stable, reusable regularities such as:

- fire transforms materials
- cold preserves food
- pressure and flow can move water
- some animals can be controlled and bred
- some plants yield much more under repeated cultivation
- stored fuel can unlock heat-intensive production
- buried carbon-rich deposits can be extracted as concentrated energy sources

In backend terms, they are discovering **affordances**, **process chains**, and **control strategies**.

## 3. Discovery Model

### 3.1 Discovery is not random magic

A discovery occurs when a decision unit repeatedly encounters a useful state transition and stores it as reusable knowledge.

That transition can be:

- observed accidentally
- produced intentionally
- socially transmitted
- institutionalised

### 3.2 Knowledge units

The backend should represent discovery as generic knowledge units rather than named story beats.

Suggested unit types:

- `material_property`
- `process_template`
- `biological_control`
- `environmental_exploitation`
- `structure_pattern`
- `coordination_pattern`
- `preservation_method`
- `energy_source`

Frontend labels like "fire", "granary", "refrigeration", or "animal husbandry" are human-readable names applied to combinations of these units.

### 3.3 Discovery loop

Each discovery-capable decision unit, usually an individual, household, or group, follows a loop:

1. encounter materials, organisms, or environmental conditions
2. perform actions for immediate utility
3. observe outcome differences
4. record repeatable high-value transitions
5. compress them into reusable heuristics
6. diffuse them across kin, group, or institution
7. embed them into structures, routines, or norms

This keeps discovery grounded in utility and repetition rather than arbitrary unlock events.

## 4. Resource And Material Affordances

Resources must have properties, not just quantities.

A serious backend should define materials by functional affordances such as:

- combustibility
- caloric value
- structural strength
- brittleness
- ductility
- thermal retention
- insulation quality
- permeability
- preservation potential
- toxicity
- fertility impact
- tool compatibility
- transportability

Examples:

- wood can be fuel, structure, and tool stock
- clay can be shaped, dried, and fired into containers
- stone can provide edge retention or compressive strength
- ice and snow can serve as cold storage if climate and handling permit
- peat, coal, and oil can function as concentrated fuels if extraction and control methods exist
- certain animals can convert low-value biomass into labour, meat, traction, or transport

Without this property layer, discovery remains shallow.

## 5. Process Templates

The engine should define a limited set of generic process templates that can operate over many materials.

These templates are more scalable than named inventions.

Suggested template families:

- heating
- cooling
- drying
- grinding
- cutting
- binding
- fermenting
- preserving
- storing
- transporting
- breeding
- irrigating
- enclosing
- refining
- smelting
- pumping
- insulating
- signalling

Each template should define:

- input property requirements
- environmental requirements
- labour requirements
- skill requirements
- tool requirements
- expected output transformations
- failure modes
- observability of results

### 5.1 Example: refrigeration

The backend should not require a hardcoded "refrigeration" technology.

Instead, refrigeration-like behaviour emerges if agents discover:

- cold environments preserve perishables
- insulation slows warming
- enclosed storage stabilises temperature
- transported ice or snow can preserve food locally
- evaporative cooling works in suitable climates
- later, mechanical cooling can emerge from pressure, phase change, and power systems

This is a chain of process templates:

- `cooling`
- `insulating`
- `storing`
- `transporting`
- later `mechanical_work`

### 5.2 Example: fossil fuels

Likewise, "fossil fuel use" should not be a single tech unlock.

It should emerge from:

- discovery of combustible underground material
- extraction methods
- storage and transport methods
- controlled combustion
- heat-transfer infrastructure
- later pressure and engine systems

This is a chain of:

- `energy_source`
- `extracting`
- `transporting`
- `heating`
- `mechanical_conversion`

## 6. Biological Precursors

Animals, farming, and domestication should be precursor systems, not isolated feature packs.

### 6.1 Wild organisms as opportunity space

Plants and animals should expose behavioural and ecological traits such as:

- growth rate
- seasonality
- reproduction rate
- temperament
- groupability
- tractability
- diet flexibility
- migration tendency
- disease burden
- labour potential
- preservation value
- breeding variance

These traits create discovery opportunities.

### 6.2 Farming

Farming should emerge when repeated interventions show stable gains in yield, reliability, or control.

Precursor observations include:

- some plants regrow near waste sites
- disturbed soil changes growth
- water control changes output
- seed selection changes future yield
- storage supports delayed planting

Farming is therefore a compound system built from:

- observation of growth cycles
- delayed-return planning
- land tenure or control
- storage
- labour coordination
- seasonal prediction

### 6.3 Domestication

Domestication should emerge when the expected utility of managed biological control exceeds hunting or foraging alone.

Preconditions may include:

- repeat contact with target species
- enclosure capability
- surplus feed or grazing access
- low enough aggression or flight response
- breeding continuity
- social transmission of handling techniques

Domestication can then unlock:

- traction
- pack transport
- manure
- milk or secondary outputs
- selective breeding
- mounted warfare
- pastoral mobility

### 6.4 Animal systems as precursor ladders

Animals are not just food units.

They can become precursors for:

- agriculture
- transport
- warfare
- trade range expansion
- refrigeration support through transport of perishables and ice
- fertilisation of crop systems
- textile and leather production

This gives biological systems real backend importance.

## 7. Generic Structures

The simulation should use structure archetypes rather than huge lists of hand-authored buildings.

### 7.1 Core archetypes

Suggested structure families:

- shelter
- storage
- production
- defence
- transport
- governance
- ritual
- knowledge
- water control
- thermal control
- animal control

### 7.2 Structure function model

Each structure instance should be defined by:

- footprint
- build cost
- maintenance cost
- environmental constraints
- capacity
- durability
- labour demand
- process templates enabled
- protection or efficiency modifiers

Examples:

- a granary is `storage` with preservation modifiers
- a kiln is `production` plus `heating`
- a stable is `animal_control`
- an irrigation ditch is `water_control`
- an ice house is `thermal_control` plus `storage`
- a watchtower is `defence` plus `signalling`

This lets the backend stay generic while the frontend presents recognizable forms.

## 8. Template Heuristics

Template heuristics are reusable rules for trying new combinations without brute-forcing the entire possibility space.

They should guide experimentation under bounded rationality.

### 8.1 Heuristic families

Suggested heuristic families:

- repeat what worked in similar conditions
- vary one input while holding others stable
- preserve scarce outputs
- substitute one material for another with similar properties
- copy a neighbour's high-payoff process
- test higher-intensity versions of useful processes
- combine structures and processes that have complementary outputs
- invest in control of uncertain but high-payoff resources

### 8.2 Why templates matter for scale

Without heuristics, agents must search too much.

With overly bespoke scripting, emergence becomes fake.

Template heuristics sit in the middle:

- generic enough to support novelty
- constrained enough to scale
- interpretable enough to inspect scientifically

### 8.3 Discovery scoring

A discovered template should be retained when it produces measurable benefit in one or more of:

- survival
- preservation
- labour efficiency
- resource extraction
- storage reliability
- mobility
- military advantage
- environmental resilience
- coordination efficiency

Retention should depend on repeated success, not one lucky event.

## 9. Knowledge Diffusion And Institutionalisation

Discovery only matters if it spreads and stabilises.

Knowledge should diffuse through:

- kin learning
- imitation
- trade contact
- apprenticeships
- leadership orders
- ritual and norm embedding
- written or stored records in later phases

Some discoveries should remain fragile unless institutions support them.

Examples:

- seed selection requires seasonal continuity
- refrigeration methods require storage and logistics discipline
- domestication requires multigenerational control
- fossil-fuel exploitation requires extraction infrastructure and organised labour

This creates a distinction between:

- accidental observation
- practical technique
- standardised craft
- institutionally maintained system

## 10. Backend Representation

For implementation, the discovery system should probably use three layers.

### 10.1 Property graph

A graph of:

- materials
- organisms
- environmental states
- structures
- process templates
- output states

Edges express possible transformations and requirements.

### 10.2 Heuristic policy layer

Decision units do not search the whole graph.

They sample from locally plausible actions using heuristic priors based on:

- known templates
- observed neighbour success
- current pressure
- intelligence and curiosity
- available labour and surplus

### 10.3 Knowledge state

Each individual, household, group, or institution can hold:

- discovered templates
- confidence scores
- environmental applicability tags
- failure memories
- diffusion status

This allows uneven development across the map.

## 11. Research Value

This system supports questions such as:

- Under what conditions do preservation systems emerge?
- When does domestication beat mobile hunting?
- How much surplus is needed before high-latency discoveries persist?
- Does cold climate drive storage and thermal-control innovation earlier?
- Do concentrated fuels accelerate hierarchy and extraction systems?
- When does coordination matter more than intelligence for complex discovery chains?

## 12. Non-Negotiable Constraints

1. Discovery logic must be generic, not a disguised fixed tech tree.
2. Specific content should be data, not buried in engine code.
3. Resource and organism properties must be rich enough to support meaningful discovery.
4. Template heuristics must reduce search space without killing novelty.
5. Named frontend inventions must correspond to backend combinations of properties, templates, and structures.
6. Discovery should support uneven regional development rather than global instant unlocks.
7. Precursor systems such as animals, farming, storage, and thermal control must matter before advanced industry appears.

## 13. One-Sentence Summary

POLIS should model discovery as the gradual capture, reuse, and diffusion of useful world regularities, allowing societies to uncover fire, farming, domestication, refrigeration, concentrated fuels, and later industry through generic heuristics operating over rich materials, organisms, environments, and collective structures.
