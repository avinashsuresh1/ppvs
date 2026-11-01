pub mod newtonian;
pub mod special_relativity;
pub mod general_relativity;
pub mod maxwell;
pub mod thermodynamics;
pub mod quantum;
pub mod flawed_candidate;

pub use newtonian::NewtonianMechanics;
pub use special_relativity::SpecialRelativity;
pub use general_relativity::GeneralRelativity;
pub use maxwell::MaxwellianElectrodynamics;
pub use thermodynamics::ClassicalThermodynamics;
pub use quantum::QuantumMechanics;
pub use flawed_candidate::FlawedMomentumTheory;
