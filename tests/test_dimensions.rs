use ppvs::dimensions::{Dimension, DimensionError, Quantity};

#[test]
fn test_base_dimensions_and_combinations() {
    let mass = Dimension::mass();
    let length = Dimension::length();
    let time = Dimension::time();

    // Velocity = L * T^-1
    let velocity = length / time;
    assert_eq!(velocity, Dimension::velocity());

    // Acceleration = L * T^-2
    let acceleration = velocity / time;
    assert_eq!(acceleration, Dimension::acceleration());

    // Force = M * L * T^-2
    let force = mass * acceleration;
    assert_eq!(force, Dimension::force());

    // Energy = Force * L = M * L^2 * T^-2
    let energy = force * length;
    assert_eq!(energy, Dimension::energy());

    // Power = Energy / T = M * L^2 * T^-3
    let power = energy / time;
    assert_eq!(power, Dimension::power());

    // Dimensionless
    let ratio = length / length;
    assert!(ratio.is_dimensionless());
}

#[test]
fn test_quantity_addition_and_subtraction_dimensional_safety() {
    let q1 = Quantity::length(10.0);
    let q2 = Quantity::length(25.5);
    let q_mass = Quantity::mass(5.0);

    // Matching dimensions add cleanly
    let sum = (q1 + q2).unwrap();
    assert_eq!(sum.value, 35.5);
    assert_eq!(sum.dimension, Dimension::length());

    // Matching dimensions subtract cleanly
    let diff = (q2 - q1).unwrap();
    assert_eq!(diff.value, 15.5);
    assert_eq!(diff.dimension, Dimension::length());

    // Dimensional mismatch must return an error
    let mismatch = q1 + q_mass;
    assert_eq!(
        mismatch,
        Err(DimensionError::Mismatch {
            expected: Dimension::length(),
            actual: Dimension::mass(),
        })
    );
}

#[test]
fn test_quantity_multiplication_and_division() {
    let mass = Quantity::mass(10.0);
    let acc = Quantity::acceleration(9.81);

    // F = m * a
    let force = mass * acc;
    assert_eq!(force.dimension, Dimension::force());
    assert!((force.value - 98.1).abs() < 1e-10);

    // a = F / m
    let recovered_acc = (force / mass).unwrap();
    assert_eq!(recovered_acc.dimension, Dimension::acceleration());
    assert!((recovered_acc.value - 9.81).abs() < 1e-10);

    // Division by zero
    let zero_mass = Quantity::mass(0.0);
    assert_eq!(force / zero_mass, Err(DimensionError::DivisionByZero));
}

#[test]
fn test_quantity_powers_and_sqrt() {
    let length = Quantity::length(4.0);
    let area = length.powi(2);
    assert_eq!(area.value, 16.0);
    assert_eq!(area.dimension, Dimension::new(0, 2, 0, 0, 0, 0, 0));

    let sqrt_area = area.sqrt().unwrap();
    assert_eq!(sqrt_area.value, 4.0);
    assert_eq!(sqrt_area.dimension, Dimension::length());

    // Sqrt of odd dimension exponent fails
    let bad_sqrt = length.sqrt();
    assert!(bad_sqrt.is_err());
}

#[test]
fn test_electrodynamic_dimensions_and_light_speed_relation() {
    let eps0_dim = Dimension::permittivity();
    let mu0_dim = Dimension::permeability();

    // eps0 * mu0 = M^-1 L^-3 T^4 I^2 * M L T^-2 I^-2 = L^-2 T^2
    let prod_dim = eps0_dim * mu0_dim;
    assert_eq!(prod_dim, Dimension::new(0, -2, 2, 0, 0, 0, 0));

    // 1 / (eps0 * mu0) = L^2 T^-2 (velocity squared)
    let inv_prod = Dimension::dimensionless() / prod_dim;
    assert_eq!(inv_prod, Dimension::velocity().powi(2));
}

#[test]
fn test_thermodynamic_dimensions_and_gas_law() {
    let p = Dimension::pressure();
    let v = Dimension::volume();

    // P * V = Energy = M L^2 T^-2
    let pv = p * v;
    assert_eq!(pv, Dimension::energy());

    // R = P * V / (n * T)
    let r_dim = pv / (Dimension::amount() * Dimension::temperature());
    assert_eq!(r_dim, Dimension::gas_constant());
}
