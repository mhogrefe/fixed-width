#[macro_export]
macro_rules! define_u_arithmetic {
    ($name: ident, $width: expr, $t: ident) => {
        impl Add for $name {
            type Output = $name;

            #[inline]
            fn add(self, other: $name) -> $name {
                self.checked_add(other).unwrap()
            }
        }

        impl AddAssign for $name {
            #[inline]
            fn add_assign(&mut self, other: $name) {
                *self = *self + other;
            }
        }

        impl CheckedAdd for $name {
            type Output = $name;

            #[inline]
            fn checked_add(self, other: $name) -> Option<$name> {
                let sum = self.0.checked_add(other.0)?;
                if sum <= $name::MAX.0 {
                    Some($name(sum))
                } else {
                    None
                }
            }
        }

        impl WrappingAdd for $name {
            type Output = $name;

            #[inline]
            fn wrapping_add(self, other: $name) -> $name {
                $name(self.0.wrapping_add(other.0) & $name::MAX.0)
            }
        }

        impl WrappingAddAssign for $name {
            #[inline]
            fn wrapping_add_assign(&mut self, other: $name) {
                *self = self.wrapping_add(other);
            }
        }

        impl SaturatingAdd for $name {
            type Output = $name;

            #[inline]
            fn saturating_add(self, other: $name) -> $name {
                let sum = if let Some(sum) = self.0.checked_add(other.0) {
                    sum
                } else {
                    return $name::MAX;
                };
                if sum <= $name::MAX.0 {
                    $name(sum)
                } else {
                    $name::MAX
                }
            }
        }

        impl SaturatingAddAssign for $name {
            #[inline]
            fn saturating_add_assign(&mut self, other: $name) {
                *self = self.saturating_add(other);
            }
        }
    };
}
