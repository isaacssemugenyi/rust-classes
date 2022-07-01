let mut array: [i64; 4] = [1, 2, 3, 4];
let mut slices: &[i64] = &array[0..3] //Lower range is inclusive and upper range is exclusive

println!("The elements of the slices are {slices:?}");