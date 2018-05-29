extern crate prime_utils;

fn main() {
    let mut counter = 1;
    let mut test_num: u64 = 1;
    while counter <= 10001 {
        if prime_utils::prime_test(test_num) == true {
            counter +=1;
            }
        if counter == 10001 + 1 {
        println!("the {}th prime is: {}", counter, test_num);
        break;
        }
        test_num += 1;
        }
}
