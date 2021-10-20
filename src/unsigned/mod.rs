pub mod basic;

#[macro_export]
macro_rules! fixed_width_imports {
    () => {
        use fixed_width::{define_u_basic, raw_define_u};
        use malachite_base::comparison::traits::{Max, Min};
        use malachite_base::num::basic::traits::{One, Two, Zero};
        use std::fmt::{Display, Formatter, Result};
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
macro_rules! define_u_with_type {
    ($macro_macro_name: ident, $t: ident) => {
        #[macro_export]
        macro_rules! $macro_macro_name {
            ($macro_name: ident, $width: expr) => {
                #[macro_export]
                macro_rules! $macro_name {
                    ($name: ident) => {
                        raw_define_u!($name, $width, $t);
                    };
                }
            };
        }
    };
}

// Generated using `cargo run`

define_u_with_type!(define_u_u8, u8);
define_u_u8!(define_u2, 2);
define_u_u8!(define_u3, 3);
define_u_u8!(define_u4, 4);
define_u_u8!(define_u5, 5);
define_u_u8!(define_u6, 6);
define_u_u8!(define_u7, 7);

define_u_with_type!(define_u_u16, u16);
define_u_u16!(define_u9, 9);
define_u_u16!(define_u10, 10);
define_u_u16!(define_u11, 11);
define_u_u16!(define_u12, 12);
define_u_u16!(define_u13, 13);
define_u_u16!(define_u14, 14);
define_u_u16!(define_u15, 15);

define_u_with_type!(define_u_u32, u32);
define_u_u32!(define_u17, 17);
define_u_u32!(define_u18, 18);
define_u_u32!(define_u19, 19);
define_u_u32!(define_u20, 20);
define_u_u32!(define_u21, 21);
define_u_u32!(define_u22, 22);
define_u_u32!(define_u23, 23);
define_u_u32!(define_u24, 24);
define_u_u32!(define_u25, 25);
define_u_u32!(define_u26, 26);
define_u_u32!(define_u27, 27);
define_u_u32!(define_u28, 28);
define_u_u32!(define_u29, 29);
define_u_u32!(define_u30, 30);
define_u_u32!(define_u31, 31);

define_u_with_type!(define_u_u64, u64);
define_u_u64!(define_u33, 33);
define_u_u64!(define_u34, 34);
define_u_u64!(define_u35, 35);
define_u_u64!(define_u36, 36);
define_u_u64!(define_u37, 37);
define_u_u64!(define_u38, 38);
define_u_u64!(define_u39, 39);
define_u_u64!(define_u40, 40);
define_u_u64!(define_u41, 41);
define_u_u64!(define_u42, 42);
define_u_u64!(define_u43, 43);
define_u_u64!(define_u44, 44);
define_u_u64!(define_u45, 45);
define_u_u64!(define_u46, 46);
define_u_u64!(define_u47, 47);
define_u_u64!(define_u48, 48);
define_u_u64!(define_u49, 49);
define_u_u64!(define_u50, 50);
define_u_u64!(define_u51, 51);
define_u_u64!(define_u52, 52);
define_u_u64!(define_u53, 53);
define_u_u64!(define_u54, 54);
define_u_u64!(define_u55, 55);
define_u_u64!(define_u56, 56);
define_u_u64!(define_u57, 57);
define_u_u64!(define_u58, 58);
define_u_u64!(define_u59, 59);
define_u_u64!(define_u60, 60);
define_u_u64!(define_u61, 61);
define_u_u64!(define_u62, 62);
define_u_u64!(define_u63, 63);

define_u_with_type!(define_u_u128, u128);
define_u_u128!(define_u65, 65);
define_u_u128!(define_u66, 66);
define_u_u128!(define_u67, 67);
define_u_u128!(define_u68, 68);
define_u_u128!(define_u69, 69);
define_u_u128!(define_u70, 70);
define_u_u128!(define_u71, 71);
define_u_u128!(define_u72, 72);
define_u_u128!(define_u73, 73);
define_u_u128!(define_u74, 74);
define_u_u128!(define_u75, 75);
define_u_u128!(define_u76, 76);
define_u_u128!(define_u77, 77);
define_u_u128!(define_u78, 78);
define_u_u128!(define_u79, 79);
define_u_u128!(define_u80, 80);
define_u_u128!(define_u81, 81);
define_u_u128!(define_u82, 82);
define_u_u128!(define_u83, 83);
define_u_u128!(define_u84, 84);
define_u_u128!(define_u85, 85);
define_u_u128!(define_u86, 86);
define_u_u128!(define_u87, 87);
define_u_u128!(define_u88, 88);
define_u_u128!(define_u89, 89);
define_u_u128!(define_u90, 90);
define_u_u128!(define_u91, 91);
define_u_u128!(define_u92, 92);
define_u_u128!(define_u93, 93);
define_u_u128!(define_u94, 94);
define_u_u128!(define_u95, 95);
define_u_u128!(define_u96, 96);
define_u_u128!(define_u97, 97);
define_u_u128!(define_u98, 98);
define_u_u128!(define_u99, 99);
define_u_u128!(define_u100, 100);
define_u_u128!(define_u101, 101);
define_u_u128!(define_u102, 102);
define_u_u128!(define_u103, 103);
define_u_u128!(define_u104, 104);
define_u_u128!(define_u105, 105);
define_u_u128!(define_u106, 106);
define_u_u128!(define_u107, 107);
define_u_u128!(define_u108, 108);
define_u_u128!(define_u109, 109);
define_u_u128!(define_u110, 110);
define_u_u128!(define_u111, 111);
define_u_u128!(define_u112, 112);
define_u_u128!(define_u113, 113);
define_u_u128!(define_u114, 114);
define_u_u128!(define_u115, 115);
define_u_u128!(define_u116, 116);
define_u_u128!(define_u117, 117);
define_u_u128!(define_u118, 118);
define_u_u128!(define_u119, 119);
define_u_u128!(define_u120, 120);
define_u_u128!(define_u121, 121);
define_u_u128!(define_u122, 122);
define_u_u128!(define_u123, 123);
define_u_u128!(define_u124, 124);
define_u_u128!(define_u125, 125);
define_u_u128!(define_u126, 126);
define_u_u128!(define_u127, 127);
