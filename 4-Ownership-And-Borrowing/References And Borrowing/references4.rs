/*

 Fix error
fn main() {
    let mut s = String::from("hello, ");

    push_str(s);

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world")
}
*/
fn main() {
    let mut s = String::from("hello, ");

    push_str(&mut s); // s is holding mut string so passing it

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world")
}
