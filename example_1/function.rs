struct Penguin {
    genus: String,
    species: String,
    extinct: bool,
    classified: u64
}

fn main() {
    let p = Penguin {
        genus: "Pygoscelis".to_owned(),
        species: "R adeli".to_owned(),
        extinct: false,
        classified: 1841
    };

    printer(p);
}

fn printer(p: Penguin) {
    println!("Species: {}", p.species);
    println!("Genus: {}", p.genus);
    println!("Classified in {}", p.classified);

    if p.extinct == true {
        println!("Sadly this penguin has been made extinct");
    }
}