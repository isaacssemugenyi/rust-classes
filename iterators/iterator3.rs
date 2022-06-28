fn main(){
    let arr = [10, 20, 30];
    for i in arr.iter() {
        println!("{}", i);
    }

    // slices will be converted implicitly to iterators
    println!("============================");
    let slice = &arr;
    for i in slice {
        println!("{}", i);
    }
}

// it is more efficient to iterate over an array or slice like above than to use 
// for i in 0..slice.len(){} because Rust does not have to obsessively check every index operation.