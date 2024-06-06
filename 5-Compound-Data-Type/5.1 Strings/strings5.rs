//replace can be used to replace substring

/*
fn main() {
    let s = String::from("I like dogs");
    / Allocate new memory and store the modified string there
    let s1 = s.__("dogs", "cats");

    assert_eq!(s1, "I like cats");

    println!("Success!");
}
 */
fn main() {
    let mut s: String = String::from("I like dogs");
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats");

    println!("Success!");
}
