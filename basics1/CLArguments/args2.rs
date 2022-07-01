use std::env;

fn main() {
    let first = env::args().nth(1).expect("please supply an argument");
    let n: i32 = first.parse().expect("not an integer!");
    // do your magic

    println!("{}", n);

    /*
     nth(1) gives you the second value of the iterator, and expect is like an unwrap with a readable message.
     Converting a string into a number is straightforward, but you do need to specify the type of the value - how else could parse know?
    This program can panic, which is fine for dinky test programs. But don't get too comfortable with this convenient habit.
    */
}