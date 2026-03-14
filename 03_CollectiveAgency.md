# POLIS Collective Agency And Scale Transition

**Version:** 0.1  
**Date:** 14 March 2026  
**Document:** 03 of the POLIS spec suite  
**Purpose:** Define when coordinated groups become meaningful decision units in POLIS, what must remain individual, how collective actors stabilize and fragment, and how downward causation is represented without breaking the simulation’s causal discipline.

## 1. Scope

This document defines:

- the collective categories relevant to POLIS
- the difference between coordination, aggregation, and true collective agency
- criteria for promoting groups into collective actors
- which decisions belong at collective versus individual level
- how internal heterogeneity and dissent are preserved
- how collective actors merge, stabilize, fragment, and re-expand

This document does **not** define:

- the full institution catalog
- the full social variable set
- low-level implementation details

## 2. Dependencies

This document depends on:

- [01_WorldModel.md](/abs/path/e:/Drive/WORLDS/01_WorldModel.md)
- [02_StateModel.md](/abs/path/e:/Drive/WORLDS/02_StateModel.md)

It provides core foundations for:

- `07_SocietyAndInstitutions.md`
- `08_ValidationAndExperiments.md`
- later technical architecture and runtime optimization decisions

## 3. Guiding Principle

POLIS must distinguish sharply between:

- coordination
- stable grouping
- collective agency
- institutions
- aggregates

Not every large or synchronized group is a true actor.

Collective agency should be granted only when a group has enough internal structure, continuity, and control that treating it as a decision unit improves explanation and remains consistent with the lower-level model.

## 4. Core Collective Categories

POLIS should use the following collective categories.

### 4.1 CoordinationCluster

A `CoordinationCluster` is a short-lived alignment of individuals around a local situation or goal.

Examples:

- fleeing crowd
- hunting party
- raid group
- temporary caravan
- spontaneous protest

A coordination cluster:

- may be highly synchronized
- may matter strategically
- does not yet imply durable group memory or collective identity

It is not automatically a persistent actor.

### 4.2 StableGroup

A `StableGroup` is a recurrent set of individuals with durable ties and repeated interaction.

Examples:

- kin cluster
- neighborhood faction
- work team
- ritual circle
- recurring militia core

A stable group has:

- continuity
- memory of interaction
- boundary tendency

But may still lack true collective decision authority.

### 4.3 CollectiveActor

A `CollectiveActor` is a group that can make decisions as a unit under an explicit or implicit constitution.

In POLIS, a valid collective actor must have:

- identifiable membership or boundary rules
- some shared resources, rights, or obligations
- a decision procedure
- persistence across issues or time
- external recognition by other actors or institutions

### 4.4 HouseholdActor

A `HouseholdActor` is a special collective actor organized around:

- co-residence
- pooled resources
- shared labor
- reproduction and care
- household bargaining or authority

Households should not be treated as perfectly unitary.

### 4.5 OrganizationActor

An `OrganizationActor` is a collective actor with:

- explicit roles
- routines
- controlled assets
- task continuity

Examples:

- guild
- firm
- militia company
- temple organization
- trade syndicate

### 4.6 GovernanceActor

A `GovernanceActor` is a collective actor whose main function is:

- rule-making
- adjudication
- enforcement
- coordination of collective obligations

Examples:

- council
- court
- tax office
- command staff

### 4.7 PolityActor

A `PolityActor` is a higher-level collective actor that claims authority over:

- territory
- population
- extraction
- enforcement
- institutional order

Examples:

- chiefdom
- city-state
- kingdom
- federation

### 4.8 Settlement

A `Settlement` remains a spatial and social container, not automatically a collective actor.

A settlement becomes a true actor only when governance structures and recognized collective decision procedures exist.

### 4.9 Aggregate

An `Aggregate` remains descriptive only.

Examples:

- peasants of region X
- all traders
- urban poor

Aggregates may matter analytically but do not decide.

## 5. Category Boundaries

### 5.1 Coordination vs stable group

- `CoordinationCluster` is temporary and situational
- `StableGroup` persists across multiple interactions and contexts

### 5.2 Stable group vs collective actor

- `StableGroup` has continuity
- `CollectiveActor` has continuity plus decision constitution, shared assets or obligations, and external interface

### 5.3 Collective actor vs institution

- `CollectiveActor` acts
- `Institution` constrains, enables, and regulates action

They may be tightly linked, but they are not the same thing.

### 5.4 Settlement vs polity

- `Settlement` is a place-based social concentration
- `PolityActor` is a governing actor with jurisdictional claims

## 6. Criteria For Collective Agency

Not every stable group should be promoted into a collective actor.

### 6.1 Required criteria

A group should become a collective actor only when all of the following are sufficiently present:

1. boundary clarity
2. recurrent membership or explicit membership rules
3. shared resources, rights, or liabilities
4. internal decision procedure
5. externally meaningful action capacity

### 6.2 Supporting criteria

Additional strengthening criteria include:

- role differentiation
- internal records or memory
- recognized spokespersons or offices
- institutional embedding
- legitimacy among members

### 6.3 External recognition

External recognition matters.

A group becomes more actor-like when others routinely treat it as:

- a negotiation partner
- a taxable unit
- an ally
- an enemy
- a property holder

## 7. Collective Decision Domains

Different kinds of decisions belong at different scales.

### 7.1 Plausible collective domains

Collective actors are especially appropriate for:

- pooled resource allocation
- coordinated migration
- diplomatic stance
- military mobilization
- taxation and extraction policy
- infrastructure investment
- rule adoption or amendment
- large-scale ritual or public works

### 7.2 Individual domains that should usually remain individual

Even inside strong collectives, the following should generally remain individual:

- daily effort choice
- micro-mobility
- norm compliance
- opportunistic theft or help
- local innovation
- preference change
- dissent and defection

### 7.3 Shared domains

Some domains are mixed:

- household fertility
- division of labor
- marriage alliances
- career specialization

These may involve both collective constraints and individual bargaining.

## 8. Internal Structure Of Collective Actors

Collective actors must not be modeled as internally homogeneous blobs.

### 8.1 Required internal state

A serious collective actor should retain at least:

- membership structure
- role distribution
- influence or bargaining weights
- pooled assets
- legitimacy level
- factional patterning
- internal inequality indicators

### 8.2 Constitution

Each collective actor should have a `Constitution` in the functional sense:

- how member inputs are combined
- who has authority over what
- how disagreement is resolved
- how leadership changes
- what counts as a valid collective decision

Examples:

- majority vote
- weighted council
- patriarchal or oligarchic dominance
- command hierarchy
- consensus rule

This constitution is what makes the actor more than a loose group.

### 8.3 Factionalism

Factionalism should remain explicit.

Groups may contain:

- sub-coalitions
- rival lineages
- elite-commoner splits
- center-periphery tensions

These internal structures should affect stability, compliance, and fracture risk.

## 9. What Must Remain Individual

Collective actors may dominate some decisions, but individuals remain the fundamental bearers of:

- beliefs
- preferences
- memories
- skills
- personal ties
- partial wealth
- compliance propensity
- grievance

This rule is essential.

Without it, collective actors become puppet-master abstractions detached from the microfoundations of the simulation.

## 10. Downward Causation

POLIS should allow downward causation, but only in a disciplined form.

### 10.1 Valid mechanisms

Collective actors may influence members through:

- directives
- norms
- incentives
- sanctions
- information control
- resource allocation
- role assignment

### 10.2 Invalid mechanism

Collective actors must not directly overwrite an individual’s beliefs, preferences, or actions as if the individual had no agency.

### 10.3 Proper representation

Downward causation should work by changing:

- available actions
- payoffs
- risks
- obligations
- information visibility
- expected sanction intensity

This preserves individual-level causality while still allowing real top-down structure.

## 11. Group Life Cycle

POLIS should define an explicit life cycle for collective formation and breakdown.

### 11.1 Life-cycle states

Recommended lifecycle:

1. ephemeral coordination
2. proto-group
3. unstable collective actor
4. stabilized collective actor
5. fragmenting collective actor
6. dissolved or split state

### 11.2 Transition metrics

Transitions should depend on measurable state such as:

- cohesion
- institutionalization
- legitimacy
- factionalism
- resource centralization
- external pressure

### 11.3 Emergence

`CoordinationCluster -> StableGroup` when:

- repeated coordination recurs
- membership overlap remains high
- ties strengthen over time

`StableGroup -> CollectiveActor` when:

- decision constitution appears
- pooled resources matter
- external interface becomes meaningful

### 11.4 Stabilization

An unstable collective becomes stabilized when:

- decisions repeat successfully
- legitimacy remains high enough
- members comply often enough
- institutional scaffolding deepens
- external recognition persists

### 11.5 Fragmentation

Fragmentation becomes likely when:

- legitimacy falls
- internal inequality sharpens
- factions consolidate
- resource disputes intensify
- external stress exposes weak integration

### 11.6 Re-expansion and federation

Fragmented groups may later:

- reunify under threat
- federate into larger actors
- merge through institutional integration

This should be possible without assuming permanent hierarchical consolidation.

## 12. Merge And Split Rules

The simulation should support explicit collective merge and split rules compatible with the state model.

### 12.1 Merge

A merge should require:

- compatible institutions or negotiable constitutions
- sufficient coordination benefit
- manageable factional distance
- some mechanism for integrating assets and roles

### 12.2 Split

A split should require:

- identifiable subgroups
- sufficient independent cohesion
- severe enough disagreement, inequality, or stress
- some path to independent action

### 12.3 Scale integrity

Merge and split must not duplicate authority across levels.

When a collective actor forms or dissolves, state ownership and decision rights must transition explicitly under the state contract.

## 13. Approximation And Scale Use

Collective actors are useful partly because they can reduce complexity, but they must not become fake shortcuts.

### 13.1 When collective representation is justified

Collective representation is justified when:

- it improves causal clarity
- it matches observed coordination structure
- it reduces unnecessary micro-resolution
- it preserves important variation through retained individual substate

### 13.2 When it is not justified

It is not justified when:

- the group is only a statistical grouping
- there is no decision constitution
- members are too heterogeneous and uncoordinated
- the group has no meaningful external action interface

## 14. Risks And Anti-Patterns

The following design failures must be avoided.

### 14.1 Unitary group fallacy

Treating households, firms, or states as perfectly unified utility maximizers.

### 14.2 Aggregate-as-actor error

Treating descriptive categories as if they automatically possess agency.

### 14.3 Fake emergence

Creating macro-actors without explicit micro-to-macro mechanisms.

### 14.4 Double counting

Letting both the collective and all members independently own the same strategic decision.

### 14.5 Puppet individuals

Allowing top-down directives to erase dissent, non-compliance, or deviation.

### 14.6 Over-promotion

Promoting every recurring coordination pattern into a persistent actor.

### 14.7 Static collectives

Failing to model merger, fragmentation, decay, and reorganization.

## 15. Open Questions

These questions remain for later refinement:

- Which collective constitutions deserve explicit early support in v1?
- How much factional detail is necessary before collective simulation becomes too expensive?
- When should a settlement itself become a governance actor rather than only hosting one?
- Which collective forms should be allowed to nest recursively in v1?

These questions do not block the collective-agency model, but they matter for later institutional and technical work.

## 16. Summary

POLIS should model collective agency as a life-cycle process in which:

- temporary coordination may become stable grouping
- stable grouping may become true collective actors
- collective actors act through constitutions, roles, and shared assets
- individual heterogeneity remains real inside them
- downward causation works through constraints and incentives
- collectives can stabilize, fragment, merge, and re-expand

This gives POLIS a defensible multi-level agency model that is consistent with the ontology, state contract, and serious experimental goals of the project.
