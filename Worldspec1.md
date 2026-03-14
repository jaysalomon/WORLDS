# POLIS Design Spec

**Version:** 1.2  
**Date:** 14 March 2026  
**Project:** Scientific Civilisation Sandbox  
**Working title:** POLIS, a scientifically grounded, Vulkan-accelerated civilisation and emergence simulator with a game-readable front end.

**Status:** Legacy umbrella draft. The authoritative design baseline is the numbered suite indexed by [SpecSuite.md](/abs/path/e:/Drive/WORLDS/SpecSuite.md). This file is retained as source history and may contain superseded assumptions.

## 1. Purpose

Build a simulation platform that looks approachable like WorldBox, but is architected as a serious experimental system for studying:

- emergence of social organisation
- resource dynamics
- technological development
- conflict and cooperation
- intelligence/resource asymmetry
- environmental pressure and adaptation
- path dependence and civilisational collapse and recovery

**Core principle:** cute front end, rigorous back end.

POLIS is not just a game. It is a large-scale agent-based modelling and dynamical systems platform with visualisation, instrumentation, reproducible scenario tooling, and batch experimentation.

The foundational modelling idea is:

**A society should be simulated at the scale at which meaningful decisions are actually being made.**

That means:

- weakly coordinated populations are simulated as individual agents
- strongly coordinated populations can transition into collective decision units
- some processes remain individual even inside a coordinated group
- groups can fragment back into individuals when cohesion fails

This is not merely a performance trick. It is a modelling claim about how collective agency emerges at scale.

## 2. Core Research Question

How do resource distribution, cognitive capacity, environmental volatility, social memory, institutional coordination, and incentive structure interact to produce distinct developmental trajectories in simulated societies?

Example prompts:

- Does high intelligence compensate for poor starting resources?
- Does resource abundance reduce or increase innovation pressure?
- At what threshold does inequality destabilise social coherence?
- Under what conditions do institutions emerge spontaneously?
- When does warfare become adaptive versus maladaptive?
- What environmental noise level produces the richest long-term complexity?
- When does a coordinated group outperform a smarter but fragmented population?

## 3. Design Philosophy

### 3.1 Scientific goals

The simulator must be:

- parameterised
- reproducible
- inspectable
- batch-runnable
- statistically analysable
- visually legible

### 3.2 Product goals

The simulator should also be:

- compelling to watch
- understandable to non-specialists
- capable of generating content worth sharing
- modular enough to support both research mode and sandbox mode

### 3.3 Technical philosophy

Use:

- Vulkan compute, not CUDA
- data-oriented design
- GPU-first simulation where parallel
- CPU orchestration where sparse or branch-heavy
- deterministic stepping where possible
- hybrid fidelity, not brute-force realism everywhere

### 3.4 Non-negotiable principles

1. Every major mechanic must expose measurable outputs.
2. Every scenario must be serialisable and reproducible.
3. GPU acceleration must serve the model, not dictate bad science.
4. Visual charm must never replace instrumentation.
5. Start simple, then add coupling only after validation.
6. Prefer composable subsystems over one giant civilisation equation.
7. Role labels are analytic outputs, not decision rules.
8. Collective agents are allowed only when their approximation error is measured and bounded.

## 4. Scope

### 4.1 In scope

- agent-based population simulation
- spatial world model
- ecological and resource systems
- diffusion, transport, and environmental fields
- social networks and institutions
- combat and conflict
- technology and culture accumulation
- disease, stress, and morale dynamics
- dynamic scale transition from individuals to coordinated groups
- scenario editor
- replay, metrics, and export pipeline
- visual front end

### 4.2 Out of scope for v1

- full language simulation
- full 3D physics
- detailed individual biomechanics
- exact economics of modern finance
- photorealistic rendering
- true consciousness modelling
- perfect anthropological realism

This system should be useful, not encyclopaedically mad.

## 5. Research Framing

### 5.1 Modelling paradigm

The system combines:

1. Agent-based modelling
2. Cell or continuous spatial field simulation
3. Differential equation solvers for field processes
4. Network dynamics for social structure
5. Event-driven discrete transitions
6. Monte Carlo sampling for stochasticity
7. Regime-switching between individual and collective agency

### 5.2 Multi-scale structure

The simulation operates across three scales.

**Micro: individual agents**

- energy
- health
- skills
- memory
- preferences
- threat perception
- social ties
- fertility
- ideology and culture markers

**Meso: groups and settlements**

- cohesion
- norms
- leadership
- institutions
- shared storage
- defence
- specialisation
- trade role
- collective grievance
- coordination quality

**Macro: civilisation-scale**

- territory
- total productivity
- inequality
- innovation rate
- war frequency
- resilience
- complexity index
- collapse probability

## 6. Scientific Model Architecture

### 6.1 World substrate

Represent the world as a 2D tiled map initially, with optional later migration to sparse continuous regions.

Each cell contains:

- elevation
- biome
- fertility
- water availability
- mineral abundance
- temperature
- pollution
- carrying capacity
- hazard level
- infrastructure level
- ownership and control

Why 2D first:

- easier debugging
- better readability
- better scaling
- simpler GPU kernels
- easier replay and batch analysis

### 6.2 Agent model

Each agent is a compressed behavioural unit, not a fully simulated human.

#### 6.2.1 Agent state vector

Suggested fields:

**Identity**

- agent_id
- group_id
- settlement_id
- family_id

**Demography**

- age
- sex and reproductive status
- lifespan expectancy
- offspring count

**Physical state**

- energy
- health
- injury
- hunger
- fatigue
- disease load

**Cognitive and social state**

- intelligence
- planning horizon
- conformity
- aggression
- trust
- fear
- curiosity
- memory depth
- learning rate

**Economic state**

- inventory
- productivity coefficients
- tool access
- mobility cost

**Cultural and institutional state**

- norm adherence
- ideology vector
- innovation adoption tendency
- obedience versus autonomy
- in-group loyalty
- out-group hostility

**Network state**

- local kin ties
- alliance degree
- prestige
- reputation
- grievance

#### 6.2.2 Utility model

Each decision unit, whether individual or collective, evaluates a common utility form:

```math
U = w_1 S + w_2 R + w_3 P + w_4 E + w_5 C
```

Where all terms are normalised to `[0, 1]`:

- `S`: survival, including energy, health, and safety
- `R`: reproduction, including fertility success and offspring viability
- `P`: power, including control leverage, extraction rights, and coercive influence
- `E`: esteem, including prestige, reciprocal ties, and social legitimacy
- `C`: curiosity, including novelty seeking and learning value

Weights are mutable scenario parameters and may also vary across populations.

Intelligence does not act as a vague buff. It changes:

- action evaluation quality
- planning horizon
- model error under uncertainty
- adaptation speed

Lower-intelligence agents use cheaper heuristics that approximate the same utility space more poorly.

#### 6.2.3 Minimal behavioural loop

Each active decision unit executes:

1. perceive local environment
2. update internal state
3. estimate action values
4. execute action
5. apply consequences
6. learn and adapt
7. write outputs to metrics buffers

### 6.3 Decision-unit transition: individual to group agency

This is the core regime switch of the system.

#### 6.3.1 Motivation

Below a certain coordination threshold, meaningful decisions are made by individuals. Above that threshold, meaningful decisions may be made by the group as a coherent social actor.

The transition from many agents to one collective agent is therefore both:

- a scientific claim about coordinated behaviour at scale
- a computational strategy for large populations

#### 6.3.2 Merge conditions

A set of agents may transition into a `GroupEntity` when all of the following are true:

- membership is stable over a configured time window
- cohesion exceeds `theta_c`
- internal trust density exceeds `theta_t`
- policy alignment exceeds `theta_a`
- spatial variance remains below `theta_r`
- size exceeds `N_min`

This must be evaluated over rolling summaries, not a single noisy tick.

#### 6.3.3 Group-level decision domain

Once merged, the group becomes the primary decision unit for:

- migration and relocation
- alliance and hostility stance
- organised defence
- organised raids and warfare
- resource extraction policy
- storage allocation policy
- labour allocation policy
- norm enforcement intensity
- infrastructure and institutional investments

These are actions where coordinated intent matters more than isolated choices.

#### 6.3.4 Retained individual processes

Even inside a merged group, some processes remain individual or household-level:

- reproduction
- ageing
- mortality
- injury and disease burden
- individual loyalty and grievance
- local consumption
- trait inheritance and drift
- exceptional defection or dissent
- rare innovation events

The backend should therefore treat a merged group as:

- one collective decision unit
- many retained biological and social substates

#### 6.3.5 Internal distribution model

A group decision is not applied magically. It must be pushed back down into members through explicit allocation rules.

Examples:

- group migration becomes individual movement targets with compliance probabilities
- production allocation becomes household work quotas plus elite extraction rules
- organised warfare becomes combatant assignment plus support burden on non-combatants
- norm enforcement becomes local conformity pressure and grievance updates

This distribution layer is where inequality, coercion, and defection become measurable.

#### 6.3.6 Split conditions

A merged group re-expands to active individuals when any of the following hold:

- cohesion drops below `theta_split`
- internal inequality exceeds a configured threshold
- faction divergence exceeds a configured threshold
- leadership legitimacy collapses
- spatial spread exceeds operational bounds
- a shock event occurs, such as raid, famine, epidemic, or migration pressure

Split conditions should include hysteresis to avoid merge-split thrashing.

#### 6.3.7 Approximation contract

Collective agency is only acceptable if approximation error is measured.

For small reference worlds, the engine must compare:

- full individual simulation
- merged collective simulation

The following must remain within configurable tolerance bands:

- population change
- resource consumption
- conflict frequency
- migration behaviour
- inequality trajectory
- institutional emergence timing

If the error exceeds tolerance, the group must either split or run at higher fidelity.

### 6.4 Environment and field equations

Not every system needs a PDE, but these families are appropriate.

#### A. Resource regeneration

For renewable local resources:

```math
\frac{dR}{dt} = rR\left(1 - \frac{R}{K}\right) - H
```

Where:

- `R` is resource stock
- `r` is regeneration rate
- `K` is carrying capacity
- `H` is harvest pressure

Use for forest biomass, fish stocks, grazing quality, and local fertility recovery.

#### B. Diffusion fields

For water, pollution, disease pressure, and information saturation:

```math
\frac{\partial \phi}{\partial t} = D \nabla^2 \phi + S - L
```

Where:

- `phi` is the field quantity
- `D` is diffusion coefficient
- `S` is sources
- `L` is sinks or losses

#### C. Population-resource coupling

For ecological pressure:

```math
\frac{dN}{dt} = N(b - d - \alpha P)
```

Or Lotka-Volterra variants where relevant.

#### D. Epidemiology

At settlement or regional scale:

```math
\frac{dS}{dt} = -\beta SI
\frac{dI}{dt} = \beta SI - \gamma I
\frac{dR}{dt} = \gamma I
```

Use compartmental approximations where full per-agent disease simulation is not justified.

#### E. Opinion and belief dynamics

For ideological drift or factional alignment:

```math
x_i(t+1) = x_i(t) + \sum_j w_{ij}(x_j - x_i) + \eta_i
```

Bounded-confidence variants may be substituted later.

#### F. Innovation accumulation

Technology stock:

```math
\frac{dT}{dt} = \alpha C^\mu E^\nu S^\lambda - \delta T
```

Where:

- `C` is cognitive capital
- `E` is surplus energy and material capacity
- `S` is social connectivity and specialisation
- `delta` is knowledge loss

#### G. Conflict pressure

Conflict propensity can be modelled as a hazard:

```math
W = aG + bR_c + cI - dM
```

Where:

- `G` is grievance
- `R_c` is resource competition
- `I` is ideological distance
- `M` is mutual trade or interdependence

### 6.5 Social and institutional emergence

Institutions should emerge from system pressures, not from hand-scripted magic buildings.

#### 6.5.1 Institution emergence candidates

When a settlement or group crosses thresholds in:

- population
- stored surplus
- conflict frequency
- trade complexity
- trust density
- specialisation diversity
- coordination burden

It may generate structures such as:

- leadership hierarchy
- granary or logistics system
- militia
- council
- priesthood
- market
- taxation
- record-keeping

#### 6.5.2 Formalisation

```math
P(I_k) = \sigma(\alpha_1 N + \alpha_2 S + \alpha_3 D + \alpha_4 C - \alpha_5 X)
```

Where:

- `N` is population
- `S` is surplus
- `D` is division of labour
- `C` is coordination pressure
- `X` is instability or noise
- `sigma` is a sigmoid

#### 6.5.3 Role classification

Emergent roles such as military cohort, watch, traders, or priesthood should be detected from observed behaviour clusters.

These labels are:

- analytic
- optional in the UI
- useful for logging and narrative summaries

They are not primary causal rules. The simulation should run without requiring the label to exist.

### 6.6 Intelligence modelling

Intelligence is a major experimental axis and should be decomposed into components:

- perception bandwidth
- planning horizon
- causal inference quality
- learning rate
- abstraction capacity
- innovation tendency
- coordination ability
- deception and social manipulation ability

A high-intelligence, low-resource society should differ from a low-intelligence, high-resource one in:

- adaptation speed
- tool diversity
- institutional complexity
- strategic warfare
- resilience to shocks
- exploitation efficiency

### 6.7 Resource asymmetry experiments

Scenario family A: intelligence-resource asymmetry.

Create multiple populations with different initial conditions.

| Population | Resources | Intelligence | Fertility | Mobility | Threat level |
| --- | --- | --- | --- | --- | --- |
| A | High | Low | Medium | Low | Low |
| B | Low | High | Medium | Medium | Low |

Outputs:

- survival time
- population growth
- tech level
- territorial expansion
- warfare initiation rate
- cooperation density
- collapse frequency
- inequality
- complexity index

Requirement:

Run hundreds or thousands of seeds, not one anecdotal showcase run.

## 7. Hardware And Runtime Architecture

### 7.1 Why Vulkan

Vulkan compute is appropriate because it is:

- cross-platform
- explicit
- suitable for headless compute workloads
- capable of compute-only pipelines
- compatible with graphics and simulation in one API
- more portable long term than CUDA-only tooling

### 7.2 Compute architecture split

**GPU responsibilities**

- field updates
- diffusion
- resource map updates
- parallel utility evaluation
- path cost precomputation
- combat resolution batches
- neighbour statistics
- map rendering
- reduction passes for metrics

**CPU responsibilities**

- scenario orchestration
- save and load
- event scheduling
- sparse high-level decisions
- UI and state management
- experiment batch control
- debug logging
- data export

Pure GPU everything is seductive and idiotic. Use GPU where arithmetic density justifies it.

### 7.3 Data layout

Use structure of arrays, not array of structures, for hot simulation kernels.

Good:

```text
health[N]
energy[N]
group_id[N]
x[N]
y[N]
trust[N]
fear[N]
inventory_food[N]
```

Avoid:

```cpp
struct Agent {
    float health;
    float energy;
};
```

### 7.4 Execution model

Use multiple tick rates.

**Fast tick**

- movement
- consumption
- local sensing
- simple interactions
- utility evaluation for active decision units

**Medium tick**

- production
- trade
- local conflict
- disease updates
- group distribution passes

**Slow tick**

- institutions
- tech diffusion
- demographic change
- ideology drift
- merge and split evaluation
- classifier and summary metrics

### 7.5 Reproducibility contract

Every run must store:

- scenario definition
- parameter set
- RNG seed
- engine version and hash
- metrics schema version
- merge and split event log
- precision mode
- build target and relevant backend settings

A run should be reproducible from:

`scenario + seed + build version + backend settings`

### 7.6 Collective-agent runtime contract

When a group is merged, the runtime must still preserve:

- member count
- demographic distribution
- inequality summaries
- stored grievance distribution
- health and fertility summaries
- dissent probability summaries

This prevents the collective layer from becoming a black box.

## 8. Experimental Design Framework

### 8.1 Scenario definition format

Use human-readable config such as YAML.

```yaml
world:
  width: 1024
  height: 1024
  biome_seed: 12345
  climate_variability: 0.15

factions:
  - name: ResourceRich
    population: 500
    intelligence_mean: 0.35
    starting_region: west_basin
    initial_food: 100000
    initial_minerals: 50000

  - name: IntelligenceRich
    population: 500
    intelligence_mean: 0.78
    starting_region: east_hills
    initial_food: 20000
    initial_minerals: 12000

collective_agency:
  enabled: true
  merge:
    cohesion: 0.65
    trust_density: 0.60
    policy_alignment: 0.70
    min_size: 32
  split:
    cohesion: 0.45
    inequality: 0.55
    faction_divergence: 0.40
  retained_individual_processes:
    - reproduction
    - mortality
    - disease
    - grievance
    - defection

simulation:
  steps: 200000
  output_interval: 100
  seed_batch: 256
```

### 8.2 Research workflow

**Stage 1: toy models**

Validate:

- resource growth
- diffusion
- basic movement
- population growth
- local combat

**Stage 2: asymmetry experiments**

Run controlled comparisons:

- high-resource versus high-intelligence
- high-cohesion versus high-aggression
- high-innovation versus high-stability

**Stage 3: collective agency**

Add:

- merge and split transitions
- group policy decisions
- retained individual sub-processes
- approximation error checks

**Stage 4: meso structures**

Add:

- trade
- institutions
- cultural transmission
- leadership

**Stage 5: full ecology and validation**

Add:

- disease
- climate shocks
- migration waves
- cascade failures
- calibration and robustness analysis

### 8.3 Statistical outputs

Track at minimum:

**Population metrics**

- total population
- mortality
- fertility
- lifespan

**Economic metrics**

- per-capita production
- surplus
- storage
- trade volume

**Social metrics**

- network clustering
- inequality
- trust
- factional fragmentation
- institution count
- coordination density
- merge and split frequency

**Conflict metrics**

- skirmish frequency
- casualties
- territorial turnover
- grievance accumulation

**Cognitive and technical metrics**

- tech stock
- innovation rate
- diffusion speed
- specialisation diversity

**Ecological metrics**

- average fertility
- resource depletion
- pollution or disease burden
- carrying-capacity utilisation

**Macro derived metrics**

- resilience index
- collapse index
- complexity index
- adaptive efficiency

## 9. Visual Front End

### 9.1 Front-end goals

The front end should make emergent structure visible without pretending to be the science itself.

Required visual modes:

- terrain and biome view
- ownership and political map
- resource density heatmap
- trade flow overlay
- conflict heatmap
- trust and cohesion map
- disease and pollution map
- tech and infrastructure map
- replay timeline
- collective-boundary overlay

### 9.2 Visual style

Recommended style:

- stylised low-poly or pixel hybrid
- clean iconography
- very readable overlays
- not photoreal
- toy diorama with scientific instrumentation

The front end can be playful. The backend cannot.

### 9.3 Dual-mode UI

**Sandbox mode**

- drag hazards
- spawn populations
- tweak sliders
- watch chaos

**Research mode**

- batch runs
- fixed seeds
- parameter sweeps
- CSV or Parquet export
- graphs and significance summaries
- event log inspection

## 10. Validation Strategy

### 10.1 Model validity types

**Internal validity**

Do equations behave correctly under controlled conditions?

**Computational validity**

Do GPU and CPU paths agree within tolerances?

**Behavioural validity**

Do aggregate patterns make qualitative sense?

**Experimental validity**

Do scenario manipulations produce stable, interpretable shifts?

### 10.2 Validation tasks

- unit tests for each equation family
- conservation checks where appropriate
- seed repeatability tests
- sensitivity analysis
- ablation studies
- CPU reference implementation for small worlds
- numerical stability tests for timestep choices
- merged versus unmerged comparison tests
- split hysteresis tests

## 11. Numerical Methods

### 11.1 Recommended methods

- explicit Euler for simple toy systems
- semi-implicit or RK2 or RK4 where instability appears
- finite differences for diffusion fields
- stochastic sampling for discrete event hazards
- graph-based propagation for social transmission

### 11.2 Stability constraints

Document:

- timestep restrictions
- field clamping rules
- conservation rules
- noise injection policies
- precision choices, with fp32 as default
- tolerance bands for merged collective approximation

Use fp32 for most simulation. Reach for fp64 only where error genuinely matters.

## 12. Suggested Software Architecture

### 12.1 Engine layers

**Layer A: core simulation**

- state containers
- stepping logic
- equations
- RNG
- metrics

**Layer B: collective-agency layer**

- merge detection
- split detection
- group state builders
- internal distribution passes
- approximation monitors

**Layer C: compute backend**

- Vulkan device abstraction
- buffers and images
- pipeline management
- dispatch scheduling
- synchronisation
- profiling

**Layer D: world systems**

- ecology
- demography
- social
- conflict
- institutions
- technology

**Layer E: experiment system**

- scenario loader
- seed batching
- sweep manager
- output writer

**Layer F: presentation**

- renderer
- overlays
- replay controls
- inspector tools

### 12.2 Language stack

Reasonable choice:

- Rust or C++ core engine
- Vulkan compute and render backend
- GLSL or HLSL to SPIR-V shader pipeline
- Python analysis notebooks and parameter fitting
- Arrow or Parquet for batch data output

Rust fits this well if the priority is safer systems architecture. C++ also works if the team wants tighter low-level control and accepts the cost.

## 13. Performance Targets

### 13.1 v1 target

Interactive simulation of:

- 100k to 1M simplified agents
- `512^2` to `2048^2` world fields
- multiple active scalar fields
- replayable at useful speeds

### 13.2 Batch target

Headless scenario sweeps:

- hundreds to thousands of runs
- compressed state snapshots
- summary metrics per run
- deterministic experiment manifests

### 13.3 Performance strategy

- level of detail for cognition
- collective agency for coordinated populations
- sparse updates for inactive regions
- multi-rate stepping
- GPU reductions instead of CPU readbacks
- minimal synchronisation points

## 14. Initial Milestone Plan

**Milestone 0: technical prototype**

- Vulkan compute setup
- tiled world buffers
- simple render output
- deterministic RNG
- performance instrumentation

**Milestone 1: ecology sandbox**

- terrain
- fertility
- resource growth
- diffusion fields
- harvesting agents

**Milestone 2: social agents**

- needs
- movement
- local ties
- basic conflict and cooperation
- utility-based decision loop

**Milestone 3: collective agency**

- merge and split logic
- group decision domains
- retained individual sub-processes
- approximation validation harness

**Milestone 4: institutions and technology**

- division of labour
- specialisation
- institution emergence
- innovation system

**Milestone 5: publishable simulator**

- scenario pack
- visual replay
- results dashboard
- documented validation

## 15. Key Risks

### 15.1 Scientific risks

- false realism
- too many arbitrary parameters
- emergent behaviour that is actually a coding artefact
- seductive visuals masking weak inference
- collective-agent approximation distorting outcomes

### 15.2 Technical risks

- overcomplicated GPU pipeline too early
- synchronisation bottlenecks
- branch divergence from over-detailed agents
- impossible debugging in fully coupled systems
- merge and split churn causing unstable runtime behaviour

### 15.3 Product risks

- neither good enough as a game nor rigorous enough as a simulator

## 16. Recommended Next Research Questions

### A. Adjacent systems

- agent-based modelling frameworks
- artificial life platforms
- strategy-game simulation architectures
- ecological and epidemiological field solvers
- GPU ABM literature
- historical simulation design

### B. Mathematical design questions

- bounded rationality models for utility estimation
- conflict hazard models
- network formation dynamics
- inequality and cohesion relationships
- collapse indicators in complex systems
- thresholds for coordinated collective action

### C. Vulkan-specific engineering questions

- best practices for compute-only simulation pipelines
- buffer layouts for very large ABMs
- subgroup operations for reductions and statistics
- async compute and render separation
- profiling and debugging workflow
- headless batch execution architecture

## 17. One-Sentence Product Pitch

POLIS is a Vulkan-accelerated scientific civilisation sandbox where a playful, readable front end sits on top of a serious multi-scale simulation in which individuals, groups, and institutions emerge, coordinate, fragment, and adapt under measurable experimental conditions.
