fn main() {
    let mut current_triangle: u32 = 1;
    let mut triangle_count: u32 = 1;
    let mut print_count: u32 = 0;

    loop {
        if count_divisors(current_triangle) == 500 {
           break; 
        }
        if print_count % 10 == 0 {
            println!("the current triangle number is: {} and has {} divisors", current_triangle, count_divisors(current_triangle)); 
        }
        print_count += 1;
        triangle_count += 1;
        current_triangle += triangle_count;
    }
    println!("The first triangle number with 500 divisors is: {}", current_triangle);

}

fn count_divisors(number: u32) -> u32 {
    let upper_bound: u32 = (number as f64).sqrt() as u32;
    let mut divisor_count: u32 = 0;
    for x in 2..upper_bound {
        if number % x == 0 {
            divisor_count += 2;
        }
    }
    divisor_count
}


