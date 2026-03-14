# POLIS Spec Suite

**Version:** 0.2  
**Date:** 14 March 2026  
**Purpose:** Define the authoritative document suite for POLIS before implementation begins.

## 1. Why This Exists

POLIS is broad enough that a single master document is not a stable design format. The project needs a disciplined specification suite so ontology, state, institutions, biology, validation, presentation, and implementation architecture do not drift apart or get redefined ad hoc during coding.

This file defines:

- which documents are canonical
- what each document owns
- how the numbered documents depend on each other
- which older files are still useful but no longer authoritative

The governing rule is:

**Earlier numbered documents define vocabulary and constraints. Later numbered documents must inherit them rather than quietly replacing them.**

## 2. Current Status

The core numbered POLIS specification suite now exists from `01` through `10`.

These numbered documents are the authoritative design baseline for implementation.

## 3. Canonical Numbered Documents

### 3.1 `01_WorldModel.md`

- File: [01_WorldModel.md](/abs/path/e:/Drive/WORLDS/01_WorldModel.md)
- Owns: core ontology, entity classes, meta-categories, relation types, actor versus non-actor distinctions
- Must not own: detailed state schema, scheduling, frontend rules, engine implementation

### 3.2 `02_StateModel.md`

- File: [02_StateModel.md](/abs/path/e:/Drive/WORLDS/02_StateModel.md)
- Owns: authoritative state categories, ownership boundaries, mutation rules, event/state contract, provenance baseline
- Must not own: ontology redefinition, frontend design, product framing

### 3.3 `03_CollectiveAgency.md`

- File: [03_CollectiveAgency.md](/abs/path/e:/Drive/WORLDS/03_CollectiveAgency.md)
- Owns: coordination clusters, stable groups, collective actors, constitutions, merge and split logic, internal heterogeneity
- Must not own: base ontology, generic material physics, raw state schema

### 3.4 `04_ResourcesAndMaterials.md`

- File: [04_ResourcesAndMaterials.md](/abs/path/e:/Drive/WORLDS/04_ResourcesAndMaterials.md)
- Owns: resources, material properties, affordances, transport/storage implications, environmental constraints
- Must not own: discovery logic in full, biology in full, institutional behavior

### 3.5 `05_DiscoveryHeuristics.md`

- File: [05_DiscoveryHeuristics.md](/abs/path/e:/Drive/WORLDS/05_DiscoveryHeuristics.md)
- Owns: discovery, knowledge units, precursor chains, process schemas, diffusion, retention, institutionalization of knowledge
- Must not own: raw material ontology, collective-agency rules, frontend presentation

### 3.6 `06_BiologyAndDomestication.md`

- File: [06_BiologyAndDomestication.md](/abs/path/e:/Drive/WORLDS/06_BiologyAndDomestication.md)
- Owns: ecology, domestication, farming, life-history implications, soils, agroecosystems, disease spillover foundations
- Must not own: general discovery framework, political institutions, engine architecture

### 3.7 `07_SocietyAndInstitutions.md`

- File: [07_SocietyAndInstitutions.md](/abs/path/e:/Drive/WORLDS/07_SocietyAndInstitutions.md)
- Owns: ties, norms, institutions, organizations, legitimacy, grievance, coercion, trade, conflict, collapse pressures
- Must not own: material ontology, low-level state storage, renderer behavior

### 3.8 `08_ValidationAndExperiments.md`

- File: [08_ValidationAndExperiments.md](/abs/path/e:/Drive/WORLDS/08_ValidationAndExperiments.md)
- Owns: validity categories, experiment design, sensitivity analysis, calibration stance, reproducibility and review rules
- Must not own: engine runtime architecture, frontend rendering design

### 3.9 `09_FrontendAndPresentation.md`

- File: [09_FrontendAndPresentation.md](/abs/path/e:/Drive/WORLDS/09_FrontendAndPresentation.md)
- Owns: presentation layers, overlays, replay views, dual-mode usage, uncertainty display, public versus research-facing presentation
- Must not own: backend truth, simulation ontology, hidden causal rules

### 3.10 `10_TechnicalArchitecture.md`

- File: [10_TechnicalArchitecture.md](/abs/path/e:/Drive/WORLDS/10_TechnicalArchitecture.md)
- Owns: architectural layers, runtime boundaries, scheduling model, persistence/provenance plumbing, experiment orchestration integration, CPU/GPU strategy
- Must not own: ontology redefinition, validation philosophy, frontend semantics

## 4. Dependency Order

The numbered suite should be read and implemented in this order:

1. [01_WorldModel.md](/abs/path/e:/Drive/WORLDS/01_WorldModel.md)
2. [02_StateModel.md](/abs/path/e:/Drive/WORLDS/02_StateModel.md)
3. [03_CollectiveAgency.md](/abs/path/e:/Drive/WORLDS/03_CollectiveAgency.md)
4. [04_ResourcesAndMaterials.md](/abs/path/e:/Drive/WORLDS/04_ResourcesAndMaterials.md)
5. [05_DiscoveryHeuristics.md](/abs/path/e:/Drive/WORLDS/05_DiscoveryHeuristics.md)
6. [06_BiologyAndDomestication.md](/abs/path/e:/Drive/WORLDS/06_BiologyAndDomestication.md)
7. [07_SocietyAndInstitutions.md](/abs/path/e:/Drive/WORLDS/07_SocietyAndInstitutions.md)
8. [08_ValidationAndExperiments.md](/abs/path/e:/Drive/WORLDS/08_ValidationAndExperiments.md)
9. [09_FrontendAndPresentation.md](/abs/path/e:/Drive/WORLDS/09_FrontendAndPresentation.md)
10. [10_TechnicalArchitecture.md](/abs/path/e:/Drive/WORLDS/10_TechnicalArchitecture.md)

Rationale:

- `01` and `02` define the nouns and state contract.
- `03` through `07` define the major world process domains.
- `08` defines how claims made with POLIS will be validated.
- `09` defines how the system is shown without polluting backend truth.
- `10` defines how the whole design becomes an implementable architecture.

## 5. Legacy Drafts And Supporting Notes

These files remain useful, but they are not authoritative over the numbered suite.

### 5.1 Legacy drafts

- [Worldspec1.md](/abs/path/e:/Drive/WORLDS/Worldspec1.md)
  Status: legacy umbrella draft
- [DiscoveryHeuristics.md](/abs/path/e:/Drive/WORLDS/DiscoveryHeuristics.md)
  Status: earlier discovery draft
- [DesignSpec.md](/abs/path/e:/Drive/WORLDS/DesignSpec.md)
  Status: earlier engineering draft

These should be treated as source history, not final authority.

### 5.2 Concept notes

- [Concept_SwarmToSociety.md](/abs/path/e:/Drive/WORLDS/Concept_SwarmToSociety.md)
- [Concept_MLAndNNUsage.md](/abs/path/e:/Drive/WORLDS/Concept_MLAndNNUsage.md)
- [Concept_FrontendDesign.md](/abs/path/e:/Drive/WORLDS/Concept_FrontendDesign.md)

These are supporting notes for exploration, tradeoffs, or future optional subsystems. They may inform implementation, but they do not override numbered documents.

### 5.3 Planning notes

- [Plan_RepoStructure.md](/abs/path/e:/Drive/WORLDS/Plan_RepoStructure.md)
- [Plan_BuildOrder.md](/abs/path/e:/Drive/WORLDS/Plan_BuildOrder.md)

These should now be updated against the numbered suite rather than treated as independent planning authority.

## 6. Document Rules

Every numbered spec should continue to follow these rules:

1. Define scope clearly.
2. State dependencies on earlier numbered docs.
3. Introduce terms only when they are truly owned by that document.
4. Avoid redefining concepts already owned elsewhere.
5. Keep backend causality separate from frontend labeling.
6. State open questions explicitly.
7. Prefer stable terminology over rhetorical variety.

## 7. Pre-Build Transition Tasks

Before implementation starts in earnest, the suite should go through a final transition pass:

1. Full cross-document review for contradictions, duplicate concepts, and terminology drift.
2. Encoding and punctuation cleanup so the suite is clean ASCII or intentionally formatted Unicode.
3. Conversion of planning notes into an implementation roadmap aligned with `01` through `10`.
4. Identification of unresolved design questions that still block coding.

## 8. One-Sentence Summary

POLIS now has a complete numbered specification suite from ontology through technical architecture, and those ten documents should be treated as the canonical design baseline for implementation.
