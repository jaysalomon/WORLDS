# POLIS Society, Institutions, And Conflict

**Version:** 0.1  
**Date:** 14 March 2026  
**Document:** 07 of the POLIS spec suite  
**Purpose:** Define the social variables, institutional mechanisms, exchange systems, coercive structures, and collapse dynamics through which trust, hierarchy, trade, conflict, and governance emerge and transform in POLIS.

## 1. Scope

This document defines:

- the social and institutional ontology relevant to POLIS
- the core social pressures and variables that drive cooperation and conflict
- how institutions emerge, stabilize, transform, and collapse
- how trade, alliance, feud, coercion, and warfare connect to resources and governance

This document does **not** define:

- the full collective-agency model
- the full biology or material ontology
- low-level implementation details

## 2. Dependencies

This document depends on:

- [01_WorldModel.md](/abs/path/e:/Drive/WORLDS/01_WorldModel.md)
- [02_StateModel.md](/abs/path/e:/Drive/WORLDS/02_StateModel.md)
- [03_CollectiveAgency.md](/abs/path/e:/Drive/WORLDS/03_CollectiveAgency.md)
- [04_ResourcesAndMaterials.md](/abs/path/e:/Drive/WORLDS/04_ResourcesAndMaterials.md)
- [05_DiscoveryHeuristics.md](/abs/path/e:/Drive/WORLDS/05_DiscoveryHeuristics.md)
- [06_BiologyAndDomestication.md](/abs/path/e:/Drive/WORLDS/06_BiologyAndDomestication.md)

It provides major inputs to:

- `08_ValidationAndExperiments.md`
- later frontend and technical architecture work

## 3. Guiding Principle

POLIS must not use vague faction labels or scripted buildings as substitutes for social explanation.

Society should be modeled through explicit mechanisms:

- social ties
- roles
- norms
- institutions
- organizations
- bargaining
- coercion
- legitimacy
- trade enforcement
- conflict pressures

Macro structures should emerge from and feed back into these mechanisms, not bypass them.

## 4. Core Social Ontology

POLIS should represent social structure through a compact set of linked concepts.

### 4.1 SocialTie

A `SocialTie` is a directed or undirected relation between actors.

Important tie types include:

- kinship
- alliance
- trade partnership
- patronage
- obedience relation
- hostility
- debt

Ties are not institutions. They are relationship-level structure.

### 4.2 SocialNetwork

A `SocialNetwork` is the patterned topology of ties among actors.

Network structure matters because it shapes:

- information flow
- reputation spread
- coalition formation
- diffusion of practices
- enforcement reach

### 4.3 Norm

A `Norm` is a socially shared expectation about what actors should do.

Norms may be:

- informal
- weakly enforced
- internalized
- locally variable

Norms are stronger than habits, but weaker than formal rules.

### 4.4 Institution

An `Institution` is a structured rule bundle that constrains and enables action in defined contexts.

Institutions may define:

- who may participate
- what actions are permitted
- what obligations apply
- what sanctions follow violations
- who monitors compliance

Institutions are not actors, but they are among the most important causal structures in the model.

### 4.5 Organization

An `Organization` is a collective actor with explicit roles, assets, and routines.

Examples:

- guild
- militia
- merchant coalition
- council
- temple administration

Organizations enact, maintain, or contest institutions, but are not identical to them.

### 4.6 ActionArena

An `ActionArena` is a structured context in which actors interact under:

- biophysical conditions
- community attributes
- rules-in-use

Examples:

- market exchange
- tax collection
- irrigation turn allocation
- court dispute
- elite council
- raid planning

This concept is useful because institutions do not operate everywhere equally; they are activated in contexts.

### 4.7 SettlementOrder

A `SettlementOrder` is the patterned institutional and social arrangement of a settlement.

It may include:

- local governance forms
- market structures
- defense arrangements
- sanitation and storage norms
- internal divisions

This is not a separate top-level entity from settlement, but a useful analytical layer.

### 4.8 PolityOrder

A `PolityOrder` is the layered institutional arrangement through which a polity governs territory, extraction, force, and adjudication.

It may include:

- office hierarchy
- tax system
- military chain
- legal rules
- public works obligations

## 5. Category Boundaries

### 5.1 Tie vs network vs institution

- `SocialTie` is a relation between specific actors
- `SocialNetwork` is the structure of many ties
- `Institution` is a rule bundle governing action

### 5.2 Institution vs organization

- `Institution` is the rule structure
- `Organization` is the actor or collective body operating under or through those rules

### 5.3 Settlement vs polity

- `Settlement` is local social concentration
- `PolityOrder` concerns higher-level jurisdiction and rule coordination across territory and groups

### 5.4 Norm vs formal rule

- `Norm` may rely on shame, approval, and expectation
- formal institutional rules rely on explicit monitoring and sanction pathways

## 6. Core Social Variables And Pressures

POLIS should track a small number of social variables that drive much of the higher-order behavior.

### 6.1 Trust

Trust is the expected probability that another actor will:

- comply
- reciprocate
- deliver
- refrain from opportunistic betrayal

Trust should be contextual rather than global.

### 6.2 Reciprocity

Reciprocity is the tendency to reward cooperation and punish cheating over repeated interactions.

This is a major basis of social stability in small and medium-scale systems.

### 6.3 Prestige

Prestige is influence granted voluntarily because an actor is viewed as:

- skilled
- wise
- successful
- admirable

Prestige-based rank tends to support imitation and prosocial coordination.

### 6.4 Dominance

Dominance is influence gained through:

- coercion
- intimidation
- asymmetric force
- control over scarce essentials

Dominance-based order can be effective but tends to generate grievance and monitoring cost.

### 6.5 Legitimacy

Legitimacy is the perceived rightfulness of authority and rules.

It affects:

- compliance
- tax acceptance
- military cohesion
- rebellion risk

### 6.6 Grievance

Grievance is accumulated resentment arising from:

- unmet needs
- exploitation
- humiliation
- blocked mobility
- unequal treatment
- institutional failure

Grievance is one of the main pathways from inequality to instability.

### 6.7 Coordination burden

Coordination burden is the cost of making and enforcing collective decisions at scale.

It should increase with:

- group size
- heterogeneity
- weak communication
- poor role clarity
- low legitimacy

### 6.8 Elite pressure

Elite pressure arises when:

- too many actors compete for limited high-rank positions
- surplus extraction becomes politically contested
- counter-elites emerge

This is a major candidate driver of polity instability and institutional transformation.

## 7. Action Arenas And Rules-In-Use

Social processes should occur in explicit action contexts.

### 7.1 ActionArena structure

An action arena should specify:

- participants
- positions or roles
- available actions
- information conditions
- active rules
- sanction structure
- possible outcomes

### 7.2 Exogenous conditions

Each arena is shaped by:

- biophysical conditions
- community attributes
- rules-in-use

This ensures that the same actors may behave differently in different institutional contexts.

### 7.3 Operational use

Action arenas are especially important for:

- trade
- dispute resolution
- taxation
- recruitment
- redistributive allocation
- coalition building

## 8. Norms, Rules, And Procedures

POLIS should distinguish informal and formal regulation clearly.

### 8.1 Shared strategy

A shared strategy is a repeated pattern followed because it is useful or efficient.

It has low formal enforcement.

### 8.2 Norm

A norm adds social expectation and disapproval or esteem effects.

Violation is noticeable and socially meaningful.

### 8.3 Formal rule

A formal rule includes:

- attribute or target
- deontic force, such as obligation, permission, or prohibition
- action or required aim
- condition
- sanction pathway

This is the minimum serious shape of institutional rule logic.

### 8.4 Nested enforcement

Formal rules should not assume magic enforcement.

If a rule says “pay tax or else goods are seized,” then:

- some monitor must detect the violation
- some enforcer must act
- some higher authority must handle enforcer failure

This nested enforcement logic is essential to modeling corruption, decay, and institutional fragility.

## 9. Institutional Emergence

Institutions should emerge from pressures, not scripted stages.

### 9.1 Emergence pressures

Primary pressures include:

- coordination burden
- stored surplus
- repeated conflict
- trade complexity
- commons management problems
- inequality
- disease and sanitation burden
- external threat

### 9.2 Early institution families

POLIS should treat these as especially important early forms:

- property and access rules
- reciprocity and obligation norms
- dispute resolution procedures
- leadership and office rules
- militia or defense coordination rules
- storage and redistribution rules
- trade and measurement rules
- irrigation and commons-use rules

### 9.3 Emergence mechanism

Institutional emergence is most defensible when:

- repeated action arenas expose recurring problems
- existing informal norms become insufficient
- groups invest in more explicit rules and monitoring
- roles emerge to manage the burden

## 10. Institutional Persistence, Adaptation, And Collapse

Institutions should be dynamic, not static.

### 10.1 Persistence

Institutions persist when:

- compliance remains high enough
- enforcement chains remain functional
- legitimacy remains sufficient
- the cost of operation remains bearable
- the institution still solves a real coordination problem

### 10.2 Adaptation

Institutions adapt when:

- rules are amended
- offices change powers
- sanctions intensify or weaken
- jurisdiction expands or contracts
- procedures become more formal or more local

### 10.3 Decay

Institutions decay when:

- sanctions stop being carried out
- monitoring weakens
- elites exempt themselves
- information flows break down
- compliance falls below a viable threshold

### 10.4 Collapse

Institutional collapse occurs when:

- enforcement chains fail broadly
- legitimacy collapses
- rival orders or factions outcompete the old structure
- fiscal or material maintenance costs exceed returns

Collapse should be understood as loss of effective rules-in-use, not only disappearance of nominal rules.

## 11. Scalar Stress And Hierarchical Complexity

As societies grow, face-to-face coordination becomes insufficient.

### 11.1 Scalar stress

Scalar stress is the rising burden of communication and coordination as the number of interacting decision units increases.

This pressure should promote:

- modular organization
- delegation
- office creation
- representative structures
- command hierarchies

### 11.2 Sequential versus simultaneous hierarchy

POLIS should allow different scaling strategies:

- modular or sequential hierarchy
- direct simultaneous hierarchy
- mixed systems with delegated layers

These different solutions should have different costs, resilience, and inequality consequences.

### 11.3 Ritual and identity buffering

Shared ritual, identity, and symbolic order may temporarily reduce coordination burden by:

- simplifying expectations
- reinforcing legitimacy
- reducing perceived social distance

This can delay the need for heavier formal administration.

## 12. Trade, Trust, And Enforcement

Trade should connect directly to institutions and information quality.

### 12.1 Private-order exchange

Small-scale and high-trust trade may rely on:

- reputation
- kinship
- coalition enforcement
- ostracism

This works well in small, dense information networks.

### 12.2 Public-order exchange

Large-scale and impersonal trade requires more formal support:

- standard measures
- records
- arbitration
- contract enforcement
- sanctioned fraud response

### 12.3 Transition pressure

The shift from reputation-only exchange to formal trade institutions should occur when:

- exchange extends beyond tight networks
- monitoring costs rise
- agent mobility increases
- coalition enforcement becomes too weak or too narrow

## 13. Conflict, Coercion, And Warfare

Conflict should be socially structured, not merely a combat toggle.

### 13.1 Conflict pressures

Conflict pressure should rise with:

- grievance
- resource competition
- status competition
- ideological distance
- weak legitimacy
- elite rivalry

### 13.2 Coercive structures

Coercive capacity depends on:

- organized force
- command chain
- material support
- sanction credibility
- legitimacy or fear

### 13.3 Prestige versus dominance orders

Hierarchy should be allowed to tilt between:

- prestige-based leadership
- dominance-based coercion

These should generate different social climates, compliance patterns, and collapse risks.

### 13.4 Warfare-complexity feedback

Persistent inter-group competition may select for:

- better extraction
- more hierarchy
- stronger administration
- military specialization
- larger coordinated units

But it also increases fragility and maintenance burden.

### 13.5 Minimal combat resolution model (v1)

For v1, POLIS should use a transparent bounded combat model with a technology multiplier and a small number of interpretable modifiers.

Per side combat power:

`CombatPower = ForceSize * TechMultiplier * SupplyMultiplier * CohesionMultiplier * TerrainMultiplier * LeadershipMultiplier`

Recommended default ranges:

- `TechMultiplier`: `0.7..1.8`
- `SupplyMultiplier`: `0.5..1.2`
- `CohesionMultiplier`: `0.6..1.3`
- `TerrainMultiplier`: `0.7..1.4`
- `LeadershipMultiplier`: `0.8..1.2`

These values should be scenario-configurable, not hardcoded forever.

Outcome probability (attacker perspective):

`P(AttackerWin) = CombatPower_A / (CombatPower_A + CombatPower_D)`

Resolution should be deterministic under fixed seed by using seeded stochastic draw from this probability.

### 13.6 Casualties and aftermath

After outcome is sampled, both sides take losses.

Loss rate should depend on:

- relative power ratio
- terrain
- supply collapse
- retreat vs rout condition

Minimal v1 approach:

- winner loss fraction: `0.05..0.20`
- loser loss fraction: `0.15..0.50`
- routed loser may take additional attrition in retreat phase

Post-battle state updates should include:

- surviving force
- morale/cohesion shock
- grievance increase in affected population
- logistics stock drawdown

### 13.7 Conquest and occupation

Battle win should not automatically imply stable conquest.

Use a separate control transition step:

`ControlScore = MilitaryPresence * Legitimacy * AdministrativeReach * SupplyContinuity`

`ResistanceScore = LocalGrievance * DefenderNetworkDensity * TerrainFriction * ExternalSupport`

If `ControlScore > ResistanceScore` by configured margin for `N` ticks, territory shifts from contested to controlled.

Territorial states:

- `Unclaimed`
- `Claimed`
- `Contested`
- `Controlled`
- `Occupied`

Occupation should decay if garrison or legitimacy collapses.

### 13.8 Territory mapping mechanics

Each partition should track:

- controlling actor (nullable)
- claimant set (0..many)
- control strength (`0..100`)
- contest pressure (`0..100`)
- frontier flag

Control should diffuse from connected controlled partitions and decay under:

- low garrison
- weak supply line continuity
- high local grievance
- repeated raids

This keeps map changes path-dependent rather than binary.

### 13.9 Validation requirements for conflict subsystem

Minimum checks before accepting conflict mechanics:

- superior technology increases expected win rate ceteris paribus
- poor supply reduces expected performance even with larger force
- terrain defensive advantage is measurable and bounded
- repeated victories without administrative reach fail to produce stable control
- serial/parallel parity holds under fixed seed and scenario

## 14. Social Complexity And Collapse Dynamics

POLIS should allow both growth and overreach.

### 14.1 Reinforcing loop

Surplus can support:

- administration
- infrastructure
- organized force
- specialization

Which may in turn increase extraction and coordination capacity.

### 14.2 Counteracting loop

But rising complexity also creates:

- maintenance cost
- bureaucracy burden
- corruption opportunities
- elite competition
- legitimacy strain

### 14.3 Structural-demographic pressure

The simulator should support crisis dynamics where:

- population pressure rises
- living standards fall
- elite competition intensifies
- grievance spreads
- state finances weaken

This is a plausible route to rebellion, civil war, or institutional simplification.

## 15. Biological And Material Coupling

Social institutions must remain grounded in the earlier documents.

### 15.1 Agricultural surplus

Agricultural productivity changes:

- extraction potential
- labor specialization
- storage dependence
- tax feasibility

### 15.2 Resource control

Institutions should matter most where resources are:

- shared
- storable
- countable
- strategically scarce

### 15.3 Disease burden

Disease should affect institutions by increasing demand for:

- sanitation rules
- quarantine practices
- water management
- burial and waste procedures

## 16. Risks And Anti-Patterns

The following design failures must be avoided.

### 16.1 Scripted institutions

Treating councils, markets, armies, or states as pre-authored stage unlocks.

### 16.2 Faction shorthand

Using labels as substitutes for explicit social mechanisms.

### 16.3 Magic enforcement

Assuming rules are effective without monitors, enforcers, incentives, and sanction pathways.

### 16.4 Static trust

Treating trust as a permanent scalar instead of a context-sensitive expectation updated through interaction and institutions.

### 16.5 State as a person

Treating polities as unitary beings rather than collective orders with internal tensions and enforcement dependencies.

### 16.6 Conflict without structure

Reducing warfare and feud to raw aggression without social ties, grievance, logistics, or institutions.

### 16.7 Collapse without mechanism

Treating collapse as a random disaster rather than the loss of effective coordination, legitimacy, and enforcement under stress.

## 17. Open Questions

These questions remain for later refinement:

- How far should explicit rule grammar go in v1 before the model becomes too heavy?
- Which institutional domains deserve explicit early support versus later extension?
- How much of trade law and contract enforcement should be modeled before the exchange system becomes overcomplicated?
- Which legitimacy signals should matter most across different polity forms?

These do not block the social/institutional model, but they matter for later implementation and validation.

## 18. Summary

POLIS should model society and institutions through explicit mechanisms in which:

- ties form networks
- networks support norms and coalition structures
- institutions encode rules-in-use
- organizations and governance actors enact those rules
- trust, prestige, dominance, legitimacy, and grievance shape compliance and conflict
- trade and warfare depend on information, enforcement, and resource control
- rising complexity can generate both stronger coordination and stronger fragility

This gives POLIS a scientifically defensible social backbone in which governance, order, coercion, exchange, and collapse arise from explicit social processes rather than scripted abstractions.
