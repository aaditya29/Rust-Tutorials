# Compound Data Types

## Strings

There are two types of strings in Rust: `String` and `&str`.<br>

A `String` is stored as a vector of bytes (Vec<u8>), but guaranteed to always be a valid UTF-8 sequence. String is heap allocated, growable and not null terminated.<br>

Where `&str` is a slice (&[u8]) that always points to a valid UTF-8 sequence, and can be used to view into a String, just like &[T] is a view into Vec<T>.<br>

### String Literals

There are multiple ways to write string literals with special characters in them. All result in a similar &str so it's best to use the form that is the most convenient to write. Similarly there are multiple ways to write byte string literals, which all result in &[u8; N].<br>

## Arrays

An array is a collection of objects of the same type T, stored in contiguous memory. Arrays are created using brackets [], and their length, which is known at compile time, is part of their type signature [T; length].<br>

### Primitive Type Array

It is a fixed-size array, denoted `[T; N]`, for the element type, `T`, and the non-negative compile-time constant size, `N`.<br>
There are two syntactic forms for creating an array:

- A list with each element, i.e., [x, y, z].
- A repeat expression [expr; N] where N is how many times to repeat expr in the array. expr must either be: - A value of a type implementing the Copy trait - A const value
  - Note that [expr; 0] is allowed, and produces an empty array.
  - This will still evaluate expr, however, and immediately drop the resulting value, so be mindful of side effects.<br>

Arrays of any size implement the following traits if the element type allows it:

- Copy
- Clone
- Debug
- IntoIterator (implemented for [T; N], &[T; N] and &mut [T; N])
- PartialEq, PartialOrd, Eq, Ord
- Hash
- AsRef, AsMut
- Borrow, BorrowMut

#### For Example:

```Rust
let mut array: [i32; 3] = [0; 3];

array[1] = 1;
array[2] = 2;

assert_eq!([1, 2], &array[1..]);

// This loop prints: 0 1 2
for x in array {
    print!("{x} ");
}
```

## Structs

> A struct, or structure, is a custom data type that lets us package together and name multiple related values that make up a meaningful group. If you’re familiar with an object-oriented language, a struct is like an object’s data attributes.

### Defining and Instantiating Structs

Structs are similar to tuples. They both hold multiple related values. Like tuples, the pieces of a struct can be different types. Unlike with tuples, in a struct we’ll name each piece of data so it’s clear what the values mean. Adding these names means that structs are more flexible than tuples: we don’t have to rely on the order of the data to specify or access the values of an instance.<br>
