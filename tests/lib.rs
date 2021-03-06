use fixed_width::{define_u20, define_u3, define_u5, fixed_width_imports};

fixed_width_imports!();
define_u3!(U3);
define_u5!(U5);
define_u20!(U20);

define_from_u_u_same!(U3, 3, U5, 5, u8);

pub mod unsigned {
    pub mod basic;
}
