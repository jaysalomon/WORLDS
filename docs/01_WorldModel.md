# POLIS World Model And Ontology

**Version:** 0.1  
**Date:** 14 March 2026  
**Document:** 01 of the POLIS spec suite  
**Purpose:** Define what kinds of things exist in the POLIS simulation, how they differ, and where causality is allowed to live.

## 1. Scope

This document defines the core ontology of POLIS.

It answers:

- what exists in the simulation
- which entities are actors
- which entities are rule systems
- which entities are physical or biological substrate
- which entities are processes
- which entities are only aggregates or observed patterns
- how these categories relate across scale

This document does **not** define:

- detailed state schemas
- update order
- numerical methods
- detailed equations
- frontend presentation rules
- implementation architecture

Those belong in later documents.

## 2. Dependencies

This is the first numbered specification document in the POLIS suite.

It defines vocabulary for later documents, especially:

- `02_StateModel.md`
- `03_CollectiveAgency.md`
- `04_ResourcesAndMaterials.md`
- `05_DiscoveryHeuristics.md`
- `06_BiologyAndDomestication.md`
- `07_SocietyAndInstitutions.md`

## 3. Guiding Principle

POLIS should use a small, orthogonal ontology.

The model should separate:

- who acts
- what constrains action
- what exists physically and biologically
- what changes over time
- what is merely observed as a pattern

If those categories blur, the simulation will become incoherent.

## 4. Meta-Categories

POLIS uses five ontology meta-categories.

### 4.1 Actor

An `Actor` is an entity with an internal decision process that selects among actions based on perceived state, constraints, and incentives.

Actors may be:

- individual
- collective
- conditional, depending on modelling resolution

### 4.2 RuleSystem

A `RuleSystem` is a durable set of norms, procedures, permissions, obligations, or sanctions that shapes what actors may do in given contexts.

Rule systems constrain actors but are not actors themselves.

### 4.3 Substrate

A `Substrate` entity is a physical, biological, spatial, or informational entity that exists in the world and can be transformed, occupied, consumed, controlled, or modified.

Substrate entities have state but no endogenous decision process.

### 4.4 Process

A `Process` is a time-bounded or time-structured transformation involving actors, substrate, and rule systems.

Processes are where state changes occur.

### 4.5 Aggregate

An `Aggregate` is a derived pattern, indicator, grouping, or summary computed from other entities.

Aggregates do not own primary world state and do not act unless they are explicitly reified into another valid entity class.

## 5. Canonical Entity Classes

POLIS should use the following canonical entity classes.

### 5.1 Actors

- `Individual`
- `CollectiveActor`

### 5.2 Conditional actor forms

- `Household`
- `Organism`

These may act in some models or be treated as substrate or aggregate depending on resolution and purpose.

### 5.3 Rule systems

- `Institution`
- `Role`

### 5.4 Substrate entities

- `Place`
- `Settlement`
- `ResourceSystem`
- `ResourceStock`
- `Structure`
- `EnvironmentalField`

### 5.5 Process entities

- `ActionContext`
- `Event`

### 5.6 Aggregate entities

- `Aggregate`

This class may later include named derived forms such as:

- population
- community
- class
- culture cluster
- economy indicator
- regime indicator

These remain non-actor summaries unless explicitly promoted into another class.

## 6. Actor Classification

The simulation must be explicit about which entity types can choose actions.

| Entity class | Classification | Notes |
| --- | --- | --- |
| `Individual` | actor | Primary unit of human decision-making |
| `CollectiveActor` | actor | Group-level decision unit when coherence criteria are met |
| `Household` | conditional actor | May be modelled as a collective actor or as a derived grouping |
| `Organism` | conditional actor | Non-human organisms may be agents when individual behaviour matters |
| `Institution` | non-actor | Constrains action but does not choose |
| `Role` | non-actor | Defines position and authority, not agency |
| `Place` | non-actor | Spatial substrate |
| `Settlement` | non-actor | Socially occupied place, but not inherently a chooser |
| `ResourceSystem` | non-actor | Dynamic biophysical system, not an intentional agent |
| `ResourceStock` | non-actor | Stored or localised resource quantity |
| `Structure` | non-actor | Physical artifact with affordances |
| `EnvironmentalField` | non-actor | Spatial field of conditions |
| `ActionContext` | non-actor | Decision context definition |
| `Event` | non-actor | Process instance that records and applies change |
| `Aggregate` | non-actor | Derived summary only |

## 7. Entity Definitions

This section defines what each class is, what it is not, and why it exists.

### 7.1 Individual

An `Individual` is a human actor with:

- bodily state
- cognitive state
- motivational state
- social ties
- local knowledge
- action heuristics

An `Individual` is:

- the default human decision unit
- a member of one or more social entities
- the main carrier of local behaviour and variation

An `Individual` is not:

- a household
- a settlement
- an institution
- a role

### 7.2 CollectiveActor

A `CollectiveActor` is a coordinated group that can make decisions as a unit.

It exists when a group has:

- identifiable membership
- internal structure or coordination
- a decision procedure, explicit or implicit
- collective control over actions or assets

Examples may include:

- a household acting as one unit
- a council
- a militia
- a trading caravan
- a guild
- a polity

A `CollectiveActor` is not:

- a loose crowd
- a statistical grouping
- an institution

### 7.3 Household

A `Household` is a bounded domestic or kin-based unit that shares some combination of:

- resources
- residence
- labour
- reproduction context
- routine decision-making

In POLIS, `Household` is a special case rather than a fully separate ontology family.

It may be represented in one of two valid ways:

- as a subtype or configuration of `CollectiveActor`
- as a named aggregate of `Individual` members when no separate agency is modelled

The simulation must not treat the same household as both a fully independent actor and a purely derived grouping at the same resolution.

### 7.4 Organism

An `Organism` is a non-human living entity.

It exists to support:

- ecology
- predation
- herd behaviour
- domestication
- farming and husbandry
- disease ecology

An `Organism` may be:

- actor-like when individual behaviour matters
- substrate-like when represented as stock or population

The ontology therefore allows `Organism` to be conditional.

### 7.5 Institution

An `Institution` is a rule system.

It specifies some combination of:

- permissions
- obligations
- prohibitions
- sanctions
- procedures
- jurisdictions
- office definitions

An `Institution` constrains or enables action.

An `Institution` is not:

- a building
- an officeholder
- a council as a social body
- a government actor

Those may implement or embody institutions, but they are not identical to them.

### 7.6 Role

A `Role` is a position within an institution or collective structure.

It defines:

- expected duties
- powers
- authority scope
- participation rights in specific action contexts

A `Role` is not an entity that decides on its own.

The occupant of a role must be an actor.

### 7.7 Place

A `Place` is a spatial region, site, patch, tile, zone, or location in the world.

`Place` exists to support:

- location
- containment
- adjacency
- terrain
- environmental variation
- jurisdictional scope

A `Place` is pure spatial substrate.

It is not automatically inhabited, built, or socially organised.

### 7.8 Settlement

A `Settlement` is a socially occupied and materially developed place.

It is defined by:

- resident population
- concentration of structures
- local resource organisation
- social occupancy
- possible local governance

A `Settlement` is related to `Place`, but not identical to it.

Useful distinction:

- `Place` is where something is
- `Settlement` is a socially organised occupation of place

A `Settlement` is not inherently an actor. Its councils, factions, and households may be actors.

### 7.9 ResourceSystem

A `ResourceSystem` is a larger-scale environmental or managed system that generates, stores, routes, or constrains resource availability.

Examples:

- a forest
- a pasture
- a river basin
- an agricultural zone
- a fishery
- a mineral region

`ResourceSystem` represents structure and renewal context, not a small local quantity.

### 7.10 ResourceStock

A `ResourceStock` is a localised or stored quantity of usable resource.

Examples:

- grain in a granary
- fish biomass in a local patch
- fuel in storage
- ore in a seam
- water in a reservoir

`ResourceStock` is the quantity that actors consume, move, preserve, or fight over.

`ResourceStock` is usually part of a broader `ResourceSystem`.

### 7.11 Structure

A `Structure` is a durable human-made artifact or built modification of the world.

Examples:

- dwelling
- wall
- road
- granary
- kiln
- stable
- canal
- dock

A `Structure` exists because it changes affordances.

It may:

- protect
- store
- route
- process
- signal
- constrain movement
- enable specialised production

A `Structure` is not an actor, even when it strongly affects outcomes.

### 7.12 EnvironmentalField

An `EnvironmentalField` is a spatially distributed condition.

Examples:

- temperature
- rainfall
- fertility gradient
- disease pressure
- pollution
- information intensity
- social tension field

Fields may be discrete or continuous in implementation, but ontologically they are distributed conditions over space and time.

### 7.13 ActionContext

An `ActionContext` is a structured situation in which actors choose among actions under specific constraints.

It defines:

- participating actor types
- relevant roles
- available actions
- applicable institutions
- information conditions
- possible outcomes

Examples:

- trade exchange
- dispute resolution
- harvest decision
- military command
- mating or household formation

`ActionContext` is not a persistent actor or place. It is a process template.

### 7.14 Event

An `Event` is a concrete instance of state change.

Examples:

- a raid
- a harvest
- a migration step
- a birth
- a treaty
- a storage transfer
- a law change

An `Event` instantiates one or more process rules and records what changed.

`Event` is the main ontological unit of discrete historical change.

### 7.15 Aggregate

An `Aggregate` is a derived summary or grouping.

Examples:

- local population
- social class distribution
- average cohesion
- faction count
- trade volume
- inequality index
- collapse risk

Aggregates are analytically important but must not be mistaken for primary causal entities.

## 8. Core Relations

The ontology should support a compact set of canonical relations.

### 8.1 Membership and composition

- `member_of`
- `has_member`
- `part_of`
- `contains`

### 8.2 Spatial relations

- `located_in`
- `adjacent_to`
- `within_range_of`

### 8.3 Control and governance

- `controls`
- `owns`
- `occupies`
- `implements`
- `constrains`
- `applies_to`

### 8.4 Resource and process relations

- `produces`
- `consumes`
- `extracts_from`
- `stores`
- `transforms`
- `enables`

### 8.5 Social and institutional relations

- `allied_with`
- `hostile_to`
- `trades_with`
- `subject_to`
- `holds_role`

### 8.6 Derivation relations

- `derived_from`
- `summarises`

Later documents may extend these, but should prefer reusing them before inventing new relation types.

## 9. Scale And Level Rules

POLIS is a multi-scale simulation, but scale changes must remain ontologically consistent.

### 9.1 Micro level

The micro level is centered on:

- `Individual`
- actor-like `Organism`
- local `ResourceStock`
- nearby `Structure`
- local `Place`

### 9.2 Meso level

The meso level is centered on:

- `CollectiveActor`
- `Household`
- `Settlement`
- local `Institution`
- regional `ResourceSystem`

### 9.3 Macro level

The macro level is centered on:

- large `CollectiveActor`
- nested `Institution`
- broad `EnvironmentalField`
- large-scale `Aggregate`

### 9.4 Valid scale transitions

Scale transitions are allowed when:

- a collection of lower-level actors becomes a valid `CollectiveActor`
- a fine-grained substrate is summarized into a higher-level form
- an aggregate is used analytically without being confused for an actor

Scale transitions are not allowed to violate ownership or duplicate authority.

## 10. Rules For Group Agentification

Not every group should be promoted into a `CollectiveActor`.

A grouping becomes a valid collective actor only when it has:

1. stable membership or bounded membership rules
2. coordinated behaviour
3. some internal decision procedure
4. control over assets, labour, or force beyond isolated individuals
5. externally meaningful action capacity

If these conditions are not met, the group should remain:

- an aggregate
- a population label
- a network cluster
- or a settlement component

This distinction is critical for later documents on collective agency and institutions.

## 11. Ownership Boundaries

This document does not define the full state schema, but it does define ownership boundaries.

### 11.1 Actors own

- intentions
- beliefs
- preferences
- skills
- commitments
- directly controlled portable resources

### 11.2 Rule systems own

- permissions
- obligations
- sanctions
- office definitions
- jurisdiction

### 11.3 Substrate owns

- physical quantities
- locations
- environmental conditions
- built capacities
- ecological properties

### 11.4 Processes own

- participation records
- contextual parameters
- state transition records

### 11.5 Aggregates own

- no primary causal state
- only derived summaries or cached indicators

## 12. Design Constraints

The ontology must obey the following constraints.

1. Every entity class must have a distinct modelling purpose.
2. Every piece of primary state must have one authoritative owner.
3. Institutions must remain rule systems, not vague stand-ins for organizations.
4. Aggregates must remain analytic summaries unless explicitly reified.
5. Group agency must be justified, not assumed.
6. Place and settlement must remain distinct.
7. Resource system and resource stock must remain distinct.
8. Frontend labels must never override backend ontology.
9. No later document may introduce a new top-level entity class without stating why the existing ontology is insufficient.

## 13. Open Questions

These questions remain for later documents:

- When should non-human organisms be fully agentic versus stock-like?
- Should household always be treated as a subtype of collective actor in v1?
- Should relation networks remain implicit in state ownership, or become a named substrate form later?
- How should polities be represented: always as collective actors, or sometimes as settlement-plus-institution bundles?

These are not blockers for the ontology, but they must be resolved before detailed state design is finalised.

## 14. Summary

POLIS should model the world using a compact ontology built around:

- actors that choose
- rule systems that constrain
- substrate entities that exist and can be transformed
- processes that produce change
- aggregates that summarise outcomes

This gives the project a defensible foundation for later documents on state, collective agency, resources, discovery, biology, institutions, validation, and implementation.
