use serde::{Deserialize, Serialize};
use crate::dimensions::Quantity;
use crate::empirical::PhenomenaRegistry;
use crate::reduction::{check_backward_compatibility, RegressionTestResult};
use crate::theory::{Symmetry, Theory, TheoryError};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TestStatus {
    Passed {
        deviation_sigma: f64,
        predicted: Quantity,
        observed: Quantity,
    },
    Failed {
        deviation_sigma: f64,
        predicted: Quantity,
        observed: Quantity,
        reason: String,
    },
    OutOfRegime {
        reason: String,
    },
    Error {
        message: String,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhenomenonVerificationResult {
    pub phenomenon_id: String,
    pub title: String,
    pub status: TestStatus,
}

/// Actionable diagnostic information for an empirical test failure.
/// Provides the theorist with exact discrepancy numbers, units, and reasons so they know where to refine the theory.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestFailure {
    pub phenomenon_id: String,
    pub title: String,
    pub citation: String,
    pub predicted: Option<Quantity>,
    pub observed: Quantity,
    pub uncertainty: f64,
    pub deviation_sigma: Option<f64>,
    pub reason: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TestOutcome {
    AllPassed,
    NeedsWork,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerificationReport {
    pub theory_id: String,
    pub theory_name: String,
    pub total_phenomena: usize,
    pub in_regime_phenomena: usize,
    pub passed_phenomena: usize,
    pub failed_phenomena: usize,
    pub out_of_regime_phenomena: usize,
    pub results: Vec<PhenomenonVerificationResult>,
    pub failures: Vec<TestFailure>,
    pub symmetries: Vec<Symmetry>,
    pub regression_results: Option<Vec<RegressionTestResult>>,
    pub outcome: TestOutcome,
}

pub struct Verifier {
    pub sigma_threshold: f64,
}

impl Verifier {
    pub fn new(sigma_threshold: f64) -> Self {
        Self { sigma_threshold }
    }

    pub fn default_3sigma() -> Self {
        Self::new(3.0)
    }

    pub fn default_5sigma() -> Self {
        Self::new(5.0)
    }

    /// Evaluates a candidate theory against the empirical phenomena registry,
    /// producing constructive diagnostic reports for all failures.
    pub fn verify(
        &self,
        theory: &dyn Theory,
        registry: &PhenomenaRegistry,
        prior_baseline: Option<&dyn Theory>,
    ) -> VerificationReport {
        let mut results = Vec::new();
        let mut failures = Vec::new();
        let mut in_regime_count = 0;
        let mut passed_count = 0;
        let mut failed_count = 0;
        let mut out_of_regime_count = 0;

        for obs in registry.all() {
            match theory.is_in_regime(&obs.id, &obs.context) {
                Ok(()) => {
                    in_regime_count += 1;
                    match theory.predict(&obs.id, &obs.context) {
                        Ok(pred) => match obs.deviation_sigma(&pred) {
                            Ok(sigma) => {
                                if sigma <= self.sigma_threshold {
                                    passed_count += 1;
                                    results.push(PhenomenonVerificationResult {
                                        phenomenon_id: obs.id.clone(),
                                        title: obs.title.clone(),
                                        status: TestStatus::Passed {
                                            deviation_sigma: sigma,
                                            predicted: pred,
                                            observed: obs.measured,
                                        },
                                    });
                                } else {
                                    failed_count += 1;
                                    let reason = format!(
                                        "Exceeds tolerance: {:.2}σ > {:.2}σ threshold (difference: {:.3e})",
                                        sigma, self.sigma_threshold, (pred.value - obs.measured.value).abs()
                                    );
                                    results.push(PhenomenonVerificationResult {
                                        phenomenon_id: obs.id.clone(),
                                        title: obs.title.clone(),
                                        status: TestStatus::Failed {
                                            deviation_sigma: sigma,
                                            predicted: pred,
                                            observed: obs.measured,
                                            reason: reason.clone(),
                                        },
                                    });
                                    failures.push(TestFailure {
                                        phenomenon_id: obs.id.clone(),
                                        title: obs.title.clone(),
                                        citation: obs.citation.clone(),
                                        predicted: Some(pred),
                                        observed: obs.measured,
                                        uncertainty: obs.uncertainty,
                                        deviation_sigma: Some(sigma),
                                        reason,
                                    });
                                }
                            }
                            Err(e) => {
                                failed_count += 1;
                                results.push(PhenomenonVerificationResult {
                                    phenomenon_id: obs.id.clone(),
                                    title: obs.title.clone(),
                                    status: TestStatus::Error { message: e.clone() },
                                });
                                failures.push(TestFailure {
                                    phenomenon_id: obs.id.clone(),
                                    title: obs.title.clone(),
                                    citation: obs.citation.clone(),
                                    predicted: Some(pred),
                                    observed: obs.measured,
                                    uncertainty: obs.uncertainty,
                                    deviation_sigma: None,
                                    reason: format!("Evaluation error: {e}"),
                                });
                            }
                        },
                        Err(e) => {
                            failed_count += 1;
                            let msg = e.to_string();
                            results.push(PhenomenonVerificationResult {
                                phenomenon_id: obs.id.clone(),
                                title: obs.title.clone(),
                                status: TestStatus::Error {
                                    message: msg.clone(),
                                },
                            });
                            failures.push(TestFailure {
                                phenomenon_id: obs.id.clone(),
                                title: obs.title.clone(),
                                citation: obs.citation.clone(),
                                predicted: None,
                                observed: obs.measured,
                                uncertainty: obs.uncertainty,
                                deviation_sigma: None,
                                reason: format!("Prediction calculation failed: {msg}"),
                            });
                        }
                    }
                }
                Err(TheoryError::OutsideRegime(_, _, reason)) => {
                    out_of_regime_count += 1;
                    results.push(PhenomenonVerificationResult {
                        phenomenon_id: obs.id.clone(),
                        title: obs.title.clone(),
                        status: TestStatus::OutOfRegime { reason },
                    });
                }
                Err(TheoryError::OutsideDomain(_, _)) => {
                    out_of_regime_count += 1;
                    results.push(PhenomenonVerificationResult {
                        phenomenon_id: obs.id.clone(),
                        title: obs.title.clone(),
                        status: TestStatus::OutOfRegime {
                            reason: "Outside theoretical domain".to_string(),
                        },
                    });
                }
                Err(e) => {
                    failed_count += 1;
                    let msg = e.to_string();
                    results.push(PhenomenonVerificationResult {
                        phenomenon_id: obs.id.clone(),
                        title: obs.title.clone(),
                        status: TestStatus::Error {
                            message: msg.clone(),
                        },
                    });
                    failures.push(TestFailure {
                        phenomenon_id: obs.id.clone(),
                        title: obs.title.clone(),
                        citation: obs.citation.clone(),
                        predicted: None,
                        observed: obs.measured,
                        uncertainty: obs.uncertainty,
                        deviation_sigma: None,
                        reason: format!("Regime check error: {msg}"),
                    });
                }
            }
        }

        let regression_results = prior_baseline.map(|prior| {
            check_backward_compatibility(theory, prior, registry, self.sigma_threshold)
        });

        let has_regressions = regression_results
            .as_ref()
            .map(|r| r.iter().any(|res| res.broken))
            .unwrap_or(false);

        let outcome = if failed_count == 0 && !has_regressions && passed_count > 0 {
            TestOutcome::AllPassed
        } else {
            TestOutcome::NeedsWork
        };

        VerificationReport {
            theory_id: theory.id().to_string(),
            theory_name: theory.name().to_string(),
            total_phenomena: registry.len(),
            in_regime_phenomena: in_regime_count,
            passed_phenomena: passed_count,
            failed_phenomena: failed_count,
            out_of_regime_phenomena: out_of_regime_count,
            results,
            failures,
            symmetries: theory.symmetries(),
            regression_results,
            outcome,
        }
    }
}
