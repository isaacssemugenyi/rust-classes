// c && d   => Both are true (AND)
// c || d   => Either are true (OR)
// !c       => c is false (NOT)

let (c, d) = (true, false);

let and = c && d; // => false
let or = c || d; // => true
let not = !c;   // => false