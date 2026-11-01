use ppvs::dimensions::Quantity;
use ppvs::empirical::{Context, Observation, PhenomenaRegistry};

#[test]
fn test_context_set_and_get() {
    let mut ctx = Context::new();
    ctx.set("speed", Quantity::velocity(100.0));
    ctx.set("mass", Quantity::mass(2.5));

    assert_eq!(ctx.get("speed"), Some(&Quantity::velocity(100.0)));
    assert_eq!(ctx.get("non_existent"), None);

    let required = ctx.get_required("mass").unwrap();
    assert_eq!(required, Quantity::mass(2.5));

    assert!(ctx.get_required("missing").is_err());
}

#[test]
fn test_observation_deviation_sigma() {
    let obs = Observation::new(
        "test_speed",
        "Test Speed Measurement",
        ppvs::empirical::FundamentalDomain::SpacetimeGravitation,
        Quantity::velocity(100.0),
        2.0, // sigma = 2.0
        Context::new(),
        "Synthetic Lab 2026",
    );

    // Exact match => 0.0 sigma
    let pred_exact = Quantity::velocity(100.0);
    assert_eq!(obs.deviation_sigma(&pred_exact).unwrap(), 0.0);

    // 104.0 => (104 - 100) / 2 = 2.0 sigma
    let pred_2sigma = Quantity::velocity(104.0);
    assert_eq!(obs.deviation_sigma(&pred_2sigma).unwrap(), 2.0);

    // Dimensional mismatch returns error
    let pred_bad_dim = Quantity::length(100.0);
    assert!(obs.deviation_sigma(&pred_bad_dim).is_err());
}

#[test]
fn test_standard_registry_completeness() {
    let reg = PhenomenaRegistry::standard();
    assert!(reg.len() >= 5);

    assert!(reg.get("gravitational_constant:codata2018").is_some());
    assert!(reg.get("kepler:earth_orbital_period").is_some());
    assert!(reg.get("relativity:muon_dilated_lifetime").is_some());
    assert!(reg.get("relativity:electron_momentum_at_0.8c").is_some());
    assert!(reg.get("gr:mercury_perihelion_excess").is_some());
}

#[test]
fn test_fundamental_domains_disjoint_partition_and_zero_overlap() {
    let reg = PhenomenaRegistry::standard();

    // 1. Must form a true mathematical partition
    assert!(reg.is_disjoint_partition());

    // 2. Count phenomena in each disjoint fundamental pillar
    let p1 = reg.in_domain(ppvs::empirical::FundamentalDomain::SpacetimeGravitation);
    let p2 = reg.in_domain(ppvs::empirical::FundamentalDomain::QuantumGaugeFields);
    let p3 = reg.in_domain(ppvs::empirical::FundamentalDomain::StatisticalThermodynamics);

    assert_eq!(p1.len(), 5, "SpacetimeGravitation should have 5 phenomena");
    assert_eq!(p2.len(), 4, "QuantumGaugeFields should have 4 phenomena");
    assert_eq!(p3.len(), 2, "StatisticalThermodynamics should have 2 phenomena");

    // 3. Total sum must equal 100% of all registered reality with zero overlap
    assert_eq!(p1.len() + p2.len() + p3.len(), reg.len());

    // 4. Zero overlap: verify disjoint IDs
    let mut seen_ids = std::collections::HashSet::new();
    for obs in reg.all() {
        assert!(seen_ids.insert(obs.id.clone()), "Duplicate phenomenon ID detected: {}", obs.id);
    }
}
