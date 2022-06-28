// This first implementation brings an error
// refer to the resource on the structs page

// fn dump(s: String) {
//     println!("{}", s);
// }

// fn main() {
//     let s1 = "hello dolly".to_string();
//     // move occurs because `s1` has type `String`, which does not implement the `Copy` trait

//     dump(s1);
//     println!("s1 {}", s1); //value borrowed here after move
// }

// Working way is below:
// Here, you have a choice. You may pass a reference to that 
// string, or explicitly copy it using its clone method. 
// Generally, the first is the better way to go.

// =============================
// fn dump(s: &String) {
//     println!("{}", s);
// }

// =========================
fn dump(s: &str) {
    println!("{}", s);
}

fn main() {
    let s1 = "hello dolly".to_string();
    dump(&s1);
    println!("s1 {}", s1);
}

// But you'll rarely see a plain String reference like this, since to pass a string literal is really ugly and involves creating a temporary string
// dump(&"hello world".to_string());

// so altogether the best way to declare that function is:
// fn dump(s: &str) {
//     println!("{}", s);
// }

// Then both dump(&s1) and dump("hello world") work properly.
// Here Deref coercion kicks in and Rust will convert &String to &str for you

// To summarise, assignment of a non-Copy value moves the value from one location to another. 
// Otherwise, Rust would be forced to implicitly do a copy and break its promise to make allocations explicit.