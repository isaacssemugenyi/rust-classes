// There is a very intimate relation between vectors and slices
fn dump(arr: &[i32]){
    println!("arr is {:?}", arr); // arr is [10, 20, 30]
}

fn main(){
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    
    dump(&v);

    let slice = &v[1..]; // pick 20 and 30 from the vector
    println!("slice is {:?}", slice); // slice is [20, 30]

    /**
     * when a vector is modified or created, it allocates from the heap and becomes the owner of that memory. 
     * The slice borrows the memory from the vector. When the vector dies or drops, it lets the memory go
    */

    /*
    If you come from a dynamic language, now is time for that little talk. In systems languages, 
    program memory comes in two kinds: the stack and the heap. It is very fast to allocate data on the stack, 
    but the stack is limited; typically of the order of megabytes. The heap can be gigabytes, but allocating is 
    relatively expensive, and such memory must be freed later. In so-called 'managed' languages (like Java, Go 
    and the so-called 'scripting' languages) these details are hidden from you by that convenient municipal 
    utility called the garbage collector. Once the system is sure that data is no longer referenced by other data, 
    it goes back into the pool of available memory.
    */
}