use std::cmp::Ordering;

pub fn main() {
    const SIZES: [u64; 5] = [8, 16, 32, 64, 128];
    for n in SIZES {
        println!("#[macro_export]");
        println!("macro_rules! define_u_u{} {{", n);
        println!("    ($macro_name: ident, $width: expr) => {{");
        println!("        #[macro_export]");
        println!("        macro_rules! $macro_name {{");
        println!("            ($name: ident) => {{");
        println!("                raw_define_u!($name, $width, u{});", n);
        for t in SIZES {
            let s = match t.cmp(&n) {
                Ordering::Less => format!(
                    "define_from_u_u_builtin_smaller!($name, $width, u{}, u{});",
                    n, t
                ),
                Ordering::Equal => format!("define_from_u_u_builtin_same!($name, $width, u{});", n),
                Ordering::Greater => format!(
                    "define_from_u_u_builtin_larger!($name, $width, u{}, u{});",
                    n, t
                ),
            };
            println!("                {}", s);
        }
        println!("            }};");
        println!("        }}");
        println!("    }};");
        println!("}}");

        let min = if n == 8 { 2 } else { (n >> 1) + 1 };
        for w in min..n {
            println!("define_u_u{0}!(define_u{1}, {1});", n, w);
        }
        println!();
    }
}
