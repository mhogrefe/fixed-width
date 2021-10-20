use crate::{U20, U3};
use malachite_base::comparison::traits::{Max, Min};
use malachite_base::num::basic::traits::{One, Two, Zero};

#[test]
fn test_eq() {
    assert_eq!(U3::new(2), U3::new(2));
    assert_eq!(U20::new(10), U20::new(10));

    assert_ne!(U3::new(2), U3::new(3));
    assert_ne!(U20::new(10), U20::new(12));
}

#[test]
fn test_ord() {
    assert!(U3::new(2) < U3::new(5));
    assert!(U20::new(12) > U20::new(10));
}

#[test]
fn test_constants() {
    assert_eq!(U3::ZERO, U3::new(0));
    assert_eq!(U3::ONE, U3::new(1));
    assert_eq!(U3::TWO, U3::new(2));
    assert_eq!(U20::ZERO, U20::new(0));
    assert_eq!(U20::ONE, U20::new(1));
    assert_eq!(U20::TWO, U20::new(2));
}

#[test]
fn test_min_max() {
    assert_eq!(U3::MIN, U3::new(0));
    assert_eq!(U3::MAX, U3::new(7));
    assert_eq!(U20::MIN, U20::new(0));
    assert_eq!(U20::MAX, U20::new(1048575));
}

#[test]
fn test_display() {
    assert_eq!(U3::new(2).to_string(), "2");
    assert_eq!(U20::new(10).to_string(), "10");
    assert_eq!(U3::new(2).to_string(), "2");
    assert_eq!(U20::new(10).to_string(), "10");
}
