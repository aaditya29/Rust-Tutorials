/*

/ Use two approaches to fix the error and without adding a new line
fn main() {
    let s = "hello, world".to_string();
    let s1: &str = s;

    println!("Success!");
}
*/
fn main() {
    let s: String = "hello, world".to_string();
    let s1: &str = &s; //passing s as reference

    println!("Success!");
}
