// iterators, the for-loop over a range was using an iterator 0..n similar to python 3 range function
// An iterator is easy to define informally. It is an 'object' with a next method which
// returns an Option. As long as that value is not None, we keep calling next:

fn main(){
    let mut iter = 0..3;
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);

    // And that is exactly what for var in iter {} does.
    // This may seem an inefficient way to define a for-loop, but rustc does 
    // carzy-ass optimizations in release mode and it will be just as fast as a while loop.
}