use crate::{U3, U4};

#[test]
fn test() {
    assert_eq!(U3::new(2), U3::new(2));
    assert_eq!(U4::new(10), U4::new(10));

    assert_ne!(U3::new(2), U3::new(3));
    assert_ne!(U4::new(10), U4::new(12));
}
