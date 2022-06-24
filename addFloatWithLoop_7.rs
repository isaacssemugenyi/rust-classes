// if to return a float, we will have to cast the value to a floating-point value explicitly
fn main(){
    let mut sum = 0.0;
    for i in 0..5 {
        sum += i as f64;
    }
    println!("sum is {}", sum); //sum is 10
}