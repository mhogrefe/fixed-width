pub mod basic;

#[macro_export]
macro_rules! fixed_width_imports {
    () => {
        use malachite_base::num::basic::traits::{One, Two, Zero};
        use malachite_base::num::logic::traits::SignificantBits;
        use fixed_width::{raw_define_u, define_u_basic};
    };
}

#[macro_export]
macro_rules! raw_define_u {
    ($name: ident, $width: expr, $underlying: ident) => {
        #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd)]
        pub struct $name($underlying);

        define_u_basic!($name, $width, $underlying);
    };
}

#[macro_export]
macro_rules! define_u3 {
    ($name: ident) => {
        raw_define_u!($name, 3, u8);
    };
}

#[macro_export]
macro_rules! define_u4 {
    ($name: ident) => {
        raw_define_u!($name, 4, u8);
    };
}
