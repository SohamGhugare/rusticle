use rusticle::linalg::Matrix;
use rusticle::complex::{Complex, ComplexVector};

fn main() {
    let mut matrix = Matrix::new(2, 2, vec![
        Complex::new(2.0, 0.0), Complex::new(3.0, 0.0),
        Complex::new(1.0, 0.0), Complex::new(4.0, 0.0)
    ]);
 
    let vector = ComplexVector::new(vec![Complex::new(1.0, 0.0), Complex::new(2.0, 0.0)]);
    matrix.mul_vector(&vector);

    println!("{:?}", matrix);
     
    assert_eq!(matrix.get(0, 0), &Complex::new(8.0, 0.0));
    assert_eq!(matrix.get(1, 0), &Complex::new(9.0, 0.0));
}