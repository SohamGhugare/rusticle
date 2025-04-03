//! Rusticle - A high-performance Rust library for numerical computing
//! 
//! This library provides functionality for complex number computations,
//! including basic arithmetic operations, mathematical functions, and
//! polar form conversions.

pub mod complex;

// Re-export commonly used types
pub use complex::{Angle, Complex};
