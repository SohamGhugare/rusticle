//! Matrix and their operations
//!
//! This module provides a generic matrix implementation that supports both real numbers (f64)
//! and complex numbers (Complex).

use std::ops::{Add, Sub, Mul, Neg};
use std::fmt;
use crate::complex::{Complex, ComplexVector};

/// A matrix that can contain either real numbers (f64) or complex numbers (Complex)
#[derive(Clone, PartialEq)]
pub struct Matrix<T> {
    /// Number of rows in the matrix
    rows: usize,
    /// Number of columns in the matrix
    cols: usize,
    /// The elements of the matrix stored in row-major order
    data: Vec<T>,
}

impl<T> Matrix<T> {
    /// Creates a new matrix with the given dimensions and data
    /// 
    /// # Arguments
    /// 
    /// * `rows` - Number of rows in the matrix
    /// * `cols` - Number of columns in the matrix
    /// * `data` - Vector of elements in row-major order
    /// 
    /// # Panics
    /// 
    /// Panics if the length of data does not match rows * cols
    /// 
    /// # Example
    /// ```rust
    /// use rusticle::linalg::Matrix;
    /// 
    /// let matrix = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    /// assert_eq!(matrix.get(0, 1), &2.0);
    /// ```
    pub fn new(rows: usize, cols: usize, data: Vec<T>) -> Self {
        assert_eq!(data.len(), rows * cols, "Data length must match matrix dimensions");
        Matrix { rows, cols, data }
    }

    /// Creates a new matrix filled with zeros
    /// 
    /// # Example
    /// ```rust
    /// use rusticle::linalg::Matrix;
    /// 
    /// let zeros: Matrix<f64> = Matrix::zeros(3, 3);
    /// assert_eq!(zeros.get(1, 1), &0.0);
    /// ```
    pub fn zeros(rows: usize, cols: usize) -> Self 
    where
        T: Default + Clone,
    {
        Matrix {
            rows,
            cols,
            data: vec![T::default(); rows * cols],
        }
    }

    /// Gets the element at the specified position
    /// 
    /// # Arguments
    /// 
    /// * `row` - Row index (0-based)
    /// * `col` - Column index (0-based)
    /// 
    /// # Returns
    /// 
    /// Reference to the element at the specified position
    /// 
    /// # Example
    /// ```rust
    /// use rusticle::linalg::Matrix;
    /// 
    /// let matrix = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    /// assert_eq!(matrix.get(0, 1), &2.0);
    /// ```
    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.data[row * self.cols + col]
    }

    /// Sets the element at the specified position
    /// 
    /// # Arguments
    /// 
    /// * `row` - Row index (0-based)
    /// * `col` - Column index (0-based)
    /// * `value` - New value to set
    /// 
    /// # Example
    /// ```rust
    /// use rusticle::linalg::Matrix;
    /// 
    /// let mut matrix = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
    /// matrix.set(0, 1, 5.0);
    /// assert_eq!(matrix.get(0, 1), &5.0);
    /// ```
    pub fn set(&mut self, row: usize, col: usize, value: T) {
        self.data[row * self.cols + col] = value;
    }

    /// Returns the number of rows in the matrix
    /// 
    /// # Example
    /// ```rust
    /// use rusticle::linalg::Matrix;
    /// 
    /// let matrix = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    /// assert_eq!(matrix.rows(), 2);
    /// ```
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the number of columns in the matrix
    /// 
    /// # Example
    /// ```rust
    /// use rusticle::linalg::Matrix;
    /// 
    /// let matrix = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    /// assert_eq!(matrix.cols(), 3);
    /// ```
    pub fn cols(&self) -> usize {
        self.cols
    }
}

impl<T: fmt::Debug> fmt::Debug for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Matrix({}x{})", self.rows, self.cols)?;
        for row in 0..self.rows {
            for col in 0..self.cols {
                write!(f, "{:?} ", self.get(row, col))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// Matrix addition
impl<T: Add<Output = T> + Clone + Default> Add for Matrix<T> {
    type Output = Matrix<T>;

    fn add(self, other: Matrix<T>) -> Matrix<T> {
        assert_eq!(self.rows, other.rows, "Matrices must have same number of rows");
        assert_eq!(self.cols, other.cols, "Matrices must have same number of columns");
        
        let mut result = Matrix::zeros(self.rows, self.cols);
        for i in 0..self.data.len() {
            result.data[i] = self.data[i].clone() + other.data[i].clone();
        }
        result
    }
}

// Matrix subtraction
impl<T: Sub<Output = T> + Clone + Default> Sub for Matrix<T> {
    type Output = Matrix<T>;

    fn sub(self, other: Matrix<T>) -> Matrix<T> {
        assert_eq!(self.rows, other.rows, "Matrices must have same number of rows");
        assert_eq!(self.cols, other.cols, "Matrices must have same number of columns");
        
        let mut result = Matrix::zeros(self.rows, self.cols);
        for i in 0..self.data.len() {
            result.data[i] = self.data[i].clone() - other.data[i].clone();
        }
        result
    }
}

// Matrix negation
impl<T: Neg<Output = T> + Clone + Default> Neg for Matrix<T> {
    type Output = Matrix<T>;

    fn neg(self) -> Matrix<T> {
        let mut result = Matrix::zeros(self.rows, self.cols);
        for i in 0..self.data.len() {
            result.data[i] = -self.data[i].clone();
        }
        result
    }
}


// Matrix multiplication
impl Mul<&Matrix<Complex>> for &Matrix<Complex> {
    type Output = Matrix<Complex>;

    fn mul(self, other: &Matrix<Complex>) -> Matrix<Complex> {
        assert_eq!(self.cols, other.rows, "Number of columns in first matrix must match number of rows in second matrix");
        
        let mut result = Matrix::zeros(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = Complex::new(0.0, 0.0);
                for k in 0..self.cols {
                    sum = sum + *self.get(i, k) * *other.get(k, j);
                }
                result.set(i, j, sum);
            }
        }
        result
    }
}

// Special implementations for Complex numbers
impl Matrix<Complex> {
    /// Creates an identity matrix of the given size
    /// 
    /// # Example
    /// ```rust
    /// use rusticle::linalg::Matrix;
    /// use rusticle::complex::Complex;
    /// 
    /// let identity = Matrix::identity(2);
    /// assert_eq!(identity.get(0, 0), &Complex::new(1.0, 0.0));
    /// assert_eq!(identity.get(1, 1), &Complex::new(1.0, 0.0));
    /// ```
    pub fn identity(size: usize) -> Self {
        let mut result = Matrix::zeros(size, size);
        for i in 0..size {
            result.set(i, i, Complex::new(1.0, 0.0));
        }
        result
    }

    /// Multiplies this matrix by a vector
    /// 
    /// # Panics
    /// 
    /// Panics if the number of columns in the matrix does not match the vector dimension
    /// 
    /// # Example
    /// ```rust
    /// use rusticle::linalg::Matrix;
    /// use rusticle::complex::{Complex, ComplexVector};
    /// 
    /// let matrix = Matrix::new(2, 2, vec![
    ///     Complex::new(1.0, 0.0), Complex::new(0.0, 1.0),
    ///     Complex::new(0.0, 1.0), Complex::new(1.0, 0.0)
    /// ]);
    /// 
    /// let vector = ComplexVector::new(vec![Complex::new(1.0, 0.0), Complex::new(0.0, 1.0)]);
    /// let result = matrix.mul_vector(&vector);
    /// 
    /// assert_eq!(result.dimension(), 2);
    /// assert_eq!(result.components[0], Complex::new(0.0, 0.0));
    /// assert_eq!(result.components[1], Complex::new(0.0, 2.0));
    /// ```
    pub fn mul_vector(&self, vector: &ComplexVector) -> ComplexVector {
        assert_eq!(self.cols(), vector.dimension(), "Matrix columns must match vector dimension");
        
        let mut result = ComplexVector::zeros(self.rows());
        for i in 0..self.rows() {
            let mut sum = Complex::new(0.0, 0.0);
            for j in 0..self.cols() {
                sum = sum + *self.get(i, j) * vector.components[j];
            }
            result.components[i] = sum;
        }
        result
    }

    /// Computes the conjugate transpose of the matrix
    /// 
    /// # Example
    /// ```rust
    /// use rusticle::linalg::Matrix;
    /// use rusticle::complex::Complex;
    /// 
    /// let matrix = Matrix::new(2, 2, vec![
    ///     Complex::new(1.0, 2.0),
    ///     Complex::new(3.0, 4.0),
    ///     Complex::new(5.0, 6.0),
    ///     Complex::new(7.0, 8.0)
    /// ]);
    /// 
    /// let conjugate_transpose = matrix.conjugate_transpose();
    /// assert_eq!(conjugate_transpose.get(0, 0), &Complex::new(1.0, -2.0));
    /// ```
    pub fn conjugate_transpose(&self) -> Self {
        let mut result = Matrix::zeros(self.cols, self.rows);
        for row in 0..self.rows {
            for col in 0..self.cols {
                result.set(col, row, self.get(row, col).conjugate());
            }
        }
        result
    }

    /// Checks if the matrix is unitary
    /// 
    /// A matrix is unitary if its conjugate transpose is its inverse
    /// 
    /// # Example
    /// ```rust
    /// use rusticle::linalg::Matrix;
    /// use rusticle::complex::Complex;
    /// 
    /// let unitary = Matrix::new(2, 2, vec![
    ///     Complex::new(1.0 / 2.0f64.sqrt(), 0.0),
    ///     Complex::new(1.0 / 2.0f64.sqrt(), 0.0),
    ///     Complex::new(1.0 / 2.0f64.sqrt(), 0.0),
    ///     Complex::new(-1.0 / 2.0f64.sqrt(), 0.0)
    /// ]);
    /// 
    /// assert!(unitary.is_unitary());
    /// ```
    pub fn is_unitary(&self) -> bool {
        if self.rows != self.cols {
            return false;
        }

        let size = self.rows;
        let identity = Matrix::identity(size);
        let product = self * &self.conjugate_transpose();
        
        // Check if product is approximately equal to identity matrix
        for i in 0..size {
            for j in 0..size {
                let diff = *product.get(i, j) - *identity.get(i, j);
                if diff.magnitude() > 1e-10 {
                    return false;
                }
            }
        }
        true
    }
}

