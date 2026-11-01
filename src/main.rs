use ppvs::dimensions::Quantity;
use ppvs::empirical::{Context, PhenomenaRegistry};
use ppvs::established::EstablishedStatus;
use ppvs::reduction::verify_asymptotic_limit;
use ppvs::theories::{
    ClassicalThermodynamics, FlawedMomentumTheory, GeneralRelativity, MaxwellianElectrodynamics,
    NewtonianMechanics, QuantumMechanics, SpecialRelativity,
};
use ppvs::verifier::{TestStatus, Verifier};
use ppvs::Theory;

fn print_separator() {
    println!("{}", "=".repeat(78));
}

fn main() {
    print_separator();
    println!("  PPVS: PHYSICAL PRINCIPLES VERIFICATION SYSTEM");
    println!("  Formalizing Physics as an Executable Specification & Regression Suite");
    print_separator();

    let registry = PhenomenaRegistry::standard();
    println!("\n[1] EMPIRICAL PHENOMENA REGISTRY (Reality Baseline):");
    println!("Loaded {} benchmark empirical phenomena partitioned into 3 disjoint fundamental pillars:", registry.len());
    let p_grav = registry.in_domain(ppvs::empirical::FundamentalDomain::SpacetimeGravitation);
    let p_qft = registry.in_domain(ppvs::empirical::FundamentalDomain::QuantumGaugeFields);
    let p_stat = registry.in_domain(ppvs::empirical::FundamentalDomain::StatisticalThermodynamics);
    println!("  • Pillar I   [Spacetime & Gravitation]   : {} phenomena ({:.1}%)", p_grav.len(), (p_grav.len() as f64 / registry.len() as f64) * 100.0);
    println!("  • Pillar II  [Quantum Gauge Fields]      : {} phenomena ({:.1}%)", p_qft.len(), (p_qft.len() as f64 / registry.len() as f64) * 100.0);
    println!("  • Pillar III [Statistical Thermodynamics]: {} phenomena ({:.1}%)", p_stat.len(), (p_stat.len() as f64 / registry.len() as f64) * 100.0);
    println!("  --> Disjoint Mathematical Partition Verified: {} (100% Coverage, 0% Overlap)",
        if registry.is_disjoint_partition() { "TRUE" } else { "FALSE" });

    for obs in registry.all() {
        println!(
            "  • [{:?}] [{}] {}\n    Value: {:.6e} [{}] ± {:.2e} | Citation: {}",
            obs.domain, obs.id, obs.title, obs.measured.value, obs.measured.dimension, obs.uncertainty, obs.citation
        );
    }

    let verifier = Verifier::default_3sigma();

    // 1. Verify Newtonian Mechanics
    println!("\n");
    print_separator();
    println!("  VERIFYING THEORY: NEWTONIAN MECHANICS");
    print_separator();
    let newton = NewtonianMechanics::new();
    let report_newton = verifier.verify(&newton, &registry, None);
    print_report(&report_newton);
    let r_newton = ppvs::explainability::calculate_relevance(&newton, &registry, 3.0);
    println!("  --> Relevance: R(t) = (number of phenomena explained) / (total known * assumptions) = {} / ({} * {}) = {:.4} ({:.1}% coverage)",
        report_newton.passed_phenomena, registry.len(), newton.principles().len(), r_newton, (report_newton.passed_phenomena as f64 / registry.len() as f64) * 100.0);

    // 2. Verify Special Relativity & Asymptotic Correspondence to Newton
    println!("\n");
    print_separator();
    println!("  VERIFYING THEORY: SPECIAL RELATIVITY (FLAT SPACETIME)");
    print_separator();
    let sr = SpecialRelativity::new();
    let report_sr = verifier.verify(&sr, &registry, None);
    print_report(&report_sr);

    // 3. Verify Maxwellian Electrodynamics & Wave Optics
    println!("\n");
    print_separator();
    println!("  VERIFYING THEORY: MAXWELL'S ELECTRODYNAMICS & WAVE OPTICS");
    print_separator();
    let maxwell = MaxwellianElectrodynamics::new();
    let report_maxwell = verifier.verify(&maxwell, &registry, None);
    print_report(&report_maxwell);
    let r_maxwell = ppvs::explainability::calculate_relevance(&maxwell, &registry, 3.0);
    println!("  --> Relevance: R(t) = (number of phenomena explained) / (total known * assumptions) = {} / ({} * {}) = {:.4} ({:.1}% coverage)",
        report_maxwell.passed_phenomena, registry.len(), maxwell.principles().len(), r_maxwell, (report_maxwell.passed_phenomena as f64 / registry.len() as f64) * 100.0);

    // 4. Verify Classical Thermodynamics & Statistical Mechanics
    println!("\n");
    print_separator();
    println!("  VERIFYING THEORY: CLASSICAL THERMODYNAMICS");
    print_separator();
    let thermo = ClassicalThermodynamics::new();
    let report_thermo = verifier.verify(&thermo, &registry, None);
    print_report(&report_thermo);
    let r_thermo = ppvs::explainability::calculate_relevance(&thermo, &registry, 3.0);
    println!("  --> Relevance: R(t) = (number of phenomena explained) / (total known * assumptions) = {} / ({} * {}) = {:.4} ({:.1}% coverage)",
        report_thermo.passed_phenomena, registry.len(), thermo.principles().len(), r_thermo, (report_thermo.passed_phenomena as f64 / registry.len() as f64) * 100.0);

    // 5. Verify Quantum Mechanics & Atomic Physics
    println!("\n");
    print_separator();
    println!("  VERIFYING THEORY: QUANTUM MECHANICS & ATOMIC PHYSICS");
    print_separator();
    let quantum = QuantumMechanics::new();
    let report_quantum = verifier.verify(&quantum, &registry, None);
    print_report(&report_quantum);
    let r_quantum = ppvs::explainability::calculate_relevance(&quantum, &registry, 3.0);
    println!("  --> Relevance: R(t) = (number of phenomena explained) / (total known * assumptions) = {} / ({} * {}) = {:.4} ({:.1}% coverage)",
        report_quantum.passed_phenomena, registry.len(), quantum.principles().len(), r_quantum, (report_quantum.passed_phenomena as f64 / registry.len() as f64) * 100.0);

    println!("\n  --> Checking Asymptotic Correspondence: SR -> Newton as v/c -> 0");
    let mut electron_base_ctx = Context::new();
    electron_base_ctx.set("rest_mass", Quantity::mass(9.1093837e-31));
    let c = 299792458.0;
    electron_base_ctx.set("speed_of_light", Quantity::velocity(c));

    let velocities = [0.1 * c, 0.01 * c, 0.001 * c, 0.0001 * c, 1e-6 * c];
    let corr_report = verify_asymptotic_limit(
        "Relativistic Momentum -> Classical Momentum as v/c -> 0",
        &sr,
        &newton,
        "relativity:electron_momentum_at_0.8c",
        &electron_base_ctx,
        "velocity",
        &velocities,
        Quantity::velocity,
    );
    match corr_report {
        Ok(cr) => {
            println!("      {}", cr.summary);
            for s in &cr.steps {
                println!(
                    "      v = {:<12.3e} m/s | p_SR = {:<12.4e} | p_Newton = {:<12.4e} | RelDiff = {:.2e}",
                    s.parameter_value, s.new_value, s.prior_value, s.relative_difference
                );
            }
        }
        Err(e) => println!("      Correspondence check error: {e}"),
    }

    // 3. Verify General Relativity & Backward Compatibility
    println!("\n");
    print_separator();
    println!("  VERIFYING THEORY: GENERAL RELATIVITY (EINSTEIN FIELD EQUATIONS)");
    print_separator();
    let gr = GeneralRelativity::new();
    let report_gr = verifier.verify(&gr, &registry, Some(&newton));
    print_report(&report_gr);
    let r_gr = ppvs::explainability::calculate_relevance(&gr, &registry, 3.0);
    println!("  --> Relevance: R(t) = (number of phenomena explained) / (total known * assumptions) = {} / ({} * {}) = {:.4} ({:.1}% coverage)",
        report_gr.passed_phenomena, registry.len(), gr.principles().len(), r_gr, (report_gr.passed_phenomena as f64 / registry.len() as f64) * 100.0);

    // Theory Progression Analysis: E(t1) vs E(t2)
    println!("\n  --> Theory Progression Algebra: E(t1) -> E(t2) = p1 + u");
    let progression = ppvs::explainability::TheoryProgression::evaluate(&newton, &gr, &registry, 3.0);
    println!("      E(t1: {}) = {} phenomena", progression.t1_id, progression.t1_count);
    println!("      E(t2: {}) = {} phenomena", progression.t2_id, progression.t2_count);
    println!("      Preserved E(t1) ∩ E(t2): {} phenomena", progression.preserved.len());
    for p in &progression.preserved {
        println!("        • [Preserved] {p}");
    }
    println!("      Regressions E(t1) \\ E(t2): {} phenomena (Must be 0)", progression.regressions.len());
    println!("      Innovation u = E(t2) \\ E(t1): {} phenomena (Must be > 0)", progression.innovations.len());
    for u in &progression.innovations {
        println!("        • [Innovation u] {u}");
    }
    println!(
        "      Verdict: Is t2 a valid successor to t1? -> {}",
        if progression.is_valid_successor() { "VALID SUCCESSOR (E(t1) ⊆ E(t2) ∧ u ≠ ∅)" } else { "INVALID" }
    );

    // Theory Unification Analysis: E(t_A) ∪ E(t_B) -> E(t_U) = E(t_A) ∪ E(t_B) ∪ u_emergent
    println!("\n  --> Theory Unification Algebra: E(t_U) = E(t_A) ∪ E(t_B) ∪ u_emergent");
    let unification = ppvs::explainability::TheoryUnification::evaluate(&newton, &sr, &gr, &registry, 3.0);
    println!("      t_A (Newtonian Mechanics): {} phenomena", unification.t_a_count);
    println!("      t_B (Special Relativity):  {} phenomena", unification.t_b_count);
    println!("      t_U (General Relativity):  {} phenomena", unification.unified_count);
    println!("      Regressions on t_A: {} phenomena", unification.t_a_regressions.len());
    println!("      Regressions on t_B: {} phenomena", unification.t_b_regressions.len());
    println!("      Emergent Innovations u_emergent: {} phenomena", unification.emergent_innovations.len());
    for em in &unification.emergent_innovations {
        println!("        ★ [Emergent Synthesis] {em} (Deduced purely from unifying Gravity with Relativity!)");
    }
    println!(
        "      Verdict: Is t_U a true Unification? -> {}",
        if unification.is_valid_unification() { "VALID UNIFICATION (Zero Regressions ∧ u_emergent ≠ ∅)" } else { "INVALID" }
    );

    // 4. Established Theories Audit for Proposed Theories
    println!("\n");
    print_separator();
    println!("  PPVS AUDIT: COMPATIBILITY WITH ALL ESTABLISHED THEORIES");
    println!("  Core Principle: Any proposed theory must NOT violate established theories");
    print_separator();
    let established = ppvs::established::EstablishedRegistry::standard();
    println!("Loaded {} established experimental-data-backed theories:", established.len());
    for est in established.iter() {
        println!("  • [{}] {}", est.id(), est.name());
    }

    println!("\n  [Audit 1] Proposed Candidate: General Relativity");
    let audit_gr = established.audit_candidate(&gr, &registry, 3.0);
    println!("  Result: {} (Violations: {})",
        if audit_gr.is_compliant { "COMPLIANT - ZERO VIOLATIONS OF ESTABLISHED PHYSICS" } else { "NON-COMPLIANT" },
        audit_gr.violation_count
    );
    for entry in &audit_gr.entries {
        match &entry.status {
            EstablishedStatus::Subsumed { preserved_count } => {
                println!("    ✓ Subsumes & Preserves: {} ({} phenomena)", entry.established_theory_name, preserved_count);
            }
            EstablishedStatus::CompatibleDomain => {
                println!("    • Non-overlapping / Compatible domain: {}", entry.established_theory_name);
            }
            EstablishedStatus::Violated { broken_phenomena, .. } => {
                println!("    ✗ VIOLATED: {} (Breaks: {:?})", entry.established_theory_name, broken_phenomena);
            }
        }
    }

    let flawed = FlawedMomentumTheory;
    println!("\n  [Audit 2] Proposed Candidate: Flawed Momentum Theory");
    let audit_flawed = established.audit_candidate(&flawed, &registry, 3.0);
    println!("  Result: {} (Violations: {})",
        if audit_flawed.is_compliant { "COMPLIANT" } else { "NON-COMPLIANT - VIOLATES ESTABLISHED THEORIES" },
        audit_flawed.violation_count
    );
    for entry in &audit_flawed.entries {
        match &entry.status {
            EstablishedStatus::Subsumed { preserved_count } => {
                println!("    ✓ Compatible with established: {} ({} phenomena)", entry.established_theory_name, preserved_count);
            }
            EstablishedStatus::CompatibleDomain => {
                println!("    • Non-overlapping domain: {}", entry.established_theory_name);
            }
            EstablishedStatus::Violated { broken_phenomena, diagnostics } => {
                println!("    ✗ VIOLATED: {} [{}]", entry.established_theory_name, entry.established_theory_id);
                for (b, diag) in broken_phenomena.iter().zip(diagnostics.iter()) {
                    println!("      -> Breaks: {b} | {diag}");
                }
            }
        }
    }

    // 5. Verify Flawed Theory (PPVS Must Reject It!)
    println!("\n");
    print_separator();
    println!("  VERIFYING CANDIDATE: FLAWED MOMENTUM THEORY (NEGATIVE TEST)");
    print_separator();
    let report_flawed = verifier.verify(&flawed, &registry, Some(&newton));
    print_report(&report_flawed);

    print_separator();
    println!("  PPVS RUN COMPLETE: Physics verified deterministically against empirical reality.");
    print_separator();
}

fn print_report(report: &ppvs::verifier::VerificationReport) {
    println!("Theory: {} [{}]", report.theory_name, report.theory_id);
    println!("Evaluation Outcome: {:?}", report.outcome);
    println!(
        "Phenomena Evaluated: {} Total | {} In-Regime | {} Passed | {} Failed | {} Out-of-Regime",
        report.total_phenomena,
        report.in_regime_phenomena,
        report.passed_phenomena,
        report.failed_phenomena,
        report.out_of_regime_phenomena
    );
    println!("Symmetries Preserved:");
    for sym in &report.symmetries {
        println!("  • {} [{}] => Conserved: {}", sym.name, sym.transformation_group, sym.conserved_quantity);
    }
    println!("Phenomenon Status:");
    for res in &report.results {
        match &res.status {
            TestStatus::Passed { deviation_sigma, .. } => {
                println!("  [PASS] {} ({:.2}σ deviation)", res.title, deviation_sigma);
            }
            TestStatus::Failed { deviation_sigma, .. } => {
                println!("  [FAIL] {} ({:.2}σ deviation)", res.title, deviation_sigma);
            }
            TestStatus::OutOfRegime { reason } => {
                println!("  [SKIP] {} - Regime: {}", res.title, reason);
            }
            TestStatus::Error { message } => {
                println!("  [ERR ] {} - Error: {}", res.title, message);
            }
        }
    }

    if !report.failures.is_empty() {
        println!("\n  Diagnostic Failure Reports (Points to Refine):");
        for f in &report.failures {
            println!("  • [{}] {}", f.phenomenon_id, f.title);
            println!("    Observed Benchmark:   {}", f.observed);
            if let Some(pred) = f.predicted {
                println!("    Theory's Prediction:  {}", pred);
            }
            if let Some(sigma) = f.deviation_sigma {
                println!("    Deviation Discrepancy: {:.2}σ", sigma);
            }
            println!("    Why Failure Occurred: {}", f.reason);
            println!("    Experimental Reference: {}", f.citation);
        }
    }

    if let Some(reg_results) = &report.regression_results {
        println!("\nBackward Compatibility with Prior Theory:");
        for reg in reg_results {
            if reg.broken {
                println!("  [REGRESSION] {} - {}", reg.title, reg.notes);
            } else {
                println!("  [PRESERVED]  {} - {}", reg.title, reg.notes);
            }
        }
    }
}
