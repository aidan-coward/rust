fn main() {
    let mut current_triangle: u32 = 1;
    let mut triangle_count: u32 = 1;
    let mut print_count: u32 = 0;

    loop {
        if count_divisors(current_triangle) > 500 {
           break; 
        }
        if print_count % 200 == 0 {
            println!("the current triangle number is: {} and has {} divisors", current_triangle, count_divisors(current_triangle)); 
        }
        print_count += 1;
        triangle_count += 1;
        current_triangle += triangle_count;
    }
    println!("The first triangle number with over 500 divisors is: {}", current_triangle);

}

fn count_divisors(number: u32) -> u32 {
    let upper_bound: u32 = (number as f64).sqrt() as u32;       // set the square root of number as the upper bound for divisors
    let mut divisor_count: u32 = 2;                             // always has at least 1 and itself as divisors
    if number / upper_bound == upper_bound {                     // if number is a perfect square, is one divisor, and not two
        divisor_count = 3;
    }
    for x in 2..upper_bound {                                   // if an x < upper_bound divides number evenly, there is a y > upper_bound such that x * y = number 
        if number % x == 0 {
            divisor_count += 2;                                 // counts both divisors
        }
    }
    divisor_count
}


