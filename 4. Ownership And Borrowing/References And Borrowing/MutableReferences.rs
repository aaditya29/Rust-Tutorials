/*
In Rust, mutable references are a way to introduce mutability to values in a safe and controlled manner. By default, variables in Rust are immutable, meaning their values cannot be changed after they are bound. However, we can create mutable references to allow modifying the value.

fn main() {
    let mut x = 5; // Create a mutable variable
    println!("x = {}", x); // Output: x = 5

    let y = &mut x; // Create a mutable reference to x
    (*)y = 10;//Dereference y and assign a new value to x

    println!("x = {}", x); // Output: x = 10
}

Here
let mut x = 5; creates a mutable variable x and initializes it with the value 5.
let y = &mut x; creates a mutable reference y that points to the memory location of x.
(*)y = 10; dereferences the mutable reference y and assigns the value 10 to the value it points to, which is x.
*/

fn main() {
    let mut name = String::from("abc");
    let name1 = &mut name; // creating a mutable reference to name
    *name1 = String::from("def");
    println!("{name1}");
    println!("{name}");
}
