use rusticle::{Complex, Angle};

/// Test suite for the Complex type.
/// 
/// These tests verify the core functionality of the Complex type, including:
/// - Basic arithmetic operations
/// - Conversion between Cartesian and polar forms
/// - Advanced mathematical operations
mod complex_tests {
    use super::*;

    /// Tests basic arithmetic operations on complex numbers.
    #[test]
    fn test_complex_arithmetic() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, 4.0);
        
        // Addition
        let sum = a + b;
        assert_eq!(sum.real, 4.0);
        assert_eq!(sum.imag, 6.0);
        
        // Subtraction
        let diff = a - b;
        assert_eq!(diff.real, -2.0);
        assert_eq!(diff.imag, -2.0);
        
        // Multiplication
        let product = a * b;
        assert_eq!(product.real, -5.0);
        assert_eq!(product.imag, 10.0);
        
        // Division
        let quotient = a / b;
        assert!((quotient.real - 0.44).abs() < 1e-10);
        assert!((quotient.imag - 0.08).abs() < 1e-10);
        
        // Negation
        let neg = -a;
        assert_eq!(neg.real, -1.0);
        assert_eq!(neg.imag, -2.0);
    }

    /// Tests conversion between Cartesian and polar forms.
    #[test]
    fn test_polar_conversion() {
        // Test conversion from polar to Cartesian
        let z = Complex::from_polar(2.0, Angle::from_degrees(60.0));
        
        // cos(60°) = 0.5, sin(60°) ≈ 0.866
        assert!((z.real - 1.0).abs() < 1e-10);
        assert!((z.imag - 1.732).abs() < 1e-3);
        
        // Test conversion back to polar
        assert!((z.magnitude() - 2.0).abs() < 1e-10);
        assert!((z.angle().to_degrees() - 60.0).abs() < 1e-10);
    }

    /// Tests advanced mathematical operations on complex numbers.
    #[test]
    fn test_advanced_operations() {
        let z = Complex::new(3.0, 4.0);
        
        // Magnitude
        assert_eq!(z.magnitude(), 5.0);
        
        // Conjugate
        let conj = z.conjugate();
        assert_eq!(conj.real, 3.0);
        assert_eq!(conj.imag, -4.0);
        
        // Argument
        let arg = z.argument();
        assert!((arg - 0.927295218).abs() < 1e-6);
    }
    
    /// Tests the Debug trait implementation for Complex.
    #[test]
    fn test_complex_debug() {
        // Test positive imaginary part
        let z1 = Complex::new(3.0, 4.0);
        assert_eq!(format!("{:?}", z1), "3+4i");
        
        // Test negative imaginary part
        let z2 = Complex::new(3.0, -4.0);
        assert_eq!(format!("{:?}", z2), "3-4i");
        
        // Test zero imaginary part
        let z3 = Complex::new(3.0, 0.0);
        assert_eq!(format!("{:?}", z3), "3");
        
        // Test zero real part
        let z4 = Complex::new(0.0, 4.0);
        assert_eq!(format!("{:?}", z4), "0+4i");
        
        // Test both parts zero
        let z5 = Complex::new(0.0, 0.0);
        assert_eq!(format!("{:?}", z5), "0");
    }
} 