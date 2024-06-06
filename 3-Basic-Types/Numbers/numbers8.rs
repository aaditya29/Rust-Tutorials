/*
Make it work in two distinct ways


fn main() {
    assert!(0.1+0.2==0.3);

    println!("Success!");
}

*/

fn main() {
    assert!(0.1 as f32 + 0.2 as f32 == 0.3 as f32);

    println!("Success!");
}
