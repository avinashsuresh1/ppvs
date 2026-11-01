use std::f64::consts::PI;
use crate::dimensions::{Dimension, Quantity};
use crate::empirical::Context;
use crate::theory::{Principle, Symmetry, Theory, TheoryError};

pub const DEFAULT_EPSILON_0: f64 = 8.8541878128e-12;
pub const DEFAULT_MU_0: f64 = 1.25663706212e-6;

pub struct MaxwellianElectrodynamics {
    pub epsilon_0: f64,
    pub mu_0: f64,
}

impl MaxwellianElectrodynamics {
    pub fn new() -> Self {
        Self {
            epsilon_0: DEFAULT_EPSILON_0,
            mu_0: DEFAULT_MU_0,
        }
    }

    pub fn with_constants(epsilon_0: f64, mu_0: f64) -> Self {
        Self { epsilon_0, mu_0 }
    }
}

impl Default for MaxwellianElectrodynamics {
    fn default() -> Self {
        Self::new()
    }
}

impl Theory for MaxwellianElectrodynamics {
    fn id(&self) -> &str {
        "maxwellian_electrodynamics"
    }

    fn name(&self) -> &str {
        "Maxwell's Classical Electrodynamics and Wave Optics"
    }

    fn principles(&self) -> Vec<Principle> {
        vec![
            Principle {
                name: "Gauss's Law for Electricity".to_string(),
                description: "div(E) = rho / epsilon_0; electric flux out of a closed surface is proportional to enclosed charge."
                    .to_string(),
            },
            Principle {
                name: "Gauss's Law for Magnetism".to_string(),
                description: "div(B) = 0; no magnetic monopoles exist; magnetic flux out of a closed surface is zero."
                    .to_string(),
            },
            Principle {
                name: "Faraday's Law of Electromagnetic Induction".to_string(),
                description: "curl(E) = -dB/dt; a changing magnetic field induces a circulating electric field."
                    .to_string(),
            },
            Principle {
                name: "Ampère-Maxwell Circuital Law".to_string(),
                description: "curl(B) = mu_0*J + mu_0*epsilon_0*dE/dt; electric currents and displacement currents generate magnetic fields."
                    .to_string(),
            },
            Principle {
                name: "Electromagnetic Wave Propagation".to_string(),
                description: "Self-propagating transverse electromagnetic wave oscillations traveling at c = 1 / sqrt(epsilon_0 * mu_0)."
                    .to_string(),
            },
        ]
    }

    fn symmetries(&self) -> Vec<Symmetry> {
        vec![
            Symmetry::new("U(1) Gauge Invariance", "U(1)", "Electric Charge Conservation"),
            Symmetry::time_translation(),
            Symmetry::space_translation(),
            Symmetry::rotation_so3(),
            Symmetry::lorentz_boost(),
        ]
    }

    fn is_in_regime(&self, _phenomenon_id: &str, _context: &Context) -> Result<(), TheoryError> {
        // Classical electrodynamic regime: large photon numbers (macroscopic fields / S >> hbar)
        Ok(())
    }

    fn predict(&self, phenomenon_id: &str, context: &Context) -> Result<Quantity, TheoryError> {
        match phenomenon_id {
            "em:maxwell_wave_speed" => {
                let eps = context
                    .get("permittivity")
                    .map(|q| q.value)
                    .unwrap_or(self.epsilon_0);
                let mu = context
                    .get("permeability")
                    .map(|q| q.value)
                    .unwrap_or(self.mu_0);

                let product = eps * mu;
                if product <= 0.0 {
                    return Err(TheoryError::CalculationError(
                        "Non-positive permittivity or permeability".to_string(),
                    ));
                }
                let c = 1.0 / product.sqrt();
                Ok(Quantity::velocity(c))
            }
            "em:coulomb_force" => {
                let q1 = context
                    .get_required("charge1")
                    .map_err(TheoryError::MissingParameter)?;
                q1.check_dimension(Dimension::charge())?;

                let q2 = context
                    .get_required("charge2")
                    .map_err(TheoryError::MissingParameter)?;
                q2.check_dimension(Dimension::charge())?;

                let r = context
                    .get_required("separation")
                    .map_err(TheoryError::MissingParameter)?;
                r.check_dimension(Dimension::length())?;

                if r.value <= 0.0 {
                    return Err(TheoryError::CalculationError(
                        "Separation distance must be positive".to_string(),
                    ));
                }

                // F = q1 * q2 / (4 * pi * eps0 * r^2)
                let denominator = 4.0 * PI * self.epsilon_0 * r.value * r.value;
                let force_val = (q1.value * q2.value) / denominator;
                Ok(Quantity::force(force_val))
            }
            _ => Err(TheoryError::OutsideDomain(
                phenomenon_id.to_string(),
                self.id().to_string(),
            )),
        }
    }
}
