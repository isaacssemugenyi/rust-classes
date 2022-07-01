// Ushing an entire string
let mut hi = String::from("Hey there...");
hi.push_str("How are you doing??");

// => hey there...Hoe are you doing??
println!("{hi}");