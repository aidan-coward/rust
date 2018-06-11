fn main() {
    let test_routes = vec![vec![(0, 0)]];
    
    let output_routes = next_steps(test_routes);

    for x in output_routes {
        println!("{:?}", x);
    }
}

    






fn next_steps(route: Vec<(u8, u8)>) -> (Option<u8>, Option<u8>) {
    let first_route: Vec<(u8, u8)> = route.clone();
    let second_route: Vec<(u8, u8)> = route.clone();
    let current_point = route.pop();
    let point_1: Option<(u8, u8)>;
    let point_2: Option<(u8, u8)>;

// add next points based on position of current point
// option type in case there is no next point

    if current_point[0] < 40 { 
        point_1 = Some(current_point[0] + 1, current_point[1]);
    }
        else { 
            point_1 = None;
    }
    if current_point[1] < 40 {
        point_2 = Some(current_point[0], current_point[1 + 0]);
    }
        else {
            point_2 = None;
    }

    match point_1 {
        Some(x) => first_route = 

    route.push(current_point);
    (point_1, point_2)
}
   
