/*

fn main() {
    let mut s = String::from("hello, ");

    Fill the blank to make it work
    let p = __;

    p.push_str("world");

    println!("Success!");
}
*/

fn main() {
    let mut s = String::from("hello, ");

    let p: &mut String = &mut s;

    p.push_str("world");

    println!("Success!");
}
