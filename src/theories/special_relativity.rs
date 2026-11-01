use crate::dimensions::{Dimension, Quantity};
use crate::empirical::Context;
use crate::theory::{Principle, Symmetry, Theory, TheoryError};

pub const DEFAULT_C: f64 = 299792458.0;

pub struct SpecialRelativity {
    pub c: f64,
}

impl SpecialRelativity {
    pub fn new() -> Self {
        Self { c: DEFAULT_C }
    }

    pub fn with_speed_of_light(c: f64) -> Self {
        Self { c }
    }

    pub fn lorentz_factor(&self, v: f64) -> Result<f64, TheoryError> {
        let beta = v / self.c;
        if beta.abs() >= 1.0 {
            return Err(TheoryError::CalculationError(format!(
                "Tachyonic or light-speed velocity for massive particle: beta = {beta}"
            )));
        }
        Ok(1.0 / (1.0 - beta * beta).sqrt())
    }
}

impl Default for SpecialRelativity {
    fn default() -> Self {
        Self::new()
    }
}

impl Theory for SpecialRelativity {
    fn id(&self) -> &str {
        "special_relativity"
    }

    fn name(&self) -> &str {
        "Einstein's Special Relativity (Flat Spacetime)"
    }

    fn principles(&self) -> Vec<Principle> {
        vec![
            Principle {
                name: "Principle of Relativity".to_string(),
                description: "The laws of physics are invariant in all inertial reference frames."
                    .to_string(),
            },
            Principle {
                name: "Constancy of the Speed of Light".to_string(),
                description: "The speed of light in vacuum has the same value c for all inertial observers regardless of emitter motion."
                    .to_string(),
            },
            Principle {
                name: "Lorentz Covariance".to_string(),
                description: "Spacetime coordinates transform according to the Lorentz group SO(1,3), preserving ds^2 = -c^2 dt^2 + dx^2 + dy^2 + dz^2."
                    .to_string(),
            },
            Principle {
                name: "Relativistic Energy-Momentum Equivalence".to_string(),
                description: "E^2 = (p*c)^2 + (m0*c^2)^2, with 4-momentum P^mu = (E/c, p)."
                    .to_string(),
            },
        ]
    }

    fn symmetries(&self) -> Vec<Symmetry> {
        vec![
            Symmetry::time_translation(),
            Symmetry::space_translation(),
            Symmetry::rotation_so3(),
            Symmetry::lorentz_boost(),
        ]
    }

    fn is_in_regime(&self, phenomenon_id: &str, context: &Context) -> Result<(), TheoryError> {
        // Flat spacetime: applicable when gravitational potential is negligible: GM / (r c^2) << 1
        if let (Some(m), Some(r)) = (context.get("central_mass"), context.get("semi_major_axis")) {
            let g = 6.67430e-11;
            let potential = (g * m.value) / (r.value * self.c * self.c);
            if potential > 1e-4 {
                return Err(TheoryError::OutsideRegime(
                    phenomenon_id.to_string(),
                    self.id().to_string(),
                    format!("Gravitational field GM/(r c^2) = {potential:.2e} requires curved spacetime (General Relativity)"),
                ));
            }
        }
        Ok(())
    }

    fn predict(&self, phenomenon_id: &str, context: &Context) -> Result<Quantity, TheoryError> {
        match phenomenon_id {
            "relativity:muon_dilated_lifetime" => {
                let tau_0 = context
                    .get_required("rest_lifetime")
                    .map_err(TheoryError::MissingParameter)?;
                tau_0.check_dimension(Dimension::time())?;

                let v = context
                    .get_required("velocity")
                    .map_err(TheoryError::MissingParameter)?;
                v.check_dimension(Dimension::velocity())?;

                let gamma = self.lorentz_factor(v.value)?;
                let tau_dilated = gamma * tau_0.value;
                Ok(Quantity::time(tau_dilated))
            }
            "relativity:electron_momentum_at_0.8c" => {
                let m0 = context
                    .get_required("rest_mass")
                    .map_err(TheoryError::MissingParameter)?;
                m0.check_dimension(Dimension::mass())?;

                let v = context
                    .get_required("velocity")
                    .map_err(TheoryError::MissingParameter)?;
                v.check_dimension(Dimension::velocity())?;

                let gamma = self.lorentz_factor(v.value)?;
                let p_relativistic = gamma * m0.value * v.value;
                Ok(Quantity::momentum(p_relativistic))
            }
            _ => Err(TheoryError::OutsideDomain(
                phenomenon_id.to_string(),
                self.id().to_string(),
            )),
        }
    }
}
