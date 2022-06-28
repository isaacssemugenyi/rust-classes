// But, you cannot index strings! This is because they use the 
// One True Encoding, UTF-8, where a 'character' may be a number of bytes.

fn main() {
    let multilingual = "Hi! iHola! npglf!";
    for ch in multilingual.chars() {
        print!("'{}' ", ch); // 'H', 'i', '!' ' ' 'i' 'H' 'o' 'l' 'a' '!' ' ' 'n' 'p' 'g' 'l' 'f' '!'
    }

    println!("");
    println!("len {}", multilingual.len()); // len 17 -> wrong len 25
    println!("count {}", multilingual.chars().count()); // count 17

    let maybe = multilingual.find('n');
    if maybe.is_some() {
        let hi = &multilingual[maybe.unwrap()..];
        println!("Russian hi {}", hi); // Russian hi npglf!
    }

    // Now, let that sink in - there are 25 bytes, but only 18 characters! However, if you use a method like find, you will get a valid index (if found) and then any slice will be fine.
}