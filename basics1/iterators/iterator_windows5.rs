// The windows method gives you an iterator of slices 
// - overlapping windows values!

fn main() {
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;

    for s in slice.windows(2) {
        println!("window {:?}", s);
        // windw [1, 2]
        // windw [2, 3]
        // windw [3, 4]
        // windw [4, 5]
    }
    
    // Or chunks:
    for s in slice.chunks(2) {
        println!("chunks {:?}", s);
        // chunks [1, 2]
        // chunks [3, 4]
        // chunks [5]
    }
}

