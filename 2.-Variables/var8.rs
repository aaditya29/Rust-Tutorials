// Destructuring

/*
We can use a pattern with let to destructure a tuple to separate variables.
*/

fn main() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3); // x should be equal to 3
    assert_eq!(y, 2); //y should be equal to 2

    println!("Success!");
}
