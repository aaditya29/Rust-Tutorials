# Variables In Rust

- In rust variables are immutable. When a variable is immutable, once a value is bound to a name, you can’t change that value.
- Assigned using `let` keywords
- Scope of variables is block in which it is declared.
- Shadowing allows variables to be <b>redeclared</b> in the same scope with the same name.
- Although variables are immutable by default, we can make them mutable by adding `mut` in front of the variables. Adding mut also conveys intent to future readers of the code by indicating that other parts of the code will be changing this variable’s value.

```Rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

#### Output:

The value of x is: 5
The value of x is: 6
