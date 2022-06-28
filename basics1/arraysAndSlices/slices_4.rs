// Slices give you different views of the same array

fn main() {
    let ints = [1, 2, 3, 4, 5];
    let slice1 = &ints[0..2];
    let slice2 = &ints[1..]; // open range!

    println!("ints {:?}", ints); // ints [1, 2, 3, 4, 5]
    println!("slice1 {:?}", slice1); // slice1 [1, 2]
    println!("slice2 {:?}", slice2); // slice2 [2, 3, 4, 5]
}

// In slices, a copy of data is never made, These slices all borrow their data from their arrays.