//! # Rusticle - A high-performance Rust library for numerical computing
//! 
//! Hello, and welcome to the only Rust library you'll ever need for math.
//! 
//! This library provides a comprehensive set of tools for numerical computing,
//! including support for complex numbers, linear algebra, and more.

pub mod core;

// Global exports
pub use core::{ Complex, Vector, Matrix };
