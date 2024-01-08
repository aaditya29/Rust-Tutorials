/*
Don't use clone ,use copy instead
fn main() {
    let x = (1, 2, (), "hello".to_string());
    let y = x.clone();
    println!("{:?}, {:?}", x, y);
}
*/

fn main() {
    let x: (i32, i32, (), &str) = (1, 2, (), "hello"); // using string literal instead of string to avoid not using clone method
    let y: (i32, i32, (), &str) = x;
    println!("{:?}, {:?}", x, y);
}
