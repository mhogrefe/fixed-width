#[macro_export]
macro_rules! define_u_logic {
    ($name: ident, $width: expr, $t: ident) => {
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
    };
}
