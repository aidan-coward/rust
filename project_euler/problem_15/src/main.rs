// to find the number of paths in an nxn grid, represent the path as a 2n-digit binary string
// right is 0, down is 1 or vice versa
//
// the number of 2n-digit binary strings with n 1s or 0s is the number of ways of choosing n slots
// from the total of 2n slots, thus '2n choose n'
// to calculate this - (n + n)!/(n!)^2
// according to
// wiki(https://en.wikipedia.org/wiki/Binomial_coefficient#Combinatorics_and_statistics), there are 
// '(n+k) choose n' or '(n+k) choose k' strings with n ones and k zeros 
//
// http://mathworld.wolfram.com/StaircaseWalk.html , there are (m + n) choose m(or n) different
// staircase walks on a grid with m horizontal and n vertical lines
// (m + n)!/(m!*n!)

extern crate num;

fn main() {
    for x in 0..21 {
        println!("the number of routes in a {}x{} grid is: {}", x, x, staircase_walks(x, x, "squares"));
    }
}

fn factorial(x: u8) -> u128 {
    if x == 0 {
        return 1;
    }
    let mut current_num: u128 = 1;
    for y in 2..(x + 1) {
        current_num = current_num * (y as u128);
    }
    current_num
}

fn staircase_walks(n: u8, m: u8, grid_type: &str) -> u128 {
    let number_walks: u128;
    match grid_type {
        "squares" => {
            number_walks = factorial(n + m + 2) / (factorial(n + 1) * factorial(m + 1));
        }
        "lines" => {
            number_walks = factorial(n + m) / (factorial(n) * factorial(m));
        }
        _ => number_walks = 0,
    }
    number_walks
}
