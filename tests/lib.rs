use fixed_width::{define_u4, define_u3, fixed_width_imports};

fixed_width_imports!();
define_u3!(U3);
define_u4!(U4);

pub mod unsigned {
    pub mod basic;
}
