/*
fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1579);

    println!("Success!");
}

*/

fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111; //1024+255+63+255
    assert!(v == 1597); //checking value of v by adding in different number system

    println!("Success!");
}
