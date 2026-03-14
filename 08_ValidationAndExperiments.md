# POLIS Validation, Experiments, And Scientific Workflow

**Version:** 0.1  
**Date:** 14 March 2026  
**Document:** 08 of the POLIS spec suite  
**Purpose:** Define how POLIS is validated, tested, calibrated, experimented with, and interpreted as a serious scientific simulation platform.

## 1. Scope

This document defines:

- the validation categories POLIS should use
- what kinds of tests and analyses are required
- how experiments should be designed and reported
- what outputs and metrics are required for interpretation
- how reproducibility, uncertainty, and calibration should be handled

This document does **not** define:

- the substantive ontology of the world model
- detailed implementation of testing infrastructure
- frontend presentation design

## 2. Dependencies

This document depends on the prior POLIS model documents:

- [01_WorldModel.md](/abs/path/e:/Drive/WORLDS/01_WorldModel.md)
- [02_StateModel.md](/abs/path/e:/Drive/WORLDS/02_StateModel.md)
- [03_CollectiveAgency.md](/abs/path/e:/Drive/WORLDS/03_CollectiveAgency.md)
- [04_ResourcesAndMaterials.md](/abs/path/e:/Drive/WORLDS/04_ResourcesAndMaterials.md)
- [05_DiscoveryHeuristics.md](/abs/path/e:/Drive/WORLDS/05_DiscoveryHeuristics.md)
- [06_BiologyAndDomestication.md](/abs/path/e:/Drive/WORLDS/06_BiologyAndDomestication.md)
- [07_SocietyAndInstitutions.md](/abs/path/e:/Drive/WORLDS/07_SocietyAndInstitutions.md)

It informs:

- `09_FrontendAndPresentation.md`
- `10_TechnicalArchitecture.md`

## 3. Guiding Principle

POLIS should be treated as a scientific instrument, not just a world generator.

That means:

- correctness is not enough
- plausibility is not enough
- interesting outcomes are not enough

POLIS must separate:

- verification
- validation
- uncertainty analysis
- calibration
- experimental design
- interpretation

If those layers blur, the simulator may look scientific without actually earning trust.

## 4. Core Validity Dimensions

POLIS should distinguish five validity dimensions.

### 4.1 Internal validity

`InternalValidity` asks whether the model’s causal structure is coherent relative to its declared assumptions and research purpose.

This includes:

- ontology coherence
- mechanism coherence
- alignment between model structure and stated questions
- absence of obvious conceptual contradictions

### 4.2 Numerical and computational validity

`NumericalValidity` asks whether algorithms and implementations behave correctly, stably, and predictably.

This includes:

- code correctness
- numerical convergence
- stability under resolution changes
- absence of obvious computational artifacts

### 4.3 Behavioural validity

`BehaviouralValidity` asks whether emergent outputs match:

- stylized facts
- known qualitative regimes
- theoretically expected patterns

This is not exact prediction. It is structured pattern comparison.

### 4.4 Experimental validity

`ExperimentalValidity` asks whether a study run on POLIS was designed and analyzed in a scientifically defensible way.

This includes:

- proper controls
- explicit hypotheses
- suitable ensemble sizes
- appropriate metrics
- clear scenario design

### 4.5 External and historical plausibility

`ExternalPlausibility` asks whether scenarios and results remain plausible given empirical evidence, comparative history, and domain knowledge.

This is not a claim that POLIS should exactly reproduce one civilization.

It is a claim that the model should not require implausible worlds to generate its core claims.

## 5. Canonical Validation Categories

These five validity dimensions should be organized through seven practical validation categories.

### 5.1 Conceptual and structural validation

Checks whether the model design itself is coherent and justified.

### 5.2 Code verification and numerical analysis

Checks whether the implementation faithfully realizes the conceptual model and behaves numerically well.

### 5.3 Behavioural and pattern validation

Checks whether the simulator reproduces important patterns, regimes, or relationships at relevant scales.

### 5.4 Sensitivity and uncertainty analysis

Checks how fragile or robust outcomes are to parameter changes, initial conditions, and stochasticity.

### 5.5 Calibration and history matching

Constrains parameter regions and model forms against empirical or stylized targets without pretending there is one unique true fit.

### 5.6 Scenario and experiment design validation

Checks whether a specific experiment is structured well enough to support meaningful inference.

### 5.7 Reproducibility and provenance

Checks whether every result can be re-run, audited, and compared under documented conditions.

## 6. Conceptual And Structural Validation

The first question is always whether the model itself is fit for the question being asked.

### 6.1 Assumption audit

Each major subsystem must maintain an explicit assumptions record.

For each subsystem, POLIS should document:

- what it represents
- what it does not represent
- why the chosen abstraction is acceptable
- what literature or reasoning supports it
- known limits and likely failure cases

### 6.2 Causal coherence review

Major revisions should undergo structured causal review:

- are new variables clearly owned?
- are new mechanisms duplicated elsewhere?
- do they violate earlier ontology or state assumptions?
- are causal claims explicit rather than implied?

### 6.3 Question-fit rule

No experiment should be interpreted outside the domain for which the model structure is adequate.

If POLIS has not been validated for a question, it may still be used exploratorily, but not as strong evidence.

## 7. Code Verification And Numerical Validity

Code and numerics must be trusted before model outputs are interpreted.

### 7.1 Verification tasks

POLIS should require at least:

- unit tests
- integration tests
- property or invariant tests
- regression tests for known scenarios

### 7.2 Invariant checks

Important invariants may include:

- no impossible negative stocks
- valid population accounting
- conservation where appropriate
- no illegal ownership states
- valid event causality

### 7.3 Deterministic baselines

For fixed seeds and controlled settings, key scenarios should be repeatable according to the reproducibility contract.

### 7.4 Numerical analysis

Important numerical checks include:

- timestep sensitivity
- spatial-resolution sensitivity
- alternative-scheme comparison where relevant
- approximation-tolerance checks for scale abstraction

No published or trusted experiment should rely on a build that has not passed this gate.

## 8. Behavioural And Pattern Validation

POLIS should be judged against patterns, not only anecdotes.

### 8.1 Stylized fact library

POLIS should maintain a library of stylized facts and theoretical patterns relevant to its intended research domains.

Examples may include:

- settlement-size distributions
- boom-bust cycles
- inequality dynamics
- institutional turnover patterns
- disease-density relationships
- conflict under scarcity

### 8.2 Multi-scale validation

Behavioural validation should be performed at multiple scales:

- micro patterns
- meso patterns
- macro patterns

The model should not match macro patterns only by producing implausible micro dynamics.

### 8.3 Regime validation

POLIS should identify whether it can generate qualitatively distinct regimes such as:

- resilient equilibrium
- oscillatory instability
- fragile growth
- collapse under stress
- divergent regional trajectories

Validation here is about the presence, conditions, and transitions of regimes, not about point prediction.

## 9. Sensitivity And Uncertainty Analysis

A serious simulator must show which conclusions are robust and which are contingent.

### 9.1 Sensitivity analysis

POLIS experiments should support:

- screening methods for large parameter sets
- local sensitivity checks
- global sensitivity analysis for important outputs

### 9.2 Sources of uncertainty

Relevant uncertainty sources include:

- parameter uncertainty
- structural uncertainty
- stochasticity
- initial-condition uncertainty
- approximation error

### 9.3 Reporting rule

No strong directional or causal claim should be reported without some explicit statement of sensitivity or robustness.

## 10. Calibration And History Matching

Calibration in POLIS should be disciplined and modest.

### 10.1 Calibration purpose

Calibration should be used to:

- rule out implausible parameter regions
- improve empirical grounding
- support domain-specific studies

It should not be used to cosmetically force the simulator into one preferred narrative.

### 10.2 History matching

POLIS should support history matching against:

- ensembles of target statistics
- stylized facts
- broad empirical ranges
- domain-specific benchmarks

The goal is to eliminate implausible regions, not find a single magic parameter set.

### 10.3 Submodel calibration

Some submodels may deserve tighter calibration than the full civilization model.

Examples:

- crop yield behavior
- water stress response
- disease spread submodel
- transport-cost scaling

### 10.4 Calibration humility rule

POLIS should be calibrated to classes of worlds and processes, not to exactly recreate one historical civilization unless the study is explicitly narrow and limited.

## 11. Scenario And Experiment Design

POLIS experiments should be formal objects, not loose exploratory play alone.

### 11.1 Experiment specification

Every serious experiment should specify:

- research question or hypothesis
- scenario family
- control cases
- treatment variables
- parameter ranges
- sampling strategy
- ensemble size
- primary metrics
- planned analysis

### 11.2 Scenario families

Important scenario families may include:

- asymmetry experiments
- institutional variation experiments
- resource-distribution experiments
- climate stress tests
- collapse and recovery experiments
- discovery-pathway experiments

### 11.3 Controls and baselines

Each experiment should define:

- baseline scenario
- treatment scenario
- optional simplified or ablated comparison model

### 11.4 Ensemble-first inference

For path-dependent stochastic systems, the primary object of inference should usually be:

- distributions of outcomes
- regime frequencies
- quantile trajectories

not a single favored run.

## 12. Ablation And Stress Testing

POLIS should make it easy to test whether mechanisms actually matter.

### 12.1 Ablation studies

Important ablations include disabling or simplifying:

- institutional adaptation
- discovery system
- trade system
- biological disease pressure
- collective agency transitions

This helps determine which mechanisms are necessary for which observed outcomes.

### 12.2 Stress tests

Important stress tests include:

- drought
- epidemic shock
- trade disruption
- invasion pressure
- elite competition shock
- infrastructure failure

Stress tests should be applied across ensembles, not only to one narrative run.

## 13. Metrics And Outputs

POLIS must emit outputs that support real analysis.

### 13.1 Provenance metadata

Every run should record:

- model version
- schema version
- configuration
- parameter vector
- initial conditions
- seed information
- enabled subsystems
- relevant numerical settings

### 13.2 Multi-scale outputs

Outputs should include:

- micro-level samples or distributions
- meso-level structure and network metrics
- macro-level time series

### 13.3 Ensemble outputs

For experiment families, POLIS should support:

- outcome distributions
- quantiles
- regime counts
- sensitivity summaries
- calibration distances

### 13.4 Pattern metrics

The model should provide explicit metrics for:

- stylized fact comparison
- regime detection
- cross-scale consistency
- divergence across treatments

## 14. Reproducibility, Repeatability, And Replicability

POLIS must make scientific claims auditable.

### 14.1 Repeatability

Same code, same configuration, same seeds, same controlled environment should reproduce the same run within the engine’s reproducibility guarantee.

### 14.2 Reproducibility

A documented experiment should be regenerable by others using the recorded model, parameters, and workflow.

### 14.3 Replicability

Where feasible, key claims should be checkable by:

- independent analyses
- alternative implementations
- simplified comparison models

### 14.4 Provenance bundle

Each serious experiment should have a machine-readable bundle containing:

- model reference
- configuration
- scenario definition
- seeds
- run metadata
- analysis specification

This is the minimum standard for auditable results.

## 15. Review And Release Rules

POLIS should distinguish exploratory use from research-grade use.

### 15.1 Exploratory mode

Allowed for:

- hypothesis generation
- intuition building
- rapid scenario probing

### 15.2 Research-grade mode

Requires at minimum:

- verified build
- explicit experiment specification
- adequate ensemble size
- defined metrics
- provenance bundle
- reported uncertainty or sensitivity

### 15.3 Review checklist

Before accepting a result as research-grade, POLIS should be able to answer:

- was the relevant model portion conceptually validated?
- did the build pass verification?
- are outputs from ensembles rather than cherry-picked runs?
- was sensitivity assessed at least at screening level?
- are claims limited to validated domains?
- is provenance sufficient for rerun and audit?

## 16. Risks And Anti-Patterns

The following must be explicitly avoided.

### 16.1 Verification-validation confusion

Bug-free code does not imply a good model.

### 16.2 Single-run storytelling

Interesting individual trajectories should not be treated as evidence of typical behavior.

### 16.3 Overfitting to one history

Calibrating to one famous case and pretending that proves general validity.

### 16.4 Uncertainty blindness

Reporting neat plots without variability, robustness, or sensitivity context.

### 16.5 Hidden numerical artifacts

Mistaking timestep, discretization, or approximation artifacts for substantive findings.

### 16.6 Pattern vagueness

Claiming outputs “look realistic” without explicit metrics or target patterns.

### 16.7 Provenance gaps

Publishing outcomes that cannot be re-run or audited.

### 16.8 Use mismatch

Applying POLIS to questions it has not been validated to answer.

## 17. Open Questions

These questions remain for later refinement:

- Which stylized-fact library should be treated as the initial validation core for v1?
- What minimum ensemble sizes are acceptable for different classes of claim?
- How much automated history matching is justified before the workflow becomes too expensive for regular use?
- Which cross-implementation replication targets are realistic for early milestones?

These do not block the framework, but they matter for later operationalization.

## 18. Summary

POLIS should adopt a layered validation and experimentation framework in which:

- conceptual validity checks model structure
- verification checks code and numerics
- behavioural validation checks multi-scale patterns
- sensitivity and uncertainty analysis qualify conclusions
- calibration rules out implausible parameter regions
- experiments are formally specified and ensemble-based
- provenance makes all claims auditable

This is what will allow POLIS to function as a serious experimental platform instead of only a sophisticated sandbox.
