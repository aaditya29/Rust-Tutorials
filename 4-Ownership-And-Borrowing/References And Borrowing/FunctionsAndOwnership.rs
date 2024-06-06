fn greet(first: String, last: String) {
    println!("Hello {first} {last}");
}

fn main() {
    let first = String::from("ABC");
    let last = String::from("def");
    greet(first, last); //calling function
}
