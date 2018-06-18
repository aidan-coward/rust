fn main() {
    let mut list_routes: Vec<Vec<[u8; 2]>> = vec![vec![[0, 0]]];
    
    let mut next_list_routes: Vec<Vec<[u8; 2]>> = Vec::new();
    let mut none_counter: i32 = 0;
    let mut new_route: Vec<[u8; 2]> = Vec::new();

     for i in 0..list_routes.len() {
        for y in next_steps(list_routes[i]).iter() {
            match y {
                Some(next_point) => {
                    new_route.push(*next_point);
                    next_list_routes.push(new_route);
                }
                None => none_counter += 1,
            }
            new_route = vec![];
            if none_counter == 2 {
                break;
            }
        }
        list_routes = next_list_routes;
        next_list_routes = vec![];
    }
    for x in list_routes {
        println!("{:?}", x);
    }
}


fn next_steps(mut route: Vec<[u8; 2]>) -> [Option<[u8; 2]>; 2] {
    // let first_route: Vec<[u8; 2]> = route.clone();
    // let second_route: Vec<[u8; 2]> = route.clone();
    let current_point: [u8; 2];
    match route.pop() {
        Some(x) => current_point = x,
        None => panic!("invalid route"),
    }
    let point_1: Option<[u8; 2]>;
    let point_2: Option<[u8; 2]>;

// add next points based on position of current point
// option type in case there is no next point

    if current_point[0] < 40 { 
        point_1 = Some([current_point[0] + 1, current_point[1]]);
    }
        else { 
            point_1 = None;
    }

    if current_point[1] < 40 {
        point_2 = Some([current_point[0], current_point[1] + 1]);
    }
        else {
            point_2 = None;
    }

    route.push(current_point);
    [point_1, point_2]
}
   
