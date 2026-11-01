use crate::dimensions::{Dimension, Quantity};
use crate::empirical::Context;
use crate::theory::{Principle, Symmetry, Theory, TheoryError};

pub const DEFAULT_PLANCK_H: f64 = 6.62607015e-34;
/// Rydberg constant for Hydrogen (incorporating electron-proton reduced mass mu = m_e*m_p/(m_e+m_p))
pub const DEFAULT_RYDBERG_RH: f64 = 10967758.341;

pub struct QuantumMechanics {
    pub h: f64,
    pub r_h: f64,
}

impl QuantumMechanics {
    pub fn new() -> Self {
        Self {
            h: DEFAULT_PLANCK_H,
            r_h: DEFAULT_RYDBERG_RH,
        }
    }
}

impl Default for QuantumMechanics {
    fn default() -> Self {
        Self::new()
    }
}

impl Theory for QuantumMechanics {
    fn id(&self) -> &str {
        "quantum_mechanics"
    }

    fn name(&self) -> &str {
        "Non-Relativistic Quantum Mechanics and Wave-Particle Duality"
    }

    fn principles(&self) -> Vec<Principle> {
        vec![
            Principle {
                name: "Wave-Particle Duality & Planck-Einstein Relation".to_string(),
                description: "Light and matter exhibit dual characteristics; E = h*nu and p = h / lambda."
                    .to_string(),
            },
            Principle {
                name: "Born Rule & State Space".to_string(),
                description: "Physical states are vectors in a complex Hilbert space; |psi(x)|^2 represents the probability density of observation."
                    .to_string(),
            },
            Principle {
                name: "Heisenberg Uncertainty Principle".to_string(),
                description: "Canonical non-commuting observables obey [x, p] = i*hbar, requiring Delta(x) * Delta(p) >= hbar / 2."
                    .to_string(),
            },
            Principle {
                name: "Schrödinger Dynamics & Energy Quantization".to_string(),
                description: "Time evolution obeys i*hbar*d(psi)/dt = H*psi; bounded systems possess discrete energy eigenvalues."
                    .to_string(),
            },
        ]
    }

    fn symmetries(&self) -> Vec<Symmetry> {
        vec![
            Symmetry::new("Global U(1) Phase Invariance", "U(1)", "Total Probability Conservation"),
            Symmetry::time_translation(),
            Symmetry::space_translation(),
            Symmetry::rotation_so3(),
        ]
    }

    fn is_in_regime(&self, _phenomenon_id: &str, _context: &Context) -> Result<(), TheoryError> {
        // Quantum regime: action scale S ~ hbar
        Ok(())
    }

    fn predict(&self, phenomenon_id: &str, context: &Context) -> Result<Quantity, TheoryError> {
        match phenomenon_id {
            "quantum:photoelectric_stopping_voltage" => {
                let nu = context
                    .get_required("frequency")
                    .map_err(TheoryError::MissingParameter)?;

                let phi = context
                    .get_required("work_function")
                    .map_err(TheoryError::MissingParameter)?;
                phi.check_dimension(Dimension::energy())?;

                let e_charge = context
                    .get_required("elementary_charge")
                    .map_err(TheoryError::MissingParameter)?;
                e_charge.check_dimension(Dimension::charge())?;

                // K_max = h * nu - Phi
                let photon_energy = self.h * nu.value;
                if photon_energy < phi.value {
                    // Below threshold frequency: no electrons emitted => stopping potential = 0
                    return Ok(Quantity::electric_potential(0.0));
                }
                let k_max = photon_energy - phi.value;
                let v_stopping = k_max / e_charge.value;
                Ok(Quantity::electric_potential(v_stopping))
            }
            "quantum:hydrogen_balmer_alpha" => {
                let n_init = context
                    .get_required("n_initial")
                    .map_err(TheoryError::MissingParameter)?;
                let n_final = context
                    .get_required("n_final")
                    .map_err(TheoryError::MissingParameter)?;

                let n1 = n_final.value;
                let n2 = n_init.value;
                if n1 <= 0.0 || n2 <= n1 {
                    return Err(TheoryError::CalculationError(
                        "Initial level must exceed final level in emission".to_string(),
                    ));
                }

                // Rydberg formula: 1 / lambda = R_H * (1 / n1^2 - 1 / n2^2)
                let inv_lambda = self.r_h * (1.0 / (n1 * n1) - 1.0 / (n2 * n2));
                let lambda = 1.0 / inv_lambda;
                Ok(Quantity::length(lambda))
            }
            _ => Err(TheoryError::OutsideDomain(
                phenomenon_id.to_string(),
                self.id().to_string(),
            )),
        }
    }
}
