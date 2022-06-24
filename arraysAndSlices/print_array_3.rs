// Slicing and Dicing
// You cant print an array with {} but use the debug print {:?}

fn main() {
    let ints = [1, 2, 3];
    let floats = [1.1, 2.1, 3.1];
    let strings = ["hello", "world"];
    let ints_ints = [[1, 2], [10, 20]];

    // Prints
    println!("ints {:?}", ints); //ints [1, 2, 3]
    println!("floats {:?}", floats); //floats [1.1, 2.1, 3.1]
    println!("strings {:?}", strings); //strings ["hello", "world"]
    println!("ints_ints {:?}", ints_ints); //ints_ints [[1, 2], [10, 20]]
}

// An array should contains values/ elements of one type
