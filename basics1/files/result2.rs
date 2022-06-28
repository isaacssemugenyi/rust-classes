/*
This 'throwing away errors' thing is getting too much of a habit. 
You do not want to put this code into a function, knowing that it 
could so easily crash the whole program. So now we have to talk 
about exactly what File::open returns. If Option is a value that 
may contain something or nothing, then Result is a value that may 
contain something or an error. They both understand unwrap (and its 
cousin expect) but they are quite different. Result is defined by two 
type parameters, for the Ok value and the Err value. The Result 'box' 
has two compartments, one labelled Ok and the other Err.
*/

fn good_or_bad(good: bool) -> Result<i32, String> {
    if good {
        Ok(42)
    } else {
        Err("bad".to_string())
    }
}

fn main() {
    println!("{:?}", good_or_bad(true)); // Ok(42)

    println!("{:?}", good_or_bad(false)); // Err("bad")

    match good_or_bad(true){
        Ok(n) => println!("Cool, I got {}", n),
        Err(e) => println!("Huh, I just got {}", e)
    }
    // Cool, I got 42
}