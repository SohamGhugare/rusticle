use std::ops::{Add, Sub, Mul, Div};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex<T> {
    pub real: T,
    pub imag: T,
}

impl<T> Complex<T> {
    pub fn new(real: T, imag: T) -> Self {
        Self { real, imag }
    }
}

impl<T: Display + PartialEq + Default + PartialOrd> Display for Complex<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let zero = T::default();

        match (self.real == zero, self.imag == zero) {
            (true, true) => write!(f, "0"),
            (false, true) => write!(f, "{}", self.real),
            (true, false) => write!(f, "{}i", self.imag),
            (false, false) if self.imag < zero => write!(f, "{}{}i", self.real, self.imag),
            (false, false) => write!(f, "{}+{}i", self.real, self.imag),
        }
    }
}


impl<T: Copy + Add<Output = T>> Add for Complex<T> {
    type Output = Self;

    /// Adds two complex numbers.
    /// 
    /// # Example
    /// ```
    /// use rusticle::Complex;
    /// 
    /// let a = Complex::new(1.0, 2.0);
    /// let b = Complex::new(3.0, 4.0);
    /// 
    /// let c = a + b;
    /// 
    /// assert_eq!(c, Complex::new(4.0, 6.0));
    /// ```
    fn add(self, other: Self) -> Self {
        Self {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl<T: Copy + Sub<Output = T>> Sub for Complex<T> {
    type Output = Self;

    /// Subtracts two complex numbers.
    /// 
    /// # Example
    /// ```
    /// use rusticle::Complex;
    /// 
    /// let a = Complex::new(3.0, 4.0);
    /// let b = Complex::new(1.0, 2.0);
    /// 
    /// let c = a - b;
    /// 
    /// assert_eq!(c, Complex::new(2.0, 2.0));
    /// ```
    fn sub(self, other: Self) -> Self {
        Self {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }
}

impl<T: Copy + Mul<Output = T> + Sub<Output = T> + Add<Output = T>> Mul for Complex<T> {
    type Output = Self;

    /// Multiplies two complex numbers.
    /// 
    /// # Example
    /// ```
    /// use rusticle::Complex;
    /// 
    /// let a = Complex::new(1.0, 2.0);
    /// let b = Complex::new(3.0, 4.0);
    /// 
    /// let c = a * b;
    /// 
    /// assert_eq!(c, Complex::new(-5.0, 10.0));
    /// ```
    fn mul(self, other: Self) -> Self {
        Self {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
}

impl<T: Copy + Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Div<Output = T>> Div for Complex<T> {
    type Output = Self;

    /// Divides two complex numbers.
    /// 
    /// # Example
    /// ```
    /// use rusticle::Complex;
    /// 
    /// let a = Complex::new(3.0, 2.0);
    /// let b = Complex::new(1.0, 1.0);
    /// 
    /// let c = a / b;
    /// 
    /// assert_eq!(c, Complex::new(2.5, 0.5));
    /// ```
    fn div(self, other: Self) -> Self {
        let denominator = other.real * other.real + other.imag * other.imag;
        Self {
            real: (self.real * other.real + self.imag * other.imag) / denominator,
            imag: (self.imag * other.real - self.real * other.imag) / denominator,
        }
    }
}