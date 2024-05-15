//Using field init shorthand syntax to reduce repetitions.
/*
/ Fill the blank
struct Person {
    name: String,
    age: u8,
}
fn main() {
    println!("Success!");
}

fn build_person(name: String, age: u8) -> Person {
    Person {
        age,
        __
    }
}
*/

// Fill the blank
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let name = String::from("Arfan");
    let age = 28;
    let arfan = build_person(name, age);

    println!("Person 'arfan': {:?}", arfan);
    println!("Success!");
}

fn build_person(name: String, age: u8) -> Person {
    Person { age, name }
}
