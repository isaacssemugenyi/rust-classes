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

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

fn main() {
    let p = Person::new("John", "Smith");
    println!("{}, {}, full name {}", p.first_name, p.last_name, p.full_name());
    // John, Smith, full name John Smith
}

// The self is used explicitly and is passed as a reference. (You can think of &self as short for self: &Person.)

// The keyword Self refers to the struct type - you can mentally substitute Person for Self here:
    // fn copy(&self) -> Self {
    //     Self::new(&self.first_name,&self.last_name)
    // }

// Methods may allow the data to be modified using a mutable self argument:
    // fn set_first_name(&mut self, name: &str) {
    //     self.first_name = name.to_string();
    // }

// And the data will move into the method when a plain self argument is used:
    // fn to_tuple(self) -> (String,String) {
    //     (self.first_name, self.last_name)
    // }  

/*
(Try that with &self - structs will not let go of their data without a fight!)

Note that after v.to_tuple() is called, then v has moved and is no longer available.

To summarize:

    no self argument: you can associate functions with structs, like the new "constructor".
    &self argument: can use the values of the struct, but not change them
    &mut self argument: can modify the values
    self argument: will consume the value, which will move.
*/