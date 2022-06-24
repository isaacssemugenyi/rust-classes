// Adding all numbers from 0 to 4
fn main() {
    // make the sum mutable with mut keyword to be able to add to it
    let mut sum = 0;
    for i in 0..5 {
        sum += i;
    }
    println!("sum is {}", sum); // 10
}

