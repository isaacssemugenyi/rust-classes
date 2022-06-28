// Some operations can be elegantly expressed recursively
fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n-1)
    }
}

fn main(){
   println!("Factorial value of {} is {}", 3, factorial(3)); 
}

