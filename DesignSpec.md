# Design Specification for POLIS

## A Vulkan-Accelerated Scientific Civilization Sandbox

**Status:** Legacy technical draft. The authoritative design baseline is the numbered suite indexed by [SpecSuite.md](/abs/path/e:/Drive/WORLDS/SpecSuite.md), especially [10_TechnicalArchitecture.md](/abs/path/e:/Drive/WORLDS/10_TechnicalArchitecture.md). This file is retained as source history and may contain superseded GPU-first assumptions.

## Architectural Foundation and the Evolution of Agent-Based Modeling

The development of the POLIS architecture represents a fundamental paradigm shift in the construction of large-scale Agent-Based Modeling (ABM) environments and scientific civilization sandboxes.

Historically, simulating complex adaptive systems has been constrained by the sequential processing limitations of central processing units (CPUs). Traditional ABM platforms exhibit severe performance bottlenecks when scaling the number of autonomous agents, which inherently restricts the depth of parameter exploration, increases development time, and prevents the simulation of highly dense, interdependent populations.

Recent advances in distributed simulation engines, such as BioDynaMo and TeraAgent, have demonstrated the necessity of scaling computation across multiple servers to handle billions of agents by utilizing shared-memory parallelism and optimized grid algorithms for neighbor searching.

POLIS bypasses traditional CPU-bound distributed network latencies by migrating the entirety of the core computational workload directly to the Graphics Processing Unit (GPU) via the Vulkan Application Programming Interface (API).

By decoupling simulation logic from the CPU, POLIS achieves real-time scalability, capable of rendering and simulating hundreds of thousands of complex, state-driven agents simultaneously. The sandbox operates on multiple abstraction layers, integrating micro-level particle physics with macro-level sociological phenomena:

1. Foundational layer: GLSL compute shaders for N-body interactions, spatial dynamics, and flocking kinematics.
2. Intermediate layer: strategic decision-making through multi-objective utility functions driven by stochastic differential equations governing emotion, intrinsic motivation, and evolutionary game theory.
3. Top layer: unsupervised temporal clustering for behavior classification and Small Language Models (SLMs) for dynamic, human-readable historical narratives.

The orchestration of these layers requires precise memory management, sophisticated parallel reduction algorithms, and seamless interoperability between hardware-accelerated kernels and high-level scripting environments such as Python and Lua.

## Vulkan Compute Pipeline and Explicit Memory Synchronization

The transition from legacy APIs such as OpenGL or DirectX 11 to Vulkan necessitates a manual, explicit approach to memory handling and execution synchronization. Unlike legacy systems that provide substantial driver-level abstraction, Vulkan requires the application to explicitly allocate memory heaps, manage buffer lifetimes, and define execution barriers.

Traditional GPU particle systems often relied on Framebuffer Object (FBO) ping-pong techniques, which limited data complexity due to texture memory format constraints.

To overcome these limitations, POLIS uses Shader Storage Buffer Objects (SSBOs), structured according to std140 and std430 memory layout rules. These layouts define exact alignment and padding requirements for host-device data exchange.

By defining agent properties in continuous SSBO memory, compute shaders can perform arbitrary read and write operations over complex structures containing:

- positional vectors
- velocity vectors
- multidimensional utility-weight arrays

Because Vulkan strictly separates compute and graphics stages, POLIS instantiates two distinct pipeline objects. To maximize throughput, it employs double buffering:

- one SSBO for current state
- one SSBO for next state

Each simulation tick:

1. Compute reads current state.
2. Compute integrates kinematic and psychological updates.
3. Compute writes next state.
4. A memory barrier ensures global visibility before graphics consumes the updated buffer.

This removes the CPU from the active simulation loop after initialization.

## GPU-Accelerated Flocking, Kinematics, and Spatial Dynamics

Agent locomotion in POLIS is based on flocking systems pioneered by Craig Reynolds. Each agent state is represented as:

$$P = \{x, v, m\}$$

where $x$ is position, $v$ velocity, and $m$ mass.

Because continuous dynamics across massive populations are intractable in real-time, POLIS uses Euler numerical integration. For sufficiently small time steps $\Delta t$, velocity and acceleration are approximated as constant per step:

$$p = p + v \cdot \Delta t$$
$$v = v + a \cdot \Delta t$$

Core steering behaviors:

- alignment: match neighbor velocity
- cohesion: move toward local center of mass
- separation: apply repulsion to avoid crowding

Naive all-pairs interaction yields $\mathcal{O}(N^2)$ complexity. POLIS mitigates this with grid-based spatial partitioning:

- agents are binned into regular cells
- neighbor checks are restricted to local and adjacent cells

This sharply reduces per-invocation reads.

### GLSL Pseudocode: Kinematics and Flocking Kernel

```glsl
#version 450
layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

// Define the precise memory layout for the agent state
struct AgentState {
    vec4 position_mass;   // xyz: world position, w: physical mass
    vec4 velocity;        // xyz: velocity vector, w: emotional_valence
    vec4 utility_weights; // x: alignment, y: cohesion, z: separation, w: prestige_score
};

// SSBO definitions using std430 alignment for optimized storage
layout(std430, binding = 0) readonly buffer CurrentState {
    AgentState agents_in[];
};

layout(std430, binding = 1) writeonly buffer NextState {
    AgentState agents_out[];
};

// Uniform parameters governing the physical simulation bounds
layout(std140, binding = 2) uniform SimParams {
    float dt;
    float view_radius;
    float max_speed;
    float max_force;
    uint num_agents;
} params;

// Mathematical utility function to clamp vector magnitudes
vec3 limit_vector(vec3 v, float max_val) {
    float length_sq = dot(v, v);
    if (length_sq > max_val * max_val) {
        return normalize(v) * max_val;
    }
    return v;
}

void main() {
    uint id = gl_GlobalInvocationID.x;
    if (id >= params.num_agents) return;

    AgentState self = agents_in[id];
    vec3 pos = self.position_mass.xyz;
    vec3 vel = self.velocity.xyz;

    vec3 center_of_mass = vec3(0.0);
    vec3 separation_force = vec3(0.0);
    vec3 alignment_force = vec3(0.0);
    int neighbor_count = 0;

    // NOTE: Spatial grid uniform buffer lookup omitted for pseudocode brevity.
    // The loop iterates over adjacent grid cells rather than the entire population.
    for (uint i = 0; i < params.num_agents; i++) {
        if (i == id) continue;

        vec3 other_pos = agents_in[i].position_mass.xyz;
        vec3 offset = pos - other_pos;
        float dist_sq = dot(offset, offset);

        if (dist_sq > 0.0 && dist_sq < (params.view_radius * params.view_radius)) {
            float dist = sqrt(dist_sq);
            center_of_mass += other_pos;
            alignment_force += agents_in[i].velocity.xyz;
            // Repulsion scales inversely with distance
            separation_force += normalize(offset) / dist;
            neighbor_count++;
        }
    }

    vec3 acceleration = vec3(0.0);
    if (neighbor_count > 0) {
        float f_count = float(neighbor_count);
        center_of_mass /= f_count;
        alignment_force /= f_count;

        // Calculate desired steering vectors
        vec3 cohesion_steer = limit_vector(center_of_mass - pos, params.max_speed) - vel;
        vec3 align_steer = limit_vector(alignment_force, params.max_speed) - vel;
        vec3 sep_steer = limit_vector(separation_force, params.max_speed) - vel;

        // Force application modulated by the agent's internal utility state
        acceleration += limit_vector(cohesion_steer, params.max_force) * self.utility_weights.y;
        acceleration += limit_vector(align_steer, params.max_force) * self.utility_weights.x;
        acceleration += limit_vector(sep_steer, params.max_force) * self.utility_weights.z;
    }

    // Euler integration for continuous motion
    vec3 next_vel = limit_vector(vel + acceleration * params.dt, params.max_speed);
    vec3 next_pos = pos + next_vel * params.dt;

    // Write persistent data to the output buffer
    agents_out[id].position_mass.xyz = next_pos;
    agents_out[id].position_mass.w = self.position_mass.w;
    agents_out[id].velocity.xyz = next_vel;
    agents_out[id].velocity.w = self.velocity.w;
    agents_out[id].utility_weights = self.utility_weights;
}
```

## Parallel Reduction for Spatial Variance and Macro-Aggregation

Localized flocking captures micro-interactions, but macro-level understanding requires global aggregation, such as:

- global center of mass
- spatial variance of specific cultural factions

Single-pass global summation is unsafe on GPU architectures due to synchronization constraints and potential deadlock scenarios.

POLIS therefore uses tree-based parallel reduction:

1. Thread blocks compute partial sums in shared memory.
2. Partial sums are emitted to global memory.
3. Subsequent kernel launches continue logarithmic reduction.
4. Final scalar outputs are produced.

For grouped operations (such as role-specific variance), POLIS uses a multipass hash-based groupby aggregation algorithm (MP-EGA). CPU streams overlap transfer and compute to hide latency.

Spatial cluster distributions are analyzed against Wigner's semicircle distribution:

$$f(x) = \frac{2}{\pi R^2} \sqrt{R^2 - x^2}$$

By evaluating eigenvalues of the spatial interaction matrix, POLIS detects whether faction cohesion remains stable or exceeds semicircle bounds, signaling fragmentation.

## Differential Equations for Human Emotion and Sociodynamics

Physical movement provides visual structure; intrinsic behavior is governed by differential-equation models of affect and social interaction. Emotional state is treated as a continuous variable evolving under internal regulation and external stochastic influence.

### Natural Emotional Decay and Strategic Intervention

The emotional continuum is modeled as a coupled system of:

- natural emotional decay
- non-linear group feedback
- composite regulatory strategies

Emotional magnitude decays at rate $\gamma$. Group feedback induces secondary shifts through compensation and contrast effects.

Agents counteract decay through strategies that induce dissonance or resonance. Without intervention, emotional evolution converges according to:

$$\beta \kappa E^2 - (2\beta\kappa + \gamma) E + \beta\kappa = 0$$

Where:

- $\gamma$: decay rate
- $\beta$, $\kappa$: strategy intensity and feedback parameters

Stability analysis yields a smaller root $E_-^* \in (0,1)$ as the stable equilibrium. Agents evaluate deviation from this equilibrium and adjust behavior to either seek social resonance or isolate to preserve internal state.

### Mathematical Models of Curiosity and Intrinsic Motivation

Exploration is driven by intrinsic reward bonuses independent of external reward. POLIS implements four major curiosity elicitors, plus learning progress, evaluated during compute updates.

| Curiosity Modality | Mathematical Formulation | Mechanism of Action in Simulation |
|---|---|---|
| Action Novelty | $R_{Action\text{-}Nov}^n(s,a)=f(n(s,a))$ | Biases actions rarely taken in state $s$; $f$ decreases as selection count grows. |
| Predictive Surprise | $R_{Surprise}^{Pred}(s,a,s')=-\log(P_n(s'\mid s,a))$ | Rewards mismatch between prediction and observed outcome. |
| Update Surprise | $R_{Surprise}^{Up}(s,a,s')=D_{KL}(P_{n+1}\parallel P_n)$ | Rewards large model updates and knowledge gain. |
| Expected Uncertainty | $R_{Entropy}^n(s,a)=\mathbb{E}_{s''}[-\log(P_n(s''\mid s,a))]$ | Drives exploration of high-entropy states. |
| Learning Progress | $R_{Learning\text{-}Progress}^n=\vert Metric_n - Metric_{n-k} \vert$ | Promotes domains where learning velocity is highest. |

These mechanisms pair with active inference: agents minimize variational free energy while still pursuing epistemic exploration.

## Evolutionary Game Theory and the Multi-Objective Utility Function

Balancing emotional, strategic, and exploratory goals creates a high-dimensional objective space. Instead of fixed scalarization, POLIS uses Evolutionary Game Theory (EGT), specifically replicator dynamics, to evolve utility weights via population competition.

### Replicator Dynamics and Strategy Propagation

Continuous replicator dynamics are modeled as:

$$\dot{x}_i = x_i [f_i(x) - \phi(x)]$$

Where:

- $x_i$: population frequency of strategy $i$
- $f_i(x)$: fitness of strategy $i$
- $\phi(x)=\sum x_j f_j(x)$: average population fitness

Agents compare cumulative payoff with a sampled neighbor. If neighbor payoff is higher, adoption probability is proportional to the payoff gap. Successful utility vectors propagate rapidly; weak configurations fade.

### Duality of Dominance and Prestige

Fitness evaluation follows the dual hierarchy model:

- Dominance: status through coercion, intimidation, and force; stable but brittle.
- Prestige: status through valued competence and social learning; cohesion through voluntary deference.

High-prestige agents attract social learners, forming coherent clusters. High-dominance agents may extract compliance, but with elevated emotional decay among subordinates.

### GLSL Pseudocode: Evolutionary Utility Kernel

```glsl
#version 450
layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

// Struct containing variables evaluated during replicator dynamics
struct UtilityData {
    float dominance_score;
    float prestige_score;
    float environmental_payoff;
    uint strategy_id;
    vec4 utility_weights; // Weights applied in kinematic pass
};

layout(std430, binding = 0) buffer AgentUtilities {
    UtilityData utilities[];
};

// Global parameters governing evolutionary pressure
layout(std140, binding = 1) uniform GameTheoryParams {
    float baseline_fitness;
    float dominance_multiplier;
    float prestige_multiplier;
    uint total_agents;
    uint seed;
} gt_params;

// Simple procedural random number generator
uint hash(uint x) {
    x ^= x >> 16;
    x *= 0x7feb352dU;
    x ^= x >> 15;
    x *= 0x846ca68bU;
    x ^= x >> 16;
    return x;
}

void main() {
    uint id = gl_GlobalInvocationID.x;
    if (id >= gt_params.total_agents) return;

    UtilityData self = utilities[id];

    // Sample a random neighbor using a hashed pseudo-random index
    uint rng_state = hash(id + gt_params.seed);
    uint random_neighbor_id = rng_state % gt_params.total_agents;
    UtilityData neighbor = utilities[random_neighbor_id];

    // Compute fitness f_i(x) using dual hierarchy model
    float my_fitness = gt_params.baseline_fitness
                     + (self.prestige_score * gt_params.prestige_multiplier)
                     + (self.dominance_score * gt_params.dominance_multiplier)
                     + self.environmental_payoff;

    float neighbor_fitness = gt_params.baseline_fitness
                           + (neighbor.prestige_score * gt_params.prestige_multiplier)
                           + (neighbor.dominance_score * gt_params.dominance_multiplier)
                           + neighbor.environmental_payoff;

    // Apply replicator learning rule
    if (neighbor_fitness > my_fitness) {
        float probability = (neighbor_fitness - my_fitness) / max(neighbor_fitness, 0.0001);
        float rand_val = float(hash(rng_state) % 10000) / 10000.0;

        if (rand_val < probability) {
            utilities[id].strategy_id = neighbor.strategy_id;
            utilities[id].utility_weights = neighbor.utility_weights;
        }
    }
}
```

## Dynamic Group Collapse and Faction Stratification

As replicator dynamics alter strategies, groups form based on utility similarity and spatial proximity. Stability is sensitive to the dominance-prestige ratio.

Dominance helps early consolidation but degrades over long trajectories, especially in cooperative structures. Overuse of coercion increases internal dissonance and perturbs emotional equilibrium.

POLIS models faction fracturing with Markov-process competition dynamics inspired by cultural transition models (including Abrams-Strogatz forms):

$$U_i = s_i \cdot x_i^a$$

Where:

- $s_i$: prestige metric of group $i$
- $x_i$: population fraction of group $i$

Inter-faction transition divergence is measured using Cohen's $d$:

$$
\text{Cohen's } d =
\frac{\bar{X}_1 - \bar{X}_2}
{\sqrt{\frac{(n_1 - 1)sd_1^2 + (n_2 - 1)sd_2^2}{n_1 + n_2 - 2}}}
$$

Interpretation thresholds:

- $d > 0.8$: systemic transition divergence, indicating large-scale faction defection.
- Bayes factor $BF_{31} > 1.6$: evidence that collapse is socially driven rather than purely stochastic.

Spatially, reduction kernels detect a variance spike as semicircle-bound cohesion breaks.

## High-Level Logic and Reference Model Implementations

Vulkan and GLSL are optimal for parallel numerical integration, but inefficient for branching scenario logic, discrete event control, and rich statistical workflow orchestration. POLIS therefore uses high-level scripting for control-plane logic.

### Python: Offline Statistical Modeling and Scenario Validation

Python is used for:

- equation validation
- parameter sweeps
- data collection and analysis

Mesa serves as a reference ABM framework before shader integration.

```python
from mesa import Agent, Model
from mesa.time import RandomActivation
from mesa.datacollection import DataCollector


class PolisAgent(Agent):
    def __init__(self, unique_id, model, beta, kappa, gamma):
        super().__init__(unique_id, model)
        self.emotion = 0.5  # Initial emotional state E
        self.beta = beta    # Strategy intensity
        self.kappa = kappa  # Feedback strength
        self.gamma = gamma  # Natural decay rate
        self.prestige = 1.0

    def step(self):
        forcing_function = self.model.get_social_pressure(self.pos)

        # Differential change from quadratic equilibrium model
        # beta*kappa*E^2 - (2*beta*kappa + gamma)*E + beta*kappa = 0
        decay_term = (2 * self.beta * self.kappa + self.gamma) * self.emotion
        growth_term = (self.beta * self.kappa * (self.emotion ** 2)) + (self.beta * self.kappa)
        dE = growth_term - decay_term

        # Euler integration step
        self.emotion += (dE + forcing_function) * self.model.dt
        self.emotion = max(0.0, min(1.0, self.emotion))


class PolisModel(Model):
    def __init__(self, N):
        self.num_agents = N
        self.schedule = RandomActivation(self)
        self.dt = 0.01

        for i in range(self.num_agents):
            a = PolisAgent(i, self, beta=1.2, kappa=0.8, gamma=0.05)
            self.schedule.add(a)

        self.datacollector = DataCollector(
            agent_reporters={"Emotion": "emotion", "Prestige": "prestige"}
        )

    def get_social_pressure(self, pos):
        return 0.02  # Simplified placeholder

    def step(self):
        self.datacollector.collect(self)
        self.schedule.step()
```

### Lua and SCAR: Real-Time Tactical Utility AI

Lua supports real-time scenario control, UI integration, and tactical behavior scripting, modeled after Relic's SCAR architecture.

SCAR-style APIs support high-level group management abstractions (for example, entity groups and squad groups), enabling concise scenario directives without touching C++ or shader code.

Utility AI replaces brittle large finite-state or behavior-tree expansions by continuously scoring candidate actions and selecting the highest-utility choice.

```lua
-- SCAR-style reference implementation: Tactical Utility AI

function DecideBestAction(sgroup_id, environment_data)
    local actions = {
        { name = "GatherResources", score = EvaluateGatherUtility(environment_data) },
        { name = "ExpandTerritory", score = EvaluateExpansionUtility(environment_data) },
        { name = "ConsolidatePower", score = EvaluateDominanceUtility(sgroup_id, environment_data) }
    }

    local best_action = actions[1]
    for i = 2, #actions do
        if actions[i].score > best_action.score then
            best_action = actions[i]
        end
    end

    ExecuteAction(sgroup_id, best_action.name)
end

function ExecuteAction(sgroup, action_name)
    if action_name == "ConsolidatePower" then
        local target_egroup = GetNearestCompetitiveFaction(sgroup)
        Cmd_CaptureTeamWeapon(sgroup, target_egroup, false)
    elseif action_name == "GatherResources" then
        if not SGroup_Contains(sg_economy_workers, sgroup) then
            SGroup_AddGroup(sg_economy_workers, sgroup)
        end
    end
end
```

## Unsupervised Behavior Classification and Temporal Observation

At POLIS scale, manual interpretation is infeasible. The architecture therefore includes unsupervised role discovery over temporal behavioral sequences.

Observed features include:

- trajectories
- local density
- emotional variance
- prestige
- utility configurations

Sequences are segmented into temporal windows (for example, 16-sample clips). Deep Embedded Clustering (DEC) plus LSTM autoencoding compresses these sequences into latent embeddings. Clustering over that embedding space yields emergent role classes.

The output is a $K$-dimensional role-composition vector for global or faction-level snapshots, such as:

$$[0.15\ \text{Innovators},\ 0.55\ \text{Foragers},\ 0.30\ \text{Enforcers}]$$

This compact representation is a stronger predictor of collapse, transition, and breakthrough events than raw game-state tensors alone.

## SLM Narrative Generation and Historical Orchestration

The final layer translates quantitative simulation state into interpretable historical narrative.

Using large LLMs in tight simulation loops is often impractical due to cost, latency, and drift risk. POLIS instead uses compact SLMs (such as Phi-4-family style architectures) for high-fidelity, low-latency narrative synthesis.

At scheduled intervals, the narrative layer ingests:

- role composition vectors
- reduced emotional equilibrium statistics
- dominance-prestige strategic ratios

To reduce drift, the system combines Memory RAG and Mixture of Memory Experts (MoME) style retrieval/control mechanisms. Multimodal inputs can include rendered heatmaps alongside numerical tensors.

When indicators align, for example:

- dominance-prestige imbalance
- Cohen's $d > 0.8$
- semicircle-bound anomaly

the SLM synthesizes these into context-rich chronicle outputs rather than raw logs.

## Synthesis and Conclusion

POLIS defines a practical blueprint for next-generation scientific civilization sandboxes by enforcing a clear architectural separation of concerns:

- GPU Vulkan compute for large-scale dynamics
- mathematically grounded socio-emotional and curiosity modeling
- high-level scripting for tactical and scenario orchestration
- unsupervised temporal role analytics
- compact, retrieval-stabilized narrative generation

This design enables real-time, large-population simulation while preserving interpretability and scientific rigor across both physical and sociological layers.
