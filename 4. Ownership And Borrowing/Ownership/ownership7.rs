/*
/*

fn main() {
    let x = Box::new(5);

    let

    y = 4;

    assert_eq!(*x, 5);

    println!("Success!");
}
*/
*/

fn main() {
    let x: Box<i32> = Box::new(5);

    let mut y: Box<i32> = Box::new(1); // Changing value of y hence using y as a mutable
    *y = 4;

    assert_eq!(*x, 5);

    println!("Success!");
}
