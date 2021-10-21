#[macro_export]
macro_rules! define_u_basic {
    ($name: ident, $width: expr, $t: ident) => {
        impl $name {
            #[inline]
            pub fn new(u: $t) -> $name {
                assert!(
                    u <= Self::MAX.0,
                    "{} does not fit in a {}",
                    u,
                    stringify!($name)
                );
                $name(u)
            }

            pub const fn x(self) -> $t {
                self.0
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

        impl Min for $name {
            const MIN: $name = $name(0);
        }

        impl Max for $name {
            const MAX: $name = $name((1 << $width) - 1);
        }

        impl Display for $name {
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> Result {
                Display::fmt(&self.0, f)
            }
        }
    };
}
