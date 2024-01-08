/*

fn main() {
    let s = give_ownership();
    println!("{}", s);
}

 Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("Hello world");
     Convert String to Vec
    let _s = s.into_bytes();
    s
}
*/

fn main() {
    let s = give_ownership();
    println!("{}", s);
}

fn give_ownership() -> String {
    let s: String = String::from("Hello world");
    let _s = s.as_bytes();
    /*
    The into_bytes method consumes the CString and returns the underlying byte buffer.
    The returned buffer does not contain the trailing null terminator, and it is guaranteed to not have any interior nul bytes.
    To overcome it we use the as_bytes so that the variable is conserved not consumed.
     */
    s
}
