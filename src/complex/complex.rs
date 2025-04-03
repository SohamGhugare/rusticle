use std::ops::{Add, Sub, Mul, Div, Neg};
use super::angle::Angle;

/// A complex number represented as a + bi
/// 
/// This type provides a comprehensive set of operations for working with complex numbers,
/// including basic arithmetic, conversion between Cartesian and polar forms, and
/// advanced mathematical operations.
/// 
/// # Examples
/// 
/// ```
/// use rusticle::{Complex, Angle};
/// 
/// // Create a complex number in Cartesian form
/// let z1 = Complex::new(3.0, 4.0);
/// 
/// // Create a complex number in polar form
/// let z2 = Complex::from_polar(5.0, Angle::from_degrees(30.0));
/// 
/// // Basic arithmetic
/// let sum = z1 + z2;
/// let product = z1 * z2;
/// 
/// // Advanced operations
/// let magnitude = z1.magnitude();
/// let conjugate = z1.conjugate();
/// let argument = z1.argument();
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    /// The real part of the complex number
    pub real: f64,
    /// The imaginary part of the complex number
    pub imag: f64,
}

impl Complex {
    /// Creates a new complex number from its real and imaginary parts (Cartesian form)
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rusticle::Complex;
    /// 
    /// let z = Complex::new(3.0, 4.0);
    /// assert_eq!(z.real, 3.0);
    /// assert_eq!(z.imag, 4.0);
    /// ```
    pub fn new(real: f64, imag: f64) -> Self {
        Complex { real, imag }
    }

    /// Creates a complex number from its polar form (magnitude and angle)
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rusticle::{Complex, Angle};
    /// use std::f64::consts::PI;
    /// 
    /// let z = Complex::from_polar(2.0, Angle::from_radians(PI / 4.0));
    /// assert!((z.real - 2.0 * (PI / 4.0).cos()).abs() < 1e-10);
    /// assert!((z.imag - 2.0 * (PI / 4.0).sin()).abs() < 1e-10);
    /// ```
    pub fn from_polar(magnitude: f64, angle: Angle) -> Self {
        let radians = angle.to_radians();
        Complex {
            real: magnitude * radians.cos(),
            imag: magnitude * radians.sin(),
        }
    }

    /// Returns the magnitude (absolute value) of the complex number
    /// 
    /// The magnitude is the distance from the origin to the point in the complex plane.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rusticle::Complex;
    /// 
    /// let z = Complex::new(3.0, 4.0);
    /// assert_eq!(z.magnitude(), 5.0);
    /// ```
    pub fn magnitude(&self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }

    /// Returns the argument (angle) of the complex number in radians
    /// 
    /// The argument is the angle between the positive real axis and the line
    /// joining the origin to the point in the complex plane.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rusticle::Complex;
    /// use std::f64::consts::PI;
    /// 
    /// let z = Complex::new(0.0, 1.0);
    /// assert!((z.argument() - PI / 2.0).abs() < 1e-10);
    /// ```
    pub fn argument(&self) -> f64 {
        self.imag.atan2(self.real)
    }

    /// Returns the argument as an Angle
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rusticle::{Complex, Angle};
    /// 
    /// let z = Complex::new(0.0, 1.0);
    /// assert_eq!(z.angle().to_degrees(), 90.0);
    /// ```
    pub fn angle(&self) -> Angle {
        Angle::from_radians(self.argument())
    }

    /// Returns the complex conjugate of this number
    /// 
    /// The complex conjugate of a + bi is a - bi.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rusticle::Complex;
    /// 
    /// let z = Complex::new(3.0, 4.0);
    /// let conjugate = z.conjugate();
    /// assert_eq!(conjugate.real, 3.0);
    /// assert_eq!(conjugate.imag, -4.0);
    /// ```
    pub fn conjugate(&self) -> Self {
        Complex {
            real: self.real,
            imag: -self.imag,
        }
    }

    /// Returns the square of the magnitude of the complex number
    /// 
    /// This method calculates the square of the magnitude of the complex number,
    /// which is equivalent to the product of the complex number with its conjugate.
    /// 
    /// # Examples  
    /// 
    /// ```
    /// use rusticle::Complex;
    /// 
    /// let z = Complex::new(3.0, 4.0);
    /// assert_eq!(z.magnitude_squared(), 25.0);
    /// ```
    pub fn magnitude_squared(&self) -> f64 {
        self.real * self.real + self.imag * self.imag
    }

}

// Implement standard arithmetic operations
impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl Sub for Complex {
    type Output = Complex;

    fn sub(self, other: Complex) -> Complex {
        Complex {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        Complex {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
}

impl Div for Complex {
    type Output = Complex;

    fn div(self, other: Complex) -> Complex {
        let denominator = other.magnitude_squared();
        Complex {
            real: (self.real * other.real + self.imag * other.imag) / denominator,
            imag: (self.imag * other.real - self.real * other.imag) / denominator,
        }
    }
}

impl Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Complex {
        Complex {
            real: -self.real,
            imag: -self.imag,
        }
    }
}

// Implement scalar operations
impl Mul<f64> for Complex {
    type Output = Complex;

    fn mul(self, scalar: f64) -> Complex {
        Complex {
            real: self.real * scalar,
            imag: self.imag * scalar,
        }
    }
}

impl Div<f64> for Complex {
    type Output = Complex;

    fn div(self, scalar: f64) -> Complex {
        Complex {
            real: self.real / scalar,
            imag: self.imag / scalar,
        }
    }
}

// Implement From trait for easy conversion
impl From<f64> for Complex {
    fn from(real: f64) -> Self {
        Complex::new(real, 0.0)
    }
} 