use rusticle::complex::Complex;
use rusticle::linalg::matrix::Matrix;

/// Test suite for the Matrix type.
/// 
/// These tests verify the core functionality of the Matrix type, including:
/// - Basic matrix operations (creation, access)
/// - Arithmetic operations (addition, multiplication)
/// - Special matrix operations (identity, conjugate transpose)
/// - Complex number specific operations (unitary check)
mod matrix_tests {
    use super::*;

    /// Tests basic matrix creation and element access.
    #[test]
    fn test_matrix_creation() {
        let data = vec![
            Complex::new(1.0, 0.0), Complex::new(2.0, 0.0),
            Complex::new(3.0, 0.0), Complex::new(4.0, 0.0)
        ];
        let matrix = Matrix::new(2, 2, data.clone());
        assert_eq!(matrix.rows(), 2);
        assert_eq!(matrix.cols(), 2);
        assert_eq!(*matrix.get(0, 0), Complex::new(1.0, 0.0));
        assert_eq!(*matrix.get(1, 1), Complex::new(4.0, 0.0));
    }

    /// Tests basic arithmetic operations on matrices.
    #[test]
    fn test_matrix_arithmetic() {
        let a = Matrix::new(2, 2, vec![
            Complex::new(1.0, 0.0), Complex::new(2.0, 0.0),
            Complex::new(3.0, 0.0), Complex::new(4.0, 0.0)
        ]);
        let b = Matrix::new(2, 2, vec![
            Complex::new(5.0, 0.0), Complex::new(6.0, 0.0),
            Complex::new(7.0, 0.0), Complex::new(8.0, 0.0)
        ]);
        
        // Addition
        let sum = a.clone() + b.clone();
        assert_eq!(*sum.get(0, 0), Complex::new(6.0, 0.0));
        assert_eq!(*sum.get(1, 1), Complex::new(12.0, 0.0));
        
        // Multiplication
        let product = &a * &b;
        assert_eq!(*product.get(0, 0), Complex::new(19.0, 0.0));
        assert_eq!(*product.get(1, 1), Complex::new(50.0, 0.0));
    }

    /// Tests special matrix operations.
    #[test]
    fn test_special_operations() {
        // Test identity matrix
        let identity = Matrix::<Complex>::identity(3);
        assert_eq!(*identity.get(0, 0), Complex::new(1.0, 0.0));
        assert_eq!(*identity.get(1, 1), Complex::new(1.0, 0.0));
        assert_eq!(*identity.get(2, 2), Complex::new(1.0, 0.0));
        assert_eq!(*identity.get(0, 1), Complex::new(0.0, 0.0));
        
        // Test conjugate transpose
        let matrix = Matrix::new(2, 2, vec![
            Complex::new(1.0, 2.0), Complex::new(3.0, 4.0),
            Complex::new(5.0, 6.0), Complex::new(7.0, 8.0)
        ]);
        let ct = matrix.conjugate_transpose();
        assert_eq!(*ct.get(0, 0), Complex::new(1.0, -2.0));
        assert_eq!(*ct.get(1, 0), Complex::new(3.0, -4.0));
        assert_eq!(*ct.get(0, 1), Complex::new(5.0, -6.0));
        assert_eq!(*ct.get(1, 1), Complex::new(7.0, -8.0));
    }

    /// Tests complex number specific operations.
    #[test]
    fn test_complex_operations() {
        // Test unitary matrix check with a rotation matrix
        let theta = std::f64::consts::PI / 4.0;
        let unitary = Matrix::new(2, 2, vec![
            Complex::new(theta.cos(), 0.0), Complex::new(-theta.sin(), 0.0),
            Complex::new(theta.sin(), 0.0), Complex::new(theta.cos(), 0.0)
        ]);
        assert!(unitary.is_unitary());

        // Test non-unitary matrix
        let non_unitary = Matrix::new(2, 2, vec![
            Complex::new(1.0, 0.0), Complex::new(2.0, 0.0),
            Complex::new(3.0, 0.0), Complex::new(4.0, 0.0)
        ]);
        assert!(!non_unitary.is_unitary());
    }
} 