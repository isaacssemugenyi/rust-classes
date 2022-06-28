fn main() {
    let multilingual = "Hi! iHola! npglf!";
    for ch in multilingual.chars() {
        print!("'{}' ", ch); // 'H', 'i', '!' ' ' 'i' 'H' 'o' 'l' 'a' '!' ' ' 'n' 'p' 'g' 'l' 'f' '!'
    }

    println!("");
    println!("len {}", multilingual.len()); // len 17 -> wrong len 25
    println!("count {}", multilingual.chars().count()); // count 17

    match multilingual.find('n') {
        Some(idx) => {
            let hi = &multilingual[idx..];
            println!("Russian hi {}", hi); 
        },
        None => println!("couldn't find the greeting, Toknop")
    }       
}

/*
match consists of several patterns with a matching value following the fat arrow, 
separated by commas. It has conveniently unwrapped the value from the Option and bound it to idx. 
You must specify all the possibilities, so we have to handle None.

Once you are used to it (and by that I mean, typed it out in full a few times) it 
feels more natural than the explicit is_some check which needed an extra variable 
to store the Option.
*/