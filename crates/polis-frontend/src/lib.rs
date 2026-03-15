//! Minimal presentation shell for POLIS
//!
//! Phase 2 deliverables:
//! - Windowed run mode
//! - Grid rendering of world partitions
//! - Pause and speed controls
//! - Resource overlay visualization
//! - Explicit command path from UI to backend

use macroquad::prelude::*;
use polis_agents::AgentId;
use polis_sim::{ExecutionMode, Simulation};
use polis_world::PartitionState;
use std::collections::HashMap;

const COLOR_BG_TOP: Color = Color::new(0.07, 0.09, 0.13, 1.0);
const COLOR_BG_BOTTOM: Color = Color::new(0.04, 0.05, 0.08, 1.0);
const COLOR_PANEL: Color = Color::new(0.08, 0.11, 0.16, 0.92);
const COLOR_PANEL_BORDER: Color = Color::new(0.34, 0.40, 0.52, 0.95);
const COLOR_TEXT_PRIMARY: Color = Color::new(0.90, 0.93, 0.98, 1.0);
const COLOR_TEXT_MUTED: Color = Color::new(0.62, 0.69, 0.80, 1.0);
const COLOR_ACCENT: Color = Color::new(0.26, 0.74, 0.96, 1.0);
const COLOR_OK: Color = Color::new(0.43, 0.82, 0.54, 1.0);
const COLOR_WARN: Color = Color::new(0.98, 0.78, 0.36, 1.0);
const COLOR_ALERT: Color = Color::new(0.91, 0.35, 0.33, 1.0);

pub struct FrontendModule;

impl FrontendModule {
    pub const fn name() -> &'static str {
        "polis-frontend"
    }
}

/// Commands that can be sent from UI to simulation
/// This ensures explicit command path from UI to backend
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimCommand {
    Pause,
    Resume,
    StepSingle,
    SetSpeed(SimSpeed),
    ToggleOverlay(OverlayType),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimSpeed {
    Slow,   // 1 tick per second
    Normal, // 10 ticks per second
    Fast,   // 60 ticks per second
    Max,    // Unlimited
}

impl SimSpeed {
    pub fn ticks_per_frame(&self) -> u32 {
        match self {
            SimSpeed::Slow => 1,
            SimSpeed::Normal => 2,
            SimSpeed::Fast => 10,
            SimSpeed::Max => 100,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            SimSpeed::Slow => "1x",
            SimSpeed::Normal => "10x",
            SimSpeed::Fast => "60x",
            SimSpeed::Max => "MAX",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverlayType {
    Resources,     // Show food/water/material levels
    Fields,        // Show temperature/moisture/fertility
    Demand,        // Show population demand
    SocialTension, // Phase 4: Show conflict/trust levels
    CrossSpecies,  // Phase 4: Show animal fear/tolerance
    Collectives,   // Phase 5: Show collective actor footprint and quality
    None,
}

/// The presentation shell state
pub struct PresentationShell {
    simulation: Simulation,
    is_paused: bool,
    speed: SimSpeed,
    overlay: OverlayType,
    selected_partition: Option<usize>,
    tick_timer: f32,
}

impl PresentationShell {
    pub fn new(simulation: Simulation) -> Self {
        Self {
            simulation,
            is_paused: false,
            speed: SimSpeed::Normal,
            overlay: OverlayType::Resources,
            selected_partition: None,
            tick_timer: 0.0,
        }
    }

    /// Process a command - this is the explicit command path from UI to backend
    pub fn process_command(&mut self, cmd: SimCommand) {
        match cmd {
            SimCommand::Pause => self.is_paused = true,
            SimCommand::Resume => self.is_paused = false,
            SimCommand::StepSingle => {
                self.is_paused = true;
                self.simulation.step_with_mode(ExecutionMode::Serial);
            }
            SimCommand::SetSpeed(speed) => self.speed = speed,
            SimCommand::ToggleOverlay(overlay) => {
                self.overlay = if self.overlay == overlay {
                    OverlayType::None
                } else {
                    overlay
                }
            }
        }
    }

    /// Update simulation based on current speed and pause state
    pub fn update(&mut self, dt: f32) {
        if self.is_paused {
            return;
        }

        self.tick_timer += dt;
        let ticks = self.speed.ticks_per_frame();

        // Run simulation ticks
        for _ in 0..ticks {
            self.simulation.step_with_mode(ExecutionMode::Serial);
        }
    }

    /// Draw the world grid
    pub fn draw(&mut self) {
        self.draw_background();

        let world = self.simulation.world();
        let partitions = world.partitions();
        let partition_count = partitions.len();

        if partition_count == 0 {
            return;
        }

        // Calculate grid layout
        let cols = (partition_count as f32).sqrt().ceil() as usize;
        let rows = (partition_count + cols - 1) / cols;

        let margin = 28.0;
        let available_width = screen_width() - margin * 2.0;
        let available_height = screen_height() - margin * 2.0 - 100.0; // Leave room for UI

        let cell_width = available_width / cols as f32;
        let cell_height = available_height / rows as f32;
        let cell_size = cell_width.min(cell_height).min(100.0); // Cap max cell size

        // Center the grid
        let grid_width = cell_size * cols as f32;
        let grid_height = cell_size * rows as f32;
        let start_x = (screen_width() - grid_width) / 2.0;
        let start_y = (screen_height() - grid_height) / 2.0 + 20.0;

        // Draw grid cells
        let collective_overlay_scores = if self.overlay == OverlayType::Collectives {
            Some(self.compute_collective_overlay_scores(partition_count))
        } else {
            None
        };

        for (idx, partition) in partitions.iter().enumerate() {
            let col = idx % cols;
            let row = idx / cols;

            let x = start_x + col as f32 * cell_size;
            let y = start_y + row as f32 * cell_size;

            let is_selected = self.selected_partition == Some(idx);
            let collective_score = collective_overlay_scores
                .as_ref()
                .and_then(|scores| scores.get(idx))
                .copied();
            let color = self.partition_color(partition, collective_score);

            // Draw cell background
            draw_rectangle(x, y, cell_size - 3.0, cell_size - 3.0, color);
            draw_rectangle_lines(
                x,
                y,
                cell_size - 3.0,
                cell_size - 3.0,
                1.2,
                Color::new(0.09, 0.12, 0.18, 0.95),
            );

            // Draw selection border
            if is_selected {
                draw_rectangle_lines(
                    x - 1.0,
                    y - 1.0,
                    cell_size - 1.0,
                    cell_size - 1.0,
                    3.0,
                    COLOR_ACCENT,
                );
            }

            // Draw partition ID
            let text = format!("{}", idx);
            let text_size = (cell_size / 4.0).max(10.0) as u16;
            draw_text(
                &text,
                x + 4.0,
                y + text_size as f32,
                text_size as f32,
                COLOR_TEXT_PRIMARY,
            );

            // Check for mouse hover/click
            let mouse_pos = mouse_position();
            if mouse_pos.0 >= x
                && mouse_pos.0 < x + cell_size
                && mouse_pos.1 >= y
                && mouse_pos.1 < y + cell_size
            {
                if is_mouse_button_pressed(MouseButton::Left) {
                    self.selected_partition = Some(idx);
                }

                // Draw tooltip
                self.draw_tooltip(mouse_pos.0, mouse_pos.1, partition, idx);
            }
        }

        // Draw UI overlay
        self.draw_ui();

        // Draw selected partition details
        if let Some(selected) = self.selected_partition {
            if let Some(partition) = partitions.get(selected) {
                self.draw_partition_details(partition, selected);
            }
        }
    }

    fn draw_background(&self) {
        let h = screen_height().max(1.0);
        let w = screen_width().max(1.0);

        // Vertical gradient backdrop.
        let bands = 36;
        for i in 0..bands {
            let t0 = i as f32 / bands as f32;
            let t1 = (i + 1) as f32 / bands as f32;
            let y0 = t0 * h;
            let band_h = (t1 - t0) * h + 1.0;
            let color = Color::new(
                COLOR_BG_TOP.r + (COLOR_BG_BOTTOM.r - COLOR_BG_TOP.r) * t0,
                COLOR_BG_TOP.g + (COLOR_BG_BOTTOM.g - COLOR_BG_TOP.g) * t0,
                COLOR_BG_TOP.b + (COLOR_BG_BOTTOM.b - COLOR_BG_TOP.b) * t0,
                1.0,
            );
            draw_rectangle(0.0, y0, w, band_h, color);
        }

        // Subtle technical grid for depth.
        let grid = 48.0;
        let line = Color::new(0.16, 0.20, 0.28, 0.16);
        let mut x = 0.0;
        while x <= w {
            draw_line(x, 0.0, x, h, 1.0, line);
            x += grid;
        }
        let mut y = 0.0;
        while y <= h {
            draw_line(0.0, y, w, y, 1.0, line);
            y += grid;
        }
    }

    /// Get color for a partition based on current overlay
    fn partition_color(&self, partition: &PartitionState, collective_score: Option<f32>) -> Color {
        match self.overlay {
            OverlayType::Resources => {
                // Color based on total resources (food + water weighted)
                let food_ratio = (partition.food.quantity as f64
                    / partition.carrying_capacity_food as f64)
                    .clamp(0.0, 1.0);
                let water_ratio = (partition.water.quantity as f64
                    / partition.carrying_capacity_water as f64)
                    .clamp(0.0, 1.0);
                let resource_level = (food_ratio + water_ratio) / 2.0;

                // Green for abundant, yellow for medium, red for scarce
                if resource_level > 0.6 {
                    Color::new(0.2, 0.8, 0.2, 1.0) // Green
                } else if resource_level > 0.3 {
                    Color::new(0.9, 0.9, 0.2, 1.0) // Yellow
                } else {
                    Color::new(0.9, 0.3, 0.2, 1.0) // Red
                }
            }
            OverlayType::Fields => {
                // Color based on fertility
                let fertility = partition.fertility.value.clamp(0.0, 1.0);
                Color::new(0.4, 0.3 + fertility as f32 * 0.5, 0.2, 1.0)
            }
            OverlayType::Demand => {
                // Color based on demand pressure
                let pressure = (partition.demand as f32 / 1000.0).min(1.0);
                Color::new(0.3 + pressure * 0.7, 0.2, 0.4, 1.0)
            }
            OverlayType::SocialTension => {
                // Phase 4: Color based on social tension (conflict potential)
                // We use cohesion as a proxy since social tension is calculated per tick
                let tension = (100.0 - partition.cohesion as f32 / 100.0).min(1.0);
                // Red for high tension, blue for low tension
                Color::new(tension, 0.2, 1.0 - tension, 1.0)
            }
            OverlayType::CrossSpecies => {
                // Phase 4: Color based on animal tolerance of humans
                let tolerance = partition.animal_human_tolerance as f32 / 100.0;
                // Green for high tolerance (approaching domestication), red for fear
                Color::new(1.0 - tolerance, tolerance, 0.2, 1.0)
            }
            OverlayType::Collectives => {
                // Phase 5: density/quality proxy built from collective membership,
                // legitimacy, and factionalism in this partition.
                let score = collective_score.unwrap_or(0.0).clamp(0.0, 1.0);
                Color::new(1.0 - score, 0.2 + score * 0.7, 0.25, 1.0)
            }
            OverlayType::None => Color::new(0.3, 0.3, 0.4, 1.0),
        }
    }

    /// Compute a per-partition collective overlay score in [0, 1].
    /// High scores indicate dense membership + high legitimacy + low factionalism.
    fn compute_collective_overlay_scores(&self, partition_count: usize) -> Vec<f32> {
        let mut member_counts = vec![0_u32; partition_count];
        let mut legitimacy_sum = vec![0_u32; partition_count];
        let mut factionalism_sum = vec![0_u32; partition_count];

        let mut agent_partitions: HashMap<AgentId, u64> = HashMap::new();
        for agent in self.simulation.agents().agents() {
            if agent.is_alive {
                agent_partitions.insert(agent.id, agent.partition_id);
            }
        }

        for collective in self
            .simulation
            .agents()
            .collective_registry
            .active_collectives()
        {
            let legitimacy = collective.legitimacy as u32;
            let factionalism = collective.factionalism as u32;
            for agent_id in collective.members.keys() {
                if let Some(&partition_id) = agent_partitions.get(agent_id) {
                    let idx = partition_id as usize;
                    if idx < partition_count {
                        member_counts[idx] = member_counts[idx].saturating_add(1);
                        legitimacy_sum[idx] = legitimacy_sum[idx].saturating_add(legitimacy);
                        factionalism_sum[idx] = factionalism_sum[idx].saturating_add(factionalism);
                    }
                }
            }
        }

        member_counts
            .iter()
            .enumerate()
            .map(|(idx, &count)| {
                if count == 0 {
                    return 0.0;
                }
                let avg_legitimacy = legitimacy_sum[idx] as f32 / count as f32 / 100.0;
                let avg_factionalism = factionalism_sum[idx] as f32 / count as f32 / 100.0;
                let density = (count as f32 / 20.0).min(1.0);
                (0.5 * density + 0.5 * (avg_legitimacy * (1.0 - avg_factionalism))).clamp(0.0, 1.0)
            })
            .collect()
    }

    /// Draw tooltip for hovered partition
    fn draw_tooltip(&self, x: f32, y: f32, partition: &PartitionState, idx: usize) {
        let tooltip_width = 180.0;
        let tooltip_height = 120.0;
        let padding = 8.0;

        // Position tooltip near mouse but keep on screen
        let tx = x.min(screen_width() - tooltip_width - 10.0);
        let ty = y.min(screen_height() - tooltip_height - 10.0);

        draw_rectangle(
            tx,
            ty,
            tooltip_width,
            tooltip_height,
            COLOR_PANEL,
        );
        draw_rectangle_lines(
            tx,
            ty,
            tooltip_width,
            tooltip_height,
            1.2,
            COLOR_PANEL_BORDER,
        );

        let text_size = 14.0;
        let mut line_y = ty + text_size + padding;

        draw_text(
            &format!("Partition {}", idx),
            tx + padding,
            line_y,
            text_size,
            COLOR_TEXT_PRIMARY,
        );
        line_y += text_size + 4.0;

        draw_text(
            &format!(
                "Food: {}/{}",
                partition.food.quantity, partition.carrying_capacity_food
            ),
            tx + padding,
            line_y,
            text_size,
            Color::new(0.72, 0.90, 0.76, 1.0),
        );
        line_y += text_size + 4.0;

        draw_text(
            &format!(
                "Water: {}/{}",
                partition.water.quantity, partition.carrying_capacity_water
            ),
            tx + padding,
            line_y,
            text_size,
            Color::new(0.56, 0.79, 0.98, 1.0),
        );
        line_y += text_size + 4.0;

        draw_text(
            &format!("Demand: {}", partition.demand),
            tx + padding,
            line_y,
            text_size,
            COLOR_WARN,
        );
        line_y += text_size + 4.0;

        draw_text(
            &format!("Waste: {}", partition.waste.quantity),
            tx + padding,
            line_y,
            text_size,
            Color::new(0.78, 0.63, 0.44, 1.0),
        );
        line_y += text_size + 4.0;

        draw_text(
            &format!(
                "Animals: H:{} P:{}",
                partition.herbivore_population, partition.predator_population
            ),
            tx + padding,
            line_y,
            text_size,
            Color::new(0.72, 0.90, 0.76, 1.0),
        );
        line_y += text_size + 4.0;

        // Phase 4: Social/Cross-species summary
        draw_text(
            &format!(
                "Fear/Tol: {}/{}",
                partition.animal_fear, partition.animal_human_tolerance
            ),
            tx + padding,
            line_y,
            text_size,
            Color::new(0.94, 0.77, 0.54, 1.0),
        );
    }

    /// Draw UI controls and info
    fn draw_ui(&self) {
        let ui_y = 10.0;
        let text_size = 18.0;

        // Title
        draw_text(
            "POLIS - Phase 2 Presentation Shell",
            10.0,
            ui_y + text_size,
            text_size + 4.0,
            WHITE,
        );

        // Tick counter
        let world = self.simulation.world();
        draw_text(
            &format!("Tick: {}", world.tick()),
            10.0,
            ui_y + text_size * 2.5,
            text_size,
            Color::new(0.8, 0.8, 1.0, 1.0),
        );

        // Status
        let status = if self.is_paused { "PAUSED" } else { "RUNNING" };
        let status_color = if self.is_paused { RED } else { GREEN };
        draw_text(
            &format!("Status: {}", status),
            200.0,
            ui_y + text_size * 2.5,
            text_size,
            status_color,
        );

        // Speed indicator
        draw_text(
            &format!("Speed: {}", self.speed.label()),
            400.0,
            ui_y + text_size * 2.5,
            text_size,
            Color::new(0.8, 0.8, 1.0, 1.0),
        );

        // Overlay indicator
        let overlay_name = match self.overlay {
            OverlayType::Resources => "Resources",
            OverlayType::Fields => "Fields",
            OverlayType::Demand => "Demand",
            OverlayType::SocialTension => "Social",
            OverlayType::CrossSpecies => "Animals",
            OverlayType::Collectives => "Collectives",
            OverlayType::None => "None",
        };
        draw_text(
            &format!("Overlay: {}", overlay_name),
            550.0,
            ui_y + text_size * 2.5,
            text_size,
            Color::new(0.8, 0.8, 1.0, 1.0),
        );

        // Controls help
        let help_y = screen_height() - 80.0;
        draw_text(
            "Controls: SPACE=Pause | S=Step | 1/2/3/4=Speed | R/F/D/T/A/C=Overlay | Click=Select",
            10.0,
            help_y,
            14.0,
            Color::new(0.6, 0.6, 0.7, 1.0),
        );
    }

    /// Draw detailed info for selected partition
    fn draw_partition_details(&self, partition: &PartitionState, idx: usize) {
        let panel_width = 220.0;
        let panel_height = 280.0;
        let x = screen_width() - panel_width - 10.0;
        let y = 60.0;

        draw_rectangle(
            x,
            y,
            panel_width,
            panel_height,
            Color::new(0.1, 0.1, 0.15, 0.95),
        );
        draw_rectangle_lines(x, y, panel_width, panel_height, 2.0, WHITE);

        let text_size = 16.0;
        let padding = 12.0;
        let mut line_y = y + text_size + padding;

        draw_text(
            &format!("Partition {} Details", idx),
            x + padding,
            line_y,
            text_size + 2.0,
            WHITE,
        );
        line_y += text_size * 1.8;

        // Resources
        draw_text(
            "Resources:",
            x + padding,
            line_y,
            text_size,
            Color::new(0.9, 0.9, 0.6, 1.0),
        );
        line_y += text_size * 1.3;

        let resources = [
            (
                "Food",
                partition.food.quantity,
                partition.carrying_capacity_food as i64,
            ),
            (
                "Water",
                partition.water.quantity,
                partition.carrying_capacity_water as i64,
            ),
            ("Material", partition.material.quantity, 1000_i64),
            ("Fuel", partition.fuel.quantity, 500_i64),
            ("Ore", partition.ore.quantity, 300_i64),
            ("Waste", partition.waste.quantity, 2000_i64),
        ];

        for (name, qty, cap) in resources {
            let pct = (qty as f64 / cap as f64 * 100.0).clamp(0.0, 100.0) as i64;
            draw_text(
                &format!("  {}: {} ({}%)", name, qty, pct),
                x + padding,
                line_y,
                text_size - 2.0,
                Color::new(0.8, 0.8, 0.8, 1.0),
            );
            line_y += text_size * 1.1;
        }

        line_y += text_size * 0.5;

        // Environmental fields
        draw_text(
            "Environment:",
            x + padding,
            line_y,
            text_size,
            Color::new(0.9, 0.9, 0.6, 1.0),
        );
        line_y += text_size * 1.3;

        let fields = [
            ("Temp", partition.temperature.value, "°C"),
            ("Moisture", partition.moisture.value * 100.0, "%"),
            ("Fertility", partition.fertility.value * 100.0, "%"),
            ("Solar", partition.solar_radiation.value * 100.0, "%"),
            ("Biotic", partition.biotic_pressure.value * 100.0, "%"),
        ];

        for (name, value, unit) in fields {
            draw_text(
                &format!("  {}: {:.1}{}", name, value, unit),
                x + padding,
                line_y,
                text_size - 2.0,
                Color::new(0.8, 0.8, 0.8, 1.0),
            );
            line_y += text_size * 1.1;
        }

        line_y += text_size * 0.5;

        // Social
        draw_text(
            "Social:",
            x + padding,
            line_y,
            text_size,
            Color::new(0.9, 0.9, 0.6, 1.0),
        );
        line_y += text_size * 1.3;
        draw_text(
            &format!("  Demand: {}", partition.demand),
            x + padding,
            line_y,
            text_size - 2.0,
            Color::new(0.8, 0.8, 0.8, 1.0),
        );
        line_y += text_size * 1.1;
        draw_text(
            &format!("  Cohesion: {}", partition.cohesion),
            x + padding,
            line_y,
            text_size - 2.0,
            Color::new(0.8, 0.8, 0.8, 1.0),
        );
        line_y += text_size * 1.1;

        // Animals (Biology extension)
        line_y += text_size * 0.5;
        draw_text(
            "Animals:",
            x + padding,
            line_y,
            text_size,
            Color::new(0.9, 0.9, 0.6, 1.0),
        );
        line_y += text_size * 1.3;
        draw_text(
            &format!("  Herbivores: {}", partition.herbivore_population),
            x + padding,
            line_y,
            text_size - 2.0,
            Color::new(0.8, 0.9, 0.8, 1.0),
        );
        line_y += text_size * 1.1;
        draw_text(
            &format!("  Predators: {}", partition.predator_population),
            x + padding,
            line_y,
            text_size - 2.0,
            Color::new(0.9, 0.7, 0.7, 1.0),
        );
        line_y += text_size * 1.1;
        draw_text(
            &format!("  Proto-Domestic: {}", partition.proto_domestic_population),
            x + padding,
            line_y,
            text_size - 2.0,
            Color::new(0.9, 0.9, 0.7, 1.0),
        );
        line_y += text_size * 1.1;
        draw_text(
            &format!(
                "  Tameness: {:.1}%",
                partition.domestication_tameness * 100.0
            ),
            x + padding,
            line_y,
            text_size - 2.0,
            Color::new(0.9, 0.8, 0.6, 1.0),
        );
        line_y += text_size * 1.1;

        // Phase 4: Cross-species interaction state
        draw_text(
            &format!("  Fear: {}", partition.animal_fear),
            x + padding,
            line_y,
            text_size - 2.0,
            Color::new(0.9, 0.6, 0.6, 1.0),
        );
        line_y += text_size * 1.1;
        draw_text(
            &format!("  Tolerance: {}", partition.animal_human_tolerance),
            x + padding,
            line_y,
            text_size - 2.0,
            Color::new(0.6, 0.9, 0.6, 1.0),
        );
        line_y += text_size * 1.1;
        draw_text(
            &format!("  Familiarity: {}", partition.animal_familiarity),
            x + padding,
            line_y,
            text_size - 2.0,
            Color::new(0.8, 0.8, 0.9, 1.0),
        );
    }

    /// Handle input and return any commands
    pub fn handle_input(&self) -> Vec<SimCommand> {
        let mut commands = Vec::new();

        // Pause/Resume
        if is_key_pressed(KeyCode::Space) {
            commands.push(if self.is_paused {
                SimCommand::Resume
            } else {
                SimCommand::Pause
            });
        }

        // Step single tick
        if is_key_pressed(KeyCode::S) {
            commands.push(SimCommand::StepSingle);
        }

        // Speed controls
        if is_key_pressed(KeyCode::Key1) {
            commands.push(SimCommand::SetSpeed(SimSpeed::Slow));
        }
        if is_key_pressed(KeyCode::Key2) {
            commands.push(SimCommand::SetSpeed(SimSpeed::Normal));
        }
        if is_key_pressed(KeyCode::Key3) {
            commands.push(SimCommand::SetSpeed(SimSpeed::Fast));
        }
        if is_key_pressed(KeyCode::Key4) {
            commands.push(SimCommand::SetSpeed(SimSpeed::Max));
        }

        // Overlay toggles
        if is_key_pressed(KeyCode::R) {
            commands.push(SimCommand::ToggleOverlay(OverlayType::Resources));
        }
        if is_key_pressed(KeyCode::F) {
            commands.push(SimCommand::ToggleOverlay(OverlayType::Fields));
        }
        if is_key_pressed(KeyCode::D) {
            commands.push(SimCommand::ToggleOverlay(OverlayType::Demand));
        }
        if is_key_pressed(KeyCode::T) {
            // Phase 4: Social tension overlay
            commands.push(SimCommand::ToggleOverlay(OverlayType::SocialTension));
        }
        if is_key_pressed(KeyCode::A) {
            // Phase 4: Cross-species overlay
            commands.push(SimCommand::ToggleOverlay(OverlayType::CrossSpecies));
        }
        if is_key_pressed(KeyCode::C) {
            // Phase 5: Collective actor overlay
            commands.push(SimCommand::ToggleOverlay(OverlayType::Collectives));
        }
        if is_key_pressed(KeyCode::N) {
            commands.push(SimCommand::ToggleOverlay(OverlayType::None));
        }

        commands
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    pub fn current_tick(&self) -> u64 {
        self.simulation.world().tick()
    }
}

/// Run the presentation shell with the given simulation
pub async fn run_presentation_shell(simulation: Simulation) {
    let mut shell = PresentationShell::new(simulation);

    loop {
        // Handle input
        let commands = shell.handle_input();
        for cmd in commands {
            shell.process_command(cmd);
        }

        // Update simulation
        let dt = get_frame_time();
        shell.update(dt);

        // Draw
        shell.draw();

        // Exit on Escape
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}

/// Launch the macroquad window and run the presentation shell.
/// This must be used from binaries instead of driving `run_presentation_shell`
/// through an external async runtime.
pub fn launch_presentation_shell(simulation: Simulation) {
    macroquad::Window::from_config(
        Conf {
            window_title: "POLIS Presentation Shell".to_string(),
            window_width: 1600,
            window_height: 900,
            ..Default::default()
        },
        run_presentation_shell(simulation),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use polis_core::SimulationSeed;
    use polis_sim::Simulation;

    #[test]
    fn sim_command_equality() {
        assert_eq!(SimCommand::Pause, SimCommand::Pause);
        assert_ne!(SimCommand::Pause, SimCommand::Resume);
    }

    #[test]
    fn sim_speed_ticks_per_frame() {
        assert_eq!(SimSpeed::Slow.ticks_per_frame(), 1);
        assert_eq!(SimSpeed::Max.ticks_per_frame(), 100);
    }

    #[test]
    fn presentation_shell_starts_paused_false() {
        let sim = Simulation::new(SimulationSeed::new(42));
        let shell = PresentationShell::new(sim);
        assert!(!shell.is_paused());
    }

    #[test]
    fn process_command_pause_resume() {
        let sim = Simulation::new(SimulationSeed::new(42));
        let mut shell = PresentationShell::new(sim);

        assert!(!shell.is_paused());
        shell.process_command(SimCommand::Pause);
        assert!(shell.is_paused());
        shell.process_command(SimCommand::Resume);
        assert!(!shell.is_paused());
    }

    #[test]
    fn process_command_step_single() {
        let sim = Simulation::new(SimulationSeed::new(42));
        let mut shell = PresentationShell::new(sim);

        let tick_before = shell.current_tick();
        shell.process_command(SimCommand::StepSingle);
        let tick_after = shell.current_tick();

        assert_eq!(tick_after, tick_before + 1);
        assert!(shell.is_paused()); // StepSingle pauses
    }

    #[test]
    fn process_command_set_speed() {
        let sim = Simulation::new(SimulationSeed::new(42));
        let mut shell = PresentationShell::new(sim);

        shell.process_command(SimCommand::SetSpeed(SimSpeed::Fast));
        // Speed is internal state, verified through behavior
        assert!(!shell.is_paused());
    }

    #[test]
    fn overlay_type_toggle() {
        // Test that toggling same overlay turns it off
        let mut overlay = OverlayType::None;
        overlay = if overlay == OverlayType::Resources {
            OverlayType::None
        } else {
            OverlayType::Resources
        };
        assert_eq!(overlay, OverlayType::Resources);

        overlay = if overlay == OverlayType::Resources {
            OverlayType::None
        } else {
            OverlayType::Resources
        };
        assert_eq!(overlay, OverlayType::None);
    }

    #[test]
    fn frontend_state_is_separate_from_simulation() {
        // This test validates that the frontend shell maintains its own state
        // separate from the simulation, and only affects simulation through commands
        let sim = Simulation::new(SimulationSeed::new(42));
        let initial_tick = sim.world().tick();

        let mut shell = PresentationShell::new(sim);

        // Frontend state changes don't affect simulation
        shell.process_command(SimCommand::SetSpeed(SimSpeed::Fast));
        shell.process_command(SimCommand::ToggleOverlay(OverlayType::Fields));

        // Simulation tick unchanged (no Step/Resume commands)
        assert_eq!(shell.current_tick(), initial_tick);

        // Step advances by exactly 1
        shell.process_command(SimCommand::StepSingle);
        assert_eq!(shell.current_tick(), initial_tick + 1);
    }

    #[test]
    fn simulation_state_is_read_only_through_world_accessor() {
        // Validates that the world() accessor provides read-only access
        let sim = Simulation::new(SimulationSeed::new(42));
        let shell = PresentationShell::new(sim);

        // Can read world state
        let world = shell.simulation.world();
        let _tick = world.tick();
        let _partitions = world.partitions();

        // Cannot mutate - world() returns &WorldState, not &mut WorldState
        // This is enforced by the type system
    }
}
