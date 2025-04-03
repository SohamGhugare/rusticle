<h1 align="center">Rusticle</h1>
<p align="center">High-performance Rust library for numerical computing, built with power and simplicity.</p>

## Features

- **Complex Numbers**: Comprehensive support for complex arithmetic, mathematical functions, and polar form conversions

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
rusticle = "0.1.0"
```

### Example

```rust
use rusticle::Complex;

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

## License

This project is licensed under the MIT License - see the LICENSE file for details.