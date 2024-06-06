/*
/ Fix all errors without adding newline
fn main() {
    let s = String::from("hello");
    s.push(',');
    s.push(" world");
    s += "!".to_string();

    println!("{}", s);
}
*/

fn main() {
    let mut s = String::from("hello"); // making variable s mutable because changing further
    s.push(',');
    s.push_str("world");
    s += "!";

    println!("{}", s);
}
