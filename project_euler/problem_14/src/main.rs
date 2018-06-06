fn main() {
    let mut most_steps: u64 = 0;
    let mut most_steps_num: u64 = 0;

    for x in 2..1000000 {           // 1 is endpoint, so it is ignored
        if num_steps(x) > most_steps {
            most_steps = num_steps(x);
            most_steps_num = x;
        }
        if x % 100 == 0 {
            println!("at {}, {}: steps: {}", x, most_steps_num, most_steps);
                }
    }
    println!("The number with the most steps is: {}, with {} steps", most_steps_num, most_steps);
}

fn num_steps(mut input: u64) -> u64 {
    let mut step_counter: u64 = 0;
    while !(input == 1) {
        if input % 2 == 0 {
            input = input / 2;
            step_counter += 1;
        }
        else {
            input = 3 * input + 1;
            step_counter += 1;
        }
    }
    step_counter
}

        

