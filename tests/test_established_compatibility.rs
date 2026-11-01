use ppvs::empirical::PhenomenaRegistry;
use ppvs::established::{EstablishedRegistry, EstablishedStatus};
use ppvs::theories::{FlawedMomentumTheory, GeneralRelativity};

#[test]
fn test_general_relativity_does_not_violate_established_theories() {
    let established = EstablishedRegistry::standard();
    let phenomena = PhenomenaRegistry::standard();
    let gr = GeneralRelativity::new();

    let audit = established.audit_candidate(&gr, &phenomena, 3.0);

    // GR must be fully compliant: it cannot violate any established theory
    assert!(audit.is_compliant, "GR violated established theories: {:?}", audit.entries);
    assert_eq!(audit.violation_count, 0);
    assert_eq!(audit.total_established_checked, 5);

    // Verify Newtonian mechanics and Special Relativity are properly subsumed
    let newton_entry = audit
        .entries
        .iter()
        .find(|e| e.established_theory_id == "newtonian_mechanics")
        .unwrap();
    assert!(matches!(newton_entry.status, EstablishedStatus::Subsumed { .. }));

    let sr_entry = audit
        .entries
        .iter()
        .find(|e| e.established_theory_id == "special_relativity")
        .unwrap();
    assert!(matches!(sr_entry.status, EstablishedStatus::Subsumed { .. }));
}

#[test]
fn test_flawed_theory_violates_established_theories_and_is_flagged() {
    let established = EstablishedRegistry::standard();
    let phenomena = PhenomenaRegistry::standard();
    let flawed = FlawedMomentumTheory;

    let audit = established.audit_candidate(&flawed, &phenomena, 3.0);

    // Flawed candidate MUST be caught and flagged as non-compliant
    assert!(!audit.is_compliant);
    assert!(audit.violation_count > 0, "Flawed theory should have violations");

    // Check that it explicitly broke Special Relativity phenomena (momentum at 0.8c and muon dilation)
    let sr_violation = audit
        .entries
        .iter()
        .find(|e| e.established_theory_id == "special_relativity")
        .expect("Should have entry for Special Relativity");

    match &sr_violation.status {
        EstablishedStatus::Violated { broken_phenomena, diagnostics } => {
            assert!(broken_phenomena.contains(&"relativity:electron_momentum_at_0.8c".to_string()));
            assert!(!diagnostics.is_empty());
        }
        _ => panic!("Expected Violated status for Special Relativity under flawed candidate"),
    }
}

