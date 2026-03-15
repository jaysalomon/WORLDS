//! Probabilistic Logic Network (PLN) style inference layer for Phase 6 Pass 2
//!
//! This module implements:
//! - Advisory inference logic (NOT core deterministic replacement)
//! - Slower cadence inference (daily/weekly equivalent)
//! - Partition-level risk assessment
//! - Truth value representation with confidence
//! - Auditable risk events with contributing factors

use polis_core::DeterministicRng;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Unique identifier for a belief node in the inference network
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
pub struct BeliefId(pub u64);

/// Truth value with confidence (PLN-style)
/// strength: 0.0 to 1.0 (probability of truth)
/// confidence: 0.0 to 1.0 (certainty in the strength value)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct TruthValue {
    pub strength: f32,
    pub confidence: f32,
}

impl TruthValue {
    /// Create a new truth value
    pub fn new(strength: f32, confidence: f32) -> Self {
        Self {
            strength: strength.clamp(0.0, 1.0),
            confidence: confidence.clamp(0.0, 1.0),
        }
    }

    /// Create a default uncertain truth value
    pub fn uncertain() -> Self {
        Self {
            strength: 0.5,
            confidence: 0.0,
        }
    }

    /// Create a truth value from observation count
    /// positive: number of positive observations
    /// total: total number of observations
    pub fn from_observations(positive: u32, total: u32) -> Self {
        if total == 0 {
            return Self::uncertain();
        }
        let strength = (positive as f32) / (total as f32);
        // Confidence increases with more observations (saturating)
        let confidence = (total as f32 / (total as f32 + 10.0)).min(1.0);
        Self::new(strength, confidence)
    }

    /// Combine two truth values (revision)
    pub fn revise(&self, other: &TruthValue) -> TruthValue {
        // Simple weighted average based on confidence
        let total_confidence = self.confidence + other.confidence;
        if total_confidence == 0.0 {
            return TruthValue::uncertain();
        }
        let w1 = self.confidence / total_confidence;
        let w2 = other.confidence / total_confidence;
        let strength = self.strength * w1 + other.strength * w2;
        let confidence = (self.confidence + other.confidence - self.confidence * other.confidence).min(1.0);
        TruthValue::new(strength, confidence)
    }

    /// Check if this truth value indicates high probability
    pub fn is_likely(&self, threshold: f32) -> bool {
        self.strength >= threshold && self.confidence >= 0.3
    }

    /// Check if this truth value indicates low probability
    pub fn is_unlikely(&self, threshold: f32) -> bool {
        self.strength <= threshold && self.confidence >= 0.3
    }
}

impl Default for TruthValue {
    fn default() -> Self {
        Self::uncertain()
    }
}

/// Types of risk assessed by the inference layer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RiskType {
    /// Zoonotic spillover/outbreak risk
    ZoonoticSpillover,
    /// Trade cheating/default risk
    TradeCheating,
    /// Institution enforcement-failure risk
    EnforcementFailure,
    /// Collective fracture/escalation risk
    CollectiveFracture,
    /// Famine/crisis early-warning risk
    FamineCrisis,
}

impl RiskType {
    pub fn as_str(&self) -> &'static str {
        match self {
            RiskType::ZoonoticSpillover => "zoonotic_spillover",
            RiskType::TradeCheating => "trade_cheating",
            RiskType::EnforcementFailure => "enforcement_failure",
            RiskType::CollectiveFracture => "collective_fracture",
            RiskType::FamineCrisis => "famine_crisis",
        }
    }
}

/// A belief node in the inference network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeliefNode {
    pub id: BeliefId,
    pub name: String,
    pub truth_value: TruthValue,
    /// Last tick this belief was updated
    pub last_updated: u64,
    /// Contributing factor beliefs (for explanation)
    pub contributing_factors: Vec<(BeliefId, f32)>, // (factor_id, weight)
}

impl BeliefNode {
    pub fn new(id: BeliefId, name: &str, truth_value: TruthValue, tick: u64) -> Self {
        Self {
            id,
            name: name.to_string(),
            truth_value,
            last_updated: tick,
            contributing_factors: Vec::new(),
        }
    }

    /// Update the truth value and record contributing factors
    pub fn update(
        &mut self,
        new_truth: TruthValue,
        tick: u64,
        factors: Vec<(BeliefId, f32)>,
    ) {
        self.truth_value = new_truth;
        self.last_updated = tick;
        self.contributing_factors = factors;
    }
}

/// Risk assessment result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub risk_type: RiskType,
    pub partition_id: u64,
    pub truth_value: TruthValue,
    /// Top contributing factors (belief_id, description, weight)
    pub top_factors: Vec<(BeliefId, String, f32)>,
    /// Tick when assessed
    pub assessed_at: u64,
}

/// Inference engine for probabilistic risk assessment
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InferenceEngine {
    beliefs: BTreeMap<BeliefId, BeliefNode>,
    belief_ids_by_name: BTreeMap<String, BeliefId>,
    next_belief_id: u64,
    /// Inference cadence (run every N ticks)
    pub inference_cadence: u64,
    /// Belief retention window in ticks
    pub belief_retention_ticks: u64,
    /// Last tick inference was run
    pub last_inference_tick: u64,
}

impl InferenceEngine {
    pub fn new(inference_cadence: u64, belief_retention_ticks: u64) -> Self {
        Self {
            beliefs: BTreeMap::new(),
            belief_ids_by_name: BTreeMap::new(),
            next_belief_id: 1,
            inference_cadence,
            belief_retention_ticks,
            last_inference_tick: 0,
        }
    }

    /// Check if inference should run this tick
    pub fn should_run(&self, current_tick: u64) -> bool {
        current_tick - self.last_inference_tick >= self.inference_cadence
    }

    /// Create a new belief node
    pub fn create_belief(&mut self, name: &str, truth_value: TruthValue, tick: u64) -> BeliefId {
        let id = BeliefId(self.next_belief_id);
        self.next_belief_id += 1;
        let node = BeliefNode::new(id, name, truth_value, tick);
        self.beliefs.insert(id, node);
        self.belief_ids_by_name.insert(name.to_string(), id);
        id
    }

    /// Get a belief by ID
    pub fn get_belief(&self, id: BeliefId) -> Option<&BeliefNode> {
        self.beliefs.get(&id)
    }

    /// Update a belief's truth value
    pub fn update_belief(
        &mut self,
        id: BeliefId,
        truth_value: TruthValue,
        tick: u64,
        factors: Vec<(BeliefId, f32)>,
    ) {
        if let Some(belief) = self.beliefs.get_mut(&id) {
            belief.update(truth_value, tick, factors);
        }
    }

    /// Run inference for zoonotic spillover risk
    /// Returns the risk assessment
    pub fn infer_zoonotic_risk(
        &mut self,
        partition_id: u64,
        livestock_density: f32,
        corpse_load: f32,
        sanitation_level: f32,
        tick: u64,
        rng: &mut DeterministicRng,
    ) -> RiskAssessment {
        // Create or get beliefs for factors (partition-specific)
        let density_belief = self.get_or_create_factor_belief(
            &format!("livestock_density_p{}", partition_id),
            TruthValue::from_observations((livestock_density * 100.0) as u32, 100),
            tick,
        );

        let corpse_belief = self.get_or_create_factor_belief(
            &format!("corpse_load_p{}", partition_id),
            TruthValue::from_observations((corpse_load * 100.0) as u32, 100),
            tick,
        );

        let sanitation_belief = self.get_or_create_factor_belief(
            &format!("sanitation_level_p{}", partition_id),
            TruthValue::new(sanitation_level, 0.5),
            tick,
        );

        // Infer risk: higher density + corpse load - sanitation = higher risk
        // Using simple weighted combination for now
        let density_weight = 0.4;
        let corpse_weight = 0.4;
        let sanitation_weight = 0.2;

        let density_factor = self.beliefs.get(&density_belief).unwrap().truth_value.strength;
        let corpse_factor = self.beliefs.get(&corpse_belief).unwrap().truth_value.strength;
        let sanitation_factor = self.beliefs.get(&sanitation_belief).unwrap().truth_value.strength;

        // Risk formula: density * weight + corpse * weight - sanitation * weight
        let risk_strength = (density_factor * density_weight
            + corpse_factor * corpse_weight
            - sanitation_factor * sanitation_weight)
            .clamp(0.0, 1.0);

        // Confidence based on data quality
        let risk_confidence = 0.6 + (rng.next_bounded(20) as f32 / 100.0);

        let risk_truth = TruthValue::new(risk_strength, risk_confidence);

        // Get or create risk belief (reuses existing)
        let risk_belief_id = self.get_or_create_risk_belief(
            RiskType::ZoonoticSpillover,
            partition_id,
            risk_truth,
            tick,
        );

        // Record contributing factors
        let factors = vec![
            (density_belief, density_weight),
            (corpse_belief, corpse_weight),
            (sanitation_belief, -sanitation_weight), // Negative contribution
        ];
        self.update_belief(risk_belief_id, risk_truth, tick, factors.clone());

        // Build top factors for event
        let top_factors = factors
            .iter()
            .map(|(id, weight)| {
                let belief = self.beliefs.get(id).unwrap();
                (*id, belief.name.clone(), *weight)
            })
            .collect();

        RiskAssessment {
            risk_type: RiskType::ZoonoticSpillover,
            partition_id,
            truth_value: risk_truth,
            top_factors,
            assessed_at: tick,
        }
    }

    /// Helper to get or create a factor belief
    /// If belief exists, updates it with new observation (direct replacement for determinism)
    fn get_or_create_factor_belief(
        &mut self,
        name: &str,
        truth_value: TruthValue,
        tick: u64,
    ) -> BeliefId {
        if let Some(id) = self.belief_ids_by_name.get(name).copied() {
            // Update with new observation (direct replacement, not revision)
            // This ensures determinism across serial/parallel execution
            self.update_belief(id, truth_value, tick, Vec::new());
            return id;
        }
        self.create_belief(name, truth_value, tick)
    }

    /// Get or create a risk belief (reuses existing risk belief for partition)
    fn get_or_create_risk_belief(
        &mut self,
        risk_type: RiskType,
        partition_id: u64,
        truth_value: TruthValue,
        tick: u64,
    ) -> BeliefId {
        let name = format!("{:?}_risk_p{}", risk_type, partition_id);
        if let Some(id) = self.belief_ids_by_name.get(&name).copied() {
            return id;
        }
        self.create_belief(&name, truth_value, tick)
    }

    /// Get all beliefs
    pub fn beliefs(&self) -> &BTreeMap<BeliefId, BeliefNode> {
        &self.beliefs
    }

    /// Clear old beliefs using configured retention window.
    pub fn clear_expired_beliefs(&mut self, current_tick: u64) {
        let retention = self.belief_retention_ticks;
        self.beliefs
            .retain(|_, node| current_tick.saturating_sub(node.last_updated) < retention);
        self.belief_ids_by_name
            .retain(|_, id| self.beliefs.contains_key(id));
    }

    /// Run inference for trade cheating/default risk
    /// Factors: scarcity stress, trust deficit, enforcement gaps
    pub fn infer_trade_cheating_risk(
        &mut self,
        partition_id: u64,
        scarcity_stress: f32,    // 0.0 to 1.0 (resource scarcity)
        trust_level: f32,        // -1.0 to 1.0 (negative = deficit)
        enforcement_coverage: f32, // 0.0 to 1.0 (institutional presence)
        tick: u64,
        rng: &mut DeterministicRng,
    ) -> RiskAssessment {
        let scarcity_belief = self.get_or_create_factor_belief(
            &format!("scarcity_stress_p{}", partition_id),
            TruthValue::new(scarcity_stress, 0.6),
            tick,
        );

        // Convert trust level to deficit (0.0 to 1.0, higher = more deficit)
        let trust_deficit = ((trust_level * -1.0) + 1.0) / 2.0;
        let trust_belief = self.get_or_create_factor_belief(
            &format!("trust_deficit_p{}", partition_id),
            TruthValue::new(trust_deficit, 0.5),
            tick,
        );

        let enforcement_belief = self.get_or_create_factor_belief(
            &format!("enforcement_coverage_p{}", partition_id),
            TruthValue::new(enforcement_coverage, 0.5),
            tick,
        );

        // Risk formula: scarcity * 0.4 + trust_deficit * 0.35 - enforcement * 0.25
        let scarcity_weight = 0.4;
        let trust_weight = 0.35;
        let enforcement_weight = 0.25;

        let scarcity_factor = self.beliefs.get(&scarcity_belief).unwrap().truth_value.strength;
        let trust_factor = self.beliefs.get(&trust_belief).unwrap().truth_value.strength;
        let enforcement_factor = self.beliefs.get(&enforcement_belief).unwrap().truth_value.strength;

        let risk_strength = (scarcity_factor * scarcity_weight
            + trust_factor * trust_weight
            - enforcement_factor * enforcement_weight)
            .clamp(0.0, 1.0);

        let risk_confidence = 0.55 + (rng.next_bounded(20) as f32 / 100.0);
        let risk_truth = TruthValue::new(risk_strength, risk_confidence);

        let risk_belief_id = self.get_or_create_risk_belief(
            RiskType::TradeCheating,
            partition_id,
            risk_truth,
            tick,
        );

        let factors = vec![
            (scarcity_belief, scarcity_weight),
            (trust_belief, trust_weight),
            (enforcement_belief, -enforcement_weight),
        ];
        self.update_belief(risk_belief_id, risk_truth, tick, factors.clone());

        let top_factors = factors
            .iter()
            .map(|(id, weight)| {
                let belief = self.beliefs.get(id).unwrap();
                (*id, belief.name.clone(), *weight)
            })
            .collect();

        RiskAssessment {
            risk_type: RiskType::TradeCheating,
            partition_id,
            truth_value: risk_truth,
            top_factors,
            assessed_at: tick,
        }
    }

    /// Run inference for institution enforcement-failure risk
    /// Factors: factionalism, legitimacy deficit, resource strain
    pub fn infer_enforcement_failure_risk(
        &mut self,
        partition_id: u64,
        factionalism: f32,      // 0.0 to 1.0 (internal divisions)
        legitimacy: f32,       // 0.0 to 1.0 (lower = deficit)
        resource_strain: f32,    // 0.0 to 1.0 (pooled resources vs needs)
        tick: u64,
        rng: &mut DeterministicRng,
    ) -> RiskAssessment {
        let factionalism_belief = self.get_or_create_factor_belief(
            &format!("factionalism_p{}", partition_id),
            TruthValue::new(factionalism, 0.6),
            tick,
        );

        // Convert legitimacy to deficit
        let legitimacy_deficit = 1.0 - legitimacy;
        let legitimacy_belief = self.get_or_create_factor_belief(
            &format!("legitimacy_deficit_p{}", partition_id),
            TruthValue::new(legitimacy_deficit, 0.5),
            tick,
        );

        let strain_belief = self.get_or_create_factor_belief(
            &format!("resource_strain_p{}", partition_id),
            TruthValue::new(resource_strain, 0.5),
            tick,
        );

        // Risk formula: factionalism * 0.35 + legitimacy_deficit * 0.35 + strain * 0.3
        let faction_weight = 0.35;
        let legitimacy_weight = 0.35;
        let strain_weight = 0.3;

        let faction_factor = self.beliefs.get(&factionalism_belief).unwrap().truth_value.strength;
        let legit_factor = self.beliefs.get(&legitimacy_belief).unwrap().truth_value.strength;
        let strain_factor = self.beliefs.get(&strain_belief).unwrap().truth_value.strength;

        let risk_strength = (faction_factor * faction_weight
            + legit_factor * legitimacy_weight
            + strain_factor * strain_weight)
            .clamp(0.0, 1.0);

        let risk_confidence = 0.5 + (rng.next_bounded(25) as f32 / 100.0);
        let risk_truth = TruthValue::new(risk_strength, risk_confidence);

        let risk_belief_id = self.get_or_create_risk_belief(
            RiskType::EnforcementFailure,
            partition_id,
            risk_truth,
            tick,
        );

        let factors = vec![
            (factionalism_belief, faction_weight),
            (legitimacy_belief, legitimacy_weight),
            (strain_belief, strain_weight),
        ];
        self.update_belief(risk_belief_id, risk_truth, tick, factors.clone());

        let top_factors = factors
            .iter()
            .map(|(id, weight)| {
                let belief = self.beliefs.get(id).unwrap();
                (*id, belief.name.clone(), *weight)
            })
            .collect();

        RiskAssessment {
            risk_type: RiskType::EnforcementFailure,
            partition_id,
            truth_value: risk_truth,
            top_factors,
            assessed_at: tick,
        }
    }

    /// Run inference for collective fracture/escalation risk
    /// Factors: grievance accumulation, social tension, lack of cooperation
    pub fn infer_collective_fracture_risk(
        &mut self,
        partition_id: u64,
        grievance_level: f32,     // 0.0 to 1.0 (accumulated grievances)
        social_tension: f32,      // 0.0 to 1.0 (conflict potential)
        cooperation_rate: f32,    // 0.0 to 1.0 (lower = less cohesion)
        tick: u64,
        rng: &mut DeterministicRng,
    ) -> RiskAssessment {
        let grievance_belief = self.get_or_create_factor_belief(
            &format!("grievance_level_p{}", partition_id),
            TruthValue::new(grievance_level, 0.6),
            tick,
        );

        let tension_belief = self.get_or_create_factor_belief(
            &format!("social_tension_p{}", partition_id),
            TruthValue::new(social_tension, 0.55),
            tick,
        );

        // Invert cooperation rate (low cooperation = high risk)
        let cooperation_deficit = 1.0 - cooperation_rate;
        let cooperation_belief = self.get_or_create_factor_belief(
            &format!("cooperation_deficit_p{}", partition_id),
            TruthValue::new(cooperation_deficit, 0.5),
            tick,
        );

        // Risk formula: grievance * 0.35 + tension * 0.35 + cooperation_deficit * 0.3
        let grievance_weight = 0.35;
        let tension_weight = 0.35;
        let cooperation_weight = 0.3;

        let grievance_factor = self.beliefs.get(&grievance_belief).unwrap().truth_value.strength;
        let tension_factor = self.beliefs.get(&tension_belief).unwrap().truth_value.strength;
        let coop_factor = self.beliefs.get(&cooperation_belief).unwrap().truth_value.strength;

        let risk_strength = (grievance_factor * grievance_weight
            + tension_factor * tension_weight
            + coop_factor * cooperation_weight)
            .clamp(0.0, 1.0);

        let risk_confidence = 0.55 + (rng.next_bounded(20) as f32 / 100.0);
        let risk_truth = TruthValue::new(risk_strength, risk_confidence);

        let risk_belief_id = self.get_or_create_risk_belief(
            RiskType::CollectiveFracture,
            partition_id,
            risk_truth,
            tick,
        );

        let factors = vec![
            (grievance_belief, grievance_weight),
            (tension_belief, tension_weight),
            (cooperation_belief, cooperation_weight),
        ];
        self.update_belief(risk_belief_id, risk_truth, tick, factors.clone());

        let top_factors = factors
            .iter()
            .map(|(id, weight)| {
                let belief = self.beliefs.get(id).unwrap();
                (*id, belief.name.clone(), *weight)
            })
            .collect();

        RiskAssessment {
            risk_type: RiskType::CollectiveFracture,
            partition_id,
            truth_value: risk_truth,
            top_factors,
            assessed_at: tick,
        }
    }

    /// Run inference for famine/crisis early-warning risk
    /// Factors: food scarcity, health decline, waste/disease pressure
    pub fn infer_famine_crisis_risk(
        &mut self,
        partition_id: u64,
        food_scarcity: f32,      // 0.0 to 1.0 (inverse of food availability)
        health_decline: f32,     // 0.0 to 1.0 (average health dropping)
        disease_pressure: f32,     // 0.0 to 1.0 (zoonotic + waste pressure)
        tick: u64,
        rng: &mut DeterministicRng,
    ) -> RiskAssessment {
        let food_belief = self.get_or_create_factor_belief(
            &format!("food_scarcity_p{}", partition_id),
            TruthValue::new(food_scarcity, 0.65),
            tick,
        );

        let health_belief = self.get_or_create_factor_belief(
            &format!("health_decline_p{}", partition_id),
            TruthValue::new(health_decline, 0.6),
            tick,
        );

        let disease_belief = self.get_or_create_factor_belief(
            &format!("disease_pressure_p{}", partition_id),
            TruthValue::new(disease_pressure, 0.55),
            tick,
        );

        // Risk formula: food * 0.4 + health * 0.3 + disease * 0.3
        let food_weight = 0.4;
        let health_weight = 0.3;
        let disease_weight = 0.3;

        let food_factor = self.beliefs.get(&food_belief).unwrap().truth_value.strength;
        let health_factor = self.beliefs.get(&health_belief).unwrap().truth_value.strength;
        let disease_factor = self.beliefs.get(&disease_belief).unwrap().truth_value.strength;

        let risk_strength = (food_factor * food_weight
            + health_factor * health_weight
            + disease_factor * disease_weight)
            .clamp(0.0, 1.0);

        let risk_confidence = 0.6 + (rng.next_bounded(20) as f32 / 100.0);
        let risk_truth = TruthValue::new(risk_strength, risk_confidence);

        let risk_belief_id = self.get_or_create_risk_belief(
            RiskType::FamineCrisis,
            partition_id,
            risk_truth,
            tick,
        );

        let factors = vec![
            (food_belief, food_weight),
            (health_belief, health_weight),
            (disease_belief, disease_weight),
        ];
        self.update_belief(risk_belief_id, risk_truth, tick, factors.clone());

        let top_factors = factors
            .iter()
            .map(|(id, weight)| {
                let belief = self.beliefs.get(id).unwrap();
                (*id, belief.name.clone(), *weight)
            })
            .collect();

        RiskAssessment {
            risk_type: RiskType::FamineCrisis,
            partition_id,
            truth_value: risk_truth,
            top_factors,
            assessed_at: tick,
        }
    }
}

/// Inference event types for audit trail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InferenceEvent {
    /// Risk assessment updated
    RiskUpdated {
        tick: u64,
        partition_id: u64,
        risk_type: RiskType,
        strength: f32,
        confidence: f32,
        top_factors: Vec<(String, f32)>, // (factor_name, weight)
    },
    /// Risk incident realized
    IncidentRealized {
        tick: u64,
        partition_id: u64,
        risk_type: RiskType,
        severity: u8, // 0-100
        contributing_factors: Vec<String>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truth_value_from_observations() {
        let tv = TruthValue::from_observations(70, 100);
        assert!((tv.strength - 0.7).abs() < 0.01);
        assert!(tv.confidence > 0.0);
    }

    #[test]
    fn truth_value_revision() {
        let tv1 = TruthValue::new(0.7, 0.5);
        let tv2 = TruthValue::new(0.8, 0.3);
        let revised = tv1.revise(&tv2);
        assert!(revised.strength > 0.7 && revised.strength < 0.8);
        assert!(revised.confidence > tv1.confidence);
    }

    #[test]
    fn inference_engine_creates_beliefs() {
        let mut engine = InferenceEngine::new(10, 1_000);
        let id = engine.create_belief("test", TruthValue::new(0.5, 0.5), 0);
        assert!(engine.get_belief(id).is_some());
    }

    #[test]
    fn zoonotic_risk_inference() {
        let mut engine = InferenceEngine::new(10, 1_000);
        let mut rng = DeterministicRng::from_u64(42);

        // High density, high corpse load, low sanitation = high risk
        let assessment = engine.infer_zoonotic_risk(
            0, // partition
            0.8, // high livestock density
            0.7, // high corpse load
            0.2, // low sanitation
            100, // tick
            &mut rng,
        );

        assert_eq!(assessment.risk_type, RiskType::ZoonoticSpillover);
        assert!(assessment.truth_value.strength > 0.5); // Should be elevated risk
        assert!(!assessment.top_factors.is_empty());
    }

    #[test]
    fn inference_cadence_respected() {
        let engine = InferenceEngine::new(10, 1_000);
        assert!(!engine.should_run(5));
        assert!(engine.should_run(10));
        assert!(engine.should_run(15));
    }

    #[test]
    fn trade_cheating_risk_inference() {
        let mut engine = InferenceEngine::new(10, 1_000);
        let mut rng = DeterministicRng::from_u64(42);

        // High scarcity, low trust, low enforcement = high risk
        let assessment = engine.infer_trade_cheating_risk(
            0,    // partition
            0.8,  // high scarcity
            -0.6, // low trust (negative)
            0.2,  // low enforcement
            100,  // tick
            &mut rng,
        );

        assert_eq!(assessment.risk_type, RiskType::TradeCheating);
        assert!(assessment.truth_value.strength > 0.5); // Should be elevated risk
        assert!(!assessment.top_factors.is_empty());
    }

    #[test]
    fn enforcement_failure_risk_inference() {
        let mut engine = InferenceEngine::new(10, 1_000);
        let mut rng = DeterministicRng::from_u64(42);

        // High factionalism, low legitimacy, high strain = high risk
        let assessment = engine.infer_enforcement_failure_risk(
            0,   // partition
            0.7, // high factionalism
            0.3, // low legitimacy
            0.8, // high resource strain
            100, // tick
            &mut rng,
        );

        assert_eq!(assessment.risk_type, RiskType::EnforcementFailure);
        assert!(assessment.truth_value.strength > 0.5);
        assert!(!assessment.top_factors.is_empty());
    }

    #[test]
    fn collective_fracture_risk_inference() {
        let mut engine = InferenceEngine::new(10, 1_000);
        let mut rng = DeterministicRng::from_u64(42);

        // High grievance, high tension, low cooperation = high risk
        let assessment = engine.infer_collective_fracture_risk(
            0,   // partition
            0.8, // high grievance
            0.7, // high tension
            0.2, // low cooperation
            100, // tick
            &mut rng,
        );

        assert_eq!(assessment.risk_type, RiskType::CollectiveFracture);
        assert!(assessment.truth_value.strength > 0.5);
        assert!(!assessment.top_factors.is_empty());
    }

    #[test]
    fn famine_crisis_risk_inference() {
        let mut engine = InferenceEngine::new(10, 1_000);
        let mut rng = DeterministicRng::from_u64(42);

        // High food scarcity, health decline, disease pressure = high risk
        let assessment = engine.infer_famine_crisis_risk(
            0,   // partition
            0.8, // high food scarcity
            0.7, // health decline
            0.6, // disease pressure
            100, // tick
            &mut rng,
        );

        assert_eq!(assessment.risk_type, RiskType::FamineCrisis);
        assert!(assessment.truth_value.strength > 0.5);
        assert!(!assessment.top_factors.is_empty());
    }
}
