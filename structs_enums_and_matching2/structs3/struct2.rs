// Initializing this struct is a bit clumsy, so we want to move the construction of a Person into its own function. This function can be made into an associated function of Person by putting it into a impl block:

struct Person {
    first_name: String,
    last_name: String
}

impl Person {

    fn new(first: &str, name: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: name.to_string()
        }
    }

}

fn main() {
    let p = Person::new("John", "Smith");
    println!("person {} {}", p.first_name, p.last_name); // person John Smith
}

// There is nothing magic or reserved about the name new here. Note that it's accessed using a C++-like notation using double-colon ::.