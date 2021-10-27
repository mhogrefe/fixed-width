#[macro_export]
macro_rules! define_u_arithmetic {
    ($name: ident, $width: expr, $t: ident) => {
        impl CheckedNeg for $name {
            type Output = $name;

            #[inline]
            fn checked_neg(self) -> Option<$name> {
                if self.0 == 0 {
                    Some(self)
                } else {
                    None
                }
            }
        }

        impl WrappingNeg for $name {
            type Output = $name;

            #[inline]
            fn wrapping_neg(self) -> $name {
                $name(self.0.wrapping_neg() & $name::MAX.0)
            }
        }

        impl WrappingNegAssign for $name {
            #[inline]
            fn wrapping_neg_assign(&mut self) {
                *self = self.wrapping_neg();
            }
        }

        impl OverflowingNeg for $name {
            type Output = $name;

            #[inline]
            fn overflowing_neg(self) -> ($name, bool) {
                ($name(self.0.wrapping_neg() & $name::MAX.0), self.0 != 0)
            }
        }

        impl OverflowingNegAssign for $name {
            #[inline]
            fn overflowing_neg_assign(&mut self) -> bool {
                let (x, b) = self.overflowing_neg();
                *self = x;
                b
            }
        }

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
                let sum = self.0.wrapping_add(other.0) & $name::MAX.0;
                if sum >= self.0 {
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
                let sum = self.0.wrapping_add(other.0) & $name::MAX.0;
                if sum >= self.0 {
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

        impl OverflowingAdd for $name {
            type Output = $name;

            #[inline]
            fn overflowing_add(self, other: $name) -> ($name, bool) {
                let sum = self.0.wrapping_add(other.0) & $name::MAX.0;
                ($name(sum), sum < self.0)
            }
        }

        impl OverflowingAddAssign for $name {
            #[inline]
            fn overflowing_add_assign(&mut self, other: $name) -> bool {
                let (x, b) = self.overflowing_add(other);
                *self = x;
                b
            }
        }

        impl Sub for $name {
            type Output = $name;

            #[inline]
            fn sub(self, other: $name) -> $name {
                self.checked_sub(other).unwrap()
            }
        }

        impl SubAssign for $name {
            #[inline]
            fn sub_assign(&mut self, other: $name) {
                *self = *self - other;
            }
        }

        impl CheckedSub for $name {
            type Output = $name;

            #[inline]
            fn checked_sub(self, other: $name) -> Option<$name> {
                if self.0 >= other.0 {
                    Some($name(self.0 - other.0))
                } else {
                    None
                }
            }
        }

        impl WrappingSub for $name {
            type Output = $name;

            #[inline]
            fn wrapping_sub(self, other: $name) -> $name {
                $name(self.0.wrapping_sub(other.0) & $name::MAX.0)
            }
        }

        impl WrappingSubAssign for $name {
            #[inline]
            fn wrapping_sub_assign(&mut self, other: $name) {
                *self = self.wrapping_sub(other);
            }
        }

        impl SaturatingSub for $name {
            type Output = $name;

            #[inline]
            fn saturating_sub(self, other: $name) -> $name {
                $name(self.0.saturating_sub(other.0))
            }
        }

        impl SaturatingSubAssign for $name {
            #[inline]
            fn saturating_sub_assign(&mut self, other: $name) {
                *self = self.saturating_sub(other);
            }
        }

        impl OverflowingSub for $name {
            type Output = $name;

            #[inline]
            fn overflowing_sub(self, other: $name) -> ($name, bool) {
                let difference = self.0.wrapping_sub(other.0) & $name::MAX.0;
                ($name(difference), difference > self.0)
            }
        }

        impl OverflowingSubAssign for $name {
            #[inline]
            fn overflowing_sub_assign(&mut self, other: $name) -> bool {
                let (x, b) = self.overflowing_sub(other);
                *self = x;
                b
            }
        }

        impl Parity for $name {
            #[inline]
            fn even(self) -> bool {
                self.0.even()
            }

            #[inline]
            fn odd(self) -> bool {
                self.0.odd()
            }
        }

        define_u_arithmetic_shift_u!($name, $width, $t, u8);
        define_u_arithmetic_shift_u!($name, $width, $t, u16);
        define_u_arithmetic_shift_u!($name, $width, $t, u32);
        define_u_arithmetic_shift_u!($name, $width, $t, u64);
        define_u_arithmetic_shift_u!($name, $width, $t, u128);
        define_u_arithmetic_shift_u!($name, $width, $t, usize);

        define_u_arithmetic_shift_i!($name, $width, $t, i8);
        define_u_arithmetic_shift_i!($name, $width, $t, i16);
        define_u_arithmetic_shift_i!($name, $width, $t, i32);
        define_u_arithmetic_shift_i!($name, $width, $t, i64);
        define_u_arithmetic_shift_i!($name, $width, $t, i128);
        define_u_arithmetic_shift_i!($name, $width, $t, isize);
    };
}

#[macro_export]
macro_rules! define_u_arithmetic_shift_u {
    ($name: ident, $width: expr, $t: ident, $s: ident) => {
        impl Shl<$s> for $name {
            type Output = $name;

            #[inline]
            fn shl(self, bits: $s) -> $name {
                $name((self.0 << min(bits, $width)) & $name::MAX.0)
            }
        }

        impl ShlAssign<$s> for $name {
            #[inline]
            fn shl_assign(&mut self, bits: $s) {
                *self = *self << bits
            }
        }

        impl Shr<$s> for $name {
            type Output = $name;

            #[inline]
            fn shr(self, bits: $s) -> $name {
                $name(self.0 >> min(bits, $width))
            }
        }

        impl ShrAssign<$s> for $name {
            #[inline]
            fn shr_assign(&mut self, bits: $s) {
                *self = *self >> bits;
            }
        }
    };
}

#[macro_export]
macro_rules! define_u_arithmetic_shift_i {
    ($name: ident, $width: expr, $t: ident, $s: ident) => {
        impl Shl<$s> for $name {
            type Output = $name;

            #[inline]
            fn shl(self, bits: $s) -> $name {
                let abs_bits = bits.unsigned_abs();
                if bits >= 0 {
                    self << abs_bits
                } else {
                    self >> abs_bits
                }
            }
        }

        impl ShlAssign<$s> for $name {
            #[inline]
            fn shl_assign(&mut self, bits: $s) {
                *self = *self << bits
            }
        }

        impl Shr<$s> for $name {
            type Output = $name;

            #[inline]
            fn shr(self, bits: $s) -> $name {
                let abs_bits = bits.unsigned_abs();
                if bits >= 0 {
                    self >> abs_bits
                } else {
                    self << abs_bits
                }
            }
        }

        impl ShrAssign<$s> for $name {
            #[inline]
            fn shr_assign(&mut self, bits: $s) {
                *self = *self >> bits;
            }
        }
    };
}
