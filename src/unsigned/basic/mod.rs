#[macro_export]
macro_rules! define_u_basic {
    ($name: ident, $width: expr, $underlying: ident) => {
        impl $name {
            #[inline]
            pub fn new(u: $underlying) -> $name {
                assert!(
                    u.significant_bits() <= $width,
                    "{} does not fit in a {}",
                    u,
                    stringify!($name)
                );
                $name(u)
            }
        }

        impl Zero for $name {
            const ZERO: $name = $name(0);
        }

        impl One for $name {
            const ONE: $name = $name(1);
        }

        impl Two for $name {
            const TWO: $name = $name(2);
        }
    };
}
