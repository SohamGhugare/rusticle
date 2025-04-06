use rusticle::complex::{Complex, ComplexVector};

/// Test suite for the ComplexVector type.
/// 
/// These tests verify the core functionality of the ComplexVector type, including:
/// - Basic vector operations (addition, subtraction, scalar multiplication)
/// - Inner product and norm calculations
/// - Vector normalization
mod vector_tests {
    use super::*;

    /// Tests basic vector operations.
    #[test]
    fn test_vector_operations() {
        let v1 = ComplexVector::new(vec![Complex::new(1.0, 2.0), Complex::new(3.0, 4.0)]);
        let v2 = ComplexVector::new(vec![Complex::new(5.0, 6.0), Complex::new(7.0, 8.0)]);
        
        // Addition
        let sum = v1.clone() + v2.clone();
        assert_eq!(sum.components[0].real, 6.0);
        assert_eq!(sum.components[0].imag, 8.0);
        assert_eq!(sum.components[1].real, 10.0);
        assert_eq!(sum.components[1].imag, 12.0);
        
        // Subtraction
        let diff = v1.clone() - v2.clone();
        assert_eq!(diff.components[0].real, -4.0);
        assert_eq!(diff.components[0].imag, -4.0);
        assert_eq!(diff.components[1].real, -4.0);
        assert_eq!(diff.components[1].imag, -4.0);
        
        // Scalar multiplication
        let scaled = v1.clone() * 2.0;
        assert_eq!(scaled.components[0].real, 2.0);
        assert_eq!(scaled.components[0].imag, 4.0);
        assert_eq!(scaled.components[1].real, 6.0);
        assert_eq!(scaled.components[1].imag, 8.0);
        
        // Negation
        let neg = -v1.clone();
        assert_eq!(neg.components[0].real, -1.0);
        assert_eq!(neg.components[0].imag, -2.0);
        assert_eq!(neg.components[1].real, -3.0);
        assert_eq!(neg.components[1].imag, -4.0);
    }
    
    /// Tests inner product and norm calculations.
    #[test]
    fn test_inner_product_and_norm() {
        let v1 = ComplexVector::new(vec![Complex::new(1.0, 2.0), Complex::new(3.0, 4.0)]);
        let v2 = ComplexVector::new(vec![Complex::new(5.0, 6.0), Complex::new(7.0, 8.0)]);
        
        // Inner product
        let inner_prod = v1.inner_product(&v2);
        assert_eq!(inner_prod.real, 70.0);
        assert_eq!(inner_prod.imag, 8.0);
        
        // Norm
        let v = ComplexVector::new(vec![Complex::new(3.0, 4.0), Complex::new(0.0, 5.0)]);
        assert!((v.norm() - 7.071067811865476).abs() < 1e-10);
        
        // Norm squared
        assert!((v.norm_squared() - 50.0).abs() < 1e-10);
    }
    
    /// Tests vector normalization.
    #[test]
    fn test_normalization() {
        let v = ComplexVector::new(vec![Complex::new(3.0, 4.0), Complex::new(0.0, 5.0)]);
        let normalized = v.normalize();
        
        // Check that the normalized vector has a norm of 1
        assert!((normalized.norm() - 1.0).abs() < 1e-10);
        
        // Check that the direction is preserved (scaled by 1/norm)
        let norm = v.norm();
        assert!((normalized.components[0].real - v.components[0].real / norm).abs() < 1e-10);
        assert!((normalized.components[0].imag - v.components[0].imag / norm).abs() < 1e-10);
        assert!((normalized.components[1].real - v.components[1].real / norm).abs() < 1e-10);
        assert!((normalized.components[1].imag - v.components[1].imag / norm).abs() < 1e-10);
    }
    
    /// Tests utility methods.
    #[test]
    fn test_utility_methods() {
        // Test zeros
        let v = ComplexVector::zeros(3);
        assert_eq!(v.dimension(), 3);
        assert!(v.is_zero());
        
        // Test dimension
        let v = ComplexVector::new(vec![Complex::new(1.0, 2.0), Complex::new(3.0, 4.0)]);
        assert_eq!(v.dimension(), 2);
        
        // Test is_zero
        assert!(!v.is_zero());
        
        let v_zero = ComplexVector::new(vec![Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)]);
        assert!(v_zero.is_zero());
    }
    
    /// Tests error handling.
    #[test]
    #[should_panic(expected = "Vectors must have the same dimension for addition")]
    fn test_dimension_mismatch_addition() {
        let v1 = ComplexVector::new(vec![Complex::new(1.0, 2.0), Complex::new(3.0, 4.0)]);
        let v2 = ComplexVector::new(vec![Complex::new(5.0, 6.0)]);
        let _sum = v1 + v2; // Should panic
    }
    
    #[test]
    #[should_panic(expected = "Cannot normalize a zero vector")]
    fn test_normalize_zero_vector() {
        let v = ComplexVector::zeros(2);
        let _normalized = v.normalize(); // Should panic
    }
} 