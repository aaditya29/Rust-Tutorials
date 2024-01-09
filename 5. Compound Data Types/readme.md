# Compound Data Types

## Strings

There are two types of strings in Rust: `String` and `&str`.<br>

A `String` is stored as a vector of bytes (Vec<u8>), but guaranteed to always be a valid UTF-8 sequence. String is heap allocated, growable and not null terminated.<br>

Where `&str` is a slice (&[u8]) that always points to a valid UTF-8 sequence, and can be used to view into a String, just like &[T] is a view into Vec<T>.<br>

### String Literals

There are multiple ways to write string literals with special characters in them. All result in a similar &str so it's best to use the form that is the most convenient to write. Similarly there are multiple ways to write byte string literals, which all result in &[u8; N].<br>
