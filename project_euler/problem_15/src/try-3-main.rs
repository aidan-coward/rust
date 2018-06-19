fn main() {
    let mut list_routes: Vec<Vec<[u8; 2]>> = vec![vec![[0, 0]]];
    let mut next_routes: Vec<Vec<[u8; 2]>> = Vec::new(); 
    //let mut intermediate: Vec<[u8; 2]> = list_routes[0].clone();
    let mut intermediate: Vec<[u8; 2]>;
    let mut none_counter: u8 = 0;   
    let mut loop_counter: u64 = 0;

    //let mut step1: [u8; 2];
    //let mut step2: [u8; 2];

    let mut output_step1: Option<[u8; 2]>;
    let mut output_step2: Option<[u8; 2]>;
    while none_counter < 2 {
    for i in 0..(list_routes.len()) {
        //println!("list_routes: {:?}", list_routes);
        //println!("next_routes: {:?}", next_routes);
        none_counter = 0;   
        output_step1 = next_steps(&list_routes[i])[0];
        output_step2 = next_steps(&list_routes[i])[1];
        //println!("{:?} {:?}", output_step1, output_step2);
    match output_step1 {
        Some(step1) => {
            //println!("step 1: {:?}", step1);
            intermediate = list_routes[i].clone();
            intermediate.push(step1);
            //println!("{:?}", intermediate);
            next_routes.push(intermediate);
        }
        None => { 
            println!("nothing");
            none_counter += 1;
        }
        }

    match output_step2 {
        Some(step2) => {
            //println!("step 2: {:?}", step2);
            intermediate = list_routes[i].clone();
            intermediate.push(step2);
            //println!("{:?}", intermediate);
            next_routes.push(intermediate);
        }
        None => { 
            println!("nothing");
            none_counter += 1;
        }
    }



    //println!("loop number: {}", loop_counter);
    loop_counter += 1;
    if loop_counter % 500 == 0 {
        println!("the current number of routes is: {}, and the the first route is: {:?}", list_routes.len(), list_routes[0]);
    }
    //    for z in &list_routes {
    //        println!("{:?}", z);
    //    }
    }
    list_routes = next_routes;
    next_routes = vec![];
    }
    println!("the number of routes is: {}", list_routes.len());             
}


fn next_steps(input_vec: &Vec<[u8; 2]>) -> [Option<[u8; 2]>; 2] {
    let mut step1: Option<[u8; 2]> = None;
    let mut step2: Option<[u8; 2]> = None;

    let current_point: [u8; 2] = input_vec.last().expect("no gucci").clone();

    if current_point[0] < 20 && current_point[1] < 20 {
        step1 = Some([current_point[0] + 1, current_point[1]]);
        step2 = Some([current_point[0], current_point[1] + 1]);
    }   
    else if current_point[0] < 20 && current_point[1] == 20 {
        step1 = Some([current_point[0] + 1, current_point[1]]);
        step2 = None;
    }
    else if current_point[0] == 20 && current_point[1] < 20 {
        step1 = None;
        step2 = Some([current_point[0], current_point[1] + 1]);
    }
    else if current_point[0] == 20 && current_point[1] == 20 {
        step1 = None;
        step2 = None;
    }

    [step1, step2]
}
