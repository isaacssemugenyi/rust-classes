// Values can also be passed by reference.
// A reference is created by & and dereferenced *.

fn by_ref(x: &i32) -> i32 {
    *x + 1
}

fn main() {
    let i = 10;
    let res1 = by_ref(&i);
    let res2 = by_ref(&41);
    println!("{} {}", res1, res2); // 11 42
}