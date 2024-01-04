/*
Make it work with two ways
fn main() {
   let v = {
       let mut x = 1;
       x += 2
   };

   assert_eq!(v, 3);

   println!("Success!");
}

*/

fn main() {
    let v: i32 = {
        let mut x: i32 = 1;
        x += 2; //3

        x // returning x's value to v i.e. 3
    };

    assert_eq!(v, 3);

    println!("Success!");
}
