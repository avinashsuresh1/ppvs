use std::collections::BTreeSet;
use serde::{Deserialize, Serialize};
use crate::empirical::PhenomenaRegistry;
use crate::theory::Theory;

/// Represents the explainability set E(t) of a theory t over the empirical registry:
/// E(t) = { p in Phenomena | t is in regime for p AND |t(p) - obs(p)| <= sigma_threshold }
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Explainability {
    pub theory_id: String,
    pub theory_name: String,
    pub phenomena: BTreeSet<String>,
}

impl Explainability {
    /// Computes the explainability set E(t) for a given theory against an empirical registry.
    pub fn compute(theory: &dyn Theory, registry: &PhenomenaRegistry, sigma_threshold: f64) -> Self {
        let mut phenomena = BTreeSet::new();

        for obs in registry.all() {
            if theory.is_in_regime(&obs.id, &obs.context).is_ok() {
                if let Ok(pred) = theory.predict(&obs.id, &obs.context) {
                    if let Ok(sigma) = obs.deviation_sigma(&pred) {
                        if sigma <= sigma_threshold {
                            phenomena.insert(obs.id.clone());
                        }
                    }
                }
            }
        }

        Self {
            theory_id: theory.id().to_string(),
            theory_name: theory.name().to_string(),
            phenomena,
        }
    }

    pub fn len(&self) -> usize {
        self.phenomena.len()
    }

    pub fn is_empty(&self) -> bool {
        self.phenomena.is_empty()
    }

    pub fn contains(&self, phenomenon_id: &str) -> bool {
        self.phenomena.contains(phenomenon_id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.phenomena.iter()
    }
}

/// The formal progression check between an existing theory t1 and a candidate successor t2:
///
/// E(t1) = p1
/// E(t2) = p1 + u  where u is the empirical innovation of t2
///
/// Conditions for valid succession:
/// 1. Zero Regressions: E(t1) ⊆ E(t2), meaning E(t1) \ E(t2) = ∅ (no established phenomena broken)
/// 2. Positive Innovation: u = E(t2) \ E(t1) ≠ ∅ (t2 must explain strictly more than t1)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TheoryProgression {
    pub t1_id: String,
    pub t2_id: String,
    /// E(t1) ∩ E(t2): phenomena explained by both theories
    pub preserved: BTreeSet<String>,
    /// E(t1) \ E(t2): regressions (phenomena explained by t1 that t2 broke) -> MUST BE EMPTY
    pub regressions: BTreeSet<String>,
    /// u = E(t2) \ E(t1): innovations (new phenomena explained by t2) -> MUST BE NON-EMPTY
    pub innovations: BTreeSet<String>,
    /// Total explainability count |E(t1)|
    pub t1_count: usize,
    /// Total explainability count |E(t2)|
    pub t2_count: usize,
}

impl TheoryProgression {
    pub fn evaluate(
        t1: &dyn Theory,
        t2: &dyn Theory,
        registry: &PhenomenaRegistry,
        sigma_threshold: f64,
    ) -> Self {
        let e1 = Explainability::compute(t1, registry, sigma_threshold);
        let e2 = Explainability::compute(t2, registry, sigma_threshold);

        let preserved: BTreeSet<String> = e1
            .phenomena
            .intersection(&e2.phenomena)
            .cloned()
            .collect();

        let regressions: BTreeSet<String> = e1
            .phenomena
            .difference(&e2.phenomena)
            .cloned()
            .collect();

        let innovations: BTreeSet<String> = e2
            .phenomena
            .difference(&e1.phenomena)
            .cloned()
            .collect();

        Self {
            t1_id: t1.id().to_string(),
            t2_id: t2.id().to_string(),
            preserved,
            regressions,
            innovations,
            t1_count: e1.len(),
            t2_count: e2.len(),
        }
    }

    /// Whether t2 is a strictly valid scientific successor to t1:
    /// - Zero regressions: E(t1) ⊆ E(t2)
    /// - Non-empty innovation: u = E(t2) \ E(t1) ≠ ∅
    pub fn is_valid_successor(&self) -> bool {
        self.regressions.is_empty() && !self.innovations.is_empty()
    }
}

/// The formal Unification check: evaluating whether a unified theory t_U successfully
/// unifies two previously separate, empirically verified theories t_A and t_B:
///
/// E(t_A) = p_A
/// E(t_B) = p_B
/// E(t_U) = p_A ∪ p_B ∪ u_emergent
///
/// Conditions for true scientific unification:
/// 1. Zero Regressions on t_A: E(t_A) ⊆ E(t_U)
/// 2. Zero Regressions on t_B: E(t_B) ⊆ E(t_U)
/// 3. Emergent Unification Innovation: u_emergent = E(t_U) \ (E(t_A) ∪ E(t_B)) ≠ ∅
///    (The synthesis must deduce new cross-domain phenomena that neither ancestor could express alone)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TheoryUnification {
    pub t_a_id: String,
    pub t_b_id: String,
    pub unified_id: String,
    pub t_a_count: usize,
    pub t_b_count: usize,
    pub unified_count: usize,
    /// Phenomena from t_A broken by t_U -> MUST BE EMPTY
    pub t_a_regressions: BTreeSet<String>,
    /// Phenomena from t_B broken by t_U -> MUST BE EMPTY
    pub t_b_regressions: BTreeSet<String>,
    /// Emergent phenomena deduced exclusively by the unified synthesis -> MUST BE NON-EMPTY
    pub emergent_innovations: BTreeSet<String>,
}

impl TheoryUnification {
    pub fn evaluate(
        t_a: &dyn Theory,
        t_b: &dyn Theory,
        unified: &dyn Theory,
        registry: &PhenomenaRegistry,
        sigma_threshold: f64,
    ) -> Self {
        let e_a = Explainability::compute(t_a, registry, sigma_threshold);
        let e_b = Explainability::compute(t_b, registry, sigma_threshold);
        let e_u = Explainability::compute(unified, registry, sigma_threshold);

        let t_a_regressions: BTreeSet<String> = e_a
            .phenomena
            .difference(&e_u.phenomena)
            .cloned()
            .collect();

        let t_b_regressions: BTreeSet<String> = e_b
            .phenomena
            .difference(&e_u.phenomena)
            .cloned()
            .collect();

        let prior_union: BTreeSet<String> = e_a
            .phenomena
            .union(&e_b.phenomena)
            .cloned()
            .collect();

        let emergent_innovations: BTreeSet<String> = e_u
            .phenomena
            .difference(&prior_union)
            .cloned()
            .collect();

        Self {
            t_a_id: t_a.id().to_string(),
            t_b_id: t_b.id().to_string(),
            unified_id: unified.id().to_string(),
            t_a_count: e_a.len(),
            t_b_count: e_b.len(),
            unified_count: e_u.len(),
            t_a_regressions,
            t_b_regressions,
            emergent_innovations,
        }
    }

    /// A valid unification preserves all phenomena of both ancestor theories
    /// AND deduces new cross-domain emergent phenomena.
    pub fn is_valid_unification(&self) -> bool {
        self.t_a_regressions.is_empty()
            && self.t_b_regressions.is_empty()
            && !self.emergent_innovations.is_empty()
    }
}

/// Computes the Normalized Relevance of a theory:
///
/// Relevance = (phenomena explained) / (total known phenomena * assumptions made to explain them)
///
/// R(t) = |E(t)| / (|P_total| * |A(t)|)
///
/// Notice: |E(t)| / |P_total| is the Empirical Coverage Fraction C(t) in [0, 1].
/// Thus R(t) = C(t) / |A(t)|, measuring the fraction of total known reality
/// explained per foundational assumption. Theoretical maximum is 1.0 (a single-axiom TOE).
pub fn calculate_relevance(
    theory: &dyn Theory,
    registry: &PhenomenaRegistry,
    sigma_threshold: f64,
) -> f64 {
    let total_known = registry.len().max(1) as f64;
    let explained = Explainability::compute(theory, registry, sigma_threshold).len() as f64;
    let assumptions = theory.principles().len().max(1) as f64;
    explained / (total_known * assumptions)
}


