// Getting Command Line Arguments
// std::env::args is how you access command-line arguments; it returns an iterator over the arguments as strings, including the program name

fn main() {
            // Would it have been better to return a Vec? It's easy enough to use collect to make that vector, using the iterator skip method to move past the program name.
    let args: Vec<String> = std::env::args().skip(1).collect();
    for arg in std::env::args() {
        // println!("'{}'", arg); // prints out aurgments in the terminal

    }
}