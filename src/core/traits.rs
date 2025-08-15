//! Core traits for the rusticle library
//! 
//! This module contains foundational traits that enable generic programming
//! and provide common mathematical operations across different types.

/// Trait for types that support complex conjugation.
/// 
/// This trait is essential for quantum computing operations where the conjugate
/// transpose (Hermitian conjugate) of matrices is frequently needed.
/// 
/// For complex numbers: conjugate(a + bi) = a - bi
/// For real numbers: conjugate(r) = r (real numbers are their own conjugate)
pub trait Conjugatable {
    /// Returns the complex conjugate of the type.
    /// 
    /// # Example
    /// ```
    /// use rusticle::Complex;
    /// use rusticle::core::Conjugatable;
    /// let a = Complex::new(1.0, 2.0);
    /// let b = a.conjugate();
    /// assert_eq!(b, Complex::new(1.0, -2.0));
    /// ```
    fn conjugate(self) -> Self;
}

// Implement Conjugatable for real number types
impl Conjugatable for f64 {
    #[inline(always)]
    fn conjugate(self) -> Self {
        self
    }
}

impl Conjugatable for f32 {
    #[inline(always)]
    fn conjugate(self) -> Self {
        self
    }
}

impl Conjugatable for i32 {
    #[inline(always)]
    fn conjugate(self) -> Self {
        self
    }
}

impl Conjugatable for i64 {
    #[inline(always)]
    fn conjugate(self) -> Self {
        self
    }
}

impl Conjugatable for i16 {
    #[inline(always)]
    fn conjugate(self) -> Self {
        self
    }
}

impl Conjugatable for i8 {
    #[inline(always)]
    fn conjugate(self) -> Self {
        self
    }
}

impl Conjugatable for u32 {
    #[inline(always)]
    fn conjugate(self) -> Self {
        self
    }
}

impl Conjugatable for u64 {
    #[inline(always)]
    fn conjugate(self) -> Self {
        self
    }
}

impl Conjugatable for u16 {
    #[inline(always)]
    fn conjugate(self) -> Self {
        self
    }
}

impl Conjugatable for u8 {
    #[inline(always)]
    fn conjugate(self) -> Self {
        self
    }
}