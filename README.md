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
rusticle = "0.1.0"
```


## Features

### Complex Numbers
Comprehensive support for complex arithmetic, mathematical functions, and polar form conversions


### Example

```rust
use rusticle::complex::Complex;

// Create complex numbers
let z1 = Complex::new(3.0, 4.0);
let z2 = Complex::from_polar(5.0, std::f64::consts::PI / 6.0);

// Basic arithmetic
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

## License

This project is licensed under the MIT License - see the LICENSE file for details.