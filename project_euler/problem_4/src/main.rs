extern crate prime_utils;

fn main() {
    let mut palindrome_list: Vec<u128> = Vec::new();
    
    for x in 900..1000 {
        for y in 900..1000 {
            let mut number_vector: Vec<u128>; 
            let mut reversed_number_vector: Vec<u128>; 
            number_vector = number_to_vector(x*y);
            reversed_number_vector = reverse_vector(&number_vector);
            if palindrome_test(&number_vector, &reversed_number_vector) {
               palindrome_list.insert(0, x*y);
            }
        }
    }
    println!("{}", palindrome_list[0]);
}

            

/*    let list = number_to_vector(8640468);

    for x in &list {                    // if list is used, it can't be accessed by anything else so here a reference(&list) is used so that it gives it back once done
        println!("{}", x);
    }

    let reversed_list = reverse_vector(&list);

    for x in &reversed_list {
        println!("{}", x);
    }
    println!("{}", palindrome_test(&list, &reversed_list));
}
*/




fn number_to_vector(mut number_input: u128) -> Vec<u128> {
   let mut num_vector: Vec<u128> = Vec::new();           // creates new vector
   while number_input >= 1 {                            // if number is at least 1, take the last digit and put it into the first spot of the vector
       num_vector.insert(0, number_input % 10 );
       number_input =  number_input / 10;
   }

   num_vector
}

fn reverse_vector(input_vector: &Vec<u128>) -> Vec<u128> {
    let mut reversed_vector: Vec<u128> = Vec::new();     // take the element available and put it at the front of the list
    for element in input_vector {
        reversed_vector.insert(0, *element);
    }
    reversed_vector
}

fn palindrome_test(input_vector: &Vec<u128>, reversed_input_vector: &Vec<u128>) -> bool {
    for x in 0..(input_vector.len() / 2 + 1) {
            if input_vector[x] == reversed_input_vector[x] { continue; }
            else {return false; }
    }
    true
}    



