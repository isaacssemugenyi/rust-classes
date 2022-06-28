// match can also operate like a C switch statement, and like other Rust constructs can return a value
fn main() {
    let text = match n {
        0 => "zero",
        1 => "one",
        2 => "two",
        _ => "many"
    };

    // _ is like C default

    // Rust match statements can also match on ranges.
    // Note that these ranges have three dots and are
    // inclusive ranges, so that the first condition would match 3.

    let text = match n {
        0...3 => "small",
        4...6 => "medium",
        _ => "large"
    };
}