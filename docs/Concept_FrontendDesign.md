# POLIS Concept Note: Frontend Design And Technology

**Date:** 14 March 2026  
**Status:** Concept note  
**Purpose:** Define the frontend identity, interaction model, layer architecture, technology stack, and library selections for POLIS, with emphasis on AI-assisted development suitability.

## 1. Frontend Identity

POLIS is WorldBox with a serious backend.

The player is a scientist-spectator. The core loop is:

- configure starting parameters
- launch the simulation
- observe how systems and agents evolve
- understand why things happened

There are no player objectives, no win conditions, no direct control of agents. The player builds worlds, watches civilisations emerge, and digs into the data when something interesting happens.

The frontend must be:

- fun and approachable to watch
- visually charming
- immediately legible at surface level
- deeply inspectable for curious users
- scientifically powerful for researchers

## 2. Three-Layer Model

The frontend serves three audiences through progressively deeper layers.

### 2.1 Layer 0: Sandbox Toybox

Everyone sees this. It must be immediately fun.

Capabilities:

- drop terrain, resources, populations, hazards onto the map
- adjust sliders for faction parameters such as intelligence, aggression, fertility, resource abundance
- hit play, watch what happens
- speed controls, pause, zoom in and out
- visual feedback is immediate and charming
- no need to understand the underlying math

The visual style should be toy diorama: stylised low-poly or pixel hybrid, clean iconography, miniature-looking settlements and agents, approachable and personality-rich.

Key visual signals that make watching compelling:

- settlement growth is visible as structures appear, walls extend, fields spread, roads connect
- social tension is readable through color shifts, particle effects, or agent grouping changes
- trade and movement leave traces as paths, routes, and trails
- conflict is visceral as raids and skirmishes play out visibly
- discovery moments are marked with a visual spark or ripple when a group achieves something new
- collapse is dramatic as settlements shrink, fields go fallow, populations scatter

### 2.2 Layer 1: Curiosity Layer

For players who notice something interesting and want to know why.

Capabilities:

- click a settlement to see population, surplus, cohesion, tech level
- click an agent to see their state, ties, mood, inventory
- hover a trade route to see what flows and why
- event cards pop up for significant moments such as discovery, war, famine, institutional change
- faction summaries show relative strength, stability, development level
- simple timeline of major events

This layer differentiates POLIS from WorldBox. The emergent behavior has real, inspectable causes.

### 2.3 Layer 2: Research Layer

For users who want the real data.

Capabilities:

- overlay toggles for every field the backend tracks including fertility, disease, pollution, trust, tech density
- time-series graphs for any tracked metric
- event log with causal chains and filtering
- export to CSV, Parquet, or Arrow
- batch mode with seed control and parameter sweeps
- network visualisation for social and trade graphs
- comparison dashboards across runs

This layer is invisible to casual players unless they go looking. It can be more utilitarian in appearance.

## 3. Zoom Levels

At 100K or more agents on a large map, the viewport must work across scales.

| Zoom level | What the player sees | What communicates automatically |
|---|---|---|
| World | Territory colours, biome, resource density, trade routes, faction boundaries | Geopolitical and ecological patterns |
| Region | Settlements, fields, roads, herd movement, raiding parties | Meso-scale social dynamics |
| Settlement | Individual agents moving, structures, storage, gatherings | Micro behaviour and daily life |
| Agent | One agent's state, inventory, social ties, memory, mood | Debugging and storytelling |

Each zoom level should communicate meaningful information by default without requiring overlays. Overlays add Layer 2 depth on top.

## 4. Intervention Model

POLIS supports two modes with different intervention rules.

### 4.1 Sandbox mode

Full live intervention:

- drop terrain features, resources, hazards, populations
- trigger disasters such as drought, plague, invasion
- adjust faction parameters mid-run
- spawn or remove agents and structures

All interventions are logged as events in the Command-Event-Effect system to maintain state contract integrity. Sandbox runs may be flagged as non-standard for research reproducibility, but intervention logs make them replayable.

### 4.2 Research mode

Parameter setup only:

- configure scenario via structured editor or YAML
- set seeds, batch sizes, output intervals
- launch run with no mid-run intervention
- observe and export

Research mode runs are fully reproducible from scenario plus seed plus build version.

## 5. Time Control

Time control is the primary player interaction during observation.

Required controls:

- play and pause
- speed levels such as 1x, 2x, 5x, 10x, and max
- rewind via checkpoint seeking and event log replay
- bookmark system for marking interesting moments
- jump-to-event for navigating to specific historical moments via the event log

Rewind and bookmarking are what make POLIS feel like both a research tool and a toy. The player watches, notices something interesting, rewinds, and zooms in.

## 6. Narrative Surface

The SLM chronicle system generates human-readable narrative from simulation state. It surfaces through multiple channels.

### 6.1 Event cards

Pop-up notifications for significant simulation events:

- discovery moments
- wars and raids
- institutional changes
- faction splits or merges
- ecological collapses
- major migrations

Cards should be brief, visually styled, and clickable for deeper inspection.

### 6.2 Scrolling log

A compact text feed filterable by:

- faction
- event type
- severity
- time range

Functional and research-friendly. Always available in the UI.

### 6.3 Chronicle view

A separate screen showing evolving written history organised by era, faction, or theme. Best for post-run review and storytelling.

### 6.4 Future option: reactive narrator

A sidebar that comments on events in natural language as the player watches. Most immersive, highest SLM compute load. Not required for v1 but architecturally possible.

## 7. Technology Stack Decision

### 7.1 Decision criteria

POLIS is being built primarily with AI coding agents. The technology stack must therefore maximise:

- training-data coverage in current language models
- compiler-enforced correctness to catch AI mistakes at compile time
- ecosystem consistency so AI agents produce predictable, idiomatic code
- composability from well-documented, well-maintained libraries
- safety for GPU and concurrent code where AI-generated bugs are hardest to find

### 7.2 Language: Rust

Rust is selected for the full stack for the following reasons:

- Rust's compiler acts as a safety net for AI-generated code. When an AI agent produces slightly wrong code, the compiler reports the exact problem. In C++, wrong code can compile silently and produce runtime bugs.
- Cargo enforces consistent dependency management, build configuration, and project structure. This makes AI-generated project scaffolding more reliable.
- The borrow checker prevents data races and memory corruption in concurrent and GPU-adjacent code. This is especially valuable when AI agents produce parallel or unsafe code.
- Rust's trait system and strong type conventions mean AI agents produce more predictable and composable code.
- The Rust ecosystem has reached sufficient maturity for game and simulation development.
- All critical libraries listed below are well-represented in model training data.

### 7.3 GPU: wgpu over raw Vulkan bindings

wgpu is selected over raw ash bindings for the following reasons:

- wgpu provides safe abstractions over Vulkan, Metal, and DX12 without exposing unsafe FFI surfaces to AI-generated code
- compute shaders, storage buffers, uniform buffers, and double buffering are fully supported
- GLSL to SPIR-V compilation is supported via naga or external tooling, preserving existing shader design
- wgpu has extensive tutorial coverage and example code in model training data
- wgpu targets Vulkan on Linux and Windows, so performance characteristics match the spec
- if specific low-level Vulkan features become necessary later, targeted ash interop is possible without rewriting the whole stack

Shaders should be authored in GLSL and compiled to SPIR-V via shaderc or glslc. wgpu also supports WGSL natively, which may be preferred for new shaders where AI generation is more reliable.

### 7.4 UI: egui

egui is selected for all user interface work for the following reasons:

- one of the most popular Rust crates with high model training-data coverage
- immediate-mode architecture matches simulation inspection patterns naturally
- integrates with wgpu via egui-wgpu
- integrates with winit via egui-winit
- built-in plotting via egui_plot covers time-series, histograms, and scatter displays
- themeable with custom visuals and can be styled beyond default appearance
- supports all three frontend layers: toybox controls, inspection panels, and research dashboards
- productive for rapid iteration

### 7.5 Summary stack

| Role | Library | Purpose |
|---|---|---|
| GPU compute and render | wgpu | Simulation compute shaders and 2D rendering |
| Vulkan interop (if needed) | ash | Targeted low-level access for specific hot paths |
| GPU memory (if using ash) | vk-mem | Vulkan memory allocation |
| Windowing and input | winit | Cross-platform window management and input events |
| UI and overlays | egui, egui-wgpu, egui-winit | Panels, sliders, inspectors, graphs, tooltips |
| Plotting and data viz | egui_plot | Time-series, metrics, and overlay graphs |
| Audio | kira | Game audio with mixing, spatial, and parameter-driven effects |
| Terrain generation | noise | Procedural Perlin, Simplex, and related noise |
| Pathfinding | pathfinding | A-star, Dijkstra, BFS for agent and route computation |
| Graph structures | petgraph | Social networks, trade networks, institutional relations |
| Serialisation | serde | JSON, YAML, RON, MessagePack serialisation |
| Scenario config | serde_yaml or ron | Scenario file loading |
| Data export | arrow-rs, parquet | Arrow and Parquet columnar output for batch analysis |
| Shader compilation | shaderc or naga | GLSL to SPIR-V, or WGSL support |
| 2D tessellation | lyon | Vector path rendering for map overlays and boundaries |
| Texture atlasing | etagere | Sprite and tile atlas packing |
| Image loading | image | PNG, BMP, and other format decoding |
| Font rendering | cosmic-text or fontdue | Text shaping and rasterisation for custom render passes |
| ECS | hecs or legion | Entity management for simulation and scene state |
| SLM narrative | candle or ollama | Local language model inference for chronicle generation |
| RNG | rand, rand_chacha | Deterministic scoped random number generation |

## 8. Rendering Architecture

### 8.1 Compute-render sharing

The simulation compute pipeline and the rendering pipeline share the same wgpu device. Simulation state lives in GPU storage buffers. The renderer reads these buffers directly for drawing agents, settlements, fields, and overlays. No CPU readback is required for visual display.

### 8.2 Render passes

A suggested render pass order:

1. Terrain and biome tiles
2. Environmental field overlays when active
3. Structures and settlements
4. Agents and organism populations
5. Routes, paths, and flow lines
6. Particles and visual effects
7. Faction boundaries and territory overlays
8. UI via egui render pass

### 8.3 Sprite and tile system

Agents, structures, terrain tiles, and organisms should be represented as instanced sprite draws from texture atlases. The atlas system should use etagere for packing and wgpu instanced rendering for throughput.

At world zoom, agents should be represented as faction-colored dots or simple glyphs. At settlement zoom, agents should have expressive sprites with visible state cues such as activity, mood, or role.

### 8.4 Camera

A 2D camera with:

- smooth pan and zoom
- zoom-level-dependent detail rendering
- minimap for world-scale navigation

## 9. Audio Architecture

Audio should respond to simulation state rather than playing fixed tracks.

### 9.1 Ambient layer

Background audio should shift based on:

- biome under the camera
- population density
- time of day if modelled
- conflict proximity
- environmental stress

### 9.2 Event sounds

Short audio cues for significant events:

- discovery spark
- raid or conflict
- settlement founding
- institutional change
- collapse

### 9.3 Implementation

kira provides parameter-driven audio mixing with smooth transitions. Audio parameters should be bound to derived simulation state with minimal coupling into the core simulation loop.

## 10. Narrative Integration Architecture

### 10.1 SLM backend

The narrative system uses a local small language model via candle for embedded inference or ollama for model serving.

Input to the SLM should include:

- role composition vectors
- reduced emotional and social statistics
- event summaries
- faction indicators

### 10.2 Generation triggers

Narrative generation should be triggered by:

- significant event detection, such as Cohen d thresholds, collapse indices, or discovery events
- periodic chronicle intervals
- player inspection of a faction or settlement

### 10.3 Output routing

Generated text should route to:

- event cards for immediate display
- scrolling log for continuous record
- chronicle store for post-run review
- tooltip enrichment for Layer 1 inspection

## 11. Data Export Pipeline

### 11.1 Research mode

At configured output intervals, the engine writes:

- per-agent state snapshots in Arrow or Parquet format
- aggregate metrics per step
- event log with causal metadata
- scenario and seed metadata

### 11.2 Sandbox mode

Optional export of:

- intervention log
- event history
- final-state snapshot

### 11.3 Tooling

Python notebooks should be able to load Parquet output directly via polars or pandas for offline analysis. This is external to the POLIS binary but should be documented and supported with example scripts.

## 12. Relationship To Spec Suite

This concept note captures early frontend design decisions. It should eventually feed into:

- `09_FrontendAndPresentation.md` for visual modes, naming, labels, cosmetic versus causal rules
- `10_TechnicalArchitecture.md` for engine layers, rendering pipeline, compute-render integration

This document defines what the frontend should feel like and what technology it should use. It does not redefine simulation rules, state contracts, or backend causality.

## 13. Open Questions

These questions remain for later refinement:

- How should the minimap represent collective actor boundaries and faction territory?
- What sprite resolution and atlas size should target the 100K agent scale at interactive frame rates?
- Should egui be used for all Layer 0 controls or should a custom-drawn toybox panel exist for stronger visual identity?
- How much of the narrative SLM should run at real-time speed versus buffered generation between ticks?
- Should sandbox mode interventions be replayable as scenario macros for reproducible experimentation?
- What accessibility features should be included in v1?
- Should POLIS support modding or user-created scenario packs?

## 13.1 UI Pre-Vis Workflow

While implementation is in progress, generate reference UI images for both:

- world-scale analytical views
- settlement-scale micro views (agents moving, landscaping, construction)

Canonical prompt templates are maintained in:

- `09_FrontendAndPresentation.md`, section `16. UI Pre-Visualization Prompt Templates`

## 14. Summary

The POLIS frontend should be a charming, approachable sandbox that hides a serious simulation underneath three progressive layers: toybox, curiosity, and research. The technology stack is built around Rust, wgpu, and egui, chosen for AI-assisted development suitability, compile-time safety, and ecosystem maturity. Rendering shares the GPU device with simulation compute for zero-copy performance. Narrative generation uses local SLMs. Data export uses Arrow and Parquet for offline scientific analysis.
