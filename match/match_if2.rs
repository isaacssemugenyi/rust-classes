// But if you're not interested in failure here, then `if let` is your friend

fn main() {
    let multilingual = "Hi! iHola! npglf!";
    for ch in multilingual.chars() {
        print!("'{}' ", ch); // 'H', 'i', '!' ' ' 'i' 'H' 'o' 'l' 'a' '!' ' ' 'n' 'p' 'g' 'l' 'f' '!'
    }

    println!("");
    println!("len {}", multilingual.len()); // len 17 -> wrong len 25
    println!("count {}", multilingual.chars().count()); // count 17

    if let Some(idx) = multilingual.find('n') {
        println!("Russian hi {}", &multilingual[idx..]); 
        // This is convenient if you want to do a match and are only interested in one possible result.
    }       
}