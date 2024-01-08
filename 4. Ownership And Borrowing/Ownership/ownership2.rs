/*
 Don't modify code in main!
fn main() {
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String) {
    println!("{}", s);
}
*/
// Don't modify code in main!
fn main() {
    let s1: String = String::from("Hello world"); // s1 can't be used further
    let s2: String = take_ownership(s1); // s2 will be owner of the data because s1's data has already been passed now to s2

    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String) -> String {
    //Returning string
    println!("{}", s);
    s // returning string
}
