fn main() {
    let arr = [10, 20, 30, 40]; // array type [i32: 4];
    let first = arr[0];
    println!("first {}", first); //first 10

    for i in 0..4 {
        println!("[{}] = {}", i, arr[i]);
        /*
            [0] = 10
            [1] = 20
            [2] = 30
            [3] = 40
        */
    }
    println!("length {}", arr.len()); //length 4

    // if i try to access arr[4], it will be a compile error
}