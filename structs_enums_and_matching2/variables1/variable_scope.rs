// the rule of thumb is to prefer to keep references to the original data - to 'borrow' it.
// But a reference must not outlive the owner!
// Rust is a block-scoped language. Variables only exist for the duration of their block:

fn main() {
    let a = 10;
    let b = "hello";
    {
        let c = "hello".to_string();
        // a, b and c are visible
        println!("{}", c);
    }
    // the string c is dropped
    // a, b are visible
    for _i in 0..a { // _i underscore is added to i to denote to the complier that i is assigned but not used
        let b = &b[1..];
        // original b in no longer visible - it is shadowed.
        println!("{:?}", b);
    }
    // the slice b is dropped
    println!("{}", b);
    // i is _not_ visible!
}

// Loop variables (like i) are a little different, they are only visible in the loop block. 
// It is not an error to create a new variable using the same name ('shadowing') but it can be confusing.

