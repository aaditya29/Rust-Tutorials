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

### Numeric Operations

Rust supports the basic mathematical operations you’d expect for all the number types: addition, subtraction, multiplication, division, and remainder.<br>

```Rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}
```

### The Boolean Type

A Boolean type in Rust has two possible values: `true` and `false`. Booleans are one byte in size. The Boolean type in Rust is specified using bool. <br>
<b>For example:</b>

```Rust
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

### The Character Type

Rust’s char type is the language’s most primitive alphabetic type.<br>
<b>For Example:</b>

```Rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```

We specify `char` literals with single quotes, as opposed to string literals, which use double quotes. <br>
Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII. Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid char values in Rust.<br>

## Compound Types

Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.<br>

### The Tuple Type

A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.<br>

We create a tuple by writing a comma-separated list of values inside parentheses. Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same.<br>
<b>For Example:</b>

```Rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

The variable `tup` binds to the entire tuple because a tuple is considered a single compound element.<br>

To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value.
<b>For Example:</b>

```Rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
```

This program first creates a tuple and binds it to the variable tup. It then uses a pattern with let to take tup and turn it into three separate variables, x, y, and z. This is called `destructuring` because it breaks the single tuple into three parts. Finally, the program prints the value of y, which is 6.4.<br>

We can also access a tuple element directly by using a period (.) followed by the index of the value we want to access.<br>
</b>For Example:</b>

```Rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```
