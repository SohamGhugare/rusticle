use rusticle::Vector;

#[test]
fn test_vector_new() {
    let v = Vector::new(vec![1.0, 2.0, 3.0]);
    assert_eq!(v.len(), 3);
}

#[test]
fn test_vector_dot_product() {
    let v1 = Vector::new(vec![1.0, 2.0]);
    let v2 = Vector::new(vec![3.0, 4.0]);
    assert_eq!(v1.dot(&v2), 11.0);
}

#[test]
fn test_vector_norm() {
    let v = Vector::new(vec![3.0, 4.0]);
    let norm = v.norm();
    assert!((norm - 5.0).abs() < 1e-10);
}

#[test]
fn test_vector_normalize() {
    let v = Vector::new(vec![3.0, 4.0]);
    let normalized = v.normalize();
    assert!((normalized.norm() - 1.0).abs() < 1e-10);
}

#[test]
fn test_vector_scale() {
    let v = Vector::new(vec![1.0, 2.0, 3.0]);
    let scaled = v.scale(2.0);
    assert_eq!(scaled, Vector::new(vec![2.0, 4.0, 6.0]));
}

#[test]
fn test_vector_tensor_product() {
    let v1 = Vector::new(vec![1.0, 2.0]);
    let v2 = Vector::new(vec![3.0, 4.0]);
    let tensor = v1.tensor(&v2);
    assert_eq!(tensor, Vector::new(vec![3.0, 4.0, 6.0, 8.0]));
}

#[test]
fn test_vector_add_sub() {
    let a = Vector::new(vec![1.0, 2.0, 3.0]);
    let b = Vector::new(vec![4.0, 5.0, 6.0]);
    assert_eq!(a.clone() + b.clone(), Vector::new(vec![5.0, 7.0, 9.0]));
    assert_eq!(b - a, Vector::new(vec![3.0, 3.0, 3.0]));
}

#[test]
fn test_vector_indexing() {
    let mut v = Vector::new(vec![10, 20, 30]);
    assert_eq!(v[1], 20);
    v[1] = 99;
    assert_eq!(v[1], 99);
}