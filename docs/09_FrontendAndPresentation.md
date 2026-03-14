# POLIS Frontend And Presentation

**Version:** 0.1  
**Date:** 14 March 2026  
**Document:** 09 of the POLIS spec suite  
**Purpose:** Define how POLIS presents simulation state, analytical overlays, replay, uncertainty, and narrative without allowing presentation to distort backend causality.

## 1. Scope

This document defines:

- the presentation layers of POLIS
- the distinction between raw state, analytical overlays, and narrative summaries
- the core visual modes and inspection workflows
- how uncertainty, approximation, and replay should be shown
- the dual-mode structure for sandbox and research use

This document does **not** define:

- low-level rendering engine implementation
- the underlying simulation ontology or state model
- narrative-generation internals

## 2. Dependencies

This document depends on:

- [01_WorldModel.md](/abs/path/e:/Drive/WORLDS/01_WorldModel.md)
- [02_StateModel.md](/abs/path/e:/Drive/WORLDS/02_StateModel.md)
- [03_CollectiveAgency.md](/abs/path/e:/Drive/WORLDS/03_CollectiveAgency.md)
- [04_ResourcesAndMaterials.md](/abs/path/e:/Drive/WORLDS/04_ResourcesAndMaterials.md)
- [05_DiscoveryHeuristics.md](/abs/path/e:/Drive/WORLDS/05_DiscoveryHeuristics.md)
- [06_BiologyAndDomestication.md](/abs/path/e:/Drive/WORLDS/06_BiologyAndDomestication.md)
- [07_SocietyAndInstitutions.md](/abs/path/e:/Drive/WORLDS/07_SocietyAndInstitutions.md)
- [08_ValidationAndExperiments.md](/abs/path/e:/Drive/WORLDS/08_ValidationAndExperiments.md)

It informs:

- `10_TechnicalArchitecture.md`

## 3. Guiding Principle

POLIS should have a playful, readable frontend layered on top of a rigorous backend.

The frontend must:

- make causal structure legible
- support exploration and debugging
- support research inspection and replay
- remain compelling enough to watch

The frontend must not:

- create a second hidden ontology
- replace instrumentation with vibes
- imply certainty where only approximation exists
- let narrative summaries overwrite underlying state

## 4. Presentation Layers

POLIS should use four distinct presentation layers.

### 4.1 WorldStateView

The `WorldStateView` is the direct visual rendering of currently active world state.

It includes:

- terrain
- settlements
- structures
- visible actors or proxies
- movement
- boundaries

This view should remain as close as possible to the actual simulation state.

### 4.2 AnalyticalOverlay

An `AnalyticalOverlay` is a derived visualization layer placed on top of the world state.

Examples:

- resource density heatmap
- trust or cohesion field
- disease burden map
- conflict intensity
- institutional coverage
- transport cost surface

Analytical overlays are derived, not primary.

They should always be visually distinguishable from raw world state.

### 4.3 InspectionAndProvenanceLayer

The `InspectionAndProvenanceLayer` exposes:

- causal traces
- event histories
- state provenance
- validation diagnostics
- experiment metadata

This is the serious research-facing layer.

### 4.4 NarrativeAndPublicLayer

The `NarrativeAndPublicLayer` includes:

- chronicle summaries
- labels
- event callouts
- replay captions
- public-facing data stories

This layer improves accessibility and shareability, but must never be treated as causal truth.

## 5. Presentation Categories

To avoid confusion, the frontend should distinguish five classes of presented information.

### 5.1 Raw world state

Directly rendered simulation entities and current state.

### 5.2 Derived overlay

Computed fields, summaries, and diagnostics layered over the world.

### 5.3 Narrative summary

Human-readable interpretation or compression of events and tendencies.

### 5.4 Symbolic label

A UI convenience marker such as:

- “military cohort”
- “market town”
- “water stress”

Labels are often useful, but must not be mistaken for primitive simulation objects if they are classifier outputs.

### 5.5 Experiment dashboard

A statistical or comparative interface showing results across:

- runs
- scenarios
- parameter sets
- ensembles

## 6. World View Principles

The world view should prioritize legibility without pretending that appearance is the model.

### 6.1 Ground truth principle

The visual world view must be a projection of backend state, not an independent interactive fiction layer.

### 6.2 Level-of-detail principle

As the user zooms out, the frontend may reduce visible detail, but must preserve:

- causal continuity
- scale transition visibility
- clear distinction between individual and proxy representation

### 6.3 Actor representation

When actors are abstracted:

- individuals may become aggregates or proxies visually
- the interface must indicate that abstraction explicitly

The user should never be tricked into thinking a proxy is a literal individual.

## 7. Core Visual Modes

POLIS should support a defined set of visual modes rather than a pile of ad hoc toggles.

### 7.1 Base world mode

Shows:

- terrain
- water
- settlements
- structures
- actors or proxies

### 7.2 Political and territorial mode

Shows:

- controlled territory
- jurisdiction
- frontiers
- contested zones
- polity or group boundaries

### 7.3 Resource and ecology mode

Shows:

- fertility
- water availability
- biomass or stock density
- soil stress
- pollution or depletion

### 7.4 Social mode

Shows:

- trust
- cohesion
- legitimacy
- grievance
- factional segmentation

### 7.5 Institutional mode

Shows:

- rule coverage
- office concentration
- tax reach
- market or court presence
- governance density

### 7.6 Conflict and risk mode

Shows:

- raid pressure
- active conflict
- coercive control
- disease burden
- collapse warning indicators

### 7.7 Discovery and knowledge mode

Shows:

- technique diffusion
- practice clusters
- institutionalized procedures
- infrastructure tied to knowledge systems

### 7.8 Experiment mode

Shows:

- comparisons across runs
- scenario splits
- ensemble distributions
- parameter-response views

## 8. Inspection Workflows

POLIS should support structured inspection, not only passive viewing.

### 8.1 Entity inspection

The user should be able to inspect:

- individuals
- households
- collective actors
- settlements
- institutions
- structures
- resource systems

An entity panel should clearly separate:

- authoritative state
- derived indicators
- history
- current context

### 8.2 Event inspection

The interface should allow drilling into events such as:

- migration
- trade
- raid
- institution change
- famine
- disease outbreak

Event views should expose:

- participants
- location
- conditions
- effects
- causal parents where available

### 8.3 Process inspection

The user should be able to inspect higher-order processes such as:

- agricultural transition
- institutional decay
- escalation toward conflict
- discovery diffusion

These views should connect state change across time rather than just showing snapshots.

## 9. Replay And Provenance

Replay should be a scientific tool, not just a cinematic feature.

### 9.1 Causal replay

Replay should support:

- timeline scrubbing
- event stepping
- branching comparisons
- before/after state comparisons

### 9.2 Provenance view

The user should be able to see for a run:

- model version
- scenario configuration
- seeds
- enabled subsystems
- experiment identifier

### 9.3 Comparative replay

POLIS should support side-by-side replay of:

- different seeds
- different parameterizations
- treatment versus control

This is essential for research inspection.

## 10. Uncertainty And Approximation

The frontend must make uncertainty visible.

### 10.1 Uncertainty categories

The interface should distinguish:

- stochastic variability
- parameter uncertainty
- approximation or abstraction error
- classification uncertainty

### 10.2 Visual treatment

Uncertainty may be shown through:

- confidence bands
- ensemble envelopes
- density plots
- transparency
- fuzziness or sketchiness
- uncertainty annotations

The exact style may vary by mode, but uncertainty must not be hidden.

### 10.3 Approximation visibility

When the system uses:

- proxy actors
- aggregate overlays
- classifier labels
- reduced resolution

the interface should indicate this explicitly.

A viewer should be able to tell when they are seeing:

- raw state
- approximation
- interpretation

### 10.4 Conditionality rule

When outcomes depend strongly on path or stochasticity, the interface should prefer:

- ensemble summaries
- quantile views
- regime comparisons

over a single “hero run” presentation.

## 11. Dual-Mode Structure

POLIS should support two main usage modes.

### 11.1 Sandbox mode

Sandbox mode prioritizes:

- direct experimentation
- parameter tweaking
- event injection
- intuitive watching
- playful interaction

The UI may be more streamlined and visually expressive here.

### 11.2 Research mode

Research mode prioritizes:

- reproducibility
- run comparison
- ensemble analysis
- diagnostics
- provenance inspection
- metric export

The UI should surface more detail and less spectacle here.

### 11.3 Shared backbone

Both modes must be backed by the same underlying simulation truth.

Only the presentation emphasis should differ.

## 12. Visual Narrative And Public Communication

POLIS should support shareable explanations without compromising scientific discipline.

### 12.1 Narrative summaries

Narrative summaries may help explain:

- why a collapse occurred
- how a practice spread
- why a conflict escalated

But they should be clearly marked as summaries or interpretations.

### 12.2 Data stories

For broader audiences, POLIS may present:

- annotated replays
- event callouts
- timeline narratives
- comparative story panels

These should remain linked to underlying evidence.

### 12.3 Label discipline

Named labels such as “market town” or “watch” should be shown as:

- classifier output
- institutional designation
- user-facing summary

whichever is appropriate

and not silently conflated.

## 13. Accessibility And Legibility

Serious presentation also means usable presentation.

### 13.1 Legibility requirements

Visualizations should use:

- clear hierarchy
- readable overlays
- restrained clutter
- meaningful color systems
- progressive disclosure of complexity

### 13.2 Accessibility requirements

The interface should support:

- color-blind-safe palettes where possible
- keyboard and non-pointer navigation for key functions
- text equivalents for major charts and views
- readable contrast and typography

### 13.3 Documentation rule

Every major overlay or indicator should have:

- a title
- a definition
- an explanation of whether it is raw, derived, or interpretive

## 14. Risks And Anti-Patterns

The following presentation failures must be avoided.

### 14.1 Aesthetic causality

Making the visuals imply mechanisms that do not exist in the backend.

### 14.2 Overlay confusion

Failing to distinguish raw state from derived summaries.

### 14.3 Narrative overreach

Letting generated summaries or labels stand in for evidence.

### 14.4 Hero-run bias

Showing one compelling run and hiding ensemble variability.

### 14.5 Invisible approximation

Hiding scale abstraction, proxying, or classifier uncertainty from the user.

### 14.6 Dashboard sprawl

Adding so many charts and toggles that the system becomes unreadable.

### 14.7 Instrumentation neglect

Making the simulation watchable but not inspectable.

## 15. Open Questions

These questions remain for later refinement:

- How far should the world view go toward 3D immersion versus analytical clarity in v1?
- Which uncertainty displays are least cognitively expensive while still honest?
- How much narrative automation is useful before it starts obscuring evidence?
- What is the minimum viable comparison workflow for research mode?

These do not block the presentation model, but they matter for later design choices.

## 16. UI Pre-Visualization Prompt Templates

To keep frontend implementation aligned with design intent while code is still evolving, POLIS should maintain reusable image-model prompt templates for visual reference generation.

These prompts are for **design pre-visualization only**. They do not change backend truth or frontend data contracts.

### 16.1 World-level UI pre-vis prompt

```text
Design a UI pre-visualization for a scientific civilization simulator called “POLIS”.
Goal: show a serious research-grade simulation interface, not a fantasy game HUD.

Scene and composition:
- 16:9 desktop app screenshot mockup
- Main center panel: zoomed-out world map with patch/region overlays, settlements, routes, biome coloring
- Left panel: simulation controls (play/pause, speed, tick, seed, scenario)
- Right panel: inspector for selected region with resources, waste, fertility, moisture, temperature, demand, cohesion
- Bottom panel: time-series charts with uncertainty bands
- Top bar: run status, serial/parallel mode, checkpoint/export controls, deterministic hash badge

Visual style:
- clean, modern, high-information scientific UI
- restrained color palette, readable typography, clear visual hierarchy
- product-grade data-viz style, not cartoonish, not cyberpunk

Data/UX details:
- map layer toggles: resources, waste, disease risk, institutions, trade
- event log with timestamped events
- provenance widget: run id, seed, scenario, checkpoint id
- explicit uncertainty cues (bands/shading/labels)

Output requirements:
- polished realistic software screenshot look
- no real-product logos, no fantasy art motifs
```

### 16.2 Micro-level UI pre-vis prompt (agents + landscaping + building)

```text
Design a UI pre-visualization for POLIS at settlement/micro scale.
Goal: show agents moving, construction, and landscape transformation while preserving a scientific-sim UI tone.

Scene and composition:
- 16:9 desktop app screenshot mockup, camera focused on one settlement district
- Main viewport: dozens of small agents visibly walking along paths, carrying materials, gathering near work sites
- Active construction sites with scaffold-like structures, partial walls, expanding fields, road extension
- Nearby landscape edits visible: cleared land, terracing/irrigation channels, tree removal/replanting patches
- Left panel: play/pause/speed/tick controls and layer toggles
- Right panel: selected-agent inspector (role, task, inventory, destination, social state) and selected-site inspector (build progress, required materials, labor assigned)
- Bottom panel: local charts (resource inflow/outflow, waste accumulation, construction throughput, cohesion)
- Top status bar: run id, seed, deterministic hash, checkpoint/export buttons

Visual style:
- believable simulation game aesthetic with readable scientific overlays
- intentional color coding for tasks (build, move, gather, transport, idle)
- subtle motion cues (path trails, work icons, progress bars), not exaggerated effects

Data/UX details:
- overlays for pathing, zoning/land use, waste hotspots, water/fertility fields
- event list includes: “construction started/completed”, “land converted”, “waste threshold exceeded”
- uncertainty and approximation indicators remain visible where relevant

Output requirements:
- polished high-detail UI mockup suitable as engineering reference
- avoid fantasy UI tropes, avoid clutter, keep labels legible
```

### 16.3 Optional negative prompt

```text
cartoon HUD, fantasy RPG interface, neon cyberpunk style, unreadable tiny text, excessive particle effects, mobile layout, low-detail placeholders
```

## 17. Summary

POLIS should present itself through layered visualization in which:

- world state remains the visual ground truth
- analytical overlays remain clearly derived
- replay and provenance support audit and research
- narrative summaries improve accessibility without replacing evidence
- uncertainty and approximation are made visible
- sandbox and research modes emphasize different workflows over the same backend truth

This is how POLIS can remain both game-readable and scientifically credible.
