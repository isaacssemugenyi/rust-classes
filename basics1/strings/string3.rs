/**
 * You can convert many types to strings using to_string 
 * (if you can display them with '{}' then they can be converted). 
 * The format! macro is a very useful way to build up more complicated 
 * strings using the same format strings as println!.
*/

fn array_to_str(arr: &[i32]) -> String {
    let mut res = '['.to_string();
    for v in arr {
        res += &v.to_string();
        res.push(',');
    }

    res.pop();
    res.push(']');
    res
}

fn main() {
    let arr = array_to_str(&[10, 20, 30]);
    let res = format!("hello {}", arr);

    assert_eq!(res, "hello [10,20,30]");

    // Note the & in front of v.to_string() - the operator is defined on a string slice,
    // not a String itself, so it needs a little persuasion to match.
}