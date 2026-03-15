//! Knowledge and Discovery for Phase 6
//!
//! This module implements:
//! - Knowledge types and representation
//! - Discovery heuristics (repetition, search, accident, social transmission)
//! - Knowledge diffusion between agents
//! - Prerequisite chains for discoveries

use crate::AgentId;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// Unique identifier for a knowledge item
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
pub struct KnowledgeId(pub u64);

/// Types of knowledge in POLIS
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KnowledgeType {
    /// Practical techniques (toolmaking, building, crafting)
    Technique,
    /// Material properties and uses
    Material,
    /// Process knowledge (cooking, smelting, fermentation)
    Process,
    /// Ecological knowledge (seasons, animal behavior, plant cycles)
    Ecological,
    /// Social/institutional knowledge (rituals, governance)
    Social,
    /// Medical/healing knowledge
    Medical,
}

/// A piece of knowledge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Knowledge {
    pub id: KnowledgeId,
    pub name: String,
    pub knowledge_type: KnowledgeType,
    /// Prerequisites that must be known before this can be discovered
    pub prerequisites: Vec<KnowledgeId>,
    /// Discovery difficulty (0-100, higher = harder)
    pub difficulty: u8,
    /// How quickly this knowledge spreads socially
    pub transmission_ease: u8,
    /// How quickly this knowledge decays if not used/shared
    pub decay_rate: u8,
}

/// Discovery progress for a specific knowledge item
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiscoveryProgress {
    /// Current stage in discovery lifecycle
    pub stage: DiscoveryStage,
    /// Progress within current stage (0-100)
    pub stage_progress: u8,
    /// Total accumulated progress across all stages
    pub total_progress: u32,
    /// Tick when first observed
    pub first_observed_tick: u64,
    /// Tick when reached current stage
    pub current_stage_tick: u64,
}

impl DiscoveryProgress {
    pub fn new(tick: u64) -> Self {
        Self {
            stage: DiscoveryStage::AccidentalObservation,
            stage_progress: 0,
            total_progress: 0,
            first_observed_tick: tick,
            current_stage_tick: tick,
        }
    }

    /// Add progress and potentially advance stage
    /// Returns true if stage advanced
    pub fn add_progress(&mut self,
        amount: u8,
        difficulty: u8,
        tick: u64,
    ) -> (bool, Option<DiscoveryStage>) {
        self.stage_progress = (self.stage_progress + amount).min(100);
        self.total_progress += amount as u32;

        // Check for stage advancement (threshold based on difficulty)
        let threshold = 50 + (difficulty / 4); // 50-75 range
        if self.stage_progress >= threshold {
            if let Some(next_stage) = self.stage.next() {
                let old_stage = self.stage;
                self.stage = next_stage;
                self.stage_progress = 0;
                self.current_stage_tick = tick;
                return (true, Some(old_stage));
            }
        }

        (false, None)
    }

    /// Check if knowledge is fully discovered (at Technique stage or beyond)
    pub fn is_discovered(&self) -> bool {
        self.stage.as_u8() >= DiscoveryStage::Technique.as_u8()
    }
}

/// An agent's knowledge state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentKnowledge {
    /// Knowledge items this agent knows (at Technique+ stage)
    known: BTreeSet<KnowledgeId>,
    /// Proficiency in each knowledge (0-100)
    proficiency: BTreeMap<KnowledgeId, u8>,
    /// Discovery progress for each knowledge item
    discovery_progress: BTreeMap<KnowledgeId, DiscoveryProgress>,
    /// Last tick this knowledge was used
    last_used: BTreeMap<KnowledgeId, u64>,
}

impl AgentKnowledge {
    pub fn new() -> Self {
        Self {
            known: BTreeSet::new(),
            proficiency: BTreeMap::new(),
            discovery_progress: BTreeMap::new(),
            last_used: BTreeMap::new(),
        }
    }

    /// Check if agent knows a knowledge item (at Technique+ stage)
    pub fn knows(&self, id: KnowledgeId) -> bool {
        self.known.contains(&id)
    }

    /// Check discovery stage for a knowledge item
    pub fn discovery_stage(&self, id: KnowledgeId) -> Option<DiscoveryStage> {
        self.discovery_progress.get(&id).map(|p| p.stage)
    }

    /// Learn a new knowledge item (reaches Technique stage)
    pub fn learn(&mut self, id: KnowledgeId, tick: u64) {
        self.known.insert(id);
        self.proficiency.entry(id).or_insert(10); // Start with basic proficiency
        self.last_used.insert(id, tick);

        // Ensure discovery progress reflects known state
        if let Some(progress) = self.discovery_progress.get_mut(&id) {
            if progress.stage.as_u8() < DiscoveryStage::Technique.as_u8() {
                progress.stage = DiscoveryStage::Technique;
                progress.current_stage_tick = tick;
            }
        } else {
            let mut progress = DiscoveryProgress::new(tick);
            progress.stage = DiscoveryStage::Technique;
            progress.current_stage_tick = tick;
            self.discovery_progress.insert(id, progress);
        }
    }

    /// Get proficiency in a knowledge
    pub fn proficiency(&self, id: KnowledgeId) -> u8 {
        self.proficiency.get(&id).copied().unwrap_or(0)
    }

    /// Improve proficiency through practice
    pub fn practice(&mut self, id: KnowledgeId, tick: u64) {
        if self.known.contains(&id) {
            let current = self.proficiency.entry(id).or_insert(10);
            *current = (*current + 1).min(100);
            self.last_used.insert(id, tick);
        }
    }

    /// Initialize discovery progress for a knowledge item
    pub fn init_discovery(&mut self, id: KnowledgeId, tick: u64) {
        if !self.discovery_progress.contains_key(&id) {
            self.discovery_progress.insert(id, DiscoveryProgress::new(tick));
        }
    }

    /// Add discovery progress and potentially advance stage
    /// Returns (stage_advanced, old_stage) if stage changed
    pub fn add_discovery_progress(
        &mut self,
        id: KnowledgeId,
        amount: u8,
        difficulty: u8,
        tick: u64,
    ) -> (bool, Option<DiscoveryStage>) {
        let progress = self
            .discovery_progress
            .entry(id)
            .or_insert_with(|| DiscoveryProgress::new(tick));

        let (advanced, old_stage) = progress.add_progress(amount, difficulty, tick);

        // If reached Technique stage, add to known
        if progress.is_discovered() && !self.known.contains(&id) {
            self.known.insert(id);
            self.proficiency.entry(id).or_insert(10);
            self.last_used.insert(id, tick);
        }

        (advanced, old_stage)
    }

    /// Get discovery progress
    pub fn get_discovery_progress(&self,
        id: KnowledgeId,
    ) -> Option<&DiscoveryProgress> {
        self.discovery_progress.get(&id)
    }

    /// Check if ready to discover (at Technique stage)
    pub fn ready_to_discover(&self, id: KnowledgeId) -> bool {
        self.known.contains(&id)
    }

    /// Apply knowledge decay
    pub fn apply_decay(&mut self, current_tick: u64, decay_rate: u8) {
        let to_decay: Vec<KnowledgeId> = self
            .last_used
            .iter()
            .filter(|(_, last_tick)| current_tick.saturating_sub(**last_tick) > 1000)
            .map(|(id, _)| *id)
            .collect();

        for id in to_decay {
            if let Some(prof) = self.proficiency.get_mut(&id) {
                *prof = prof.saturating_sub(decay_rate);
                if *prof == 0 {
                    self.known.remove(&id);
                    self.proficiency.remove(&id);
                    self.discovery_progress.remove(&id);
                }
            }
        }
    }

    /// Get all known knowledge
    pub fn known_knowledge(&self) -> &BTreeSet<KnowledgeId> {
        &self.known
    }

    /// Get total knowledge count
    pub fn knowledge_count(&self) -> usize {
        self.known.len()
    }

    /// Get average proficiency
    pub fn average_proficiency(&self) -> u8 {
        if self.proficiency.is_empty() {
            return 0;
        }
        let total: u16 = self.proficiency.values().map(|p| *p as u16).sum();
        (total / self.proficiency.len() as u16) as u8
    }

    /// Get count of knowledge items at each discovery stage
    pub fn stage_counts(&self) -> BTreeMap<DiscoveryStage, usize> {
        let mut counts = BTreeMap::new();
        for progress in self.discovery_progress.values() {
            *counts.entry(progress.stage).or_insert(0) += 1;
        }
        counts
    }
}

impl Default for AgentKnowledge {
    fn default() -> Self {
        Self::new()
    }
}

/// Knowledge registry for the simulation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KnowledgeRegistry {
    knowledge_items: BTreeMap<KnowledgeId, Knowledge>,
    next_id: u64,
}

impl KnowledgeRegistry {
    pub fn new() -> Self {
        Self {
            knowledge_items: BTreeMap::new(),
            next_id: 1,
        }
    }

    /// Register a new knowledge item
    pub fn register(&mut self, name: &str, knowledge_type: KnowledgeType) -> KnowledgeId {
        let id = KnowledgeId(self.next_id);
        self.next_id += 1;

        let knowledge = Knowledge {
            id,
            name: name.to_string(),
            knowledge_type,
            prerequisites: Vec::new(),
            difficulty: 50,
            transmission_ease: 50,
            decay_rate: 5,
        };

        self.knowledge_items.insert(id, knowledge);
        id
    }

    /// Register with full configuration
    pub fn register_with_config(
        &mut self,
        name: &str,
        knowledge_type: KnowledgeType,
        prerequisites: Vec<KnowledgeId>,
        difficulty: u8,
        transmission_ease: u8,
        decay_rate: u8,
    ) -> KnowledgeId {
        let id = KnowledgeId(self.next_id);
        self.next_id += 1;

        let knowledge = Knowledge {
            id,
            name: name.to_string(),
            knowledge_type,
            prerequisites,
            difficulty,
            transmission_ease,
            decay_rate,
        };

        self.knowledge_items.insert(id, knowledge);
        id
    }

    /// Get a knowledge item
    pub fn get(&self, id: KnowledgeId) -> Option<&Knowledge> {
        self.knowledge_items.get(&id)
    }

    /// Get all knowledge of a type
    pub fn by_type(&self, knowledge_type: KnowledgeType) -> Vec<&Knowledge> {
        self.knowledge_items
            .values()
            .filter(|k| k.knowledge_type == knowledge_type)
            .collect()
    }

    /// Get all knowledge
    pub fn all(&self) -> &BTreeMap<KnowledgeId, Knowledge> {
        &self.knowledge_items
    }

    /// Check if prerequisites are met for an agent
    pub fn prerequisites_met(&self, knowledge_id: KnowledgeId, agent_knowledge: &AgentKnowledge) -> bool {
        if let Some(knowledge) = self.get(knowledge_id) {
            knowledge.prerequisites.iter().all(|pre| agent_knowledge.knows(*pre))
        } else {
            false
        }
    }

    /// Get discoverable knowledge for an agent (prerequisites met but not yet known)
    pub fn discoverable_for(&self, agent_knowledge: &AgentKnowledge) -> Vec<KnowledgeId> {
        self.knowledge_items
            .keys()
            .filter(|id| {
                !agent_knowledge.knows(**id) && self.prerequisites_met(**id, agent_knowledge)
            })
            .copied()
            .collect()
    }

    /// Initialize default knowledge catalog
    pub fn initialize_default_catalog(&mut self) {
        // Basic techniques
        let toolmaking = self.register("Basic Toolmaking", KnowledgeType::Technique);
        let fire_making = self.register("Fire Making", KnowledgeType::Technique);
        let shelter_building = self.register("Shelter Building", KnowledgeType::Technique);

        // Materials
        let stone_working = self.register("Stone Working", KnowledgeType::Material);
        let wood_working = self.register("Wood Working", KnowledgeType::Material);

        // Processes (require techniques)
        let _cooking = self.register_with_config(
            "Cooking",
            KnowledgeType::Process,
            vec![fire_making],
            40,
            60,
            3,
        );

        let _pottery = self.register_with_config(
            "Pottery",
            KnowledgeType::Process,
            vec![fire_making],
            60,
            40,
            2,
        );

        // Ecological
        let _plant_identification = self.register("Plant Identification", KnowledgeType::Ecological);
        let _animal_tracking = self.register("Animal Tracking", KnowledgeType::Ecological);
        let _seasonal_cycles = self.register("Seasonal Cycles", KnowledgeType::Ecological);

        // Social
        let _basic_rituals = self.register("Basic Rituals", KnowledgeType::Social);
        let _conflict_resolution = self.register("Conflict Resolution", KnowledgeType::Social);

        // Medical
        let _herbal_medicine = self.register_with_config(
            "Herbal Medicine",
            KnowledgeType::Medical,
            vec![], // Could require plant identification
            70,
            30,
            10,
        );

        // Advanced techniques (require prerequisites)
        let _advanced_tools = self.register_with_config(
            "Advanced Toolmaking",
            KnowledgeType::Technique,
            vec![toolmaking, stone_working],
            70,
            40,
            4,
        );
    }
}

/// Discovery lifecycle stages (6-stage process from 05_DiscoveryHeuristics.md)
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
pub enum DiscoveryStage {
    /// Stage 1: Accidental observation - episodic, no generalization
    AccidentalObservation,
    /// Stage 2: Affordance candidate - repeated observations create candidate
    AffordanceCandidate,
    /// Stage 3: Process schema - affordances chained toward goal
    ProcessSchema,
    /// Stage 4: Technique - intentionally reenactable with reliable outcomes
    Technique,
    /// Stage 5: Codified knowledge - represented beyond tacit execution
    CodifiedKnowledge,
    /// Stage 6: Institutionalized practice - anchored in institution
    InstitutionalizedPractice,
}

impl DiscoveryStage {
    /// Get the next stage in the lifecycle
    pub fn next(self) -> Option<Self> {
        match self {
            DiscoveryStage::AccidentalObservation => Some(DiscoveryStage::AffordanceCandidate),
            DiscoveryStage::AffordanceCandidate => Some(DiscoveryStage::ProcessSchema),
            DiscoveryStage::ProcessSchema => Some(DiscoveryStage::Technique),
            DiscoveryStage::Technique => Some(DiscoveryStage::CodifiedKnowledge),
            DiscoveryStage::CodifiedKnowledge => Some(DiscoveryStage::InstitutionalizedPractice),
            DiscoveryStage::InstitutionalizedPractice => None, // Terminal stage
        }
    }

    /// Get numeric value for ordering
    pub fn as_u8(self) -> u8 {
        match self {
            DiscoveryStage::AccidentalObservation => 1,
            DiscoveryStage::AffordanceCandidate => 2,
            DiscoveryStage::ProcessSchema => 3,
            DiscoveryStage::Technique => 4,
            DiscoveryStage::CodifiedKnowledge => 5,
            DiscoveryStage::InstitutionalizedPractice => 6,
        }
    }
}

/// Discovery event types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DiscoveryMethod {
    /// Discovered through repeated practice/experimentation
    Repetition,
    /// Discovered through active search/exploration
    Search,
    /// Discovered by accident
    Accident,
    /// Learned from another agent
    SocialTransmission,
}

/// A discovery event
#[derive(Debug, Clone)]
pub struct DiscoveryEvent {
    pub agent_id: AgentId,
    pub knowledge_id: KnowledgeId,
    pub method: DiscoveryMethod,
    pub tick: u64,
}

/// Knowledge statistics for metrics
#[derive(Debug, Clone, Copy, Default)]
pub struct KnowledgeStatistics {
    pub total_knowledge_items: u64,
    pub total_known_instances: u64,
    pub average_proficiency: u8,
    pub discoveries_this_tick: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agent_learns_knowledge() {
        let mut agent_knowledge = AgentKnowledge::new();
        let knowledge_id = KnowledgeId(1);

        assert!(!agent_knowledge.knows(knowledge_id));

        agent_knowledge.learn(knowledge_id, 0);

        assert!(agent_knowledge.knows(knowledge_id));
        assert_eq!(agent_knowledge.proficiency(knowledge_id), 10);
    }

    #[test]
    fn proficiency_improves_with_practice() {
        let mut agent_knowledge = AgentKnowledge::new();
        let knowledge_id = KnowledgeId(1);

        agent_knowledge.learn(knowledge_id, 0);
        agent_knowledge.practice(knowledge_id, 1);
        agent_knowledge.practice(knowledge_id, 2);

        assert_eq!(agent_knowledge.proficiency(knowledge_id), 12);
    }

    #[test]
    fn knowledge_registry_prerequisites() {
        let mut registry = KnowledgeRegistry::new();

        let basic = registry.register("Basic", KnowledgeType::Technique);
        let advanced = registry.register_with_config(
            "Advanced",
            KnowledgeType::Technique,
            vec![basic],
            50,
            50,
            5,
        );

        let mut agent_knowledge = AgentKnowledge::new();

        // Can't discover advanced without basic
        assert!(!registry.prerequisites_met(advanced, &agent_knowledge));

        agent_knowledge.learn(basic, 0);

        // Now can discover advanced
        assert!(registry.prerequisites_met(advanced, &agent_knowledge));
    }

    #[test]
    fn discoverable_knowledge() {
        let mut registry = KnowledgeRegistry::new();

        let basic = registry.register("Basic", KnowledgeType::Technique);
        let advanced = registry.register_with_config(
            "Advanced",
            KnowledgeType::Technique,
            vec![basic],
            50,
            50,
            5,
        );

        let mut agent_knowledge = AgentKnowledge::new();

        // Nothing discoverable without prerequisites
        let discoverable = registry.discoverable_for(&agent_knowledge);
        assert!(discoverable.contains(&basic));
        assert!(!discoverable.contains(&advanced));

        agent_knowledge.learn(basic, 0);

        // Now advanced is discoverable
        let discoverable = registry.discoverable_for(&agent_knowledge);
        assert!(discoverable.contains(&advanced));
    }

    #[test]
    fn knowledge_decay() {
        let mut agent_knowledge = AgentKnowledge::new();
        let knowledge_id = KnowledgeId(1);

        agent_knowledge.learn(knowledge_id, 0);
        agent_knowledge.practice(knowledge_id, 0);

        // Set last used to tick 0, decay at tick 1500
        agent_knowledge.apply_decay(1500, 10);

        // Proficiency should have decayed
        assert!(agent_knowledge.proficiency(knowledge_id) < 11);
    }

    #[test]
    fn discovery_stage_progression() {
        let mut progress = DiscoveryProgress::new(0);

        assert_eq!(progress.stage, DiscoveryStage::AccidentalObservation);
        assert!(!progress.is_discovered());

        // Add progress to advance through stages (threshold is 50 + difficulty/4 = ~62)
        // Stage progression: AccidentalObservation -> AffordanceCandidate -> ProcessSchema -> Technique
        let (advanced, old) = progress.add_progress(70, 50, 10);
        assert!(advanced);
        assert_eq!(old, Some(DiscoveryStage::AccidentalObservation));
        assert_eq!(progress.stage, DiscoveryStage::AffordanceCandidate);

        // Continue to Technique (need 3 advances total from start)
        progress.add_progress(70, 50, 20); // ProcessSchema
        progress.add_progress(70, 50, 30); // Technique
        assert_eq!(progress.stage, DiscoveryStage::Technique);
        assert!(progress.is_discovered());
    }

    #[test]
    fn agent_discovery_progress_tracks_stages() {
        let mut agent = AgentKnowledge::new();
        let id = KnowledgeId(1);

        // Initialize discovery
        agent.init_discovery(id, 0);
        assert_eq!(agent.discovery_stage(id), Some(DiscoveryStage::AccidentalObservation));

        // Add progress (threshold ~62)
        // Stage progression: AccidentalObservation -> AffordanceCandidate -> ProcessSchema -> Technique
        let (advanced, _) = agent.add_discovery_progress(id, 70, 50, 10);
        assert!(advanced);
        assert_eq!(agent.discovery_stage(id), Some(DiscoveryStage::AffordanceCandidate));
        assert!(!agent.knows(id)); // Not yet at Technique

        // Progress to Technique (need 3 advances total from start)
        agent.add_discovery_progress(id, 70, 50, 20); // ProcessSchema
        agent.add_discovery_progress(id, 70, 50, 30); // Technique

        // Now should be known
        assert!(agent.knows(id));
        assert_eq!(agent.discovery_stage(id), Some(DiscoveryStage::Technique));
    }

    #[test]
    fn discovery_stage_counts() {
        let mut agent = AgentKnowledge::new();

        // Create progress at different stages
        agent.init_discovery(KnowledgeId(1), 0);
        agent.init_discovery(KnowledgeId(2), 0);
        agent.init_discovery(KnowledgeId(3), 0);

        // Advance some (threshold ~62)
        // ID 1: AccidentalObservation -> AffordanceCandidate
        agent.add_discovery_progress(KnowledgeId(1), 70, 50, 10);
        // ID 2: AccidentalObservation -> AffordanceCandidate -> ProcessSchema
        agent.add_discovery_progress(KnowledgeId(2), 70, 50, 10);
        agent.add_discovery_progress(KnowledgeId(2), 70, 50, 20);

        let counts = agent.stage_counts();
        assert_eq!(counts.get(&DiscoveryStage::AccidentalObservation), Some(&1)); // ID 3
        assert_eq!(counts.get(&DiscoveryStage::AffordanceCandidate), Some(&1)); // ID 1
        assert_eq!(counts.get(&DiscoveryStage::ProcessSchema), Some(&1)); // ID 2
    }
}
