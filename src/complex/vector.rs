use std::ops::{Add, Sub, Mul, Neg};
use std::fmt;
use super::complex::Complex;

/// A vector of complex numbers
/// 
/// This type provides operations for working with vectors of complex numbers,
/// including basic arithmetic, inner product, and norm calculations.
/// 
/// # Examples
/// 
/// ```
/// use rusticle::complex::{Complex, ComplexVector};
/// 
/// // Create vectors
/// let v1 = ComplexVector::new(vec![Complex::new(1.0, 2.0), Complex::new(3.0, 4.0)]);
/// let v2 = ComplexVector::new(vec![Complex::new(5.0, 6.0), Complex::new(7.0, 8.0)]);
/// 
/// // Vector addition
/// let sum = v1.clone() + v2.clone();
/// 
/// // Scalar multiplication
/// let scaled = v1.clone() * 2.0;
/// 
/// // Inner product
/// let inner_prod = v1.inner_product(&v2);
/// 
/// // Vector norm
/// let norm = v1.norm();
/// ```
#[derive(Clone, PartialEq)]
pub struct ComplexVector {
    /// The components of the vector
    pub components: Vec<Complex>,
}

impl ComplexVector {
    /// Creates a new complex vector from a vector of complex numbers
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rusticle::complex::{Complex, ComplexVector};
    /// 
    /// let v = ComplexVector::new(vec![Complex::new(1.0, 2.0), Complex::new(3.0, 4.0)]);
    /// assert_eq!(v.dimension(), 2);
    /// ```
    pub fn new(components: Vec<Complex>) -> Self {
        ComplexVector { components }
    }
    
    /// Creates a zero vector of the specified dimension
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rusticle::complex::ComplexVector;
    /// 
    /// let v = ComplexVector::zeros(3);
    /// assert_eq!(v.dimension(), 3);
    /// assert!(v.is_zero());
    /// ```
    pub fn zeros(dimension: usize) -> Self {
        ComplexVector {
            components: vec![Complex::new(0.0, 0.0); dimension],
        }
    }
    
    /// Returns the dimension of the vector
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rusticle::complex::{Complex, ComplexVector};
    /// 
    /// let v = ComplexVector::new(vec![Complex::new(1.0, 2.0), Complex::new(3.0, 4.0)]);
    /// assert_eq!(v.dimension(), 2);
    /// ```
    pub fn dimension(&self) -> usize {
        self.components.len()
    }
    
    /// Checks if the vector is a zero vector
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rusticle::complex::{Complex, ComplexVector};
    /// 
    /// let v1 = ComplexVector::zeros(2);
    /// assert!(v1.is_zero());
    /// 
    /// let v2 = ComplexVector::new(vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)]);
    /// assert!(!v2.is_zero());
    /// ```
    pub fn is_zero(&self) -> bool {
        self.components.iter().all(|c| c.real == 0.0 && c.imag == 0.0)
    }
    
    /// Returns the Euclidean norm (magnitude) of the vector
    /// 
    /// The Euclidean norm is the square root of the sum of the squares of the magnitudes
    /// of each component.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rusticle::complex::{Complex, ComplexVector};
    /// 
    /// let v = ComplexVector::new(vec![Complex::new(3.0, 4.0), Complex::new(0.0, 5.0)]);
    /// assert!((v.norm() - 7.07).abs() < 0.01); // sqrt(5^2 + 5^2) = sqrt(50) ≈ 7.07
    /// ```
    pub fn norm(&self) -> f64 {
        let sum_squares = self.components.iter()
            .map(|c| c.magnitude_squared())
            .sum::<f64>();
        sum_squares.sqrt()
    }
    
    /// Returns the squared Euclidean norm of the vector
    /// 
    /// This is more efficient than computing the norm and then squaring it.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rusticle::complex::{Complex, ComplexVector};
    /// 
    /// let v = ComplexVector::new(vec![Complex::new(3.0, 4.0), Complex::new(0.0, 5.0)]);
    /// assert_eq!(v.norm_squared(), 50.0); // 5^2 + 5^2 = 50
    /// ```
    pub fn norm_squared(&self) -> f64 {
        self.components.iter()
            .map(|c| c.magnitude_squared())
            .sum::<f64>()
    }
    
    /// Returns the inner product (dot product) of this vector with another vector
    /// 
    /// The inner product is the sum of the products of corresponding components,
    /// where the second vector's components are conjugated.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rusticle::complex::{Complex, ComplexVector};
    /// 
    /// let v1 = ComplexVector::new(vec![Complex::new(1.0, 2.0), Complex::new(3.0, 4.0)]);
    /// let v2 = ComplexVector::new(vec![Complex::new(5.0, 6.0), Complex::new(7.0, 8.0)]);
    /// 
    /// let inner_prod = v1.inner_product(&v2);
    /// // (1+2i)(5-6i) + (3+4i)(7-8i) = (17+4i) + (53+4i) = 70+8i
    /// assert_eq!(inner_prod.real, 70.0);
    /// assert_eq!(inner_prod.imag, 8.0);
    /// ```
    pub fn inner_product(&self, other: &ComplexVector) -> Complex {
        assert_eq!(self.dimension(), other.dimension(), "Vectors must have the same dimension for inner product");
        
        let mut result = Complex::new(0.0, 0.0);
        for i in 0..self.dimension() {
            result = result + self.components[i] * other.components[i].conjugate();
        }
        result
    }
    
    /// Returns the normalized version of this vector (unit vector)
    /// 
    /// The normalized vector has the same direction but a magnitude of 1.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use rusticle::complex::{Complex, ComplexVector};
    /// 
    /// let v = ComplexVector::new(vec![Complex::new(3.0, 4.0), Complex::new(0.0, 5.0)]);
    /// let normalized = v.normalize();
    /// assert!((normalized.norm() - 1.0).abs() < 1e-10);
    /// ```
    pub fn normalize(&self) -> Self {
        let norm = self.norm();
        assert!(norm != 0.0, "Cannot normalize a zero vector");
        
        let mut normalized = self.clone();
        for i in 0..self.dimension() {
            normalized.components[i] = normalized.components[i] / norm;
        }
        normalized
    }
}

/// Custom Debug implementation for ComplexVector
impl fmt::Debug for ComplexVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, component) in self.components.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", component)?;
        }
        write!(f, "]")
    }
}

/// Implement vector addition
impl Add for ComplexVector {
    type Output = ComplexVector;
    
    fn add(self, other: ComplexVector) -> ComplexVector {
        assert_eq!(self.dimension(), other.dimension(), "Vectors must have the same dimension for addition");
        
        let mut result = Vec::with_capacity(self.dimension());
        for i in 0..self.dimension() {
            result.push(self.components[i] + other.components[i]);
        }
        ComplexVector::new(result)
    }
}

/// Implement vector subtraction
impl Sub for ComplexVector {
    type Output = ComplexVector;
    
    fn sub(self, other: ComplexVector) -> ComplexVector {
        assert_eq!(self.dimension(), other.dimension(), "Vectors must have the same dimension for subtraction");
        
        let mut result = Vec::with_capacity(self.dimension());
        for i in 0..self.dimension() {
            result.push(self.components[i] - other.components[i]);
        }
        ComplexVector::new(result)
    }
}

/// Implement scalar multiplication (vector * scalar)
impl Mul<f64> for ComplexVector {
    type Output = ComplexVector;
    
    fn mul(self, scalar: f64) -> ComplexVector {
        let mut result = Vec::with_capacity(self.dimension());
        for i in 0..self.dimension() {
            result.push(self.components[i] * scalar);
        }
        ComplexVector::new(result)
    }
}

/// Implement scalar multiplication (scalar * vector)
impl Mul<ComplexVector> for f64 {
    type Output = ComplexVector;
    
    fn mul(self, vector: ComplexVector) -> ComplexVector {
        vector * self
    }
}

/// Implement vector negation
impl Neg for ComplexVector {
    type Output = ComplexVector;
    
    fn neg(self) -> ComplexVector {
        let mut result = Vec::with_capacity(self.dimension());
        for i in 0..self.dimension() {
            result.push(-self.components[i]);
        }
        ComplexVector::new(result)
    }
}
