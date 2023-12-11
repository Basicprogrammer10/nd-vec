use super::*;

#[test]
fn test_bin_ops() {
    let a = vector!(1, 2, 3);
    let b = vector!(4, 5, 6);

    assert_eq!(a + b, vector!(5, 7, 9));
    assert_eq!(a - b, vector!(-3, -3, -3));
    assert_eq!(a / b, vector!(0, 0, 0));
    assert_eq!(a % b, vector!(1, 2, 3));
}

#[test]
fn test_unary_ops() {
    let a = vector!(1, 2, 3);

    assert_eq!(-a, vector!(-1, -2, -3));
}

#[test]
fn test_scalar_ops() {
    let a = vector!(1, 2, 3);

    assert_eq!(a + 2, vector!(3, 4, 5));
    assert_eq!(a - 2, vector!(-1, 0, 1));
    assert_eq!(a / 2, vector!(0, 1, 1));
    assert_eq!(a % 2, vector!(1, 0, 1));
    assert_eq!(a * 2, vector!(2, 4, 6));
}

#[test]
fn test_norms() {
    let a = vector!(1.0, 2.0, 3.0);

    assert_eq!(a.magnitude_squared(), 14.0);
    assert_eq!(a.magnitude(), 14f64.sqrt());
}

#[test]
fn test_product() {
    let a = vector!(1, 2, 3);
    let b = vector!(4, 5, 6);

    assert_eq!(a.dot(&b), 32);
    assert_eq!(a.hadamard_product(&b), vector!(4, 10, 18));
}

#[test]
fn test_distance() {
    let a = vector!(1.0, 2.0, 3.0);
    let b = vector!(4.0, 5.0, 6.0);
    assert_eq!(a.manhattan_distance(&b), 9.0);
    assert_eq!(a.distance(&b), 5.196152422706632);
}

#[test]
fn test_cast() {
    let a = vector!(1.0, 2.0, 3.0);
    let b = vector!(1, 2, 3);

    assert_eq!(a.num_cast().unwrap(), b);
    assert_eq!(b.cast(), a);
}
