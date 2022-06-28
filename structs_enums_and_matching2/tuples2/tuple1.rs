// It's sometimes very useful to return multiple values from a function. Tuples are a convenient solution:
fn add_mul(x: f64, y: f64) -> (f64, f64) {
    (x + y, x * y)
}

fn main() {
    let t = add_mul(2.0, 10.0);

    // can debug print
    println!("t {:?}", t); // t(12, 20)

    // can 'index' the values
    let (add, mul) = t;
    println!("add {} mul {}", add, mul); // add 12 mul 20
}

// Tuples may contain different types, which is the main difference from arrays.
    let tuple = ("hello", 5, 'c');
    assert_eq!(tuple.0, "hello");
    assert_eq!(tuple.1, 5);
    assert_eq!(tuple.2, 'c');
    
// They appear in some Iterator methods. enumerate is like the Python generator of the same name:
    for t in ["zero", "one", "two"].iter().enumerate() {
        print!(" {} {};", t.0, t.1); // 0 zero; 1 one; 2 two;
    }

// zip combines two iterators into a single iterator of tuples containing the values from both:
    let names = ["ten", "hundred", "thousand"];
    let nums = [10, 100, 1000];
    for p in names.iter().zip(nums.iter()) {
        print!(" {} {};", p.0, p.1); // ten 10; hundred 100; thousand 1000;
    }
