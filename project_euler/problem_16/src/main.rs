extern crate num_bigint;
extern crate num_traits;

use num_bigint::ToBigUint;
use num_traits::ops::checked::CheckedMul;

fn main() {

    let mult_num = 2.to_biguint().unwrap();
    let mut counter: u16 = 1;
    let mut big_num = 2.to_biguint().unwrap();
    let mut sum: u32 = 0; 
    let new_big_num: String;

    while counter < 1000 {
       big_num = big_num.checked_mul(&mult_num).unwrap();
       counter += 1;
    }
    //println!("{}", big_num);

    
    new_big_num = big_num.clone().to_string();

    counter = 0;
    for x in new_big_num.chars() {
        println!("{}", x.to_digit(10).unwrap());
        sum += x.to_digit(10).unwrap();
        counter += 1;
    }
    println!("2^1000 has {} digits and their sum is: {}", counter, sum);
}


/*

fn number_to_vector(mut number_input: u128) -> Vec<u128> {
   let mut num_vector: Vec<u128> = Vec::new();           // creates new vector
   while number_input >= 1 {                            // if number is at least 1, take the last digit and put it into the first spot of the vector
       num_vector.insert(0, number_input % 10 );
       number_input =  number_input / 10;
   }

   num_vector
*/
