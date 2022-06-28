// Vectors are re-sizeable arrays and behave much like Python List and C++ std::vector.
// Rust type Vec (pronounced 'vector') behaves very much like a slice in fact; the difference
// is that you can append extra values to a vector - note that it must be declared as mutable.

fn main(){
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    let first = v[0]; // will panic if out-of-range
    let maybe_first = v.get(0);

    println!("v is {:?}", v);  // v is [10, 20, 30]
    println!("first is {}", first); // first is 10
    println!("maybe_first is {:?}", maybe_first);  //maybe_first is Some(10)

}