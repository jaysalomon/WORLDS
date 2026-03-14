# POLIS Discovery Heuristics And Knowledge Systems

**Version:** 0.1  
**Date:** 14 March 2026  
**Document:** 05 of the POLIS spec suite  
**Purpose:** Define how POLIS models affordance discovery, process learning, technique refinement, social transmission, and institutional stabilization without relying on a fixed tech tree.

## 1. Scope

This document defines:

- the ontology of discovery and practical knowledge
- how observations become reusable techniques
- how techniques are searched, refined, and retained
- how knowledge spreads across individuals and groups
- how practices become institutionalized

This document does **not** define:

- the full material ontology
- the full biological or domestication model
- low-level implementation structures
- frontend narrative treatment

## 2. Dependencies

This document depends on:

- [01_WorldModel.md](/abs/path/e:/Drive/WORLDS/01_WorldModel.md)
- [02_StateModel.md](/abs/path/e:/Drive/WORLDS/02_StateModel.md)
- [04_ResourcesAndMaterials.md](/abs/path/e:/Drive/WORLDS/04_ResourcesAndMaterials.md)

It provides key foundations for:

- `06_BiologyAndDomestication.md`
- `07_SocietyAndInstitutions.md`
- `08_ValidationAndExperiments.md`

## 3. Guiding Principle

POLIS should not model technology as a scripted invention ladder.

Instead, discovery should emerge from:

- latent affordances in the world
- repeated interaction with materials and environments
- bounded heuristic search over process space
- social learning and cultural transmission
- institutional stabilization of successful practices

The simulator should therefore model discovery as a structured cultural process, not a sequence of magic unlocks.

## 4. Core Knowledge Ontology

POLIS should use a process-centric ontology for knowledge and discovery.

### 4.1 Affordance

An `Affordance` is a realized action possibility linking:

- actor capability
- material state
- tool state
- environmental context
- expected effect

An affordance is not yet a full technique.

It is a conditional recognition that:

- this thing can be done here
- with these materials
- by someone like me
- with some expected result

### 4.2 Observation

An `Observation` is an episodic record of an action and its outcome.

It may include:

- what was attempted
- under what conditions
- what happened
- whether it was useful or harmful

An observation alone is not generalized knowledge.

### 4.3 ProcessSchema

A `ProcessSchema` is a generalized operational sequence.

It represents:

- ordered or partially ordered steps
- expected inputs
- expected intermediate states
- expected outputs
- optional role structure

This corresponds to the idea that technologies are action sequences over materials rather than isolated named objects.

### 4.4 Technique

A `Technique` is a process schema that has become purposeful and reproducible enough for intentional reuse.

It includes:

- parameter ranges
- expected payoffs
- expected risks
- required inputs
- required skills or roles

A technique is more concrete than a process schema, but may still remain largely tacit.

### 4.5 Skill

A `Skill` is competence in executing one or more techniques under varying conditions.

It belongs to:

- individuals
- households
- roles
- collective actors

depending on the level of representation.

Skill is not the same thing as a technique.

The same technique may exist in a society while being executed with very different reliability by different actors.

### 4.6 CausalBelief

A `CausalBelief` is an explicit or semi-explicit proposition linking:

- conditions
- actions
- outcomes

Examples:

- heating this material hardens it
- storing food this way slows spoilage
- leaving a field fallow restores yield

Causal beliefs may be true, false, partial, or overgeneralized.

### 4.7 Routine

A `Routine` is a recurrent, socially reproduced use of one or more techniques across roles and time.

Examples:

- annual planting cycle
- seasonal fuel-making practice
- recurring kiln schedule
- established preservation workflow

### 4.8 InstitutionalProcedure

An `InstitutionalProcedure` is a routine anchored in an institution with:

- prescribed roles
- permissions
- obligations
- enforcement
- allocated resources

This is the point where discovery becomes institutionally durable.

### 4.9 DesignSchema

A `DesignSchema` is an abstract pattern for a tool, artifact, or structure that can be realized through one or more techniques.

Examples:

- storage pit design
- furnace design
- irrigation layout
- cart pattern

### 4.10 CulturalTraitBundle

A `CulturalTraitBundle` is a transmissible package containing multiple linked knowledge elements.

Examples:

- terrace agriculture package
- metalworking package
- caravan trade package

This is a composite modelling convenience, not the fundamental unit of causality.

### 4.11 PerformanceRecord

A `PerformanceRecord` is a retained summary of how well a technique or process performs under specific conditions.

It may capture:

- success frequency
- yield
- failure rate
- risk
- cost
- environmental sensitivity

This record is essential for refinement and social learning.

## 5. Discovery Lifecycle

POLIS should define discovery as a staged process.

### 5.1 Stage 1: Accidental observation

An actor encounters a useful or harmful outcome during ordinary action, play, error, or opportunistic experimentation.

At this stage:

- the event is episodic
- no stable generalization exists yet
- the outcome may still be forgotten

### 5.2 Stage 2: Affordance candidate

Repeated observations of similar action-context-material combinations create a candidate affordance.

At this stage:

- an actor expects a useful effect
- the representation remains local and context-specific
- the actor may intentionally attempt repetition

### 5.3 Stage 3: Process schema

Once multiple affordances are chained or coordinated toward a goal, a process schema emerges.

At this stage:

- sequence matters
- inputs and outputs become more structured
- multiple roles may begin to matter

### 5.4 Stage 4: Technique

A process schema becomes a technique once it can be intentionally reenacted with sufficiently reliable positive outcomes.

At this stage:

- parameter ranges are partially known
- expected benefits are estimable
- intentional teaching becomes more plausible

### 5.5 Stage 5: Codified knowledge

A technique becomes codified when it is represented beyond tacit execution.

This may occur through:

- verbal explanation
- ritualized instruction
- explicit rules
- written or material records

### 5.6 Stage 6: Institutionalized practice

A technique becomes institutionally stabilized when:

- roles are assigned
- resources are allocated
- procedures are regulated
- associated infrastructure is maintained
- transmission is socially protected

At this stage the technique becomes much harder to lose and much more likely to scale.

## 6. Discovery Mechanics

Discovery should arise from action, feedback, and bounded search.

### 6.1 Primitive interaction basis

Agents should operate through a finite repertoire of primitive actions such as:

- move
- gather
- cut
- strike
- heat
- mix
- enclose
- store
- transport
- plant
- feed
- bind

More complex processes arise from combinations of these primitives over resource and material systems.

### 6.2 Affordance formation

When repeated episodes show that a certain action-context-material combination yields above-baseline value, the system should create or strengthen an affordance representation.

Important factors:

- observed payoff
- observed variance
- resource cost
- risk
- frequency of success

### 6.3 Process construction

Process schemas should be constructed when actors chain affordances to address persistent needs or opportunities.

Examples:

- preserving food
- extracting better fuel
- shaping better tools
- controlling water flow

### 6.4 Refinement

Techniques should improve through local search over:

- parameter values
- step ordering
- material substitution
- tool substitution
- role assignment

Repeated success should strengthen confidence and retention.

Repeated failure should weaken confidence, trigger abandonment, or trigger renewed search.

## 7. Heuristic Search And Bounded Rationality

POLIS should model discovery as bounded, satisficing search rather than exhaustive optimization.

### 7.1 Search triggers

Discovery search should intensify when actors face:

- food shortfall
- safety pressure
- status competition
- resource scarcity
- new material encounters
- visible payoff gaps versus neighbors

### 7.2 Local neighbourhood search

Search should occur mainly in the neighbourhood of existing processes.

Allowed search operators should include:

- small parameter perturbation
- large parameter perturbation
- step insertion
- step deletion
- step reordering
- material substitution with similar property class
- tool substitution
- recombination of known sub-sequences

This keeps search tractable and plausible.

### 7.3 Heuristic preferences

Search should be biased by simple general heuristics:

- prefer fewer steps when payoffs are similar
- prefer reuse of existing tools
- prefer reuse of existing skills
- prefer familiar materials when performance is acceptable
- focus changes on parameters historically linked to success variance

### 7.4 Satisficing rule

Actors should usually stop searching when a candidate technique exceeds an aspiration threshold relative to current need.

This threshold may depend on:

- urgency
- risk tolerance
- scarcity pressure
- social comparison

### 7.5 Exploration versus imitation

Actors should not innovate from scratch by default.

Default order should usually be:

1. reuse known technique
2. copy visible successful technique
3. locally modify known technique
4. attempt broader search or accidental exploration

This reflects bounded rationality and cultural-evolution logic.

## 8. Social Learning

Social learning is central to cumulative knowledge.

### 8.1 Transmission modes

POLIS should support:

- vertical transmission
- oblique transmission
- horizontal transmission

These differ in:

- fidelity
- reach
- prestige structure
- delay

### 8.2 Learning biases

The model should support at least these biases:

- payoff-biased copying
- conformist copying
- prestige-biased copying
- content-biased copying

These should be parameterized rather than hard-coded as always dominant.

### 8.3 Transmission fidelity

Not all knowledge transmits equally well.

Transmission fidelity should depend on:

- observability of key steps
- tacitness of the skill
- teaching effort
- role specialization
- symbolic codification

High-complexity techniques should be easier to lose in small, poorly connected populations.

## 9. Retention And Forgetting

Knowledge must persist unevenly across levels.

### 9.1 Individual retention

Individuals should have limited memory and competence retention.

Unused knowledge should decay depending on:

- time since last use
- rarity of reinforcement
- complexity
- contextual mismatch

### 9.2 Household and group retention

Households, workshops, and collective actors should preserve knowledge more reliably than isolated individuals because:

- multiple carriers overlap
- apprenticeship becomes possible
- routines reinforce retention

### 9.3 Institutional retention

Institutions should provide the slowest-decaying form of knowledge retention through:

- rules
- roles
- records
- dedicated infrastructure
- training pathways

This is one reason institutions matter for technological persistence.

## 10. Diffusion Across Scales

Knowledge should diffuse differently across scales.

### 10.1 Within households and workshops

High frequency, high fidelity, and skill-intensive transmission.

### 10.2 Within settlements

Diffusion through:

- kinship
- labor contact
- neighborhood observation
- exchange
- ritual

### 10.3 Across settlements

Diffusion through:

- trade
- migration
- marriage
- raiding
- apprenticeship travel
- diplomatic or religious links

Cross-settlement transmission should be less frequent and often lower fidelity, but may carry larger cultural bundles.

### 10.4 Through institutions

Institutions may accelerate or constrain diffusion by:

- sponsoring training
- standardizing procedures
- imposing restrictions
- funding infrastructure
- monopolizing techniques

## 11. Stabilization Conditions

Not every useful technique should become durable culture.

POLIS should define threshold conditions for stabilization.

### 11.1 Technique to practice

A technique should stabilize into a practice only when there is sufficient:

- repeated success
- payoff advantage
- user count
- transmission frequency
- contextual reliability

### 11.2 Practice to institutionalized procedure

A practice should stabilize into an institutional procedure only when there is sufficient:

- collective dependence
- role specialization
- surplus or capacity to support regulation
- governance ability
- infrastructural anchoring

### 11.3 Fragility rule

Complex techniques should remain fragile when:

- population is too small
- connectivity is low
- teaching fidelity is poor
- institutions are weak
- required infrastructure collapses

This allows technological loss, stagnation, and uneven development.

## 12. Coupling To Materials, Ecology, And Society

Discovery should remain grounded in the material world.

### 12.1 Material coupling

Techniques must depend on:

- resource systems
- material-energy classes
- environmental fields
- process-template feasibility

No valid discovery should bypass physical affordances.

### 12.2 Social coupling

Knowledge should affect:

- prestige
- labor organization
- trade advantage
- military advantage
- institutional complexity
- settlement resilience

### 12.3 Ecological feedback

Successful techniques should feed back into:

- resource depletion
- yield increase
- pollution or waste
- demographic change
- inequality

This prevents technology from being treated as pure bonus accumulation.

## 13. Risks And Anti-Patterns

The following design failures must be avoided.

### 13.1 Scripted tech trees

Predetermined invention order disconnected from context and process.

### 13.2 Atomic technology tokens

Treating large technologies as indivisible unlocks instead of process families and techniques.

### 13.3 Perfect transmission

Assuming knowledge spreads or persists without fidelity limits, teaching constraints, or loss.

### 13.4 Over-rational search

Giving agents global foresight or exhaustive process optimization.

### 13.5 Institution blindness

Failing to model how institutions preserve, regulate, or distort knowledge.

### 13.6 Ecology-free technology

Letting techniques raise output without resource, demographic, or environmental feedback.

## 14. Open Questions

These questions remain for later documents:

- How fine-grained should causal beliefs be in early societies before symbolic codification becomes common?
- When should a cultural trait bundle become a first-class efficiency construct rather than a derived grouping?
- How much of teaching should be explicit versus inferred from routine interaction?
- Which techniques should require role-based transmission rather than general copying?

These do not block the discovery ontology, but they matter for later refinement.

## 15. Summary

POLIS should model discovery as a multi-stage cultural process in which:

- observations reveal affordances
- affordances are chained into process schemas
- process schemas are refined into techniques
- techniques are retained through skill and memory
- social learning diffuses them
- institutions stabilize them into durable practices

This gives POLIS a discovery system that is process-centric, bounded-rational, socially transmissible, materially grounded, and suitable for serious experimental use.
