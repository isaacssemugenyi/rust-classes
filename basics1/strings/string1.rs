/**
 * Strings in Rust are a little more involved than in other languages; the String type, like Vec, 
 * allocates dynamically and is resizeable. (So it's like C++'s std::string but not like the immutable 
 * strings of Java and Python.) But a program may contain a lot of string literals (like "hello") and a 
 * system language should be able to store these statically in the executable itself. In embedded micros, 
 * that could mean putting them in cheap ROM rather than expensive RAM (for low-power devices, RAM is also 
 * expensive in terms of power consumption.) A system language has to have two kinds of string, allocated or static.
*/

/**
 * So "hello" is not of type String. It is of type &str (pronounced 'string slice'). It's like the distinction 
 * between const char* and std::string in C++, except &str is much more intelligent. In fact, &str and String 
 * have a very similar relationship to each other as do &[T] to Vec<T>.
*/

fn dump(s: &str) {
    println!("str '{}'", s);
}

fn main() {
    let text = "hello dolly"; // the string slice
    let s = text.to_string(); // it's now an allocated string

    dump(text);
    dump(&s);
}

// The borrow operator can coerce String into &str, just as Vec<T> could be coerced into &[T].
// Under the hood, String is basically a Vec<u8> and &str is &[u8], but those bytes must represent valid UFT-8 text.
