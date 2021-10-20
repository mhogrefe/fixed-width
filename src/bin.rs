pub fn main() {
    for n in [8, 16, 32, 64, 128] {
        println!("define_u_with_type!(define_u_u{0}, u{0});", n);
        let min = if n == 8 { 2 } else { (n >> 1) + 1 };
        for w in min..n {
            println!("define_u_u{0}!(define_u{1}, {1});", n, w);
        }
        println!();
    }
}
