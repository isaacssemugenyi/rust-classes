/*
Loop variables (like i) are a little different, they are only visible in the loop block. It is not an error to create a new variable using the same name ('shadowing') but it can be confusing.

When a variable 'goes out of scope' then it is dropped. Any memory used is reclaimed, and any other resources owned by that variable are given back to the system - for instance, dropping a File closes it. This is a Good Thing. Unused resources are reclaimed immediately when not needed.

(A further Rust-specific issue is that a variable may appear to be in scope, but its value has moved.)

Here a reference rs1 is made to a value tmp which only lives for the duration of its block:
*/

fn main() {
    let s1 = "hello dolly".to_string();
    let mut rs1 = &s1;
    {
        let tmp = "hello world".to_string(); // `tmp` does not live long enough
        rs1 = &tmp; // borrowed value does not live long enough
    } // `tmp` dropped here while still borrowed
    println!("ref {}", rs1); // borrow later used here
}

// Where is tmp? Gone, dead, gone back to the Big Heap in the Sky: dropped. Rust is here saving you from the dreaded 'dangling pointer' problem of C - a reference that points to stale data.