use crate::{U20, U3};

#[test]
fn test_ord() {
    assert!(U3::new(2) < U3::new(5));
    assert!(U20::new(12) > U20::new(10));
}

#[test]
fn test_eq() {
    assert_eq!(U3::new(2), U3::new(2));
    assert_eq!(U20::new(10), U20::new(10));

    assert_ne!(U3::new(2), U3::new(3));
    assert_ne!(U20::new(10), U20::new(12));
}
