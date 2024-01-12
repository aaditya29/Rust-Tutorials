//String type is defined in std and stored as a vector of bytes (Vec), but guaranteed to always be a valid UTF-8 sequence. String is heap allocated, growable and not null terminated.

/*

/ Fill the blank
fn main() {
    let mut s = __;
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success!");
}
*/

fn main() {
    let mut s: String = String::from(""); // Using string not &s(string literals) because literals are immutable
    s.push_str("hello, world");
    s.push('!'); // pushing a single char

    assert_eq!(s, "hello, world!");

    println!("{}", s);
    println!("Success!");
}
