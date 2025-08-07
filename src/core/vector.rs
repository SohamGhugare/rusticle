use std::ops::{Add, Mul};

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
}
