// Optional Values
// Slices, like arrays, can be indexed
// Size of an array is known at compile-time
// Size of slice is only known at run-time, so s[i] can cause 
// out-of-bounds error when running and will panic.
// Exceptions handling cant handle these panics, instead using get method on slices

fn main() {
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;
    let first = slice.get(0);
    let last = slice.get(5);

    println!("first {:?}", first); // first Some(1)
    println!("last {:?}", last); // last None

    // last failed but returned None coz we forgot the zero-based indexing
    // first returns a value wrapped in Some. Welcome to the Option type
    // It is either Some or None

    // The Option type has some useful methods
    println!("first {} {}", first.is_some(), first.is_none()); // first true false
    println!("last {} {}", last.is_some(), last.is_none()); // last false true
    println!("first value {}", first.unwrap()); // first value 1

    // If you unwrap last, you would get a panic.
    // At least u can call is_some first to make sure like,

    let maybe_last = slice.get(5);
    let last2 = if maybe_last.is_some() {
        *maybe_last.unwrap()
        // Note the * - the precise type inside the Some is &i32, which is a reference.
        // We need to dereference this to get back to a i32 value, which is long-winded.
        // so there's a shortcut, unwrap_or, will return the value it is given if the Option was None.
        // The types must match up - get returns a reference.
        // So you have to make up a &i32 with &-1. Finally, again use * to get the value as i32.
        // like below
        // let last = *slice.get(5).unwrap_or(&-1);
        // ou can think of Option as a box which may contain a value, or nothing (None)
    } else {
        -1
    };
}