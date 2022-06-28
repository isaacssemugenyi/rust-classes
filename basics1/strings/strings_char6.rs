// (The Rust char type is a 4-byte Unicode code point. Strings are not arrays of chars!)
/**
 * String slicing may explode like vector indexing, because it uses byte offsets. 
 * In this case, the string consists of two bytes, so trying to pull out the first 
 * byte is a Unicode error. So be careful to only slice strings using valid offsets 
 * that come from string methods.
*/

/**
 * Breaking up strings is a popular and useful pastime. 
 * The string split_whitespace method returns an iterator, 
 * and we then choose what to do with it. A common need is to 
 * create a vector of the split substrings.
 */

fn main() {
    // let s = "i";
    // println!("{}", &s[0..1]); // <-- bad, first byte of a multibyte character

    // collect is very general and so needs some clues about what it is collecting - hence the explicit type.
    let text = "the red fox and the lazy dog";
    // let words: Vec<&str> = text.split_whitespace().collect(); // ["the", "red", "fox", "and", "the", "lazy", "dog"]
    // println!("{:?}", words);

    // You could also say it like this, passing the iterator into the extend method:
    let mut words = Vec::new();
    words.extend(text.split_whitespace());
    println!("{:?}", words);

    /*
     * Have a look at this cute two-liner; we get an iterator over the chars, 
     * and only take those characters which are not space. Again, collect needs 
     * a clue (we may have wanted a vector of chars, say):
    */ 

    let stripped: String = text.chars()
        .filter(|ch| ! ch.is_whitespace()).collect();
        //  theredfoxandthelazydog
    
    println!("stripped string {}", stripped);

    /*
    The filter method takes a closure, which is Rust-speak for lambdas or anonymous functions. Here the argument type is clear from the context, so the explicit rule is relaxed.
    Yes, you can do this as an explicit loop over chars, pushing the returned slices into a mutable vector, but this is shorter, reads well (when you are used to it, of course) and just as fast. It is not a sin to use a loop, however, and I encourage you to write that version as well.
    */
}

