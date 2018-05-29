fn main() {
    let mut sum: i64 = 0;
    let mut square_sum: i64 = 0;
for x in 1..101 {
    sum += x;
    square_sum += x.pow(2);
    println!("x is: {} and the sum is: {} and the sum of the squares is: {}", x, sum, square_sum);
}
println!("{}", (square_sum - sum.pow(2) ) );
}
