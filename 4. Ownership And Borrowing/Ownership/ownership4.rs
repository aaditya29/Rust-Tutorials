/*
 Fix the error without removing any code
fn main() {
    let s = String::from("Hello World");

    print_str(s);

    println!("{}", s);
}

fn print_str(s: String)  {
    println!("{}",s)
}
*/
fn main() {
    let s: String = String::from("Hello World");

    print_str(s.clone());
    println!("{}", s); //Using clone because s is transferred to print_str
}

fn print_str(s: String) {
    println!("{}", s);
}
