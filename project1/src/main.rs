use rand::Rng;

struct Penguin {
    genus: String,
    species: String,
    extinct: bool,
    classified: u64
}

fn main() {
    let p = Penguin {
        genus: "Pygoscelis".to_owned(),
        species: "R adelie".to_owned(),
        extinct: false,
        classified: 1841
    };

    printer(p);
    foo();
}

fn printer(p: Penguin) {
    println!("Species: {}", p.species);
    println!("Genus: {}", p.genus);
    println!("Classified in {}", p.classified);

    if p.extinct == true {
        println!("Sadly this penguin has been made extinct.");
    }
}

fn foo() {
    let mut rng = rand::thread_rng();
    let mut n = rng.gen_range(1..99);

    println!("Value is {}", n);
    n = rng.gen_range(1..99);
    println!("Eight is written as {}", n);
}

// https://phaiax.github.io/rust-cheatsheet/
// https://github.com/donbright/rust-lang-cheat-sheet