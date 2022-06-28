// Reading files in the program
// expect is like unwrap but gives a custom error message. Gonna throw away a few errors here.

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let first = env::args().nth(1).expect("please supply a filename");

    let mut file = File::open(&first).expect("can't open the file");

    let mut text = String::new();
    file.read_to_string(&mut text).expect("can't read the file");

    println!("file had {} bytes", text.len()); // ./read_file read_file.rs

    /*
    So open can fail because the file doesn't exist or we aren't allowed to read it, 
    and read_to_string can fail because the file doesn't contain valid UTF-8. 
    (Which is fair enough, you can use read_to_end and put the contents into a vector 
    of bytes instead.) For files that aren't too big, reading them in one gulp is useful 
    and straightforward.
    */

    /*
    If you know anything about file handling in other languages, you may wonder when 
    the file is closed. If we were writing to this file, then not closing it could 
    result in loss of data. But the file here is closed when the function ends and 
    the file variable is dropped.
    */
}