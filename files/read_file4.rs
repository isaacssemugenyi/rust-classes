// The std::io module defines a type alias io::Result<T> which is exactly the same as Result<T,io::Error> and easier to type.

use std::env;
use std::fs::File;
use std::io::Read;
use std::io;

fn read_to_string(filename: &str) -> io::Result<String> {
    let mut file = File::open(&filename)?;
    let mut text = String::new();

    file.read_to_string(&mut text)?;
    Ok(text)
}

fn main() {
    let file = env::args().nth(1).expect("please supply a filename");

    let text = read_to_string(&file).expect("bad file man!");

    println!("file had {} bytes", text.len());
}

/*
That ? operator does almost exactly what the match on File::open does; 
if the result was an error, then it will immediately return that error. 
Otherwise, it returns the Ok result. At the end, we still need to wrap 
up the string as a result.

2017 was a good year for Rust, and ? was one of the cool things that 
became stable. You will still see the macro try! used in older code:
*/