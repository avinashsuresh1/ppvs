use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::dimensions::{DimensionError, Quantity};
use crate::empirical::Context;

#[derive(Error, Debug, PartialEq)]
pub enum TheoryError {
    #[error("Phenomenon '{0}' is outside the domain of theory '{1}'")]
    OutsideDomain(String, String),
    #[error("Phenomenon '{0}' is outside the valid regime of theory '{1}': {2}")]
    OutsideRegime(String, String, String),
    #[error("Missing parameter in context: {0}")]
    MissingParameter(String),
    #[error("Calculation error: {0}")]
    CalculationError(String),
    #[error("Dimensional error: {0}")]
    Dimension(#[from] DimensionError),
}

/// A physical symmetry and its associated Noether conserved quantity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Symmetry {
    pub name: String,
    pub transformation_group: String,
    pub conserved_quantity: String,
}

impl Symmetry {
    pub fn new(
        name: impl Into<String>,
        transformation_group: impl Into<String>,
        conserved_quantity: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            transformation_group: transformation_group.into(),
            conserved_quantity: conserved_quantity.into(),
        }
    }

    pub fn time_translation() -> Self {
        Self::new("Time Translation Invariance", "T(1)", "Energy")
    }

    pub fn space_translation() -> Self {
        Self::new("Spatial Translation Invariance", "R^3", "Linear Momentum")
    }

    pub fn rotation_so3() -> Self {
        Self::new("Rotational Invariance", "SO(3)", "Angular Momentum")
    }

    pub fn galilean_boost() -> Self {
        Self::new("Galilean Boost Invariance", "Galilei Group", "Center of Mass Velocity")
    }

    pub fn lorentz_boost() -> Self {
        Self::new("Lorentz Boost Invariance", "SO(1,3) / Lorentz Group", "Relativistic 4-Momentum")
    }
}

/// A formal statement or principle of a theory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Principle {
    pub name: String,
    pub description: String,
}

/// Trait defining a formalized physical theory.
/// A theory must specify its principles, symmetries, regimes of validity,
/// and provide deterministic predictions for empirical phenomena.
pub trait Theory: Send + Sync {
    /// Unique identifier for the theory.
    fn id(&self) -> &str;

    /// Human-readable name.
    fn name(&self) -> &str;

    /// Foundational axioms / principles.
    fn principles(&self) -> Vec<Principle>;

    /// Invariance symmetries and conserved quantities.
    fn symmetries(&self) -> Vec<Symmetry>;

    /// Checks if a phenomenon and its context fall inside this theory's regime of validity.
    fn is_in_regime(&self, phenomenon_id: &str, context: &Context) -> Result<(), TheoryError>;

    /// Calculates the theory's deterministic prediction for an empirical phenomenon.
    fn predict(&self, phenomenon_id: &str, context: &Context) -> Result<Quantity, TheoryError>;
}
