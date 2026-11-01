use crate::dimensions::{Dimension, Quantity};
use crate::empirical::Context;
use crate::theory::{Principle, Symmetry, Theory, TheoryError};

/// A hypothetical flawed candidate theory that modifies momentum scaling
/// or ignores relativistic effects, serving as a test case for PPVS rejection.
pub struct FlawedMomentumTheory;

impl Theory for FlawedMomentumTheory {
    fn id(&self) -> &str {
        "flawed_momentum_candidate"
    }

    fn name(&self) -> &str {
        "Hypothetical Broken Candidate Theory"
    }

    fn principles(&self) -> Vec<Principle> {
        vec![Principle {
            name: "Ad-hoc non-relativistic scaling".to_string(),
            description: "Erroneously assumes linear scaling across all velocity domains.".to_string(),
        }]
    }

    fn symmetries(&self) -> Vec<Symmetry> {
        vec![Symmetry::space_translation()]
    }

    fn is_in_regime(&self, _phenomenon_id: &str, _context: &Context) -> Result<(), TheoryError> {
        // Unrestricted claim of validity
        Ok(())
    }

    fn predict(&self, phenomenon_id: &str, context: &Context) -> Result<Quantity, TheoryError> {
        match phenomenon_id {
            "relativity:electron_momentum_at_0.8c" => {
                // Erroneously predicts classical Newtonian momentum p = m * v even at 0.8c
                let m0 = context
                    .get_required("rest_mass")
                    .map_err(TheoryError::MissingParameter)?;
                let v = context
                    .get_required("velocity")
                    .map_err(TheoryError::MissingParameter)?;

                // Fails to apply gamma factor, missing the true value by > 40%!
                Ok(Quantity::momentum(m0.value * v.value))
            }
            "relativity:muon_dilated_lifetime" => {
                // Erroneously predicts zero time dilation: tau = tau_0
                let tau_0 = context
                    .get_required("rest_lifetime")
                    .map_err(TheoryError::MissingParameter)?;
                tau_0.check_dimension(Dimension::time())?;
                Ok(tau_0)
            }
            _ => Err(TheoryError::OutsideDomain(
                phenomenon_id.to_string(),
                self.id().to_string(),
            )),
        }
    }
}
