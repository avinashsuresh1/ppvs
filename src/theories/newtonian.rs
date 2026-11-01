use std::f64::consts::PI;
use crate::dimensions::{Dimension, Quantity};
use crate::empirical::Context;
use crate::theory::{Principle, Symmetry, Theory, TheoryError};

pub const DEFAULT_G: f64 = 6.67430e-11;

pub struct NewtonianMechanics {
    pub g: f64,
}

impl NewtonianMechanics {
    pub fn new() -> Self {
        Self { g: DEFAULT_G }
    }

    pub fn with_gravitational_constant(g: f64) -> Self {
        Self { g }
    }
}

impl Default for NewtonianMechanics {
    fn default() -> Self {
        Self::new()
    }
}

impl Theory for NewtonianMechanics {
    fn id(&self) -> &str {
        "newtonian_mechanics"
    }

    fn name(&self) -> &str {
        "Newtonian Classical Mechanics and Gravitation"
    }

    fn principles(&self) -> Vec<Principle> {
        vec![
            Principle {
                name: "Newton's First Law (Inertia)".to_string(),
                description: "An object remains at rest or in uniform motion unless acted upon by a net external force."
                    .to_string(),
            },
            Principle {
                name: "Newton's Second Law".to_string(),
                description: "F = dp/dt = m*a in an inertial frame of reference.".to_string(),
            },
            Principle {
                name: "Newton's Third Law (Action-Reaction)".to_string(),
                description: "When one body exerts a force on a second body, the second body exerts a force equal in magnitude and opposite in direction."
                    .to_string(),
            },
            Principle {
                name: "Newton's Law of Universal Gravitation".to_string(),
                description: "F_grav = G * (m1 * m2) / r^2 directed along the line between centers of mass."
                    .to_string(),
            },
            Principle {
                name: "Absolute Time and Space".to_string(),
                description: "Time flows equably without regard to anything external; space remains similar and immovable."
                    .to_string(),
            },
        ]
    }

    fn symmetries(&self) -> Vec<Symmetry> {
        vec![
            Symmetry::time_translation(),
            Symmetry::space_translation(),
            Symmetry::rotation_so3(),
            Symmetry::galilean_boost(),
        ]
    }

    fn is_in_regime(&self, phenomenon_id: &str, context: &Context) -> Result<(), TheoryError> {
        // Newtonian mechanics is strictly valid in the low velocity (v << c)
        // and weak field (GM / (r c^2) << 1) regime.
        if let Some(v_qty) = context.get("velocity") {
            let c = context
                .get("speed_of_light")
                .map(|q| q.value)
                .unwrap_or(299792458.0);
            let beta = v_qty.value / c;
            if beta > 0.05 {
                return Err(TheoryError::OutsideRegime(
                    phenomenon_id.to_string(),
                    self.id().to_string(),
                    format!("Velocity ratio beta={beta:.4} exceeds Newtonian limit (beta <= 0.05)"),
                ));
            }
        }
        Ok(())
    }

    fn predict(&self, phenomenon_id: &str, context: &Context) -> Result<Quantity, TheoryError> {
        match phenomenon_id {
            "gravitational_constant:codata2018" => {
                Ok(Quantity::gravitational_constant(self.g))
            }
            "kepler:earth_orbital_period" => {
                // T = 2 * pi * sqrt(a^3 / (G * (M + m)))
                let a = context
                    .get_required("semi_major_axis")
                    .map_err(TheoryError::MissingParameter)?;
                a.check_dimension(Dimension::length())?;

                let m_central = context
                    .get_required("central_mass")
                    .map_err(TheoryError::MissingParameter)?;
                m_central.check_dimension(Dimension::mass())?;

                let m_orbiting = context
                    .get("orbiting_mass")
                    .copied()
                    .unwrap_or_else(|| Quantity::mass(0.0));

                let total_mass = m_central.value + m_orbiting.value;
                let a_cubed = a.value.powi(3);
                let denominator = self.g * total_mass;
                if denominator <= 0.0 {
                    return Err(TheoryError::CalculationError(
                        "Non-positive mass or gravitational constant".to_string(),
                    ));
                }
                let t_period = 2.0 * PI * (a_cubed / denominator).sqrt();
                Ok(Quantity::time(t_period))
            }
            "relativity:muon_dilated_lifetime" => {
                // In Newtonian mechanics, time is absolute: tau = tau_0
                let tau_0 = context
                    .get_required("rest_lifetime")
                    .map_err(TheoryError::MissingParameter)?;
                tau_0.check_dimension(Dimension::time())?;
                Ok(tau_0)
            }
            "relativity:electron_momentum_at_0.8c" => {
                // Newtonian momentum: p = m * v
                let m = context
                    .get_required("rest_mass")
                    .map_err(TheoryError::MissingParameter)?;
                m.check_dimension(Dimension::mass())?;

                let v = context
                    .get_required("velocity")
                    .map_err(TheoryError::MissingParameter)?;
                v.check_dimension(Dimension::velocity())?;

                Ok(Quantity::momentum(m.value * v.value))
            }
            "gr:mercury_perihelion_excess" => {
                // In purely Newtonian gravity, inverse-square law yields closed Bertrand orbits.
                // Anomalous excess precession beyond planetary perturbations is exactly 0.
                Ok(Quantity::dimensionless(0.0))
            }
            _ => Err(TheoryError::OutsideDomain(
                phenomenon_id.to_string(),
                self.id().to_string(),
            )),
        }
    }
}
