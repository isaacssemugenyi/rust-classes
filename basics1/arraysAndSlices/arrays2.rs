// Slices, think of them as views into an underlying array of values.
// Slices behave much like an array, and know thier size.

// Writing a slice;
// How to write a slice's type.
// You have to use & to pass it to the function

// read as: slice of 132
fn sum(values: &[i32]) -> i32 {
    let mut res = 0;
    for i in 0..values.len() {
        res += values[i]
    }
    res
}

fn main() {
    let arr = [10, 20, 30, 40];
    // look at that &
    let res = sum(&arr);
    println!("sum {}", res);
}