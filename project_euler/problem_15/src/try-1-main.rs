fn main() {
    
let mut current_routes: Vec<Vec<(u8, u8)>> = vec![vec![(0, 0)]];
let mut next_routes: Vec<Vec<(u8, u8)>> = Vec::new();
let mut working_routes: Vec<Option<(u8, u8)>>, Vec<Option<(u8, u8>>;

while current_routes.len() < 42 {
    for route in current_routes {           // for all routes so far
        working_routes = next_steps(route);
        match working_routes.0 {           
            Some(x) => next_routes.push.vec![route.push(x)], // if the returned value is a route, push it onto the list of routes
            None => (),
        }
        match working_routes.1 {
            Some(y) => next_routes.push.vec![route.push(y)],
            None => (),
        }
        working_routes = [[()]];
    }
    
    current_routes = next_routes;
    next_routes = vec![()];
    }

println!("{}", current_routes.len);
}


// takes last point of vector and returns the acceptable next points with none if either one
// doesn't exist
fn next_steps(route: Vec<(u8, u8)>) -> (Option<u8>, Option<u8>) {
    let current_point = route.pop();
    let point_1: Option<(u8, u8)>;
    let point_2: Option<(u8, u8)>;

// add next points based on position of current point
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
    route.push(current_point);
    (point_1, point_2)
}
    


    
    
