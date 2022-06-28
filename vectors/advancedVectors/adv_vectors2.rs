/**
 * Vectors have a size and a capacity. 
 * If you clear a vector, its size becomes 
 * zero, but it still retains its old 
 * capacity. So refilling it with push, 
 * etc only requires reallocation when the 
 * size gets larger than that capacity.
*/

/**
 * Vectors can be sorted, and then 
 * duplicates can be removed - these 
 * operate in-place on the vector. 
 * (If you want to make a copy first, 
 * use clone.)
*/

fn main() {
    let mut v1 = vec![1, 10, 5, 1, 2, 11, 2, 40];
    v1.sort(); // sorts the vector
    v1.dedup(); // removes duplicates
    assert_eq!(v1, &[1, 2, 5, 10, 11, 40]);
}