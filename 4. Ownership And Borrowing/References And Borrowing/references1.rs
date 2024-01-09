/*

fn main() {
   let x = 5;
   Fill the blank
   let p = __;

   println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
}
*/
fn main() {
    let x = 5;
    let p: &i32 = &x;

    println!("the memory address of x is {:p}", p);
}
