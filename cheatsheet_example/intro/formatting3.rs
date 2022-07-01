// Single Placeholder
println!("{}", 1);

// Multiple Placeholder
println!("{} {}", 1, 3);

// Positional Arguments
println!("{0} is {1}, {2}, also {0} is a {3} programming language", "Rust", "cool", "language", "safe");

// Named Arguments
println!("{country} is a diverse nation with unity.", country = "Uganda");

// Placeholder traits :b for binary, :0x is for hex and :o is octal
println!("Let us print 76 in binary which is {:b}, and hex equivalent is {:0x} and octal equivalent is {:o}", 76, 76, 76);

// Debug Trait
println!("Print whatever we want to here using debug trait {:?}", (76,, 'A', 90));

// New Format Strings in 1.58
let x = "world";
println("Hello {x}!");