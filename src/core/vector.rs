use std::ops::{Add, Index, IndexMut, Mul, Sub};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone)]
pub struct Vector<T> {
    pub data: Vec<T>,
}

impl<T: Copy> Vector<T> {
    /// Creates a new vector from a `Vec<T>`.
    pub fn new(data: Vec<T>) -> Self {
        Self { data }
    }

    /// Returns the length of the vector.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Computes the dot product with another vector.
    ///
    /// # Example
    /// ```
    /// use rusticle::Vector;
    /// let v1 = Vector::new(vec![1.0, 2.0]);
    /// let v2 = Vector::new(vec![3.0, 4.0]);
    /// let result = v1.dot(&v2);
    /// assert_eq!(result, 11.0);
    /// ```
    pub fn dot(&self, other: &Self) -> T
    where
        T: Add<Output = T> + Mul<Output = T> + Default,
    {
        self.data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| *a * *b)
            .fold(T::default(), |acc, x| acc + x)
    }

    /// Computes the norm (magnitude) of the vector.
    /// 
    /// # Example
    /// ```
    /// use rusticle::Vector;
    /// let v = Vector::new(vec![3.0, 4.0]);
    /// let result = v.norm();
    /// assert_eq!(result, 5.0);
    /// ```
    pub fn norm(&self) -> f64
    where
        T: Copy + Into<f64> + Mul<Output = T> + Add<Output = T> + Default,
    {
        let sum_sq: f64 = self
            .data
            .iter()
            .map(|x| {
                let val: f64 = (*x).into();
                val * val
            })
            .sum();
        sum_sq.sqrt()
    }

    /// Returns a normalized version of the vector.
    /// 
    /// # Example
    /// ```
    /// use rusticle::Vector;
    /// let v = Vector::new(vec![3.0, 4.0]);
    /// let result = v.normalize();
    /// assert_eq!(result, Vector::new(vec![0.6, 0.8]));
    /// ```
    pub fn normalize(&self) -> Self
    where
        T: Copy + Into<f64> + From<f64> + Mul<Output = T> + Add<Output = T> + Default,
    {
        let norm = self.norm();
        let inv_norm = 1.0 / norm;
        Self {
            data: self
                .data
                .iter()
                .map(|x| {
                    let val: f64 = (*x).into();
                    T::from(val * inv_norm)
                })
                .collect(),
        }
    }

    /// Returns a new vector scaled by a scalar.
    /// 
    /// # Example
    /// ```
    /// use rusticle::Vector;
    /// let v = Vector::new(vec![1.0, 2.0]);
    /// let result = v.scale(2.0);
    /// assert_eq!(result, Vector::new(vec![2.0, 4.0]));
    /// ```
    pub fn scale(&self, scalar: T) -> Self
    where
        T: Mul<Output = T>,
    {
        Self {
            data: self.data.iter().map(|x| *x * scalar).collect(),
        }
    }

    /// Computes the tensor product of two vectors.
    /// 
    /// # Example
    /// ```
    /// use rusticle::Vector;
    /// let v1 = Vector::new(vec![1.0, 2.0]);
    /// let v2 = Vector::new(vec![3.0, 4.0]);
    /// let result = v1.tensor(&v2);
    /// assert_eq!(result, Vector::new(vec![3.0, 4.0, 6.0, 8.0]));
    /// ``` 
    pub fn tensor(&self, other: &Self) -> Self
    where
        T: Copy + Mul<Output = T>,
    {
        let mut result = Vec::with_capacity(self.len() * other.len());
        for &a in &self.data {
            for &b in &other.data {
                result.push(a * b);
            }
        }
        Self { data: result }
    }
}

impl<T> Index<usize> for Vector<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> From<Vec<T>> for Vector<T> {
    fn from(data: Vec<T>) -> Self {
        Self { data }
    }
}

impl<T: Add<Output = T> + Copy> Add for Vector<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let data = self
            .data
            .into_iter()
            .zip(rhs.data.into_iter())
            .map(|(a, b)| a + b)
            .collect();
        Self { data }
    }
}

impl<T: Sub<Output = T> + Copy> Sub for Vector<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let data = self
            .data
            .into_iter()
            .zip(rhs.data.into_iter())
            .map(|(a, b)| a - b)
            .collect();
        Self { data }
    }
}

impl<T: Display> Display for Vector<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let contents: Vec<String> = self.data.iter().map(|x| format!("{}", x)).collect();
        write!(f, "[{}]", contents.join(", "))
    }
}
