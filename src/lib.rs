//! # Rusticle - A high-performance Rust library for numerical computing
//! 
//! Hello, and welcome to the only Rust library you'll ever need for math.
//! 
//! This library provides a comprehensive set of tools for numerical computing,
//! including support for complex numbers, linear algebra, and more.
//! 
//! ## Usage
//! 
//! Add this to your `Cargo.toml`:
//! 
//! ```toml
//! [dependencies]
//! rusticle = "0.4.10"
//! ```
//! 
//! or use `cargo add rusticle` to add it to your project.
//! 
//! ## Features
//! 
//! | Feature | Description |
//! |---------|-------------|
//! | `complex` | Complex number support with operations like addition, subtraction, multiplication, division, and parsing from strings |
//! | `linalg` | Linear algebra support including vectors and matrices with complex number support |
//! 

pub mod complex;
pub mod linalg;

pub use complex::Angle;