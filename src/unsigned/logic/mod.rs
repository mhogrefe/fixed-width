#[macro_export]
macro_rules! define_u_logic {
    ($name: ident, $width: expr, $t: ident) => {
        impl BitAnd for $name {
            type Output = $name;

            #[inline]
            fn bitand(self, other: $name) -> $name {
                $name(self.0 & other.0)
            }
        }

        impl BitAndAssign for $name {
            #[inline]
            fn bitand_assign(&mut self, other: $name) {
                *self = *self & other;
            }
        }

        impl BitOr for $name {
            type Output = $name;

            #[inline]
            fn bitor(self, other: $name) -> $name {
                $name(self.0 | other.0)
            }
        }

        impl BitOrAssign for $name {
            #[inline]
            fn bitor_assign(&mut self, other: $name) {
                *self = *self | other;
            }
        }

        impl BitXor for $name {
            type Output = $name;

            #[inline]
            fn bitxor(self, other: $name) -> $name {
                $name(self.0 ^ other.0)
            }
        }

        impl BitXorAssign for $name {
            #[inline]
            fn bitxor_assign(&mut self, other: $name) {
                *self = *self ^ other;
            }
        }

        impl BitAccess for $name {
            #[inline]
            fn get_bit(&self, index: u64) -> bool {
                self.0.get_bit(index)
            }

            #[inline]
            fn set_bit(&mut self, index: u64) {
                assert!(index < $width);
                self.0.set_bit(index)
            }

            #[inline]
            fn clear_bit(&mut self, index: u64) {
                self.0.clear_bit(index)
            }
        }

        impl BitBlockAccess for $name {
            type Bits = $name;

            #[inline]
            fn get_bits(&self, start: u64, end: u64) -> $name {
                $name(self.0.get_bits(start, end))
            }

            #[inline]
            fn assign_bits(&mut self, start: u64, end: u64, bits: &$name) {
                let mut x = *self;
                x.0.assign_bits(start, end, &bits.0);
                assert!(x.0 <= $name::MAX.0);
                *self = x;
            }
        }
    };
}
