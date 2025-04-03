//! Complex number library for Rust
//! 
//! This module provides functionality for working with complex numbers,
//! including basic arithmetic operations, mathematical functions, and
//! polar form conversions.

pub mod angle;
pub mod complex;

// Re-export the Angle and Complex types for convenience
pub use angle::Angle;
pub use complex::Complex;
