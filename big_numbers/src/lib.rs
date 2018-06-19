// a big_num is a vector of u8 
pub fn num_to_big_num<T>(x: T) -> Vec<u8> {
    let input_num: u128 = x as u128;
    let output_vec: Vec<u8> = Vec::with_capacity(39);

    // assign each digit of the input number to the vector 
    for y in 0..40 {
        output_vec.push((input_num % 10.pow(y)) as u8);
    }
    output_vec
}

    
