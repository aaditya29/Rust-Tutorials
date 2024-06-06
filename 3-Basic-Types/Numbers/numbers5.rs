/*

Fix errors and panics to make it work
fn main() {
   let v1 = 251_u8 + 8;
   let v2 = i8::checked_add(251, 8).unwrap();
   println!("{},{}",v1,v2);
}

*/

fn main() {
    let v1: u16 = 251_u16 + 8; // Max limit of u8 is 255 so 259 will overflow.
    let v2: i16 = i16::checked_add(251, 8).unwrap(); // Max limit of i16 is 255 so 259 will overflow.

    println!("{}, {}", v1, v2);
}
