//! Core module for the Rusticle library.
//! 
//! This module contains all foundational types and traits.

pub mod complex;
pub mod vector;
pub mod matrix;

// Global exports
pub use complex::Complex;
pub use vector::Vector;
pub use matrix::Matrix;