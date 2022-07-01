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
        extinct: true,
        classified: 1841
    };

    println!("Species: {}", p.species);
    println!("Genus: {}", p.genus);
    println!("Classified in {}", p.classified);

    if p.extinct == true {
        println!("Sadly this penguin has been made extinct.");
    }
}