// arraysAndSlices/arrays2.rs
// We had example of summing up a range of integers earlier.
// it involved a mut variable and loop. 
// Here's the idiomatic, pro-level way of doing the sum.

fn main(){
    let sum: i32 = (0..5).sum(); // 5 is not inclusive
    println!("sum was {}", sum); // -> sum was 10

    println!("==========================");

    let sum: i64 = [10, 20, 30].iter().sum();
    println!("sum was {}", sum); // -> sum was 60

    // Here you need to be explicit on the type of the variable
}