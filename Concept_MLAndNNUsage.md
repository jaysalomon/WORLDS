# POLIS Concept Note: Where ML And Neural Nets Belong

**Date:** 14 March 2026  
**Status:** Concept note  
**Purpose:** Clarify where machine learning and neural networks may be useful in POLIS, and where they should not replace explicit simulation design.

## 1. Core Position

POLIS should not be built as a neural-network-first simulation.

The serious backend should remain:

- explicit
- inspectable
- reproducible
- parameterized
- scientifically legible

That means the core world model, state model, resource model, institutional logic, and multi-scale causality should be rule-based and model-explicit by default.

ML and neural networks should only be used where they add clear value without destroying interpretability or reproducibility.

## 2. Where ML Is Probably Not Appropriate

ML should generally **not** be the primary mechanism for:

- core agent utility logic
- institutional rules
- resource transformation rules
- physical affordances
- state transitions that need direct scientific interpretation
- anything that must be tightly reproducible across many seeds and experiments

If those systems are replaced by opaque learned policies too early, POLIS stops being a serious experimental simulator and becomes a black box that only imitates complexity.

## 3. Where ML May Become Useful

There are several places where proper ML or NN methods could become genuinely valuable.

### 3.1 Behaviour clustering and latent pattern detection

ML is well-suited for:

- role discovery
- behaviour clustering
- anomaly detection
- regime shift detection
- cultural or institutional pattern mining

These are primarily analysis-layer uses rather than core causal mechanisms.

### 3.2 Surrogate models for expensive subsystems

If some validated process becomes computationally expensive, ML could act as a surrogate approximation.

Examples:

- crop yield approximation
- climate or hydrology surrogate
- path-cost or logistics approximator
- dense material-process feasibility estimator

This is only acceptable if:

- the original model exists
- surrogate error is measured
- the surrogate is replaceable
- validation tolerances are explicit

### 3.3 Learned perception or compression layers

ML may help compress or summarize complex local state into usable features for agents or institutions.

Examples:

- settlement condition embeddings
- regional scarcity signatures
- compressed event-history representations
- latent social-state summaries

### 3.4 Narrative and summarization layers

Small language models or related systems are plausible for:

- chronicle generation
- post-hoc summarization
- faction self-description
- replay annotations

These should stay descriptive unless a later document explicitly defines a causal role for them.

### 3.5 Adaptive heuristics under controlled constraints

There may eventually be a place for ML in tuning bounded-rationality heuristics.

Examples:

- learning better process-template search order
- learning when to exploit versus explore
- learning which signals predict institutional failure

This is more defensible than replacing the whole simulation with end-to-end learned policy networks.

## 4. Where Neural Nets Might Matter Most

If neural networks become important in POLIS, the strongest candidates are:

1. Pattern extraction from large simulation histories.
2. Surrogate modelling for validated expensive subsystems.
3. Compression of high-dimensional local context into manageable signals.
4. Narrative, interpretation, and replay tooling.

They are least defensible as replacements for:

1. ontology
2. state contract
3. resource and process rules
4. institution definitions
5. explicit causal validation

## 5. A Good Rule Of Thumb

Use explicit models for:

- causality
- experimentation
- governance
- material transformation
- social structure

Use ML for:

- compression
- approximation
- clustering
- detection
- summarization

That is the clean split.

## 6. Requirements Before Using ML In Core Simulation

If ML is ever introduced into a causally important subsystem, POLIS should require:

- a baseline explicit model
- measured error against that baseline
- reproducibility controls
- ablation tests
- interpretability sufficient for the research question
- the ability to switch the learned component off

Without those safeguards, ML use would undermine the project's scientific value.

## 7. Working Recommendation

For now:

- keep ML out of the core simulation contract
- allow it in analysis, clustering, and narrative layers
- leave open future use as surrogate modelling once explicit systems exist and can be validated

## 8. Relationship To The Spec Suite

This note is advisory only.

Its strongest downstream relevance is to:

- [08_ValidationAndExperiments.md](/abs/path/e:/Drive/WORLDS/08_ValidationAndExperiments.md)
- [09_FrontendAndPresentation.md](/abs/path/e:/Drive/WORLDS/09_FrontendAndPresentation.md)
- [10_TechnicalArchitecture.md](/abs/path/e:/Drive/WORLDS/10_TechnicalArchitecture.md)

It should not be used to override the numbered suite.

## 9. Summary

Proper ML and neural networks may become important in POLIS, but mainly as tools for analysis, compression, surrogate modelling, and descriptive layers. The core backend should remain explicit and inspectable unless a learned component can be shown to preserve the simulator's causal clarity, reproducibility, and scientific usefulness.
