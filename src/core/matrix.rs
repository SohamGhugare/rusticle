use crate::Vector;
use std::ops::{Add, Index, IndexMut, Mul};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// A matrix of elements of type `T`
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T: Copy> Matrix<T> {
    /// Creates a new matrix from a 1D vector and the number of rows and columns.
    /// 
    /// # Example
    /// ```
    /// use rusticle::Matrix;
    /// let data = vec![1.0, 2.0, 3.0, 4.0];
    /// let matrix = Matrix::new(data, 2, 2);
    /// ```
    #[inline(always)]
    pub fn new(data: Vec<T>, rows: usize, cols: usize) -> Self {
        assert_eq!(data.len(), rows * cols, "Data length must match rows * cols");
        Self { data, rows, cols }
    }

    /// Creates a new matrix from a 2D vector.
    /// 
    /// # Example
    /// ```
    /// use rusticle::Matrix;
    /// let vec2d = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    /// let matrix = Matrix::from_2d(vec2d);
    /// ```
    #[inline(always)]
    pub fn from_2d(vec2d: Vec<Vec<T>>) -> Self {
        let rows = vec2d.len();
        let cols = if rows > 0 { vec2d[0].len() } else { 0 };
        assert!(vec2d.iter().all(|row| row.len() == cols), "All rows must have the same length");
        let data = vec2d.into_iter().flatten().collect();
        Self { data, rows, cols }
    }

    /// Creates a new identity matrix of the given size.
    /// 
    /// # Example
    /// ```
    /// use rusticle::Matrix;
    /// let matrix = Matrix::identity(2);
    /// assert_eq!(matrix, Matrix::new(vec![1.0, 0.0, 0.0, 1.0], 2, 2));
    /// ```
    #[inline(always)]
    pub fn identity(size: usize) -> Self
    where
        T: Default + From<u8>,
    {
        let mut data = vec![T::default(); size * size];
        for i in 0..size {
            data[i * size + i] = T::from(1u8);
        }
        Self::new(data, size, size)
    }

    /// Multiplies the matrix by a vector.
    /// 
    /// # Example
    /// ```
    /// use rusticle::{Matrix, Vector};
    /// let matrix = Matrix::new(vec![1.0, 2.0, 3.0, 4.0], 2, 2);
    /// let vector = Vector::new(vec![1.0, 2.0]);
    /// let result = matrix.mul_vector(&vector);
    /// assert_eq!(result, Vector::new(vec![5.0, 11.0]));
    /// ```
    #[inline(always)]
    pub fn mul_vector(&self, vector: &Vector<T>) -> Vector<T>
    where
        T: Add<Output = T> + Mul<Output = T> + Default,
    {
        assert_eq!(self.cols, vector.len(), "Matrix columns must match vector length");
        let mut result = Vec::with_capacity(self.rows);

        for row in 0..self.rows {
            let mut sum = T::default();
            for col in 0..self.cols {
                let val = self[(row, col)] * vector[col];
                sum = sum + val;
            }
            result.push(sum);
        }

        Vector::new(result)
    }

    /// Multiplies the matrix by another matrix.
    /// 
    /// # Example
    /// ```
    /// use rusticle::Matrix;
    /// let matrix1 = Matrix::new(vec![1.0, 2.0, 3.0, 4.0], 2, 2);
    /// let matrix2 = Matrix::new(vec![5.0, 6.0, 7.0, 8.0], 2, 2);
    /// let result = matrix1.mul_matrix(&matrix2);
    /// assert_eq!(result, Matrix::new(vec![19.0, 22.0, 43.0, 50.0], 2, 2));
    /// ```
    #[inline(always)]
    pub fn mul_matrix(&self, other: &Self) -> Self
    where
        T: Copy + Default + Add<Output = T> + Mul<Output = T>,
    {
        assert_eq!(self.cols, other.rows, "Matrix dimension mismatch");

        let mut result = vec![T::default(); self.rows * other.cols];

        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = T::default();
                for k in 0..self.cols {
                    sum = sum + self[(i, k)] * other[(k, j)];
                }
                result[i * other.cols + j] = sum;
            }
        }

        Matrix {
            data: result,
            rows: self.rows,
            cols: other.cols,
        }
    }


    /// Returns the number of rows in the matrix.
    /// 
    /// # Example
    /// ```
    /// use rusticle::Matrix;
    /// let matrix = Matrix::new(vec![1.0, 2.0, 3.0, 4.0], 2, 2);
    /// assert_eq!(matrix.rows(), 2);
    /// ```
    #[inline(always)]
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the number of columns in the matrix.
    /// 
    /// # Example
    /// ```
    /// use rusticle::Matrix;
    /// let matrix = Matrix::new(vec![1.0, 2.0, 3.0, 4.0], 2, 2);
    /// assert_eq!(matrix.cols(), 2);
    /// ```
    #[inline(always)]
    pub fn cols(&self) -> usize {
        self.cols
    }
}

impl<T> Mul<Matrix<T>> for Matrix<T>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T>,
{
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.cols, rhs.rows, "Matrix dimension mismatch");

        let mut result = vec![T::default(); self.rows * rhs.cols];

        for i in 0..self.rows {
            for j in 0..rhs.cols {
                let mut sum = T::default();
                for k in 0..self.cols {
                    sum = sum + self[(i, k)] * rhs[(k, j)];
                }
                result[i * rhs.cols + j] = sum;
            }
        }

        Matrix {
            data: result,
            rows: self.rows,
            cols: rhs.cols,
        }
    }
}

impl<T: Display + Copy> Display for Matrix<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for i in 0..self.rows() {
            write!(f, "[")?;
            for j in 0..self.cols() {
                write!(f, "{}", self[(i, j)])?;
                if j != self.cols() - 1 {
                    write!(f, " ")?;
                }
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;
    #[inline(always)]
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row * self.cols + col]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    #[inline(always)]
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.data[row * self.cols + col]
    }
}