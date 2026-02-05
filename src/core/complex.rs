use std::{fmt::{Formatter, Result as FmtResult}, ops::{Add, Div, Mul, Sub}};
use std::fmt::{Display};

/// Complex numbers representation
#[derive(Copy, Clone, Debug, Default)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl Complex {
    // Initializers
    pub fn new(real: f64, imag: f64) -> Self {
        Complex { real, imag }
    }

    /// Norm-squared
    #[inline]
    pub fn norm_squared(&self) -> f64 {
        self.real * self.real + self.imag * self.imag
    }

    /// Norm
    #[inline]
    pub fn norm(&self) -> f64 {
        self.norm_squared().sqrt()
    }

    // Getters
    pub fn real(&self) -> f64 {
        self.real
    }

    pub fn imag(&self) -> f64 {
        self.imag
    }
}

/// Complex number addition
impl Add for Complex {
    type Output = Complex;

    #[inline]
    fn add(self, rhs: Complex) -> Complex {
        Complex {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag
        }
    }
}

/// Complex number subtraction
impl Sub for Complex {
    type Output = Complex;

    #[inline]
    fn sub(self, rhs: Complex) -> Complex {
        Complex {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag
        }
    }
}

/// Complex number multiplication
impl Mul for Complex {
    type Output = Complex;

    #[inline]
    fn mul(self, rhs: Complex) -> Complex {
        // (a + bi)*(c + di) = (ac - bd) + (ad + bc)i
        Complex {
            real: self.real * rhs.real - self.imag * rhs.imag,
            imag: self.real * rhs.imag + self.imag * rhs.real
        }
    }
}

/// Complex number division
impl Div for Complex {
    type Output = Complex;

    #[inline]
    fn div(self, rhs: Complex) -> Complex {
        // (a + bi)/(c + di) = [(ac + bd) + (bc - ad)i] / (c^2 + d^2)
        let denom = rhs.real * rhs.real + rhs.imag * rhs.imag;
        Complex {
            real: (self.real * rhs.real + self.imag * rhs.imag) / denom,
            imag: (self.imag * rhs.real - self.real * rhs.imag) / denom
        }
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match (self.real == 0.0, self.imag == 0.0) {
            (true, true) => write!(f, "0"),
            (true, false) => write!(f, "{}i", self.imag),
            (false, true) => write!(f, "{}", self.real),
            (false, false) if self.imag < 0.0 => write!(f, "{}{}i", self.real, self.imag),
            (false, false) => write!(f, "{}+{}i", self.real, self.imag)
        }
    }
}