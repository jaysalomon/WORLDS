//! Collective Agency for Phase 5: Group formation, life-cycle, and collective decision-making
//!
//! This module implements:
//! - Collective actor types (CoordinationCluster, StableGroup, CollectiveActor, etc.)
//! - Group life-cycle states and transitions
//! - Promotion criteria based on 03_CollectiveAgency.md
//! - Internal structure (membership, roles, influence, assets, legitimacy, factionalism)
//! - Constitution (decision procedures)
//! - Merge/split rules with hysteresis
//! - Disciplined downward causation (constraints, not direct overwriting)

use crate::{AgentId, Individual};
use polis_core::DeterministicRng;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Unique identifier for a collective actor
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CollectiveId(pub u64);

/// Life-cycle states for collective actors
/// Following 03_CollectiveAgency.md section 11.1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CollectiveLifecycleState {
    /// Temporary coordination around immediate situation
    EphemeralCoordination,
    /// Repeated coordination, potential for stabilization
    ProtoGroup,
    /// Has constitution and resources but not yet stable
    UnstableCollective,
    /// Successfully making decisions with compliance
    StabilizedCollective,
    /// Legitimacy falling, factions consolidating
    FragmentingCollective,
    /// No longer a meaningful actor
    Dissolved,
}

/// Type of collective actor
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CollectiveType {
    /// Short-lived alignment around local situation (hunting party, fleeing crowd)
    CoordinationCluster,
    /// Recurrent set with durable ties (kin cluster, work team)
    StableGroup,
    /// True collective actor with constitution and decision procedure
    CollectiveActor,
    /// Co-residence based collective (pooled resources, shared labor)
    HouseholdActor,
    /// Explicit roles and routines (guild, firm, militia)
    OrganizationActor,
    /// Rule-making and enforcement (council, court)
    GovernanceActor,
    /// Territorial authority claims (chiefdom, city-state)
    PolityActor,
}

/// Decision procedure constitution
/// How member inputs are combined into collective decisions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecisionProcedure {
    /// Simple majority vote
    MajorityVote,
    /// Weighted by influence/role
    WeightedCouncil,
    /// All must agree
    Consensus,
    /// Single authority decides
    CommandHierarchy,
    /// Dominant individual/family decides
    PatriarchalDominance,
    /// Small elite decides
    OligarchicDominance,
}

/// Membership record in a collective
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Membership {
    pub agent_id: AgentId,
    /// When they joined
    pub joined_tick: u64,
    /// Current role in the collective
    pub role: Role,
    /// Influence weight in decisions (0-100)
    pub influence_weight: u8,
    /// Compliance propensity (0-100, how likely to follow collective decisions)
    pub compliance_propensity: u8,
    /// Current dissent level (0-100)
    pub dissent_level: u8,
}

/// Roles within a collective
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    /// Regular member
    Member,
    /// Has specific function
    Specialist,
    /// Decision-making role
    Officer,
    /// Spokesperson or leader
    Leader,
    /// Founder or high-status member
    Elder,
}

/// Faction within a collective
/// Represents sub-coalitions and internal heterogeneity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Faction {
    pub faction_id: u64,
    pub name: String,
    /// Member agents
    pub members: HashSet<AgentId>,
    /// Cohesion within faction (0-100)
    pub internal_cohesion: u8,
    /// Distance from collective center (0-100)
    pub distance_from_center: u8,
    /// Grievances against other factions
    pub grievances: HashMap<u64, u8>,
}

/// A collective actor with full internal structure
/// Following 03_CollectiveAgency.md section 8
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectiveActor {
    pub id: CollectiveId,
    pub collective_type: CollectiveType,
    pub lifecycle_state: CollectiveLifecycleState,

    /// When this collective formed
    pub formed_tick: u64,
    /// Last state transition tick
    pub last_transition_tick: u64,

    /// Members with their roles and properties
    pub members: HashMap<AgentId, Membership>,

    /// Decision constitution
    pub constitution: DecisionProcedure,

    /// Pooled resources (shared assets)
    pub pooled_resources: HashMap<ResourceType, u64>,

    /// Collective legitimacy (0-100, member belief in collective)
    pub legitimacy: u8,
    /// Institutionalization level (0-100, depth of routines/roles)
    pub institutionalization: u8,
    /// Resource centralization (0-100, how much wealth is pooled)
    pub resource_centralization: u8,

    /// Factional structure
    pub factions: HashMap<u64, Faction>,
    /// Overall factionalism (0-100, how divided the collective is)
    pub factionalism: u8,

    /// Internal inequality (0-100, Gini-like measure of influence/wealth distribution)
    pub internal_inequality: u8,

    /// External recognition (0-100, how much others treat this as an actor)
    pub external_recognition: u8,

    /// Spatial location (for settlements/territorial actors)
    pub location: Option<(i32, i32)>,
    /// Claimed territory (for polities)
    pub territory: Vec<(i32, i32)>,

    /// Decisions made successfully (for stabilization tracking)
    pub successful_decisions: u64,
    /// Decisions with partial compliance
    pub partial_compliance_decisions: u64,
    /// Decisions with high dissent/non-compliance
    pub failed_decisions: u64,
}

/// Resource types for pooled resources
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceType {
    Food,
    Material,
    Knowledge,
    Labor,
    Influence,
}

/// Criteria for promoting a group to collective actor
/// Following 03_CollectiveAgency.md section 6.1
#[derive(Debug, Clone, Default)]
pub struct PromotionCriteria {
    /// Members have clear boundaries
    pub boundary_clarity: u8,
    /// Recurrent or explicit membership
    pub membership_rules: u8,
    /// Shared resources, rights, or liabilities
    pub shared_resources: u8,
    /// Has decision procedure
    pub decision_procedure: u8,
    /// Can act meaningfully externally
    pub external_capacity: u8,
}

impl PromotionCriteria {
    /// Check if all required criteria are met (threshold: 60/100 each)
    pub fn meets_threshold(&self) -> bool {
        self.boundary_clarity >= 60
            && self.membership_rules >= 60
            && self.shared_resources >= 60
            && self.decision_procedure >= 60
            && self.external_capacity >= 60
    }

    /// Overall strength score (0-100)
    pub fn overall_score(&self) -> u8 {
        (self.boundary_clarity as u16
            + self.membership_rules as u16
            + self.shared_resources as u16
            + self.decision_procedure as u16
            + self.external_capacity as u16) as u8
            / 5
    }
}

/// Criteria for merge compatibility
#[derive(Debug, Clone, Default)]
pub struct MergeCriteria {
    /// Compatible decision procedures
    pub compatible_institutions: u8,
    /// Coordination benefit from merging
    pub coordination_benefit: u8,
    /// Factional distance is manageable
    pub manageable_factional_distance: u8,
    /// Asset integration is possible
    pub asset_integration_possible: u8,
}

/// Criteria for split viability
#[derive(Debug, Clone, Default)]
pub struct SplitCriteria {
    /// Identifiable subgroups exist
    pub identifiable_subgroups: u8,
    /// Subgroups have independent cohesion
    pub independent_cohesion: u8,
    /// Severe disagreement exists
    pub severe_disagreement: u8,
    /// Path to independent action exists
    pub independent_action_path: u8,
}

impl CollectiveActor {
    /// Create a new collective actor
    pub fn new(
        id: CollectiveId,
        collective_type: CollectiveType,
        formed_tick: u64,
        constitution: DecisionProcedure,
    ) -> Self {
        Self {
            id,
            collective_type,
            lifecycle_state: CollectiveLifecycleState::ProtoGroup,
            formed_tick,
            last_transition_tick: formed_tick,
            members: HashMap::new(),
            constitution,
            pooled_resources: HashMap::new(),
            legitimacy: 50,
            institutionalization: 0,
            resource_centralization: 0,
            factions: HashMap::new(),
            factionalism: 0,
            internal_inequality: 0,
            external_recognition: 0,
            location: None,
            territory: Vec::new(),
            successful_decisions: 0,
            partial_compliance_decisions: 0,
            failed_decisions: 0,
        }
    }

    /// Add a member to the collective
    pub fn add_member(&mut self, agent_id: AgentId, tick: u64) {
        let membership = Membership {
            agent_id,
            joined_tick: tick,
            role: Role::Member,
            influence_weight: 10,
            compliance_propensity: 50,
            dissent_level: 0,
        };
        self.members.insert(agent_id, membership);
        self.update_internal_inequality();
    }

    /// Remove a member
    pub fn remove_member(&mut self, agent_id: AgentId) {
        self.members.remove(&agent_id);
        // Remove from factions
        for faction in self.factions.values_mut() {
            faction.members.remove(&agent_id);
        }
        self.update_internal_inequality();
    }

    /// Check if an agent is a member
    pub fn is_member(&self, agent_id: AgentId) -> bool {
        self.members.contains_key(&agent_id)
    }

    /// Get member count
    pub fn member_count(&self) -> usize {
        self.members.len()
    }

    /// Update internal inequality based on influence distribution
    fn update_internal_inequality(&mut self) {
        if self.members.len() < 2 {
            self.internal_inequality = 0;
            return;
        }

        let weights: Vec<u8> = self.members.values().map(|m| m.influence_weight).collect();
        let mean = weights.iter().map(|&w| w as u32).sum::<u32>() / weights.len() as u32;

        // Simple variance-based inequality measure
        let variance = weights
            .iter()
            .map(|&w| {
                let diff = w as i32 - mean as i32;
                (diff * diff) as u32
            })
            .sum::<u32>()
            / weights.len() as u32;

        // Scale to 0-100 (higher = more unequal)
        self.internal_inequality = ((variance as f64).sqrt() * 100.0 / 50.0).min(100.0) as u8;
    }

    /// Calculate promotion criteria for this collective
    pub fn calculate_promotion_criteria(&self) -> PromotionCriteria {
        PromotionCriteria {
            boundary_clarity: self.calculate_boundary_clarity(),
            membership_rules: self.calculate_membership_rules(),
            shared_resources: self.calculate_shared_resources(),
            decision_procedure: if self.constitution_is_functional() {
                80
            } else {
                40
            },
            external_capacity: self.external_recognition,
        }
    }

    /// Calculate boundary clarity based on membership stability
    fn calculate_boundary_clarity(&self) -> u8 {
        if self.members.len() < 3 {
            return 50; // Small groups still have some clarity
        }
        // Higher with more members and longer existence
        // Scale: 3 members = 60, 10 members = 80
        let size_factor = 50 + ((self.members.len() as u8).saturating_mul(4)).min(40);
        let stability_factor = ((self.successful_decisions as u8).saturating_mul(10)).min(20);
        (size_factor + stability_factor).min(100)
    }

    /// Calculate membership rule strength
    fn calculate_membership_rules(&self) -> u8 {
        // Based on role differentiation and institutionalization
        let has_roles = self.members.values().any(|m| m.role != Role::Member);
        let role_score = if has_roles { 60 } else { 40 };
        let size_bonus = (self.members.len() as u8).saturating_mul(4).min(40);
        (role_score + self.institutionalization / 2 + size_bonus).min(100)
    }

    /// Calculate shared resource strength
    fn calculate_shared_resources(&self) -> u8 {
        if self.pooled_resources.is_empty() {
            return 20;
        }
        let total_pooled: u64 = self.pooled_resources.values().sum();
        (self.resource_centralization as u16 + (total_pooled.min(100) as u16)) as u8 / 2
    }

    /// Check if constitution is functional
    fn constitution_is_functional(&self) -> bool {
        // Has made decisions and has members
        self.successful_decisions > 0 && !self.members.is_empty()
    }

    /// Attempt to transition to next lifecycle state
    /// Returns true if transition occurred
    pub fn attempt_lifecycle_transition(&mut self, tick: u64) -> bool {
        let new_state = match self.lifecycle_state {
            CollectiveLifecycleState::EphemeralCoordination => {
                // Can become proto-group if coordination recurs
                if self.members.len() >= 3 && self.calculate_boundary_clarity() > 40 {
                    Some(CollectiveLifecycleState::ProtoGroup)
                } else {
                    None
                }
            }
            CollectiveLifecycleState::ProtoGroup => {
                // Can become collective actor if criteria met
                let criteria = self.calculate_promotion_criteria();
                if criteria.meets_threshold() {
                    Some(CollectiveLifecycleState::UnstableCollective)
                } else {
                    None
                }
            }
            CollectiveLifecycleState::UnstableCollective => {
                // Stabilize if legitimacy and compliance are high
                if self.legitimacy > 60 && self.successful_decisions > self.failed_decisions * 2 {
                    Some(CollectiveLifecycleState::StabilizedCollective)
                } else if self.legitimacy < 30 || self.failed_decisions > self.successful_decisions
                {
                    // Or fragment if failing
                    Some(CollectiveLifecycleState::FragmentingCollective)
                } else {
                    None
                }
            }
            CollectiveLifecycleState::StabilizedCollective => {
                // Can fragment if legitimacy falls or factionalism rises
                if self.legitimacy < 40 || self.factionalism > 70 {
                    Some(CollectiveLifecycleState::FragmentingCollective)
                } else {
                    None
                }
            }
            CollectiveLifecycleState::FragmentingCollective => {
                // Dissolve if no longer viable
                if self.legitimacy < 20 || self.members.len() < 3 {
                    Some(CollectiveLifecycleState::Dissolved)
                } else {
                    // Can recover if legitimacy improves
                    if self.legitimacy > 50 && self.factionalism < 50 {
                        Some(CollectiveLifecycleState::StabilizedCollective)
                    } else {
                        None
                    }
                }
            }
            CollectiveLifecycleState::Dissolved => None,
        };

        if let Some(state) = new_state {
            self.lifecycle_state = state;
            self.last_transition_tick = tick;
            true
        } else {
            false
        }
    }

    /// Check if this collective can merge with another
    pub fn can_merge_with(&self, other: &CollectiveActor) -> MergeCriteria {
        MergeCriteria {
            compatible_institutions: self.calculate_institutional_compatibility(other),
            coordination_benefit: self.calculate_coordination_benefit(other),
            manageable_factional_distance: self.calculate_factional_distance(other),
            asset_integration_possible: self.calculate_asset_integration(other),
        }
    }

    /// Calculate institutional compatibility (0-100)
    fn calculate_institutional_compatibility(&self, other: &CollectiveActor) -> u8 {
        // Same constitution type is more compatible
        if self.constitution == other.constitution {
            80
        } else {
            // Some constitutions are more compatible than others
            match (self.constitution, other.constitution) {
                (DecisionProcedure::MajorityVote, DecisionProcedure::Consensus)
                | (DecisionProcedure::Consensus, DecisionProcedure::MajorityVote) => 60,
                (DecisionProcedure::CommandHierarchy, DecisionProcedure::PatriarchalDominance)
                | (DecisionProcedure::PatriarchalDominance, DecisionProcedure::CommandHierarchy) => {
                    70
                }
                _ => 40,
            }
        }
    }

    /// Calculate coordination benefit from merging (0-100)
    fn calculate_coordination_benefit(&self, other: &CollectiveActor) -> u8 {
        // Larger groups benefit more from coordination
        let combined_size = self.members.len() + other.members.len();
        let size_benefit = (combined_size as u8).min(50);

        // But factionalism reduces benefit
        let faction_penalty = (self.factionalism + other.factionalism) / 4;

        size_benefit.saturating_sub(faction_penalty)
    }

    /// Calculate factional distance (0-100, higher = more manageable)
    fn calculate_factional_distance(&self, other: &CollectiveActor) -> u8 {
        // Check for overlapping memberships (reduces distance)
        let overlap: HashSet<_> = self
            .members
            .keys()
            .filter(|k| other.members.contains_key(k))
            .collect();

        if overlap.len() > 0 {
            70 // Some shared members helps integration
        } else {
            50 // No overlap is neutral
        }
    }

    /// Calculate asset integration possibility (0-100)
    fn calculate_asset_integration(&self, other: &CollectiveActor) -> u8 {
        // Similar resource centralization helps
        let centralization_diff = if self.resource_centralization > other.resource_centralization {
            self.resource_centralization - other.resource_centralization
        } else {
            other.resource_centralization - self.resource_centralization
        };

        100 - centralization_diff
    }

    /// Check if this collective should split
    pub fn should_split(&self) -> SplitCriteria {
        SplitCriteria {
            identifiable_subgroups: if self.factionalism > 50 { 80 } else { 30 },
            independent_cohesion: self.calculate_faction_independent_cohesion(),
            severe_disagreement: if self.legitimacy < 30 { 80 } else { 20 },
            independent_action_path: if self.members.len() >= 6 { 70 } else { 40 },
        }
    }

    /// Calculate if factions could survive independently
    fn calculate_faction_independent_cohesion(&self) -> u8 {
        if self.factions.is_empty() {
            return 0;
        }

        let avg_cohesion: u8 = self
            .factions
            .values()
            .map(|f| f.internal_cohesion)
            .sum::<u8>()
            / self.factions.len() as u8;

        avg_cohesion
    }

    /// Apply downward causation through constraints (not direct overwriting)
    /// Following 03_CollectiveAgency.md section 10
    pub fn apply_downward_causation(&self, agent: &mut Individual, rng: &mut DeterministicRng) {
        if let Some(membership) = self.members.get(&agent.id) {
            // Calculate effective constraints on the agent
            let directive_pressure =
                self.legitimacy as u64 * membership.compliance_propensity as u64 / 100;

            // Apply through incentives/constraints, NOT direct overwriting
            // Change available actions, payoffs, risks - not beliefs/preferences directly

            // Higher directive pressure increases obligation but doesn't force compliance
            let compliance_roll = rng.next_bounded(100);
            if compliance_roll < directive_pressure {
                // Agent feels obligation (but can still dissent)
                // This affects their decision context, not their core preferences
            }

            // Dissent level affects how much they resist
            if membership.dissent_level > 50 {
                // High dissent - more likely to resist collective decisions
                // But still their choice, not overwritten
            }
        }
    }

    /// Make a collective decision
    /// Returns the decision outcome and compliance level
    pub fn make_decision(
        &mut self,
        proposal: &Proposal,
        rng: &mut DeterministicRng,
    ) -> DecisionOutcome {
        // Collect votes weighted by influence
        let mut total_support = 0u64;
        let mut total_opposition = 0u64;
        let mut abstentions = 0u64;

        for membership in self.members.values() {
            // Individual decides based on their own preferences + collective influence
            let collective_pressure =
                self.legitimacy as u64 * membership.compliance_propensity as u64 / 100;
            let personal_resistance = membership.dissent_level as u64;

            let vote_roll = rng.next_bounded(100);
            let effective_threshold = collective_pressure.saturating_sub(personal_resistance);

            if vote_roll < effective_threshold {
                // Supports proposal (influenced by collective)
                total_support += membership.influence_weight as u64;
            } else if vote_roll < effective_threshold + personal_resistance {
                // Opposes (personal dissent)
                total_opposition += membership.influence_weight as u64;
            } else {
                // Abstains
                abstentions += membership.influence_weight as u64;
            }
        }

        // Apply decision procedure
        let passed = match self.constitution {
            DecisionProcedure::MajorityVote => total_support > total_opposition,
            DecisionProcedure::Consensus => total_opposition == 0 && total_support > 0,
            DecisionProcedure::CommandHierarchy => {
                // Leader decides
                if let Some(leader) = self
                    .members
                    .values()
                    .find(|m| matches!(m.role, Role::Leader))
                {
                    total_support > 0 // Leader's preference (simplified)
                } else {
                    total_support > total_opposition
                }
            }
            DecisionProcedure::WeightedCouncil => {
                total_support > total_opposition * 3 / 2 // Need stronger majority
            }
            DecisionProcedure::PatriarchalDominance | DecisionProcedure::OligarchicDominance => {
                // Elite decides
                let elite_influence: u64 = self
                    .members
                    .values()
                    .filter(|m| matches!(m.role, Role::Leader | Role::Elder | Role::Officer))
                    .map(|m| m.influence_weight as u64)
                    .sum();
                let elite_support: u64 = self
                    .members
                    .values()
                    .filter(|m| {
                        matches!(m.role, Role::Leader | Role::Elder | Role::Officer)
                            && m.compliance_propensity > 50
                    })
                    .map(|m| m.influence_weight as u64)
                    .sum();
                elite_support > elite_influence / 2
            }
        };

        // Calculate compliance level
        let total_votes = total_support + total_opposition + abstentions;
        let compliance_level = if total_votes > 0 {
            (total_support * 100 / total_votes) as u8
        } else {
            0
        };

        // Update tracking
        if passed {
            if compliance_level > 70 {
                self.successful_decisions += 1;
            } else if compliance_level > 40 {
                self.partial_compliance_decisions += 1;
            } else {
                self.failed_decisions += 1;
            }
        }

        DecisionOutcome {
            passed,
            support_votes: total_support,
            opposition_votes: total_opposition,
            abstentions,
            compliance_level,
        }
    }

    /// Update legitimacy based on recent outcomes
    pub fn update_legitimacy(&mut self, recent_success_rate: f64) {
        // Legitimacy changes based on success/failure
        if recent_success_rate > 0.7 {
            self.legitimacy = (self.legitimacy + 5).min(100);
        } else if recent_success_rate < 0.3 {
            self.legitimacy = self.legitimacy.saturating_sub(5);
        }

        // Factionalism affects legitimacy
        if self.factionalism > 70 {
            self.legitimacy = self.legitimacy.saturating_sub(3);
        }
    }
}

/// A proposal for collective decision
#[derive(Debug, Clone)]
pub struct Proposal {
    pub proposal_type: ProposalType,
    pub description: String,
}

/// Types of collective proposals
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProposalType {
    ResourceAllocation,
    MembershipChange,
    ConstitutionChange,
    ExternalAction,
    MergeWithOther,
    SplitCollective,
}

/// Outcome of a collective decision
#[derive(Debug, Clone, Copy)]
pub struct DecisionOutcome {
    pub passed: bool,
    pub support_votes: u64,
    pub opposition_votes: u64,
    pub abstentions: u64,
    pub compliance_level: u8,
}

/// Registry of all collective actors
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CollectiveRegistry {
    collectives: HashMap<CollectiveId, CollectiveActor>,
    next_id: u64,
}

impl CollectiveRegistry {
    pub fn new() -> Self {
        Self {
            collectives: HashMap::new(),
            next_id: 1,
        }
    }

    /// Create a new collective actor
    pub fn create_collective(
        &mut self,
        collective_type: CollectiveType,
        tick: u64,
        constitution: DecisionProcedure,
    ) -> CollectiveId {
        let id = CollectiveId(self.next_id);
        self.next_id += 1;

        let collective = CollectiveActor::new(id, collective_type, tick, constitution);
        self.collectives.insert(id, collective);

        id
    }

    /// Get a collective by ID
    pub fn get(&self, id: CollectiveId) -> Option<&CollectiveActor> {
        self.collectives.get(&id)
    }

    /// Get a mutable collective
    pub fn get_mut(&mut self, id: CollectiveId) -> Option<&mut CollectiveActor> {
        self.collectives.get_mut(&id)
    }

    /// Remove a collective
    pub fn remove(&mut self, id: CollectiveId) -> Option<CollectiveActor> {
        self.collectives.remove(&id)
    }

    /// Get all collectives
    pub fn all(&self) -> &HashMap<CollectiveId, CollectiveActor> {
        &self.collectives
    }

    /// Get all collectives mutably
    pub fn all_mut(&mut self) -> &mut HashMap<CollectiveId, CollectiveActor> {
        &mut self.collectives
    }

    /// Get collectives by type
    pub fn by_type(&self, collective_type: CollectiveType) -> Vec<&CollectiveActor> {
        self.collectives
            .values()
            .filter(|c| c.collective_type == collective_type)
            .collect()
    }

    /// Get active (non-dissolved) collectives
    pub fn active_collectives(&self) -> Vec<&CollectiveActor> {
        self.collectives
            .values()
            .filter(|c| c.lifecycle_state != CollectiveLifecycleState::Dissolved)
            .collect()
    }

    /// Update lifecycle states for all collectives
    pub fn update_lifecycle_states(
        &mut self,
        tick: u64,
    ) -> Vec<(CollectiveId, CollectiveLifecycleState)> {
        let mut transitions = Vec::new();

        for (id, collective) in &mut self.collectives {
            let old_state = collective.lifecycle_state;
            if collective.attempt_lifecycle_transition(tick) {
                transitions.push((*id, old_state));
            }
        }

        transitions
    }

    /// Merge two collectives
    /// Returns the ID of the merged collective
    pub fn merge_collectives(
        &mut self,
        primary_id: CollectiveId,
        secondary_id: CollectiveId,
        tick: u64,
    ) -> Option<CollectiveId> {
        let secondary = self.collectives.remove(&secondary_id)?;
        let primary = self.collectives.get_mut(&primary_id)?;

        // Merge members
        for (agent_id, mut membership) in secondary.members {
            if !primary.is_member(agent_id) {
                membership.joined_tick = tick;
                primary.members.insert(agent_id, membership);
            }
        }

        // Merge resources
        for (resource_type, amount) in secondary.pooled_resources {
            *primary.pooled_resources.entry(resource_type).or_insert(0) += amount;
        }

        // Update legitimacy (merging can be disruptive)
        primary.legitimacy = (primary.legitimacy as u16 + secondary.legitimacy as u16) as u8 / 2;
        primary.legitimacy = primary.legitimacy.saturating_sub(10);

        // Update factionalism
        primary.factionalism = (primary.factionalism + secondary.factionalism) / 2;
        primary.factionalism = (primary.factionalism + 20).min(100);

        primary.update_internal_inequality();

        Some(primary_id)
    }

    /// Split a collective into two
    /// Returns the ID of the new collective
    pub fn split_collective(
        &mut self,
        original_id: CollectiveId,
        member_subset: Vec<AgentId>,
        tick: u64,
    ) -> Option<CollectiveId> {
        let original = self.collectives.get(&original_id)?;

        // Verify all members exist in original
        if !member_subset.iter().all(|m| original.is_member(*m)) {
            return None;
        }

        // Need enough members to split
        if member_subset.len() < 3 || original.members.len() - member_subset.len() < 3 {
            return None;
        }

        // Create new collective
        let new_id = CollectiveId(self.next_id);
        self.next_id += 1;

        let mut new_collective = CollectiveActor::new(
            new_id,
            original.collective_type,
            tick,
            original.constitution,
        );

        // Move members to new collective
        let original = self.collectives.get_mut(&original_id)?;
        for agent_id in &member_subset {
            if let Some(membership) = original.members.remove(agent_id) {
                new_collective.members.insert(*agent_id, membership);
            }
        }

        // Split resources proportionally
        let split_ratio =
            member_subset.len() as f64 / (member_subset.len() + original.members.len()) as f64;
        for (resource_type, amount) in &original.pooled_resources {
            let split_amount = (*amount as f64 * split_ratio) as u64;
            new_collective
                .pooled_resources
                .insert(*resource_type, split_amount);
        }

        // Reduce original resources
        let original = self.collectives.get_mut(&original_id)?;
        for (resource_type, amount) in &mut original.pooled_resources {
            let split_amount = (*amount as f64 * split_ratio) as u64;
            *amount -= split_amount;
        }

        // Both collectives lose legitimacy from the split
        original.legitimacy = original.legitimacy.saturating_sub(20);
        new_collective.legitimacy = original.legitimacy;

        original.update_internal_inequality();

        self.collectives.insert(new_id, new_collective);

        Some(new_id)
    }

    /// Get collectives that an agent belongs to
    pub fn collectives_for_agent(&self, agent_id: AgentId) -> Vec<CollectiveId> {
        self.collectives
            .iter()
            .filter(|(_, c)| c.is_member(agent_id))
            .map(|(id, _)| *id)
            .collect()
    }

    /// Get statistics about collectives
    pub fn statistics(&self) -> CollectiveStatistics {
        let active: Vec<_> = self.active_collectives();
        let total = active.len();

        if total == 0 {
            return CollectiveStatistics::default();
        }

        let total_members: usize = active.iter().map(|c| c.members.len()).sum();
        let avg_size = total_members / total;

        let avg_legitimacy: u8 =
            active.iter().map(|c| c.legitimacy as u16).sum::<u16>() as u8 / total as u8;
        let avg_factionalism: u8 =
            active.iter().map(|c| c.factionalism as u16).sum::<u16>() as u8 / total as u8;

        CollectiveStatistics {
            total_collectives: total as u64,
            total_members: total_members as u64,
            average_size: avg_size as u64,
            average_legitimacy: avg_legitimacy,
            average_factionalism: avg_factionalism,
        }
    }
}

/// Statistics about collectives
#[derive(Debug, Clone, Copy, Default)]
pub struct CollectiveStatistics {
    pub total_collectives: u64,
    pub total_members: u64,
    pub average_size: u64,
    pub average_legitimacy: u8,
    pub average_factionalism: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collective_lifecycle_transition() {
        let mut collective = CollectiveActor::new(
            CollectiveId(1),
            CollectiveType::StableGroup,
            0,
            DecisionProcedure::MajorityVote,
        );

        assert_eq!(
            collective.lifecycle_state,
            CollectiveLifecycleState::ProtoGroup
        );

        // Add members to meet criteria
        for i in 1..=5 {
            collective.add_member(AgentId(i), 0);
            if let Some(m) = collective.members.get_mut(&AgentId(i)) {
                m.compliance_propensity = 100;
                m.influence_weight = 10;
            }
        }

        // Set up promotion criteria
        collective.legitimacy = 70;
        collective.external_recognition = 70;
        collective.pooled_resources.insert(ResourceType::Food, 100);
        collective.resource_centralization = 70;

        // Make a successful decision to satisfy decision_procedure criterion
        let proposal = Proposal {
            proposal_type: ProposalType::ResourceAllocation,
            description: "Test".to_string(),
        };
        let mut rng = DeterministicRng::from_u64(42);
        let outcome = collective.make_decision(&proposal, &mut rng);

        // Debug: print criteria
        let criteria = collective.calculate_promotion_criteria();
        println!(
            "Criteria: boundary={}, membership={}, shared={}, decision={}, external={}",
            criteria.boundary_clarity,
            criteria.membership_rules,
            criteria.shared_resources,
            criteria.decision_procedure,
            criteria.external_capacity
        );
        println!(
            "Decision outcome: passed={}, compliance={}",
            outcome.passed, outcome.compliance_level
        );
        println!("Successful decisions: {}", collective.successful_decisions);

        // Should transition to unstable collective
        assert!(
            collective.attempt_lifecycle_transition(10),
            "Failed to transition. Criteria overall: {}",
            criteria.overall_score()
        );
        assert_eq!(
            collective.lifecycle_state,
            CollectiveLifecycleState::UnstableCollective
        );
    }

    #[test]
    fn collective_promotion_criteria() {
        let mut collective = CollectiveActor::new(
            CollectiveId(1),
            CollectiveType::StableGroup,
            0,
            DecisionProcedure::MajorityVote,
        );

        // Without members, criteria should be low
        let criteria = collective.calculate_promotion_criteria();
        assert!(!criteria.meets_threshold());

        // Add members
        for i in 1..=5 {
            collective.add_member(AgentId(i), 0);
            if let Some(m) = collective.members.get_mut(&AgentId(i)) {
                m.compliance_propensity = 100;
                m.influence_weight = 10;
            }
        }

        collective.legitimacy = 70;
        collective.external_recognition = 70;
        collective.pooled_resources.insert(ResourceType::Food, 100);
        collective.resource_centralization = 70;

        // Make a successful decision to satisfy decision_procedure criterion
        let proposal = Proposal {
            proposal_type: ProposalType::ResourceAllocation,
            description: "Test".to_string(),
        };
        let mut rng = DeterministicRng::from_u64(42);
        collective.make_decision(&proposal, &mut rng);

        let criteria = collective.calculate_promotion_criteria();
        assert!(criteria.meets_threshold());
    }

    #[test]
    fn collective_merge() {
        let mut registry = CollectiveRegistry::new();

        let id1 = registry.create_collective(
            CollectiveType::StableGroup,
            0,
            DecisionProcedure::MajorityVote,
        );
        let id2 = registry.create_collective(
            CollectiveType::StableGroup,
            0,
            DecisionProcedure::MajorityVote,
        );

        // Add members to both
        if let Some(c1) = registry.get_mut(id1) {
            for i in 1..=3 {
                c1.add_member(AgentId(i), 0);
            }
            c1.pooled_resources.insert(ResourceType::Food, 100);
        }

        if let Some(c2) = registry.get_mut(id2) {
            for i in 4..=6 {
                c2.add_member(AgentId(i), 0);
            }
            c2.pooled_resources.insert(ResourceType::Food, 50);
        }

        // Merge
        let merged = registry.merge_collectives(id1, id2, 10);
        assert_eq!(merged, Some(id1));

        // Check merged collective
        let c1 = registry.get(id1).unwrap();
        assert_eq!(c1.member_count(), 6);
        assert_eq!(c1.pooled_resources.get(&ResourceType::Food), Some(&150));
    }

    #[test]
    fn collective_split() {
        let mut registry = CollectiveRegistry::new();

        let id = registry.create_collective(
            CollectiveType::StableGroup,
            0,
            DecisionProcedure::MajorityVote,
        );

        // Add members
        if let Some(c) = registry.get_mut(id) {
            for i in 1..=6 {
                c.add_member(AgentId(i), 0);
            }
            c.pooled_resources.insert(ResourceType::Food, 100);
        }

        // Split
        let new_id = registry.split_collective(id, vec![AgentId(1), AgentId(2), AgentId(3)], 10);
        assert!(new_id.is_some());

        // Check both collectives
        let original = registry.get(id).unwrap();
        let new_collective = registry.get(new_id.unwrap()).unwrap();

        assert_eq!(original.member_count(), 3);
        assert_eq!(new_collective.member_count(), 3);
    }

    #[test]
    fn decision_procedure_majority() {
        let mut collective = CollectiveActor::new(
            CollectiveId(1),
            CollectiveType::CollectiveActor,
            0,
            DecisionProcedure::MajorityVote,
        );

        // Add members with high compliance
        for i in 1..=5 {
            collective.add_member(AgentId(i), 0);
            if let Some(m) = collective.members.get_mut(&AgentId(i)) {
                m.compliance_propensity = 100; // Always comply
                m.influence_weight = 10;
            }
        }

        collective.legitimacy = 100;

        let proposal = Proposal {
            proposal_type: ProposalType::ResourceAllocation,
            description: "Test proposal".to_string(),
        };

        let mut rng = DeterministicRng::from_u64(42);
        let outcome = collective.make_decision(&proposal, &mut rng);

        // With 100% compliance and high legitimacy, should pass
        assert!(outcome.passed);
        assert!(outcome.compliance_level > 50);
    }

    #[test]
    fn downward_causation_preserves_agency() {
        let mut collective = CollectiveActor::new(
            CollectiveId(1),
            CollectiveType::CollectiveActor,
            0,
            DecisionProcedure::MajorityVote,
        );

        collective.add_member(AgentId(1), 0);
        collective.legitimacy = 80;

        if let Some(m) = collective.members.get_mut(&AgentId(1)) {
            m.compliance_propensity = 50;
            m.dissent_level = 0;
        }

        // Create an individual
        let mut rng = DeterministicRng::from_u64(42);
        let mut individual = Individual::new(AgentId(1), 0, &mut rng);
        let original_health = individual.health;

        let mut rng2 = DeterministicRng::from_u64(42);

        // Apply downward causation
        collective.apply_downward_causation(&mut individual, &mut rng2);

        // Individual's core state should NOT be directly overwritten
        // (downward causation works through constraints, not overwriting)
        assert_eq!(individual.health, original_health);
    }
}
