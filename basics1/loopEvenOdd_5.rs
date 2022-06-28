// fn main(){
//     for i in 0..5 {
//         if i % 2 == 0 {
//             println!("even {}", i); // if even execute
//         } else {
//             println!("odd {}", i); // if odd execute
//         }
//     }
    /*
    even 0
    odd 1
    even 2
    odd 3
    even 4
*/
// }


// Same as above
fn main() {
    for i in 0..5 {
        let even_odd = if i % 2 == 0 {"even"} else {"odd"};
        println!("{} {}", even_odd, i);
    }
}

/*
Output
even 0
odd 1
even 2
odd 3
even 4
*/