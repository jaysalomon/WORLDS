# POLIS Biology, Farming, And Domestication

**Version:** 0.1  
**Date:** 14 March 2026  
**Document:** 06 of the POLIS spec suite  
**Purpose:** Define the biological ontology and core dynamics through which wild ecologies, crops, livestock, soils, disease, and managed niches shape food production, labor, transport, settlement, inequality, and institutional development in POLIS.

## 1. Scope

This document defines:

- the core biological categories used by POLIS
- how wild, managed, and domesticated systems differ
- the key organism, soil, and disease traits that matter
- how farming and domestication emerge and persist
- how biological systems couple to labor, transport, storage, conflict, and institutions

This document does **not** define:

- the full general resource ontology
- the full discovery/knowledge model
- detailed institutional design
- implementation-level time stepping

## 2. Dependencies

This document depends on:

- [01_WorldModel.md](/abs/path/e:/Drive/WORLDS/01_WorldModel.md)
- [02_StateModel.md](/abs/path/e:/Drive/WORLDS/02_StateModel.md)
- [04_ResourcesAndMaterials.md](/abs/path/e:/Drive/WORLDS/04_ResourcesAndMaterials.md)
- [05_DiscoveryHeuristics.md](/abs/path/e:/Drive/WORLDS/05_DiscoveryHeuristics.md)

It provides foundations for:

- `07_SocietyAndInstitutions.md`
- `08_ValidationAndExperiments.md`

## 3. Guiding Principle

POLIS should treat biology as a dynamic social-ecological layer, not as a static food source.

Biological systems should matter because they shape:

- food and surplus
- labor and traction
- disease burden
- fertilizer and soil recovery
- mobility and transport
- settlement density
- taxation legibility
- wealth inheritance
- long-run path dependence

The goal is not to simulate all biology. The goal is to simulate the biologically meaningful constraints that change civilization trajectories.

## 4. Core Biological Ontology

POLIS should represent biological systems through a layered ontology that stays compatible with the existing world, state, and resource documents.

### 4.1 WildEcologicalCommunity

A `WildEcologicalCommunity` is an unmanaged assemblage of organisms within a biome or ecological patch.

It provides:

- foraged goods
- hunting opportunities
- pollination and ecosystem services
- disease reservoirs
- baseline biodiversity

It is not simply a stockpile. It is a living ecological configuration with its own regeneration and competitive structure.

### 4.2 ManagedNicheSystem

A `ManagedNicheSystem` is an environment altered by human activity to favor certain species or biological outcomes without full domestication.

Examples:

- burned landscape favoring game
- seed-rich camp margins
- protected grove
- selectively weeded stand
- lightly managed wetland

This category is essential because many transitions to farming or husbandry begin here rather than in full agriculture.

### 4.3 Cultivar

A `Cultivar` is a plant lineage or managed plant template shaped by recurrent human selection.

It defines a trait profile for:

- yield
- harvestability
- seasonality
- stress tolerance
- nutrient responsiveness

It is not a specific plant instance. It is the inherited pattern behind plant instances and populations.

### 4.4 Breed

A `Breed` is a managed animal lineage shaped by recurrent human selection.

It defines a trait profile for:

- tameness
- reproduction
- feed conversion
- product yield
- traction suitability
- disease sensitivity

It is not a single animal. It is the inherited pattern behind a domesticated or semi-domesticated population.

### 4.5 OrganismPopulation

An `OrganismPopulation` is a local population or managed biological group represented above the single-organism level.

This is useful for:

- crops in a field
- herd-level livestock
- pest populations
- forest stands

It supports scalable ecological dynamics when simulating each individual organism would be unnecessary.

### 4.6 Agroecosystem

An `Agroecosystem` is a managed social-ecological unit in which human labor, knowledge, structures, soils, plants, animals, and water control are coupled.

Examples:

- rainfed grain field
- irrigated terrace system
- mixed crop-livestock village landscape
- orchard-garden complex
- grazing and fodder system

An agroecosystem is not merely “a farm.” It is a managed production regime with ecological costs and institutional implications.

### 4.7 SoilMatrix

A `SoilMatrix` is the biologically active and chemically meaningful substrate supporting plant growth and water retention.

It must matter because it affects:

- productivity
- sustainability
- fertility recovery
- erosion risk
- carrying capacity

### 4.8 PathogenPressure

`PathogenPressure` is the disease and pest burden arising from the interaction of host density, reservoirs, proximity, and environmental conditions.

It should be represented at the level needed for:

- crop loss
- livestock disease
- zoonotic spillover
- human labor reduction
- urban mortality pressure

## 5. Category Boundaries

These boundaries must remain explicit.

### 5.1 Wild vs managed vs domesticated

- `WildEcologicalCommunity` is unmanaged or only lightly disturbed
- `ManagedNicheSystem` is intentionally biased by human action without full domestication
- domesticated systems involve stable selection for human-dependent or human-favored traits

### 5.2 Cultivar/Breed vs individual organism

- `Cultivar` and `Breed` are inherited templates
- actual plants and animals are instances or populations expressing those templates

### 5.3 Agroecosystem vs resource system

An agroecosystem is a managed productive arrangement built on top of resource systems, environmental fields, structures, and institutions.

It is not identical to any one field, patch, or crop.

### 5.4 Soil vs simple fertility score

`SoilMatrix` must be more than a decorative fertility number.

Even if simplified, it should capture:

- nutrient condition
- carbon or organic matter condition
- water-holding behavior
- degradation or recovery trajectory

## 6. Core Traits And State Variables

POLIS should focus on the biologically meaningful minimum, not total realism.

### 6.1 Plant and cultivar traits

Important plant-level traits include:

- edible yield fraction
- harvestability
- seed retention or dispersal behavior
- seasonality and maturity timing
- nutrient efficiency
- water stress tolerance
- heat or cold tolerance
- storage durability
- labor intensity of cultivation and harvest

These traits matter because they affect not just food output but also storage, taxation, and labor scheduling.

### 6.2 Animal and breed traits

Important animal-level traits include:

- tameness
- aggression or flight tendency
- feed conversion
- reproductive cycle
- growth rate
- disease sensitivity
- milk, fiber, meat, or traction yield
- carrying or pulling capacity
- manure production

These traits determine whether animals function only as meat, or become platforms for transport, labor, wealth storage, and secondary products.

### 6.3 Soil and nutrient variables

At minimum, POLIS should represent:

- soil organic matter or carbon condition
- nitrogen availability
- phosphorus availability
- water availability or retention condition
- erosion or depletion status

This is enough to support:

- fertility differences
- fallowing logic
- manuring benefits
- long-run degradation

### 6.4 Disease and pest variables

Important disease-related variables include:

- host density
- host breadth
- transmission opportunity
- virulence pressure
- latency or persistence class
- reservoir coupling

The system does not need microbiological realism, but it does need disease to act as a real structural constraint.

### 6.5 Human life-history relevant variables

Biology must feed into human demographic and labor outcomes through at least:

- caloric adequacy
- nutrient adequacy
- disease burden
- fertility pressure
- child dependency load
- labor capacity

This allows farming and settlement to raise population even when average health declines.

## 7. Farming Dynamics

Farming should not appear as a magical upgrade.

It should emerge when managed biological systems produce enough expected advantage to outweigh risk and labor costs.

### 7.1 Preconditions for farming

Farming is more likely when some combination of the following holds:

- useful plant species are available
- environmental conditions support repeated cultivation
- storage is possible
- land access is stable enough
- labor coordination exists
- population pressure or risk pressure makes low-level food production attractive

### 7.2 Low-level food production

POLIS should include intermediate states between pure foraging and full agriculture.

These include:

- selective burning
- favored seed concentration
- weed management
- transplanting
- opportunistic planting
- seasonal tending

This is important because many agricultural systems emerge gradually from niche construction rather than from a clean switch.

### 7.3 Agricultural stabilization

Cultivation becomes a stable farming regime when:

- yields are repeatable
- storage supports delayed returns
- labor cycles become routinized
- knowledge transmission stabilizes
- population size or settlement permanence makes retreat to pure foraging difficult

### 7.4 The demographic ratchet

Once a population becomes large, settled, and dependent on cultivated output, the option to return to pure foraging may disappear.

This should be a real path-dependent mechanism in POLIS rather than narrative flavor.

## 8. Domestication Dynamics

Domestication should be modeled as a repeated selection process, not a binary unlock.

### 8.1 Plant domestication

Plant domestication emerges when human harvesting, planting, protection, and selection repeatedly favor traits such as:

- seed retention
- synchronized ripening
- larger edible fraction
- easier harvest
- lower natural dispersal

This can begin accidentally and then intensify through intentional reinforcement.

### 8.2 Animal domestication

Animal domestication becomes plausible when:

- a target species can survive under human-managed conditions
- tameness is sufficiently selectable
- reproduction under management is feasible
- humans can protect and feed the population
- the expected utility exceeds hunting alone

### 8.3 Domestication ratchet

Once selection pushes a lineage toward human-dependent traits, mutual dependence can deepen:

- the species becomes easier to manage
- humans become more dependent on its outputs
- infrastructure and routines form around it

This creates a ratchet rather than a reversible temporary tactic.

### 8.4 Secondary products

POLIS should distinguish between primary and secondary animal products.

Primary products:

- meat
- hides
- bone

Secondary products:

- milk
- wool or fiber
- traction
- transport
- manure

This distinction matters because secondary products greatly increase the civilizational significance of animal systems.

## 9. Soil, Nutrient, And Water Logic

Long-run agriculture must be biologically constrained.

### 9.1 Soil depletion and recovery

Agricultural use should affect soils through:

- nutrient drawdown
- organic matter loss
- erosion
- compaction or disturbance

Recovery should occur through:

- fallowing
- reduced intensity
- manure return
- plant residue return
- ecological succession

### 9.2 Limiting factors

Yield should depend on limiting factors rather than on a single generic fertility value.

At minimum this means:

- nutrient limits
- water limits
- seasonal mismatch
- disease or pest pressure

### 9.3 Water control

Water availability should be altered by:

- rainfall regime
- storage
- irrigation
- infiltration
- terrain

This is essential because irrigation and water control are major thresholds in social complexity.

## 10. Disease And Density Effects

Disease should act as a structural force in settlement and state formation.

### 10.1 Zoonotic coupling

Close and repeated contact between humans and livestock should increase spillover risk.

Important drivers include:

- host density
- enclosure proximity
- hygiene conditions
- animal health
- environmental persistence

### 10.2 Settlement burden

Dense settlements should face rising biological costs:

- waste accumulation
- epidemic risk
- parasite load
- vector concentration

This creates real constraints on urban growth and strong incentives for sanitary institutions and infrastructure.

### 10.3 Labor and demographic impact

Disease should reduce:

- labor capacity
- travel reliability
- military endurance
- fertility or child survival

This keeps biology tied to broader societal dynamics.

## 11. Ecological And Social Coupling

Biology must connect directly to major social outcomes.

### 11.1 Labor and transport

Animal systems should affect:

- plowing capability
- hauling capacity
- mobility range
- trade volume
- monument or infrastructure construction

### 11.2 Storage and taxation

Different crops and biological outputs should vary in how easily they can be:

- counted
- stored
- moved
- seized
- hidden

This matters because some production systems are far easier to tax and centralize than others.

### 11.3 Wealth transmission and inequality

Biological production can become wealth when it is:

- storable
- heritable
- defensible
- countable
- reproducible

Livestock, land, and grain should therefore contribute differently to inequality than purely embodied skill.

### 11.4 Conflict and expansion

Biological systems should affect:

- territorial desirability
- colonization pressure
- frontier expansion
- conflict over water, pasture, and fertile land
- disease-mediated replacement or collapse

## 12. Biological Path Dependence

Early biological choices should produce long-run consequences.

Examples:

- grain-heavy systems may favor taxation and centralization
- tuber-heavy systems may reduce legibility and coercive extraction
- traction animals may greatly expand effective transport and plow range
- manure-dependent agriculture may favor crop-livestock coupling
- disease-rich dense settlements may require stronger sanitation and administration

These path dependencies are exactly the kind of dynamics POLIS should be able to study.

## 13. Risks And Anti-Patterns

The following design failures must be avoided.

### 13.1 Food-counter reductionism

Reducing all biology to `food` output with no distinction among crops, animals, soils, or disease.

### 13.2 Binary domestication

Treating domestication as a yes/no unlock instead of a gradual trait shift and management ratchet.

### 13.3 Soil as scenery

Using soil only as a fixed fertility modifier with no depletion or recovery.

### 13.4 Disease isolation

Modeling disease as detached from livestock, density, and ecology.

### 13.5 Rational farmer fallacy

Assuming farmers always optimize yield rather than balancing risk, habit, labor, and imitation.

### 13.6 No intermediate niche stage

Skipping managed wild systems and jumping directly from foraging to full agriculture.

## 14. Open Questions

These questions remain for later refinement:

- How much organism individuality is needed in v1 versus population-level representation?
- Which crops or animal systems deserve first-class early support because of their distinct social consequences?
- How many disease pathways are needed to make density and livestock coupling matter without overwhelming the model?
- How should legibility for taxation be represented: as an explicit crop trait, a storage-process trait, or both?

These do not block the biology ontology, but they matter for later implementation and validation.

## 15. Summary

POLIS should model biology through a coupled framework of:

- wild ecological communities
- managed niche systems
- cultivars and breeds
- organism populations
- agroecosystems
- soils
- disease pressures

This allows farming and domestication to emerge from niche construction, selective pressure, ecological limits, and social learning, while ensuring that biology remains a real driver of labor, transport, storage, inequality, settlement, and institutional development.
