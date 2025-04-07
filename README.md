<h1 align="center">Rusticle</h1>
<p align="center">High-performance Rust library for numerical computing, built with power and simplicity.</p>

## Installation

Run this in your terminal
```bash
cargo add rusticle
```

OR add this to your `Cargo.toml`:

```toml
[dependencies]
rusticle = "0.4.11"
```


## Features

### Complex Numbers
Comprehensive support for complex arithmetic, mathematical functions, and polar form conversions

#### Example

```rust
use rusticle::complex::Complex;

// Create complex numbers in easier ways
let z = Complex::from_str("2+3i").unwrap();
let z0 = Complex::new(2, 3);

// Basic arithmetic
let z1 = Complex::new(3.0, 4.0);
let z2 = Complex::from_polar(5.0, std::f64::consts::PI / 6.0);



let sum = z1 + z2;
let product = z1 * z2;

// Advanced operations
let magnitude = z1.magnitude();
let argument = z1.argument();
let conjugate = z1.conjugate();
```

### Complex Vectors
Powerful vector operations for complex numbers, including inner products, norms, and normalization

```rust
use rusticle::complex::{Complex, ComplexVector};

// Create complex vectors
let v1 = ComplexVector::new(vec![Complex::new(1.0, 2.0), Complex::new(3.0, 4.0)]);
let v2 = ComplexVector::new(vec![Complex::new(5.0, 6.0), Complex::new(7.0, 8.0)]);

// Vector operations
let sum = v1.clone() + v2.clone();
let scaled = v1.clone() * 2.0;

// Inner product and norm
let inner_prod = v1.inner_product(&v2);
let norm = v1.norm();

// Normalize a vector
let normalized = v1.normalize();
```

### Matrices
Flexible matrix operations supporting both real and complex numbers, with comprehensive linear algebra functionality

```rust
use rusticle::linalg::matrix::Matrix;
use rusticle::complex::Complex;

// Create real matrices
let a = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
let b = Matrix::new(2, 2, vec![5.0, 6.0, 7.0, 8.0]);

// Matrix operations
let sum = a.clone() + b.clone();
let scaled = a.clone() * 2.0;

// Create complex matrices
let c = Matrix::new(2, 2, vec![
    Complex::new(1.0, 2.0),
    Complex::new(3.0, 4.0),
    Complex::new(5.0, 6.0),
    Complex::new(7.0, 8.0)
]);

// Advanced operations
let identity = Matrix::identity(2);
let is_unitary = c.is_unitary();
let conjugate_transpose = c.conjugate_transpose();

// Accessing elements
let element = c.get(0, 0); // Gets element at row 0, column 0
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.