// a big_num is a vector of u8 
pub fn num_to_big_num(x: u128) -> Vec<u8> {
    let input_num: u128 = x as u128;
    let mut output_vec: Vec<u8> = Vec::with_capacity(40);

    // map each digit of the input number to the vector 
    for y in 1..39 {
        output_vec.push((input_num % 10_u128.pow(y) / 10_u128.pow(y-1)) as u8);
    }
    output_vec
}

pub fn add(in_vec1: Vec<u8>, in_vec2: Vec<u8>) {
    // pre-define the output vector's size based on size of both inputs
    let 

fn main() {
    let number_vec: Vec<u8> = num_to_big_num(12345678963145687465689816);
    for x in 0..number_vec.len() {
        print!("{}", number_vec[x]);
    }
    println!("");
}
