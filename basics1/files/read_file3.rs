// This version of the file reading function does not crash. It returns a Result and it is the caller who must decide how to handle the error.

use std::env;
use std::fs::File;
use std::io::Read;
use std::io;

fn read_to_string(filename: &str) -> Result<String, io::Error> {
    let mut file = match File::open(&filename) {
        Ok(f) => f,
        Err(e) => return Err(e)
    };

    let mut text = String::new();
    match file.read_to_string(&mut text) {
        Ok(_) => Ok(text),
        Err(e) => Err(e)
    }
}

fn main() {
    let file = env::args().nth(1).expect("please supply a filename");

    let text = read_to_string(&file).expect("bad file man!");

    println!("file had {} bytes", text.len());
}

/*
The first match safely extracts the value from Ok, which becomes the value of the match. If it's Err it returns the error, rewrapped as an Err.

The second match returns the string, wrapped up as an Ok, otherwise (again) the error. The actual value in the Ok is unimportant, so we ignore it with _.

This is not so pretty; when most of a function is error handling, then the 'happy path' gets lost. Go tends to have this problem, with lots of explicit early returns, or just ignoring errors. (That is, by the way, the closest thing to evil in the Rust universe.)

Fortunately, there is a shortcut.

The std::io module defines a type alias io::Result<T> which is exactly the same as Result<T,io::Error> and easier to type.
*/