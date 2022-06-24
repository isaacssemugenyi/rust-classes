// fn sqr(x: f64) -> f64 {
//     return x * x;
// }

// same as 
fn sqr(x: f64) -> f64 {
    x * x
}

fn main() {
    let res = sqr(2.0);
    println!("square is {}", res); // square is 4;
}