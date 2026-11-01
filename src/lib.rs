pub mod dimensions;
pub mod empirical;
pub mod established;
pub mod explainability;
pub mod reduction;
pub mod theories;
pub mod theory;
pub mod verifier;

pub use dimensions::{Dimension, DimensionError, Quantity};
pub use empirical::{Context, FundamentalDomain, Observation, PhenomenaRegistry};
pub use established::{CompatibilityAudit, EstablishedRegistry, EstablishedStatus, TheoryAuditEntry};
pub use explainability::{calculate_relevance, Explainability, TheoryProgression, TheoryUnification};
pub use reduction::{check_backward_compatibility, verify_asymptotic_limit, CorrespondenceReport};
pub use theory::{Principle, Symmetry, Theory, TheoryError};
pub use verifier::{TestFailure, TestOutcome, TestStatus, VerificationReport, Verifier};

