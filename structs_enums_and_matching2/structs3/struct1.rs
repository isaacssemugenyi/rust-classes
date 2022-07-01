// Tuples are convenient, but saying t.1 and keeping track of the meaning of each part is tedious for anything that isn't straightforward.

// Rust structs contain named fields;

struct Person {
    first_name: String,
    last_name: String
}

fn main() {
    let p = Person {
        first_name: "John".to_string(),
        last_name: "Smith".to_string()
    };
    println!("person {} {}", p.first_name, p.last_name); // person John Smith
}

// The values of a struct will be placed next to each other in memory, although you should not assume any particular memory layout, since the compiler will organize the memory for efficiency, not size, and there may be padding.