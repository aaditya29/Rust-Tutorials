/*

/ Fix errors without removing any line
fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1 + s2;
    assert_eq!(s3, "hello,world!");
    println!("{}", s1);
}
*/
fn main() {
    let s1: String = String::from("hello,");
    let s2: String = String::from("world!");
    let s3: String = s1 + s2.as_str(); //String ->&str NOTE: let s3 = s1+ &s2 can also be used
    assert_eq!(s3, "hello,world!");
    println!("{}", s3); //s3 will be the owner of the string not s1
}
