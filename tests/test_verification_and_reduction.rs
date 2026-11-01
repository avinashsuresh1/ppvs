use ppvs::dimensions::Quantity;
use ppvs::empirical::{Context, PhenomenaRegistry};
use ppvs::reduction::{check_backward_compatibility, verify_asymptotic_limit};
use ppvs::theories::{FlawedMomentumTheory, GeneralRelativity, NewtonianMechanics, SpecialRelativity};
use ppvs::verifier::{TestOutcome, Verifier};

#[test]
fn test_asymptotic_correspondence_sr_to_newton() {
    let sr = SpecialRelativity::new();
    let newton = NewtonianMechanics::new();

    let mut ctx = Context::new();
    ctx.set("rest_mass", Quantity::mass(9.1093837e-31));
    let c = 299792458.0;
    ctx.set("speed_of_light", Quantity::velocity(c));

    // Progressively smaller velocities approaching 0
    let velocities = [0.1 * c, 0.01 * c, 0.001 * c, 1e-5 * c];

    let report = verify_asymptotic_limit(
        "SR -> Newton momentum limit",
        &sr,
        &newton,
        "relativity:electron_momentum_at_0.8c",
        &ctx,
        "velocity",
        &velocities,
        Quantity::velocity,
    )
    .unwrap();

    assert!(report.passed, "Asymptotic limit failed: {}", report.summary);
    // At v = 1e-5 c, relative difference between relativistic and classical momentum is O(v^2 / c^2) ~ 1e-10
    let final_step = report.steps.last().unwrap();
    assert!(final_step.relative_difference < 1e-8);
}

#[test]
fn test_general_relativity_backward_compatibility_with_newton() {
    let gr = GeneralRelativity::new();
    let newton = NewtonianMechanics::new();
    let reg = PhenomenaRegistry::standard();

    let results = check_backward_compatibility(&gr, &newton, &reg, 3.0);
    // GR should NOT break any phenomenon that Newtonian gravity accurately predicted
    for r in &results {
        assert!(!r.broken, "GR introduced a regression in {}: {}", r.phenomenon_id, r.notes);
    }
}

#[test]
fn test_flawed_theory_reports_failures_with_diagnostics() {
    let flawed = FlawedMomentumTheory;
    let newton = NewtonianMechanics::new();
    let reg = PhenomenaRegistry::standard();

    let verifier = Verifier::default_3sigma();
    let report = verifier.verify(&flawed, &reg, Some(&newton));

    // Must report failures / needs work
    assert_eq!(report.outcome, TestOutcome::NeedsWork);
    assert!(report.failed_phenomena > 0, "Flawed theory should have failed phenomena");
    assert!(!report.failures.is_empty(), "Failures list should contain diagnostic reports");

    // Must have broken the relativistic electron momentum observation
    let momentum_failure = report
        .failures
        .iter()
        .find(|f| f.phenomenon_id == "relativity:electron_momentum_at_0.8c")
        .expect("Should contain momentum failure diagnostic");

    // Missing gamma factor causes a massive 80.0 sigma failure
    assert!(momentum_failure.deviation_sigma.unwrap() > 10.0);
    assert!(momentum_failure.reason.contains("Exceeds tolerance"));
}

#[test]
fn test_general_relativity_passes_full_verification() {
    let gr = GeneralRelativity::new();
    let newton = NewtonianMechanics::new();
    let reg = PhenomenaRegistry::standard();

    let verifier = Verifier::default_3sigma();
    let report = verifier.verify(&gr, &reg, Some(&newton));

    assert_eq!(report.outcome, TestOutcome::AllPassed);
    assert_eq!(report.failed_phenomena, 0);
    assert!(report.failures.is_empty());
    assert!(report.passed_phenomena >= 3);
}
