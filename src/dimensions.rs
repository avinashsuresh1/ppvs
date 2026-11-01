use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum DimensionError {
    #[error("Dimension mismatch: expected {expected}, got {actual}")]
    Mismatch {
        expected: Dimension,
        actual: Dimension,
    },
    #[error("Division by zero in quantity arithmetic")]
    DivisionByZero,
}

/// Fundamental physical dimensions in the SI system:
/// - m: Mass (kilogram)
/// - l: Length (meter)
/// - t: Time (second)
/// - i: Electric Current (ampere)
/// - th: Thermodynamic Temperature (kelvin)
/// - n: Amount of substance (mole)
/// - j: Luminous Intensity (candela)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Dimension {
    pub m: i8,
    pub l: i8,
    pub t: i8,
    pub i: i8,
    pub th: i8,
    pub n: i8,
    pub j: i8,
}

impl Dimension {
    pub const fn new(m: i8, l: i8, t: i8, i: i8, th: i8, n: i8, j: i8) -> Self {
        Self { m, l, t, i, th, n, j }
    }

    pub const fn dimensionless() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0)
    }

    pub const fn mass() -> Self {
        Self::new(1, 0, 0, 0, 0, 0, 0)
    }

    pub const fn length() -> Self {
        Self::new(0, 1, 0, 0, 0, 0, 0)
    }

    pub const fn time() -> Self {
        Self::new(0, 0, 1, 0, 0, 0, 0)
    }

    pub const fn current() -> Self {
        Self::new(0, 0, 0, 1, 0, 0, 0)
    }

    pub const fn temperature() -> Self {
        Self::new(0, 0, 0, 0, 1, 0, 0)
    }

    pub const fn amount() -> Self {
        Self::new(0, 0, 0, 0, 0, 1, 0)
    }

    pub const fn luminous_intensity() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 1)
    }

    pub const fn velocity() -> Self {
        Self::new(0, 1, -1, 0, 0, 0, 0)
    }

    pub const fn acceleration() -> Self {
        Self::new(0, 1, -2, 0, 0, 0, 0)
    }

    pub const fn force() -> Self {
        Self::new(1, 1, -2, 0, 0, 0, 0)
    }

    pub const fn energy() -> Self {
        Self::new(1, 2, -2, 0, 0, 0, 0)
    }

    pub const fn power() -> Self {
        Self::new(1, 2, -3, 0, 0, 0, 0)
    }

    pub const fn momentum() -> Self {
        Self::new(1, 1, -1, 0, 0, 0, 0)
    }

    pub const fn gravitational_constant() -> Self {
        Self::new(-1, 3, -2, 0, 0, 0, 0)
    }

    pub const fn charge() -> Self {
        Self::new(0, 0, 1, 1, 0, 0, 0)
    }

    pub const fn electric_potential() -> Self {
        Self::new(1, 2, -3, -1, 0, 0, 0)
    }

    pub const fn permittivity() -> Self {
        Self::new(-1, -3, 4, 2, 0, 0, 0)
    }

    pub const fn permeability() -> Self {
        Self::new(1, 1, -2, -2, 0, 0, 0)
    }

    pub const fn action() -> Self {
        Self::new(1, 2, -1, 0, 0, 0, 0)
    }

    pub const fn pressure() -> Self {
        Self::new(1, -1, -2, 0, 0, 0, 0)
    }

    pub const fn volume() -> Self {
        Self::new(0, 3, 0, 0, 0, 0, 0)
    }

    pub const fn gas_constant() -> Self {
        Self::new(1, 2, -2, 0, -1, -1, 0)
    }

    pub const fn boltzmann_constant() -> Self {
        Self::new(1, 2, -2, 0, -1, 0, 0)
    }

    pub const fn is_dimensionless(&self) -> bool {
        self.m == 0
            && self.l == 0
            && self.t == 0
            && self.i == 0
            && self.th == 0
            && self.n == 0
            && self.j == 0
    }

    pub const fn powi(&self, exp: i8) -> Self {
        Self {
            m: self.m * exp,
            l: self.l * exp,
            t: self.t * exp,
            i: self.i * exp,
            th: self.th * exp,
            n: self.n * exp,
            j: self.j * exp,
        }
    }
}

impl Mul for Dimension {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            m: self.m + rhs.m,
            l: self.l + rhs.l,
            t: self.t + rhs.t,
            i: self.i + rhs.i,
            th: self.th + rhs.th,
            n: self.n + rhs.n,
            j: self.j + rhs.j,
        }
    }
}

impl Div for Dimension {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            m: self.m - rhs.m,
            l: self.l - rhs.l,
            t: self.t - rhs.t,
            i: self.i - rhs.i,
            th: self.th - rhs.th,
            n: self.n - rhs.n,
            j: self.j - rhs.j,
        }
    }
}

impl fmt::Display for Dimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_dimensionless() {
            return write!(f, "1");
        }
        let parts = [
            ("M", self.m),
            ("L", self.l),
            ("T", self.t),
            ("I", self.i),
            ("Θ", self.th),
            ("N", self.n),
            ("J", self.j),
        ];
        let mut first = true;
        for (symbol, exp) in parts {
            if exp != 0 {
                if !first {
                    write!(f, "·")?;
                }
                if exp == 1 {
                    write!(f, "{symbol}")?;
                } else {
                    write!(f, "{symbol}^{exp}")?;
                }
                first = false;
            }
        }
        Ok(())
    }
}

/// A physical quantity with an explicit numerical magnitude and physical dimension.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Quantity {
    pub value: f64,
    pub dimension: Dimension,
}

impl Quantity {
    pub const fn new(value: f64, dimension: Dimension) -> Self {
        Self { value, dimension }
    }

    pub const fn dimensionless(value: f64) -> Self {
        Self::new(value, Dimension::dimensionless())
    }

    pub const fn mass(kg: f64) -> Self {
        Self::new(kg, Dimension::mass())
    }

    pub const fn length(meters: f64) -> Self {
        Self::new(meters, Dimension::length())
    }

    pub const fn time(seconds: f64) -> Self {
        Self::new(seconds, Dimension::time())
    }

    pub const fn velocity(m_per_s: f64) -> Self {
        Self::new(m_per_s, Dimension::velocity())
    }

    pub const fn acceleration(m_per_s2: f64) -> Self {
        Self::new(m_per_s2, Dimension::acceleration())
    }

    pub const fn force(newtons: f64) -> Self {
        Self::new(newtons, Dimension::force())
    }

    pub const fn energy(joules: f64) -> Self {
        Self::new(joules, Dimension::energy())
    }

    pub const fn momentum(kg_m_per_s: f64) -> Self {
        Self::new(kg_m_per_s, Dimension::momentum())
    }

    pub const fn gravitational_constant(g: f64) -> Self {
        Self::new(g, Dimension::gravitational_constant())
    }

    pub const fn charge(coulombs: f64) -> Self {
        Self::new(coulombs, Dimension::charge())
    }

    pub const fn electric_potential(volts: f64) -> Self {
        Self::new(volts, Dimension::electric_potential())
    }

    pub const fn action(joule_seconds: f64) -> Self {
        Self::new(joule_seconds, Dimension::action())
    }

    pub const fn permittivity(f_per_m: f64) -> Self {
        Self::new(f_per_m, Dimension::permittivity())
    }

    pub const fn permeability(h_per_m: f64) -> Self {
        Self::new(h_per_m, Dimension::permeability())
    }

    pub const fn temperature_kelvin(kelvin: f64) -> Self {
        Self::new(kelvin, Dimension::temperature())
    }

    pub const fn pressure(pascals: f64) -> Self {
        Self::new(pascals, Dimension::pressure())
    }

    pub const fn volume(m3: f64) -> Self {
        Self::new(m3, Dimension::volume())
    }

    pub const fn gas_constant(r: f64) -> Self {
        Self::new(r, Dimension::gas_constant())
    }

    pub const fn boltzmann_constant(k: f64) -> Self {
        Self::new(k, Dimension::boltzmann_constant())
    }

    pub fn check_dimension(&self, expected: Dimension) -> Result<(), DimensionError> {
        if self.dimension == expected {
            Ok(())
        } else {
            Err(DimensionError::Mismatch {
                expected,
                actual: self.dimension,
            })
        }
    }

    pub fn powi(self, exp: i8) -> Self {
        Self {
            value: self.value.powi(exp as i32),
            dimension: self.dimension.powi(exp),
        }
    }

    pub fn sqrt(self) -> Result<Self, String> {
        if self.value < 0.0 {
            return Err("Cannot take square root of negative quantity value".to_string());
        }
        // Dimensions must be even for an integer square root
        let d = self.dimension;
        if d.m % 2 != 0 || d.l % 2 != 0 || d.t % 2 != 0 || d.i % 2 != 0 || d.th % 2 != 0 || d.n % 2 != 0 || d.j % 2 != 0 {
            return Err(format!("Cannot take integer square root of dimension {d}"));
        }
        Ok(Self {
            value: self.value.sqrt(),
            dimension: Dimension::new(
                d.m / 2,
                d.l / 2,
                d.t / 2,
                d.i / 2,
                d.th / 2,
                d.n / 2,
                d.j / 2,
            ),
        })
    }
}

impl Add for Quantity {
    type Output = Result<Self, DimensionError>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.dimension != rhs.dimension {
            return Err(DimensionError::Mismatch {
                expected: self.dimension,
                actual: rhs.dimension,
            });
        }
        Ok(Self {
            value: self.value + rhs.value,
            dimension: self.dimension,
        })
    }
}

impl Sub for Quantity {
    type Output = Result<Self, DimensionError>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.dimension != rhs.dimension {
            return Err(DimensionError::Mismatch {
                expected: self.dimension,
                actual: rhs.dimension,
            });
        }
        Ok(Self {
            value: self.value - rhs.value,
            dimension: self.dimension,
        })
    }
}

impl Mul for Quantity {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value * rhs.value,
            dimension: self.dimension * rhs.dimension,
        }
    }
}

impl Div for Quantity {
    type Output = Result<Self, DimensionError>;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs.value == 0.0 {
            return Err(DimensionError::DivisionByZero);
        }
        Ok(Self {
            value: self.value / rhs.value,
            dimension: self.dimension / rhs.dimension,
        })
    }
}

impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.6e} [{}]", self.value, self.dimension)
    }
}
