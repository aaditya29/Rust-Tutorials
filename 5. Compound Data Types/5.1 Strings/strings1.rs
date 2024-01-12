// We can't use str type in normal ways, but we can use &str.

/*

 Fix error without adding new line
fn main() {
    let s: str = "hello, world";

    println!("Success!");
}
*/

fn main() {
    let s: &str = "hello, world";

    println!("Success!");
}
