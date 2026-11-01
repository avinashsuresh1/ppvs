use ppvs::empirical::PhenomenaRegistry;
use ppvs::explainability::{Explainability, TheoryProgression};
use ppvs::theories::{FlawedMomentumTheory, GeneralRelativity, NewtonianMechanics, SpecialRelativity};

#[test]
fn test_explainability_set_computation() {
    let reg = PhenomenaRegistry::standard();
    let newton = NewtonianMechanics::new();
    let e_newton = Explainability::compute(&newton, &reg, 3.0);

    // Newtonian mechanics successfully explains Cavendish G and Keplerian Earth orbit
    assert!(e_newton.contains("gravitational_constant:codata2018"));
    assert!(e_newton.contains("kepler:earth_orbital_period"));
    // But does not explain Mercury's anomalous precession or relativistic muons
    assert!(!e_newton.contains("gr:mercury_perihelion_excess"));
    assert!(!e_newton.contains("relativity:muon_dilated_lifetime"));
    assert_eq!(e_newton.len(), 2);
}

#[test]
fn test_theory_progression_general_relativity_subsumes_newton() {
    let reg = PhenomenaRegistry::standard();
    let newton = NewtonianMechanics::new();
    let gr = GeneralRelativity::new();

    let progression = TheoryProgression::evaluate(&newton, &gr, &reg, 3.0);

    // E(t1) = p1 = { Cavendish G, Kepler Earth orbit }
    assert_eq!(progression.t1_count, 2);
    // E(t2) = p1 + u = 7 phenomena
    assert_eq!(progression.t2_count, 7);

    // Zero regressions: E(t1) ⊆ E(t2) => regressions = ∅
    assert!(
        progression.regressions.is_empty(),
        "GR introduced regressions: {:?}",
        progression.regressions
    );

    // Preserved: both Newtonian phenomena
    assert_eq!(progression.preserved.len(), 2);
    assert!(progression.preserved.contains("gravitational_constant:codata2018"));
    assert!(progression.preserved.contains("kepler:earth_orbital_period"));

    // Innovation u = E(t2) \ E(t1) ≠ ∅
    assert!(!progression.innovations.is_empty());
    assert!(progression.innovations.contains("gr:mercury_perihelion_excess"));
    assert!(progression.innovations.contains("relativity:muon_dilated_lifetime"));
    assert!(progression.innovations.contains("relativity:electron_momentum_at_0.8c"));

    // GR is a strictly valid successor to Newtonian mechanics
    assert!(progression.is_valid_successor());
}

#[test]
fn test_theory_progression_rejects_identical_theory_as_no_innovation() {
    let reg = PhenomenaRegistry::standard();
    let newton = NewtonianMechanics::new();

    let progression = TheoryProgression::evaluate(&newton, &newton, &reg, 3.0);

    // Regressions are empty, but innovations u are also empty (u = ∅)
    assert!(progression.regressions.is_empty());
    assert!(progression.innovations.is_empty());
    // Not a valid *successor* (provides no new explainability)
    assert!(!progression.is_valid_successor());
}

#[test]
fn test_theory_progression_rejects_flawed_theory_due_to_regressions() {
    let reg = PhenomenaRegistry::standard();
    let newton = NewtonianMechanics::new();
    let flawed = FlawedMomentumTheory;

    let progression = TheoryProgression::evaluate(&newton, &flawed, &reg, 3.0);

    // Flawed theory fails to explain what Newton explained => regressions ≠ ∅
    assert!(!progression.regressions.is_empty());
    assert!(!progression.is_valid_successor());
}

#[test]
fn test_theory_unification_general_relativity_unifies_newton_and_special_relativity() {
    let reg = PhenomenaRegistry::standard();
    let newton = NewtonianMechanics::new();
    let sr = SpecialRelativity::new();
    let gr = GeneralRelativity::new();

    let unification = ppvs::explainability::TheoryUnification::evaluate(
        &newton,
        &sr,
        &gr,
        &reg,
        3.0,
    );

    // Newton explains 2 phenomena (Cavendish G, Kepler Earth orbit)
    assert_eq!(unification.t_a_count, 2);
    // SR explains 2 phenomena (Muon dilation, relativistic electron momentum)
    assert_eq!(unification.t_b_count, 2);
    // GR explains all 7 phenomena
    assert_eq!(unification.unified_count, 7);

    // Zero regressions on either branch
    assert!(unification.t_a_regressions.is_empty(), "GR broke Newtonian phenomena: {:?}", unification.t_a_regressions);
    assert!(unification.t_b_regressions.is_empty(), "GR broke Special Relativity phenomena: {:?}", unification.t_b_regressions);

    // Emergent innovation: phenomena that NEITHER Newton NOR Special Relativity could explain alone
    // (Mercury excess + electrodynamics in curved spacetime)
    assert!(unification.emergent_innovations.contains("gr:mercury_perihelion_excess"));
    assert!(!unification.emergent_innovations.is_empty());

    // Valid Grand Unification!
    assert!(unification.is_valid_unification());
}

#[test]
fn test_theory_relevance_metric() {
    let reg = PhenomenaRegistry::standard();
    let total_phenomena = reg.len() as f64; // 11 benchmark phenomena across all pillars
    assert_eq!(total_phenomena, 11.0);

    let newton = NewtonianMechanics::new();
    let gr = GeneralRelativity::new();

    // Newtonian mechanics:
    // 2 explained / (11 total * 5 assumptions) = 2 / 55
    let r_newton = ppvs::explainability::calculate_relevance(&newton, &reg, 3.0);
    assert!((r_newton - (2.0 / (11.0 * 5.0))).abs() < 1e-10);

    // General Relativity:
    // 7 explained / (11 total * 4 assumptions) = 7 / 44
    let r_gr = ppvs::explainability::calculate_relevance(&gr, &reg, 3.0);
    assert!((r_gr - (7.0 / (11.0 * 4.0))).abs() < 1e-10);

    // GR has strictly higher relevance than Newtonian mechanics
    assert!(r_gr > r_newton, "R(GR) = {r_gr} should exceed R(Newton) = {r_newton}");
}


