//! Social fabric for Phase 4: Social ties, trust/grievance, cooperation, and conflict
//!
//! This module implements:
//! - Social ties graph between agents (trust, grievance, interaction history)
//! - Deterministic trust/grievance updates from repeated interactions
//! - Cooperation rules gated by trust
//! - Conflict rules based on scarcity and grievance

use crate::AgentId;
use polis_core::DeterministicRng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A social tie between two agents
/// Tracks relationship quality and interaction history
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct SocialTie {
    /// Trust level (-100 to +100, positive = trust, negative = distrust)
    pub trust: i8,
    /// Grievance level (0 to 100, accumulated from negative interactions)
    pub grievance: u8,
    /// Number of interactions recorded
    pub interaction_count: u32,
    /// Tick of last interaction
    pub last_interaction_tick: u64,
    /// Cooperation success count (positive interactions)
    pub cooperation_success: u32,
    /// Conflict count (negative interactions)
    pub conflict_count: u32,
}

impl SocialTie {
    /// Create a new neutral social tie
    pub fn new(tick: u64) -> Self {
        Self {
            trust: 0,
            grievance: 0,
            interaction_count: 0,
            last_interaction_tick: tick,
            cooperation_success: 0,
            conflict_count: 0,
        }
    }

    /// Update trust based on an interaction outcome
    /// Positive outcome (>0) increases trust, negative decreases
    pub fn update_trust(&mut self, outcome: i8, tick: u64) {
        self.interaction_count += 1;
        self.last_interaction_tick = tick;

        // Trust changes based on outcome magnitude
        // Repeated small positive interactions build trust slowly
        // Large negative outcomes damage trust quickly
        let trust_change = if outcome > 0 {
            // Cooperation increases trust (diminishing returns at high trust)
            let max_increase = 10 - (self.trust / 10).min(10);
            (outcome as i16 * max_increase as i16 / 10).max(1) as i8
        } else {
            // Conflict decreases trust (accelerates at high grievance)
            let severity = (-(outcome as i32)) as i32; // outcome is negative, so this is positive
            let penalty = (severity * (100 + self.grievance as i32) / 100) as i16;
            -(penalty.max(1)) as i8
        };

        self.trust = (self.trust as i16 + trust_change as i16).clamp(-100, 100) as i8;

        // Update grievance on negative outcomes
        if outcome < 0 {
            self.grievance = self.grievance.saturating_add((-outcome) as u8).min(100);
            self.conflict_count += 1;
        } else {
            self.cooperation_success += 1;
        }
    }

    /// Natural decay of trust/grievance over time (forgetting)
    pub fn decay(&mut self, ticks_passed: u64) {
        // Trust slowly decays toward neutral
        let decay_amount = (ticks_passed / 100) as i8;
        if self.trust > 0 {
            self.trust = (self.trust - decay_amount).max(0);
        } else if self.trust < 0 {
            self.trust = (self.trust + decay_amount).min(0);
        }

        // Grievance decays slowly (harder to forget wrongs)
        self.grievance = self.grievance.saturating_sub((ticks_passed / 200) as u8);
    }

    /// Check if agents are likely to cooperate
    pub fn will_cooperate(&self, rng: &mut DeterministicRng) -> bool {
        // Higher trust = more likely to cooperate
        // Even at max trust, not guaranteed (individual agency preserved)
        let threshold = 50 - (self.trust as u64 * 40 / 100); // 50 at neutral, 10 at max trust
        rng.next_bounded(100) > threshold
    }

    /// Check if conflict is likely given current relationship
    pub fn conflict_likelihood(&self, scarcity_stress: u8) -> u8 {
        // Conflict more likely with:
        // - High grievance
        // - Low trust
        // - High scarcity stress
        let grievance_factor = self.grievance;
        let trust_factor = (100 - self.trust.abs() as u8).saturating_sub(50) * 2; // 0-100
        let scarcity_factor = scarcity_stress;

        // Weighted combination
        ((grievance_factor as u16 * 40 + trust_factor as u16 * 30 + scarcity_factor as u16 * 30)
            / 100) as u8
    }
}

/// Social network for an agent population
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SocialNetwork {
    /// Ties indexed by (agent_a_id, agent_b_id) where a < b
    ties: HashMap<(u64, u64), SocialTie>,
}

impl SocialNetwork {
    pub fn new() -> Self {
        Self {
            ties: HashMap::new(),
        }
    }

    /// Get or create a tie between two agents
    fn get_tie_key(a: AgentId, b: AgentId) -> (u64, u64) {
        if a.0 < b.0 { (a.0, b.0) } else { (b.0, a.0) }
    }

    /// Get a tie between two agents (returns None if no interaction yet)
    pub fn get_tie(&self, a: AgentId, b: AgentId) -> Option<&SocialTie> {
        self.ties.get(&Self::get_tie_key(a, b))
    }

    /// Get mutable access to a tie (creates if doesn't exist)
    pub fn get_or_create_tie(&mut self, a: AgentId, b: AgentId, tick: u64) -> &mut SocialTie {
        self.ties
            .entry(Self::get_tie_key(a, b))
            .or_insert_with(|| SocialTie::new(tick))
    }

    /// Record a positive interaction (cooperation)
    pub fn record_cooperation(&mut self, a: AgentId, b: AgentId, tick: u64) {
        let tie = self.get_or_create_tie(a, b, tick);
        tie.update_trust(5, tick);
    }

    /// Record a negative interaction (conflict)
    pub fn record_conflict(&mut self, a: AgentId, b: AgentId, severity: u8, tick: u64) {
        let tie = self.get_or_create_tie(a, b, tick);
        // Convert severity to negative outcome safely
        let outcome = -((severity as i16).min(100) as i8);
        tie.update_trust(outcome, tick);
    }

    /// Record a neutral interaction (just proximity/acknowledgment)
    pub fn record_neutral(&mut self, a: AgentId, b: AgentId, tick: u64) {
        let tie = self.get_or_create_tie(a, b, tick);
        tie.interaction_count += 1;
        tie.last_interaction_tick = tick;
    }

    /// Get all ties for an agent
    pub fn get_agent_ties(&self, agent_id: AgentId) -> Vec<(AgentId, &SocialTie)> {
        self.ties
            .iter()
            .filter_map(|((a, b), tie)| {
                if *a == agent_id.0 {
                    Some((AgentId(*b), tie))
                } else if *b == agent_id.0 {
                    Some((AgentId(*a), tie))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Apply time decay to all ties
    pub fn apply_decay(&mut self, current_tick: u64) {
        for tie in self.ties.values_mut() {
            let ticks_passed = current_tick.saturating_sub(tie.last_interaction_tick);
            if ticks_passed > 0 {
                tie.decay(ticks_passed);
            }
        }
    }

    /// Get network statistics
    pub fn statistics(&self) -> SocialNetworkStatistics {
        let total_ties = self.ties.len() as u64;
        if total_ties == 0 {
            return SocialNetworkStatistics::default();
        }

        let total_trust: i64 = self.ties.values().map(|t| t.trust as i64).sum();
        let total_grievance: u64 = self.ties.values().map(|t| t.grievance as u64).sum();
        let cooperation_count: u64 = self
            .ties
            .values()
            .map(|t| t.cooperation_success as u64)
            .sum();
        let conflict_count: u64 = self.ties.values().map(|t| t.conflict_count as u64).sum();

        SocialNetworkStatistics {
            total_ties,
            average_trust: (total_trust / total_ties as i64) as i8,
            average_grievance: (total_grievance / total_ties) as u8,
            total_cooperation: cooperation_count,
            total_conflict: conflict_count,
        }
    }

    /// Calculate social tension for a partition (higher = more conflict potential)
    pub fn partition_tension(&self, agent_ids: &[AgentId]) -> u8 {
        if agent_ids.len() < 2 {
            return 0;
        }

        let mut total_tension = 0u64;
        let mut pair_count = 0u64;

        for i in 0..agent_ids.len() {
            for j in (i + 1)..agent_ids.len() {
                if let Some(tie) = self.get_tie(agent_ids[i], agent_ids[j]) {
                    // Tension from low trust and high grievance
                    let tension = (100 - tie.trust.abs() as u64) + tie.grievance as u64;
                    total_tension += tension.min(200);
                    pair_count += 1;
                }
            }
        }

        if pair_count == 0 {
            return 0;
        }

        (total_tension / pair_count) as u8
    }
}

impl Default for SocialNetwork {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about the social network
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct SocialNetworkStatistics {
    pub total_ties: u64,
    pub average_trust: i8,
    pub average_grievance: u8,
    pub total_cooperation: u64,
    pub total_conflict: u64,
}

/// Cross-species interaction state for animals (Phase 4 domestication primitives)
/// Tracks how animals perceive and respond to human presence
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct CrossSpeciesState {
    /// Familiarity with humans (0-100, higher = more accustomed)
    pub familiarity: u8,
    /// Fear level (0-100, higher = more fearful)
    pub fear: u8,
    /// Aggression toward humans (0-100)
    pub aggression: u8,
    /// Tolerance of human proximity (0-100, higher = allows closer approach)
    pub human_tolerance: u8,
    /// Last interaction tick (for decay calculations)
    pub last_human_contact: u64,
    /// Cumulative positive interactions with humans
    pub positive_human_interactions: u32,
    /// Cumulative negative interactions with humans
    pub negative_human_interactions: u32,
}

impl CrossSpeciesState {
    /// Create default wild state (low familiarity, high fear)
    pub fn wild() -> Self {
        Self {
            familiarity: 5,
            fear: 80,
            aggression: 30,
            human_tolerance: 10,
            last_human_contact: 0,
            positive_human_interactions: 0,
            negative_human_interactions: 0,
        }
    }

    /// Create state for proto-domestic animals (higher baseline tolerance)
    pub fn proto_domestic() -> Self {
        Self {
            familiarity: 30,
            fear: 50,
            aggression: 20,
            human_tolerance: 40,
            last_human_contact: 0,
            positive_human_interactions: 0,
            negative_human_interactions: 0,
        }
    }

    /// Update state based on a human encounter
    /// contact_severity: negative = harsh/forceful, positive = gentle/provisioned
    /// proximity: distance of contact (lower = closer)
    pub fn update_from_human_contact(&mut self, contact_severity: i8, proximity: u8, tick: u64) {
        let ticks_since_last = tick.saturating_sub(self.last_human_contact);
        self.last_human_contact = tick;

        // Decay old states based on time passed
        self.decay(ticks_since_last);

        if contact_severity < 0 {
            // Negative contact (hunting, harsh handling, etc.)
            let severity = (-contact_severity) as u8;

            // Harsh contact increases fear and aggression
            self.fear = self.fear.saturating_add(severity * 2);
            self.aggression = self.aggression.saturating_add(severity);
            self.human_tolerance = self.human_tolerance.saturating_sub(severity);

            // But also increases familiarity (they know humans now, even if negatively)
            self.familiarity = self.familiarity.saturating_add(severity / 2);

            self.negative_human_interactions += 1;
        } else {
            // Positive contact (feeding, gentle presence, etc.)
            let benefit = contact_severity as u8;

            // Stable low-threat contact increases tolerance and reduces fear
            if proximity < self.human_tolerance {
                // Close contact within tolerance range builds trust
                self.human_tolerance = (self.human_tolerance + benefit).min(100);
                self.fear = self.fear.saturating_sub(benefit * 2);
                self.familiarity = (self.familiarity + benefit).min(100);
            }

            // Reduce aggression with positive contact
            self.aggression = self.aggression.saturating_sub(benefit);

            self.positive_human_interactions += 1;
        }

        // Bounds enforcement
        self.fear = self.fear.min(100);
        self.aggression = self.aggression.min(100);
        self.familiarity = self.familiarity.min(100);
    }

    /// Natural decay of cross-species states over time
    fn decay(&mut self, ticks_passed: u64) {
        // Fear slowly decays (animals forget fear)
        self.fear = self.fear.saturating_sub((ticks_passed / 500) as u8);

        // Familiarity slowly decays without contact
        self.familiarity = self.familiarity.saturating_sub((ticks_passed / 1000) as u8);

        // Aggression decays
        self.aggression = self.aggression.saturating_sub((ticks_passed / 800) as u8);
    }

    /// Check if animal is approaching domestication threshold
    /// Returns progress toward domestication (0-100)
    pub fn domestication_progress(&self) -> u8 {
        // Domestication requires:
        // - High tolerance (allows human proximity)
        // - Low fear (not fleeing)
        // - Low aggression (not attacking)
        // - High familiarity (recognizes humans)

        let tolerance_factor = self.human_tolerance;
        let calmness_factor = (100 - self.fear) + (100 - self.aggression) / 2;
        let familiarity_factor = self.familiarity;

        // Weighted: tolerance matters most, then calmness, then familiarity
        (tolerance_factor as u16 * 40
            + calmness_factor as u16 * 35
            + familiarity_factor as u16 * 25) as u8
            / 100
    }

    /// Check if animal will tolerate human at given proximity
    pub fn tolerates_proximity(&self, distance: u8) -> bool {
        distance <= self.human_tolerance
    }

    /// Check if animal is likely to flee from human
    pub fn will_flee(&self, rng: &mut DeterministicRng) -> bool {
        // Higher fear = more likely to flee
        let flee_threshold = self.fear as u64 * 80 / 100;
        rng.next_bounded(100) < flee_threshold
    }

    /// Check if animal is likely to attack human
    pub fn will_attack(&self, rng: &mut DeterministicRng) -> bool {
        // Aggression modified by fear (scared animals attack less)
        let effective_aggression = self.aggression.saturating_sub(self.fear / 3);
        let attack_threshold = effective_aggression as u64 * 60 / 100;
        rng.next_bounded(100) < attack_threshold
    }
}

impl Default for CrossSpeciesState {
    fn default() -> Self {
        Self::wild()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn social_tie_trust_builds_with_cooperation() {
        let mut tie = SocialTie::new(0);
        assert_eq!(tie.trust, 0);

        tie.update_trust(5, 1);
        assert!(tie.trust > 0);
        assert_eq!(tie.cooperation_success, 1);
    }

    #[test]
    fn social_tie_trust_drops_with_conflict() {
        let mut tie = SocialTie::new(0);
        tie.update_trust(10, 1); // Build some trust first
        let trust_before = tie.trust;

        tie.update_trust(-10, 2);
        assert!(tie.trust < trust_before);
        assert_eq!(tie.conflict_count, 1);
    }

    #[test]
    fn social_tie_grievance_accumulates() {
        let mut tie = SocialTie::new(0);
        assert_eq!(tie.grievance, 0);

        tie.update_trust(-5, 1);
        assert!(tie.grievance > 0);

        tie.update_trust(-5, 2);
        assert!(tie.grievance >= 5);
    }

    #[test]
    fn social_network_tracks_ties() {
        let mut network = SocialNetwork::new();
        let a = AgentId(1);
        let b = AgentId(2);

        network.record_cooperation(a, b, 10);

        let tie = network.get_tie(a, b).unwrap();
        assert!(tie.trust > 0);
        assert_eq!(tie.interaction_count, 1);
    }

    #[test]
    fn cross_species_harsh_contact_increases_fear() {
        let mut state = CrossSpeciesState::wild();
        let initial_fear = state.fear;

        state.update_from_human_contact(-10, 50, 100);

        assert!(state.fear > initial_fear);
        assert!(state.negative_human_interactions > 0);
    }

    #[test]
    fn cross_species_gentle_contact_increases_tolerance() {
        let mut state = CrossSpeciesState::wild();
        let initial_tolerance = state.human_tolerance;

        // Gentle contact within tolerance range
        state.update_from_human_contact(5, 5, 100);

        assert!(state.human_tolerance > initial_tolerance || state.fear < 80);
        assert!(state.positive_human_interactions > 0);
    }

    #[test]
    fn cross_species_domestication_progress_calculation() {
        let mut state = CrossSpeciesState::wild();
        let initial_progress = state.domestication_progress();
        // Wild state has some baseline progress (not zero due to familiarity/fear values)
        assert!(
            initial_progress < 30,
            "Wild state should have low domestication progress"
        );

        // Simulate gradual domestication
        for tick in 0..100 {
            state.update_from_human_contact(3, 5, tick);
        }

        // Progress should increase after positive interactions
        assert!(
            state.domestication_progress() > initial_progress,
            "Domestication progress should increase with positive contact"
        );
    }

    #[test]
    fn trust_decay_over_time() {
        let mut tie = SocialTie::new(0);
        tie.update_trust(20, 1);
        let trust_before = tie.trust;

        tie.decay(500);

        assert!(tie.trust <= trust_before);
    }
}
