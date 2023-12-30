# Basic Data Types

Every value in Rust is of a certain data type, which tells Rust what kind of data is being specified so it knows how to work with that data. <br>
In `rust`, there are two data type subsets: scalar and compound.<br>
<b>Note:</b>Rust is a `statically typed language`, which means that it must know the types of all variables at compile time. The compiler can usually infer what type we want to use based on the value and how we use it.<br>

In cases when many types are possible, such as when we converted a `String` to a numeric type using `parse` we must add a type annotation, like this:

```Rust
let guess: u32 = "42".parse().expect("Not a number!");
```

If we don’t add the : u32 type annotation shown in the preceding code, Rust will display the error.<br>

## Scalar Types

A scalar type represents a single value.<br>
Rust has four primary scalar types: <b>integers, floating-point numbers, Booleans, and characters</b>.<br>

### Integer Types

An integer is a number without a fractional component.<br>

### Floating Data Types

Rust also has two primitive types for floating-point numbers, which are numbers with decimal points. Rust’s floating-point types are `f32` and `f64`, which are 32 bits and 64 bits in size, respectively.

Following is an example that shows floating-point numbers in action:

```Rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```
