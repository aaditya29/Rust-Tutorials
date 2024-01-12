/*

/ Fix error with at least two solutions
fn main() {
    let s = "hello, world";
    greetings(s)
}

fn greetings(s: String) {
    println!("{}", s)
}
*/

fn main() {
    let s: &str = "hello,world";
    greetings(s.to_string()) // converting to string because s in greetings is string
}

fn greetings(s: String) {
    println!("{}", s)
}
