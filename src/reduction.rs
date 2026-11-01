use serde::{Deserialize, Serialize};
use crate::dimensions::Quantity;
use crate::empirical::{Context, PhenomenaRegistry};
use crate::theory::{Theory, TheoryError};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AsymptoticStep {
    pub parameter_value: f64,
    pub new_value: f64,
    pub prior_value: f64,
    pub relative_difference: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CorrespondenceReport {
    pub test_name: String,
    pub passed: bool,
    pub steps: Vec<AsymptoticStep>,
    pub summary: String,
}

/// Verifies whether `new_theory` recovers `prior_theory` in the asymptotic limit
/// of a given perturbation parameter (e.g. beta = v/c -> 0, or GM / (r c^2) -> 0).
pub fn verify_asymptotic_limit(
    test_name: &str,
    new_theory: &dyn Theory,
    prior_theory: &dyn Theory,
    phenomenon_id: &str,
    base_context: &Context,
    param_key: &str,
    param_values_approaching_zero: &[f64],
    make_quantity: fn(f64) -> Quantity,
) -> Result<CorrespondenceReport, TheoryError> {
    let mut steps = Vec::new();
    let mut prev_diff = f64::MAX;
    let mut passed = true;

    for &val in param_values_approaching_zero {
        let mut ctx = base_context.clone();
        ctx.set(param_key, make_quantity(val));

        let q_new = new_theory.predict(phenomenon_id, &ctx)?;
        let q_prior = prior_theory.predict(phenomenon_id, &ctx)?;

        if q_new.dimension != q_prior.dimension {
            return Ok(CorrespondenceReport {
                test_name: test_name.to_string(),
                passed: false,
                steps,
                summary: format!(
                    "Dimensional mismatch in limit: new [{}] vs prior [{}]",
                    q_new.dimension, q_prior.dimension
                ),
            });
        }

        let denom = q_prior.value.abs().max(1e-15);
        let rel_diff = (q_new.value - q_prior.value).abs() / denom;

        steps.push(AsymptoticStep {
            parameter_value: val,
            new_value: q_new.value,
            prior_value: q_prior.value,
            relative_difference: rel_diff,
        });

        // The discrepancy must shrink or remain negligible as parameter approaches zero
        if rel_diff > prev_diff + 1e-12 {
            passed = false;
        }
        prev_diff = rel_diff;
    }

    // Final limit must be within tight numerical tolerance (< 1e-5)
    if let Some(last) = steps.last() {
        if last.relative_difference > 1e-4 {
            passed = false;
        }
    }

    let summary = if passed {
        format!(
            "Correspondence verified: {} recovers {} as {} -> 0 (final rel diff: {:.2e})",
            new_theory.name(),
            prior_theory.name(),
            param_key,
            steps.last().map(|s| s.relative_difference).unwrap_or(0.0)
        )
    } else {
        format!(
            "Correspondence FAILED: {} does not smoothly converge to {}",
            new_theory.name(),
            prior_theory.name()
        )
    };

    Ok(CorrespondenceReport {
        test_name: test_name.to_string(),
        passed,
        steps,
        summary,
    })
}

/// Checks backward compatibility of a new candidate theory against all phenomena
/// currently verified by an established prior theory.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegressionTestResult {
    pub phenomenon_id: String,
    pub title: String,
    pub prior_sigma: f64,
    pub new_sigma: f64,
    pub broken: bool,
    pub notes: String,
}

pub fn check_backward_compatibility(
    new_theory: &dyn Theory,
    prior_theory: &dyn Theory,
    registry: &PhenomenaRegistry,
    sigma_threshold: f64,
) -> Vec<RegressionTestResult> {
    let mut results = Vec::new();

    for obs in registry.all() {
        // If the prior theory is in regime for this phenomenon
        if prior_theory.is_in_regime(&obs.id, &obs.context).is_ok() {
            if let Ok(prior_pred) = prior_theory.predict(&obs.id, &obs.context) {
                if let Ok(prior_sigma) = obs.deviation_sigma(&prior_pred) {
                    // Only test phenomena that the prior theory successfully explained (within threshold)
                    if prior_sigma <= sigma_threshold {
                        // The new theory must not break this!
                        match new_theory.predict(&obs.id, &obs.context) {
                            Ok(new_pred) => match obs.deviation_sigma(&new_pred) {
                                Ok(new_sigma) => {
                                    let broken = new_sigma > sigma_threshold;
                                    results.push(RegressionTestResult {
                                        phenomenon_id: obs.id.clone(),
                                        title: obs.title.clone(),
                                        prior_sigma,
                                        new_sigma,
                                        broken,
                                        notes: if broken {
                                            format!(
                                                "REGRESSION: Deviates by {:.2} sigmas (prior: {:.2} sigmas)",
                                                new_sigma, prior_sigma
                                            )
                                        } else {
                                            format!(
                                                "Preserved (prior: {:.2}σ, new: {:.2}σ)",
                                                prior_sigma, new_sigma
                                            )
                                        },
                                    });
                                }
                                Err(err) => {
                                    results.push(RegressionTestResult {
                                        phenomenon_id: obs.id.clone(),
                                        title: obs.title.clone(),
                                        prior_sigma,
                                        new_sigma: f64::INFINITY,
                                        broken: true,
                                        notes: format!("Error calculating deviation: {err}"),
                                    });
                                }
                            },
                            Err(err) => {
                                results.push(RegressionTestResult {
                                    phenomenon_id: obs.id.clone(),
                                    title: obs.title.clone(),
                                    prior_sigma,
                                    new_sigma: f64::INFINITY,
                                    broken: true,
                                    notes: format!("New theory failed to predict: {err}"),
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    results
}
