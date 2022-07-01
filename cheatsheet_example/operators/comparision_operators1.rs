// e == f   => e is equal to f
// e != f   => e is NOT equal to f
// e < f    => e is less than f
// e > f    => e is greater than f
//  e <= f  => e is less than or equal to f
//  e >= f  => e is greater or equal to f

let (e, f) = (1, 100);

let greater = f > e;        // => true
let less = f < e;           // => false
let greater_equal = f >= e; // => true
let less_equal = e <= f;    // => true
let equal_to = e == f;      // => false
let not_equal_to = e != f;  // => true