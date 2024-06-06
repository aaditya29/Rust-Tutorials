/*
Make it work
fn main() {
    let _v: () = ();

    let v = (2, 3);
    assert_eq!(v, implicitly_ret_unit());

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}
*/
fn main() {
    let v0: () = ();

    let v = (2, 3);
    assert_eq!(v0, implicitly_ret_unit())
}

fn implicitly_ret_unit() {
    println!("I will return a ()")
}

// don't use this one
fn explicitly_ret_unit() -> () {
    // unit type is a type which doesn't have any value. it's size is 0. it doesn't returns anything
    println!("I will return a ()")
}
