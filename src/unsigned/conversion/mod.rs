// Convert between builtin type T and Ux, where T is the Ux's underlying type
#[macro_export]
macro_rules! define_from_u_u_builtin_same {
    ($name: ident, $width: expr, $t: ident) => {
        impl From<$name> for $t {
            #[inline]
            fn from(u: $name) -> $t {
                u.0
            }
        }

        impl CheckedFrom<$name> for $t {
            #[inline]
            fn checked_from(u: $name) -> Option<$t> {
                Some(u.0)
            }
        }

        impl WrappingFrom<$name> for $t {
            #[inline]
            fn wrapping_from(u: $name) -> $t {
                u.0
            }
        }

        impl SaturatingFrom<$name> for $t {
            #[inline]
            fn saturating_from(u: $name) -> $t {
                u.0
            }
        }

        impl OverflowingFrom<$name> for $t {
            #[inline]
            fn overflowing_from(u: $name) -> ($t, bool) {
                (u.0, false)
            }
        }

        impl CheckedFrom<$t> for $name {
            #[inline]
            fn checked_from(u: $t) -> Option<$name> {
                if u <= $name::MAX.0 {
                    Some($name(u))
                } else {
                    None
                }
            }
        }

        impl WrappingFrom<$t> for $name {
            #[inline]
            fn wrapping_from(u: $t) -> $name {
                $name(u & $name::MAX.0)
            }
        }

        impl SaturatingFrom<$t> for $name {
            #[inline]
            fn saturating_from(u: $t) -> $name {
                $name(min(u, $name::MAX.0))
            }
        }

        impl OverflowingFrom<$t> for $name {
            #[inline]
            fn overflowing_from(u: $t) -> ($name, bool) {
                ($name(u & $name::MAX.0), u > $name::MAX.0)
            }
        }
    };
}

// Convert between builtin type T and Ux, where T is larger than Ux's underlying type
#[macro_export]
macro_rules! define_from_u_u_builtin_larger {
    ($name: ident, $width: expr, $t_1: ident, $t_2: ident) => {
        impl From<$name> for $t_2 {
            #[inline]
            fn from(u: $name) -> $t_2 {
                $t_2::from(u.0)
            }
        }

        impl CheckedFrom<$name> for $t_2 {
            #[inline]
            fn checked_from(u: $name) -> Option<$t_2> {
                Some($t_2::from(u.0))
            }
        }

        impl WrappingFrom<$name> for $t_2 {
            #[inline]
            fn wrapping_from(u: $name) -> $t_2 {
                $t_2::from(u.0)
            }
        }

        impl SaturatingFrom<$name> for $t_2 {
            #[inline]
            fn saturating_from(u: $name) -> $t_2 {
                $t_2::from(u.0)
            }
        }

        impl OverflowingFrom<$name> for $t_2 {
            #[inline]
            fn overflowing_from(u: $name) -> ($t_2, bool) {
                ($t_2::from(u.0), false)
            }
        }

        impl CheckedFrom<$t_2> for $name {
            #[inline]
            fn checked_from(u: $t_2) -> Option<$name> {
                if u <= $t_2::from($name::MAX.0) {
                    Some($name(u as $t_1))
                } else {
                    None
                }
            }
        }

        impl WrappingFrom<$t_2> for $name {
            #[inline]
            fn wrapping_from(u: $t_2) -> $name {
                $name((u as $t_1) & $name::MAX.0)
            }
        }

        impl SaturatingFrom<$t_2> for $name {
            #[inline]
            fn saturating_from(u: $t_2) -> $name {
                if u <= $t_2::from($name::MAX.0) {
                    $name(u as $t_1)
                } else {
                    $name::MAX
                }
            }
        }

        impl OverflowingFrom<$t_2> for $name {
            #[inline]
            fn overflowing_from(u: $t_2) -> ($name, bool) {
                (
                    $name((u as $t_1) & $name::MAX.0),
                    u > $t_2::from($name::MAX.0),
                )
            }
        }
    };
}

// Convert between builtin type T and Ux, where T is smaller than Ux's underlying type
#[macro_export]
macro_rules! define_from_u_u_builtin_smaller {
    ($name: ident, $width: expr, $t_1: ident, $t_2: ident) => {
        impl CheckedFrom<$name> for $t_2 {
            #[inline]
            fn checked_from(u: $name) -> Option<$t_2> {
                $t_2::checked_from(u.0)
            }
        }

        impl WrappingFrom<$name> for $t_2 {
            #[inline]
            fn wrapping_from(u: $name) -> $t_2 {
                u.0 as $t_2
            }
        }

        impl SaturatingFrom<$name> for $t_2 {
            #[inline]
            fn saturating_from(u: $name) -> $t_2 {
                $t_2::saturating_from(u.0)
            }
        }

        impl OverflowingFrom<$name> for $t_2 {
            #[inline]
            fn overflowing_from(u: $name) -> ($t_2, bool) {
                $t_2::overflowing_from(u.0)
            }
        }

        impl From<$t_2> for $name {
            #[inline]
            fn from(u: $t_2) -> $name {
                $name($t_1::from(u))
            }
        }

        impl CheckedFrom<$t_2> for $name {
            #[inline]
            fn checked_from(u: $t_2) -> Option<$name> {
                Some($name($t_1::from(u)))
            }
        }

        impl WrappingFrom<$t_2> for $name {
            #[inline]
            fn wrapping_from(u: $t_2) -> $name {
                $name($t_1::from(u))
            }
        }

        impl SaturatingFrom<$t_2> for $name {
            #[inline]
            fn saturating_from(u: $t_2) -> $name {
                $name($t_1::from(u))
            }
        }

        impl OverflowingFrom<$t_2> for $name {
            #[inline]
            fn overflowing_from(u: $t_2) -> ($name, bool) {
                ($name($t_1::from(u)), false)
            }
        }
    };
}

// Convert between builtin type T and Ux, where T the signed version of the Ux's underlying type
#[macro_export]
macro_rules! define_from_u_i_builtin_same {
    ($name: ident, $width: expr, $t: ident, $i: ident) => {
        impl CheckedFrom<$name> for $i {
            #[inline]
            fn checked_from(u: $name) -> Option<$i> {
                let i = u.0 as $i;
                if i >= 0 {
                    Some(i)
                } else {
                    None
                }
            }
        }

        impl WrappingFrom<$name> for $i {
            #[inline]
            fn wrapping_from(u: $name) -> $i {
                u.0 as $i
            }
        }

        impl SaturatingFrom<$name> for $i {
            #[inline]
            fn saturating_from(u: $name) -> $i {
                let i = u.0 as $i;
                if i >= 0 {
                    i
                } else {
                    $i::MAX
                }
            }
        }

        impl OverflowingFrom<$name> for $i {
            #[inline]
            fn overflowing_from(u: $name) -> ($i, bool) {
                let i = u.0 as $i;
                (i, i < 0)
            }
        }

        impl CheckedFrom<$i> for $name {
            #[inline]
            fn checked_from(i: $i) -> Option<$name> {
                if i >= 0 {
                    $name::checked_from(i.unsigned_abs())
                } else {
                    None
                }
            }
        }

        impl WrappingFrom<$i> for $name {
            #[inline]
            fn wrapping_from(i: $i) -> $name {
                $name((i as $t) & $name::MAX.0)
            }
        }

        impl SaturatingFrom<$i> for $name {
            #[inline]
            fn saturating_from(i: $i) -> $name {
                if i >= 0 {
                    $name::saturating_from(i.unsigned_abs())
                } else {
                    $name(0)
                }
            }
        }

        impl OverflowingFrom<$i> for $name {
            #[inline]
            fn overflowing_from(i: $i) -> ($name, bool) {
                (
                    $name((i as $t) & $name::MAX.0),
                    i < 0 || i.unsigned_abs() > $name::MAX.0,
                )
            }
        }
    };
}

// Convert between builtin type T and Ux, where T is larger than the signed version of the Ux's
// underlying type
#[macro_export]
macro_rules! define_from_u_i_builtin_larger {
    ($name: ident, $width: expr, $t: ident, $i: ident) => {
        impl From<$name> for $i {
            #[inline]
            fn from(u: $name) -> $i {
                $i::from(u.0)
            }
        }

        impl CheckedFrom<$name> for $i {
            #[inline]
            fn checked_from(u: $name) -> Option<$i> {
                Some($i::from(u.0))
            }
        }

        impl WrappingFrom<$name> for $i {
            #[inline]
            fn wrapping_from(u: $name) -> $i {
                $i::from(u.0)
            }
        }

        impl SaturatingFrom<$name> for $i {
            #[inline]
            fn saturating_from(u: $name) -> $i {
                $i::from(u.0)
            }
        }

        impl OverflowingFrom<$name> for $i {
            #[inline]
            fn overflowing_from(u: $name) -> ($i, bool) {
                ($i::from(u.0), false)
            }
        }

        impl CheckedFrom<$i> for $name {
            #[inline]
            fn checked_from(i: $i) -> Option<$name> {
                if i >= 0 {
                    $name::checked_from(i.unsigned_abs())
                } else {
                    None
                }
            }
        }

        impl WrappingFrom<$i> for $name {
            #[inline]
            fn wrapping_from(i: $i) -> $name {
                $name((i as $t) & $name::MAX.0)
            }
        }

        impl SaturatingFrom<$i> for $name {
            #[inline]
            fn saturating_from(i: $i) -> $name {
                if i >= 0 {
                    $name::saturating_from(i.unsigned_abs())
                } else {
                    $name(0)
                }
            }
        }

        impl OverflowingFrom<$i> for $name {
            #[inline]
            fn overflowing_from(i: $i) -> ($name, bool) {
                (
                    $name((i as $t) & $name::MAX.0),
                    i < 0 || i > $i::from($name::MAX.0),
                )
            }
        }
    };
}

// Convert between builtin type T and Ux, where T is smaller than the signed version of the Ux's
// underlying type
#[macro_export]
macro_rules! define_from_u_i_builtin_smaller {
    ($name: ident, $width: expr, $t: ident, $i: ident) => {
        impl CheckedFrom<$name> for $i {
            #[inline]
            fn checked_from(u: $name) -> Option<$i> {
                if u.0 <= $i::MAX as $t {
                    Some(u.0 as $i)
                } else {
                    None
                }
            }
        }

        impl WrappingFrom<$name> for $i {
            #[inline]
            fn wrapping_from(u: $name) -> $i {
                u.0 as $i
            }
        }

        impl SaturatingFrom<$name> for $i {
            #[inline]
            fn saturating_from(u: $name) -> $i {
                if u.0 <= $i::MAX as $t {
                    u.0 as $i
                } else {
                    $i::MAX
                }
            }
        }

        impl OverflowingFrom<$name> for $i {
            #[inline]
            fn overflowing_from(u: $name) -> ($i, bool) {
                (u.0 as $i, u.0 <= $i::MAX as $t)
            }
        }

        impl CheckedFrom<$i> for $name {
            #[inline]
            fn checked_from(i: $i) -> Option<$name> {
                if i >= 0 {
                    Some($name(i as $t))
                } else {
                    None
                }
            }
        }

        impl WrappingFrom<$i> for $name {
            #[inline]
            fn wrapping_from(i: $i) -> $name {
                $name(i as $t)
            }
        }

        impl SaturatingFrom<$i> for $name {
            #[inline]
            fn saturating_from(i: $i) -> $name {
                if i >= 0 {
                    $name(i as $t)
                } else {
                    $name(0)
                }
            }
        }

        impl OverflowingFrom<$i> for $name {
            #[inline]
            fn overflowing_from(i: $i) -> ($name, bool) {
                ($name(i as $t), i < 0)
            }
        }
    };
}

// width_1 must be less than width_2
#[macro_export]
macro_rules! define_from_u_u_same {
    ($name_1: ident, $width_1: expr, $name_2: ident, $width_2: expr, $t: ident) => {
        impl From<$name_1> for $name_2 {
            #[inline]
            fn from(u: $name_1) -> $name_2 {
                $name_2(u.0)
            }
        }

        impl CheckedFrom<$name_1> for $name_2 {
            #[inline]
            fn checked_from(u: $name_1) -> Option<$name_2> {
                Some($name_2(u.0))
            }
        }

        impl WrappingFrom<$name_1> for $name_2 {
            #[inline]
            fn wrapping_from(u: $name_1) -> $name_2 {
                $name_2(u.0)
            }
        }

        impl SaturatingFrom<$name_1> for $name_2 {
            #[inline]
            fn saturating_from(u: $name_1) -> $name_2 {
                $name_2(u.0)
            }
        }

        impl OverflowingFrom<$name_1> for $name_2 {
            #[inline]
            fn overflowing_from(u: $name_1) -> ($name_2, bool) {
                ($name_2(u.0), false)
            }
        }

        impl CheckedFrom<$name_2> for $name_1 {
            #[inline]
            fn checked_from(u: $name_2) -> Option<$name_1> {
                if u.0 <= $name_1::MAX.0 {
                    Some($name_1(u.0))
                } else {
                    None
                }
            }
        }

        impl WrappingFrom<$name_2> for $name_1 {
            #[inline]
            fn wrapping_from(u: $name_2) -> $name_1 {
                $name_1(u.0 & $name_1::MAX.0)
            }
        }

        impl SaturatingFrom<$name_2> for $name_1 {
            #[inline]
            fn saturating_from(u: $name_2) -> $name_1 {
                $name_1(min(u.0, $name_1::MAX.0))
            }
        }

        impl OverflowingFrom<$name_2> for $name_1 {
            #[inline]
            fn overflowing_from(u: $name_2) -> ($name_1, bool) {
                ($name_1(u.0 & $name_1::MAX.0), u.0 > $name_1::MAX.0)
            }
        }
    };
}
