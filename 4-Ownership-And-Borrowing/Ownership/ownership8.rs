/*

fn main() {
   let t = (String::from("hello"), String::from("world"));

   let _s = t.0;

   // Modify this line only, don't use `_s`
   println!("{:?}", t);
}
*/

fn main() {
    let t: (String, String) = (String::from("hello"), String::from("world"));

    let _s: String = t.0;

    // Modify this line only, don't use `_s`
    println!("{:?}", t.1); //Calling second element of tuple
}
