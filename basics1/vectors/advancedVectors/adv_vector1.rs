// there is a useful little marco vec! for initializing a vector.
// Note that you can remove values from the end of a vector using pop,
// and extend a vector using any compatible iterator.

fn main() {
    let mut v1 = vec![10, 20, 30, 40];
    v1.pop(); // removes the last element

    let mut v2 = Vec::new();
    v2.push(10);
    v2.push(20);
    v2.push(30);

    assert_eq!(v1, v2); // they are equal

    v2.extend(0..2);
    assert_eq!(v2, &[10, 20, 30, 0, 1]);

    // vectors compare with each other and with slices by value.
    /**
     * You can insert values into a vector at arbitrary positions with insert, 
     * and remove with remove. 
     * This is not as efficient as pushing and popping since the values will 
     * have to be moved to make room, so watch out for these operations on big vectors.
    */
}