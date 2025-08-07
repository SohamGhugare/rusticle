use rusticle::{Matrix, Vector};

#[test]
fn test_matrix_new() {
    let m = Matrix::new(vec![1.0, 2.0, 3.0, 4.0], 2, 2);
    assert_eq!(m.rows(), 2);
    assert_eq!(m.cols(), 2);
}

#[test]
fn test_matrix_identity() {
    let m = Matrix::identity(2);
    assert_eq!(m, Matrix::new(vec![1.0, 0.0, 0.0, 1.0], 2, 2));
}

#[test]
fn test_matrix_mul_vector() {
    let m = Matrix::new(vec![1.0, 2.0, 3.0, 4.0], 2, 2);
    let v = Vector::new(vec![1.0, 2.0]);
    let result = m.mul_vector(&v);
    assert_eq!(result, Vector::new(vec![5.0, 11.0]));
}

#[test]
fn test_matrix_mul_matrix() {
    let m1 = Matrix::new(vec![1.0, 2.0, 3.0, 4.0], 2, 2);
    let m2 = Matrix::new(vec![5.0, 6.0, 7.0, 8.0], 2, 2);
    let result = m1*m2;
    assert_eq!(result, Matrix::new(vec![19.0, 22.0, 43.0, 50.0], 2, 2));
}