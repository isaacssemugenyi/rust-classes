// What if you want a function to modify one of its arguments. Enter mutable references.
fn modifies(x: &mut f64){
    *x = 1.0;
}

fn main(){
    let mut res = 0.0;
    modifies(&mut res);
    println!("res is {}", res);
}

// rustup doc --std -> for loading the docs