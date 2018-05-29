fn main() {
    let mut found_num: bool = false;
    let mut test_num: i32 = 40;
    while !found_num {
        test_num += 20;
        for x in 3..21 {
            if !(test_num % x == 0) {
                break;
            }
            if x == 20 {
                println!("{}", test_num);
                found_num = true;
            }
            }

    }

}

