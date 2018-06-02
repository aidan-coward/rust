fn main() {
    let mut perf_squares: Vec<u32> = Vec::new();
    for c in 1..1000 {
        perf_squares.push(c*c);
    }
    for a in 1..1000 {
        for b in 1..1000 {
            if perf_squares.contains(&(a*a + b*b)) 
            && (((a*a + b*b) as f32).sqrt() as u32) + a + b == 1000 {
                println!("a: {}, b: {}, c: {}", a, b, (((a*a + b*b) as f32).sqrt() as u32));
                println!("{}", (((a*a + b*b) as f32).sqrt() as u32) * a * b);
            }
        }
    }
}

