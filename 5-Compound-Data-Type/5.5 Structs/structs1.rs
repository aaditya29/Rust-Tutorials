// We must specify concrete values for each of the fields in struct.

/*

/ Fix the error
struct Person {
    name: String,
    age: u8,
    hobby: String
}
fn main() {
    let age = 30;
    let p = Person {
        name: String::from("sunface"),
        age,
    };

    println!("Success!");
}
*/

struct Person {
    name: String,
    age: u8,
    hobby: String,
}
fn main() {
    let age: u8 = 30; // defining datatype
    let p: Person = Person {
        name: String::from("sunface"),
        age,
        hobby: String::from("Movies"),
    };

    println!("Success!");
}
