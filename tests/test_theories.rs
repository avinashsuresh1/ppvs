use ppvs::empirical::PhenomenaRegistry;
use ppvs::theories::{GeneralRelativity, NewtonianMechanics, SpecialRelativity};
use ppvs::theory::Theory;

#[test]
fn test_newtonian_kepler_prediction() {
    let reg = PhenomenaRegistry::standard();
    let obs = reg.get("kepler:earth_orbital_period").unwrap();

    let newton = NewtonianMechanics::new();
    assert!(newton.is_in_regime(&obs.id, &obs.context).is_ok());

    let pred = newton.predict(&obs.id, &obs.context).unwrap();
    let sigma = obs.deviation_sigma(&pred).unwrap();

    // Keplerian orbital period of Earth is matched within 1.0 sigma
    assert!(sigma < 1.0, "Newtonian orbital period sigma deviation: {sigma}");
}

#[test]
fn test_special_relativity_muon_dilation() {
    let reg = PhenomenaRegistry::standard();
    let obs = reg.get("relativity:muon_dilated_lifetime").unwrap();

    let sr = SpecialRelativity::new();
    assert!(sr.is_in_regime(&obs.id, &obs.context).is_ok());

    let pred = sr.predict(&obs.id, &obs.context).unwrap();
    let sigma = obs.deviation_sigma(&pred).unwrap();

    // SR prediction matches CERN 1977 muon lifetime experiment within 2.5 sigma
    assert!(sigma < 2.5, "SR muon dilation sigma deviation: {sigma}");
}

#[test]
fn test_general_relativity_mercury_perihelion_precession() {
    let reg = PhenomenaRegistry::standard();
    let obs = reg.get("gr:mercury_perihelion_excess").unwrap();

    let gr = GeneralRelativity::new();
    assert!(gr.is_in_regime(&obs.id, &obs.context).is_ok());

    let pred = gr.predict(&obs.id, &obs.context).unwrap();
    let sigma = obs.deviation_sigma(&pred).unwrap();

    // GR predicts 42.98 arcsec / century within 1 sigma of observation
    assert!(sigma < 1.0, "GR Mercury precession sigma deviation: {sigma}");
}

#[test]
fn test_maxwell_wave_speed_and_coulomb_prediction() {
    let reg = PhenomenaRegistry::standard();
    let maxwell = ppvs::theories::MaxwellianElectrodynamics::new();

    let obs_speed = reg.get("em:maxwell_wave_speed").unwrap();
    let pred_speed = maxwell.predict(&obs_speed.id, &obs_speed.context).unwrap();
    let sigma_speed = obs_speed.deviation_sigma(&pred_speed).unwrap();
    assert!(sigma_speed < 0.1, "Maxwell wave speed deviation: {sigma_speed}");

    let obs_coulomb = reg.get("em:coulomb_force").unwrap();
    let pred_coulomb = maxwell.predict(&obs_coulomb.id, &obs_coulomb.context).unwrap();
    let sigma_coulomb = obs_coulomb.deviation_sigma(&pred_coulomb).unwrap();
    assert!(sigma_coulomb < 0.1, "Coulomb force deviation: {sigma_coulomb}");
}

#[test]
fn test_thermodynamics_gas_volume_and_carnot_prediction() {
    let reg = PhenomenaRegistry::standard();
    let thermo = ppvs::theories::ClassicalThermodynamics::new();

    let obs_gas = reg.get("thermo:ideal_gas_molar_volume").unwrap();
    let pred_gas = thermo.predict(&obs_gas.id, &obs_gas.context).unwrap();
    let sigma_gas = obs_gas.deviation_sigma(&pred_gas).unwrap();
    assert!(sigma_gas < 0.5, "Ideal gas molar volume deviation: {sigma_gas}");

    let obs_carnot = reg.get("thermo:carnot_efficiency").unwrap();
    let pred_carnot = thermo.predict(&obs_carnot.id, &obs_carnot.context).unwrap();
    let sigma_carnot = obs_carnot.deviation_sigma(&pred_carnot).unwrap();
    assert!(sigma_carnot < 0.1, "Carnot efficiency deviation: {sigma_carnot}");
}

#[test]
fn test_quantum_photoelectric_and_balmer_prediction() {
    let reg = PhenomenaRegistry::standard();
    let quantum = ppvs::theories::QuantumMechanics::new();

    let obs_photo = reg.get("quantum:photoelectric_stopping_voltage").unwrap();
    let pred_photo = quantum.predict(&obs_photo.id, &obs_photo.context).unwrap();
    let sigma_photo = obs_photo.deviation_sigma(&pred_photo).unwrap();
    assert!(sigma_photo < 0.1, "Photoelectric stopping voltage deviation: {sigma_photo}");

    let obs_balmer = reg.get("quantum:hydrogen_balmer_alpha").unwrap();
    let pred_balmer = quantum.predict(&obs_balmer.id, &obs_balmer.context).unwrap();
    let sigma_balmer = obs_balmer.deviation_sigma(&pred_balmer).unwrap();
    assert!(sigma_balmer < 0.5, "Hydrogen Balmer alpha wavelength deviation: {sigma_balmer}");
}
