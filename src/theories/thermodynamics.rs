use crate::dimensions::{Dimension, Quantity};
use crate::empirical::Context;
use crate::theory::{Principle, Symmetry, Theory, TheoryError};

pub const DEFAULT_GAS_CONSTANT_R: f64 = 8.314462618;

pub struct ClassicalThermodynamics {
    pub r: f64,
}

impl ClassicalThermodynamics {
    pub fn new() -> Self {
        Self {
            r: DEFAULT_GAS_CONSTANT_R,
        }
    }
}

impl Default for ClassicalThermodynamics {
    fn default() -> Self {
        Self::new()
    }
}

impl Theory for ClassicalThermodynamics {
    fn id(&self) -> &str {
        "classical_thermodynamics"
    }

    fn name(&self) -> &str {
        "Classical Thermodynamics and Statistical Mechanics"
    }

    fn principles(&self) -> Vec<Principle> {
        vec![
            Principle {
                name: "Zeroth Law of Thermodynamics".to_string(),
                description: "Thermal equilibrium is an equivalence relation, establishing the existence of empirical temperature."
                    .to_string(),
            },
            Principle {
                name: "First Law of Thermodynamics (Energy Conservation)".to_string(),
                description: "Delta(U) = Q - W; the internal energy of a closed system is conserved across heat and work exchanges."
                    .to_string(),
            },
            Principle {
                name: "Second Law of Thermodynamics (Entropy Increase)".to_string(),
                description: "The total entropy of an isolated system never decreases over time: dS >= dQ / T."
                    .to_string(),
            },
            Principle {
                name: "Third Law of Thermodynamics (Nernst Theorem)".to_string(),
                description: "The entropy of a perfect crystal approaches a constant minimum as temperature approaches absolute zero."
                    .to_string(),
            },
        ]
    }

    fn symmetries(&self) -> Vec<Symmetry> {
        vec![
            Symmetry::time_translation(), // Energy conservation (First Law)
            Symmetry::space_translation(),
            Symmetry::rotation_so3(),
        ]
    }

    fn is_in_regime(&self, _phenomenon_id: &str, _context: &Context) -> Result<(), TheoryError> {
        // Macroscopic thermodynamic limit (N -> infinity)
        Ok(())
    }

    fn predict(&self, phenomenon_id: &str, context: &Context) -> Result<Quantity, TheoryError> {
        match phenomenon_id {
            "thermo:ideal_gas_molar_volume" => {
                let p = context
                    .get_required("pressure")
                    .map_err(TheoryError::MissingParameter)?;
                p.check_dimension(Dimension::pressure())?;

                let t = context
                    .get_required("temperature")
                    .map_err(TheoryError::MissingParameter)?;
                t.check_dimension(Dimension::temperature())?;

                let n = context
                    .get_required("amount")
                    .map_err(TheoryError::MissingParameter)?;
                n.check_dimension(Dimension::amount())?;

                if p.value <= 0.0 {
                    return Err(TheoryError::CalculationError("Pressure must be positive".to_string()));
                }

                // V = n * R * T / P
                let vol = (n.value * self.r * t.value) / p.value;
                Ok(Quantity::volume(vol))
            }
            "thermo:carnot_efficiency" => {
                let t_hot = context
                    .get_required("t_hot")
                    .map_err(TheoryError::MissingParameter)?;
                t_hot.check_dimension(Dimension::temperature())?;

                let t_cold = context
                    .get_required("t_cold")
                    .map_err(TheoryError::MissingParameter)?;
                t_cold.check_dimension(Dimension::temperature())?;

                if t_hot.value <= 0.0 || t_cold.value <= 0.0 {
                    return Err(TheoryError::CalculationError(
                        "Absolute temperatures must be positive".to_string(),
                    ));
                }
                if t_cold.value > t_hot.value {
                    return Err(TheoryError::CalculationError(
                        "T_cold cannot exceed T_hot in a heat engine".to_string(),
                    ));
                }

                // eta = 1 - T_cold / T_hot
                let efficiency = 1.0 - (t_cold.value / t_hot.value);
                Ok(Quantity::dimensionless(efficiency))
            }
            _ => Err(TheoryError::OutsideDomain(
                phenomenon_id.to_string(),
                self.id().to_string(),
            )),
        }
    }
}
