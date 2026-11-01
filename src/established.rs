use serde::{Deserialize, Serialize};
use crate::empirical::PhenomenaRegistry;
use crate::explainability::Explainability;
use crate::theory::Theory;

/// Status of an established theory when audited against a proposed candidate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EstablishedStatus {
    /// Candidate successfully subsumes and preserves this established theory's empirical successes.
    Subsumed { preserved_count: usize },
    /// Candidate does not contradict this established theory (different domain of applicability).
    CompatibleDomain,
    /// Candidate makes an erroneous prediction or regression on a phenomenon this established theory verified!
    Violated {
        broken_phenomena: Vec<String>,
        diagnostics: Vec<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TheoryAuditEntry {
    pub established_theory_id: String,
    pub established_theory_name: String,
    pub status: EstablishedStatus,
}

/// The comprehensive audit report when a proposed candidate theory is checked
/// against ALL established, empirically verified theories.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityAudit {
    pub candidate_id: String,
    pub candidate_name: String,
    pub total_established_checked: usize,
    pub entries: Vec<TheoryAuditEntry>,
    pub violation_count: usize,
    pub is_compliant: bool,
}

/// The registry of established, experimental-data-backed physical theories.
/// This represents the accumulated, empirically confirmed knowledge of humanity.
/// Any proposed theory MUST NOT violate any theory in this registry within its valid regime.
pub struct EstablishedRegistry {
    theories: Vec<Box<dyn Theory>>,
}

impl EstablishedRegistry {
    pub fn new() -> Self {
        Self {
            theories: Vec::new(),
        }
    }

    pub fn register(&mut self, theory: Box<dyn Theory>) {
        self.theories.push(theory);
    }

    pub fn len(&self) -> usize {
        self.theories.len()
    }

    pub fn is_empty(&self) -> bool {
        self.theories.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &dyn Theory> {
        self.theories.iter().map(|b| b.as_ref())
    }

    /// Default registry initialized with the established foundational pillars of physics:
    /// 1. Classical Newtonian Mechanics & Universal Gravitation
    /// 2. Einstein's Special Relativity
    /// 3. Maxwell's Classical Electrodynamics & Wave Optics
    /// 4. Classical Thermodynamics & Statistical Mechanics
    /// 5. Non-Relativistic Quantum Mechanics & Atomic Physics
    pub fn standard() -> Self {
        let mut reg = Self::new();
        reg.register(Box::new(crate::theories::NewtonianMechanics::new()));
        reg.register(Box::new(crate::theories::SpecialRelativity::new()));
        reg.register(Box::new(crate::theories::MaxwellianElectrodynamics::new()));
        reg.register(Box::new(crate::theories::ClassicalThermodynamics::new()));
        reg.register(Box::new(crate::theories::QuantumMechanics::new()));
        reg
    }

    /// Audits a proposed candidate theory against all established theories in the registry.
    ///
    /// The fundamental PPVS rule:
    /// For every established theory T_i, the proposed theory MUST NOT contradict or break
    /// any phenomenon that T_i successfully explains:
    /// - If the candidate attempts to predict a phenomenon that T_i verified, it must match observation (<= sigma_threshold).
    /// - If the candidate fails or deviates on T_i's verified phenomena, it is flagged as a VIOLATION.
    pub fn audit_candidate(
        &self,
        candidate: &dyn Theory,
        phenomena: &PhenomenaRegistry,
        sigma_threshold: f64,
    ) -> CompatibilityAudit {
        let mut entries = Vec::new();
        let mut total_violations = 0;

        for established in &self.theories {
            let e_est = Explainability::compute(established.as_ref(), phenomena, sigma_threshold);
            let mut broken_phenomena = Vec::new();
            let mut diagnostics = Vec::new();
            let mut preserved_count = 0;

            for p_id in e_est.iter() {
                if let Some(obs) = phenomena.get(p_id) {
                    // Check if candidate theory is in regime or attempts to predict this phenomenon
                    let in_regime = candidate.is_in_regime(p_id, &obs.context).is_ok();
                    let prediction_result = candidate.predict(p_id, &obs.context);

                    match prediction_result {
                        Ok(pred) => {
                            match obs.deviation_sigma(&pred) {
                                Ok(sigma) => {
                                    if sigma <= sigma_threshold {
                                        preserved_count += 1;
                                    } else {
                                        broken_phenomena.push(p_id.clone());
                                        diagnostics.push(format!(
                                            "Candidate predicted {:.3e} vs observed {:.3e} ({:.2}σ > {:.2}σ tolerance)",
                                            pred.value, obs.measured.value, sigma, sigma_threshold
                                        ));
                                    }
                                }
                                Err(e) => {
                                    broken_phenomena.push(p_id.clone());
                                    diagnostics.push(format!("Dimensional mismatch: {e}"));
                                }
                            }
                        }
                        Err(crate::theory::TheoryError::OutsideDomain(_, _)) => {
                            // Candidate explicitly acknowledges this is outside its domain
                            // Not a violation, provided it wasn't claimed in regime
                        }
                        Err(crate::theory::TheoryError::OutsideRegime(_, _, _)) => {
                            // Outside valid regime of candidate
                        }
                        Err(e) => {
                            if in_regime {
                                broken_phenomena.push(p_id.clone());
                                diagnostics.push(format!("Calculation error on established phenomenon: {e}"));
                            }
                        }
                    }
                }
            }

            let status = if !broken_phenomena.is_empty() {
                total_violations += 1;
                EstablishedStatus::Violated {
                    broken_phenomena,
                    diagnostics,
                }
            } else if preserved_count == e_est.len() && !e_est.is_empty() {
                EstablishedStatus::Subsumed { preserved_count }
            } else if preserved_count > 0 {
                EstablishedStatus::Subsumed { preserved_count }
            } else {
                EstablishedStatus::CompatibleDomain
            };

            entries.push(TheoryAuditEntry {
                established_theory_id: established.id().to_string(),
                established_theory_name: established.name().to_string(),
                status,
            });
        }

        let is_compliant = total_violations == 0;

        CompatibilityAudit {
            candidate_id: candidate.id().to_string(),
            candidate_name: candidate.name().to_string(),
            total_established_checked: self.theories.len(),
            entries,
            violation_count: total_violations,
            is_compliant,
        }
    }
}

impl Default for EstablishedRegistry {
    fn default() -> Self {
        Self::standard()
    }
}
