extern crate prime_utils;

fn main() {
    let mut sum: u64 = 0;
    for x in 2..2000000 {
        if prime_utils::prime_test(x) {
            sum += x;
        }
    }
    println!("{}", sum);
}
