// Destructuring Assignments

/*
 can now use tuple, slice, and struct patterns as the left-hand side of an assignment.
*/

fn main() {
    let (x, y);
    (x, ..) = (3, 4); // .. value is ignored hence x is equals to 3
    [.., y] = [1, 2]; // .. value is ignored hence y is equals to 2

    assert_eq!([x, y], [3, 2]); // x is equal to 3 and y is equal to 2

    println!("Success!");
}
