# Undertanding Ownership

## What is Ownership?

- Ownership is a set of rules that govern how a Rust program manages memory. All programs have to manage the way they use a computer’s memory while running. Some languages have garbage collection that regularly looks for no-longer-used memory as the program runs; in other languages, the programmer must explicitly allocate and free the memory.
- Whereas Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile. None of the features of ownership will slow down your program while it’s running.

### Ownership Rules

1. Each value in Rust has an <i>owner</i>.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

### Variable Scope

A scope is the range within a program for which an item is valid. For example:<br>

```Rust
let s = "hello";
```

The variable `s` here refers to a string literal, where the value of the string is hardcoded into the text of our program. The variable is valid from the point at which it’s declared until the end of the current scope. <br>

```Rust
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid

```

To conclude we can summarise this as following:<br>

- When `s` comes into scope, it is valid.
- It remains valid until it goes out of scope.

### Memory And Allocation

In languages with a garbage collector (GC), the GC keeps track of and cleans up memory that isn’t being used anymore, and we don’t need to think about it. In most languages without a GC, it’s our responsibility to identify when memory is no longer being used and to call code to explicitly free it, just as we did to request it. Doing this correctly has historically been a difficult programming problem. If we forget, we’ll waste memory. If we do it too early, we’ll have an invalid variable. If we do it twice, that’s a bug too. We need to pair exactly one allocate with exactly one free.<br>
But Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope.<br>

```Rust
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }                                  // this scope is now over, and s is no
                                       // longer valid
```

When a variable goes out of scope, Rust calls a special function for us. This function is called `drop`, and it’s where the author of `String` can put the code to return the memory. Rust calls `drop` automatically at the closing curly bracket. This pattern of deallocating resources at the end of an item’s lifetime is similar to C++'s Resource Acquisition Is Initialization (RAII).

### Variables and Data Interacting with Move

Multiple variables can interact with the same data in different ways in Rust

```Rust
    let x = 5;
    let y = x;
```

Here we “bind the value 5 to `x`; then make a copy of the value in `x` and bind it to `y`.” We now have two variables, x and y, and both equal `5`. The x and y are having a value 5 which is now pushed onto the stack.<br>

Now looking at the `String` version:

```Rust
    let s1 = String::from("hello");
    let s2 = s1;

```

One can assume that the way it works would be the same: that is, the second line would make a copy of the value in `s1` and bind it to `s2`.<br>

##### BUT WE ARE WRONG!!!

A `String` is made up of three parts, shown on the left: a pointer to the memory that holds the contents of the string, a length, and a capacity. This group of data is stored on the stack. On the right is the memory on the heap that holds the contents.<br>

![Representation in memory of a String holding the value "hello" bound to s1](Image124.png)

The length is how much memory, in bytes, the contents of the String are currently using. The capacity is the total amount of memory, in bytes, that the String has received from the allocator. For now we can ignore the capacity.<br>
When we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length, and the capacity that are on the stack. We do not copy the data on the heap that the pointer refers to. In other words, the data representation in memory looks like following:
![Representation in memory of the variable s2 that has a copy of the pointer, length, and capacity of s1](<Image 125.png>)

The representation does not look like figure below, which is what memory would look like if Rust instead copied the heap data as well. If Rust did this, the operation s2 = s1 could be very expensive in terms of runtime performance if the data on the heap were large.<br>

![Another possibility for what s2 = s1 might do if Rust copied the heap data as well](<Image 126.png>)

Now in string when s2 and s1 go out of scope, they will both try to free the same memory. This is known as a double free error and is one of the memory safety bugs we mentioned previously. Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.<br>
To ensure memory safety, after the line `let s2 = s1;`, Rust considers `s1` as no longer valid. Therefore, Rust doesn’t need to free anything when s1 goes out of scope.<br>

### Variables and Data Interacting with Clone

If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called `clone`.<br>
Following is an example of the clone method:

```Rust
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

```

<b>Output:</b> s1 = hello, s2 = hello<br>

When we see a call to clone, you know that some arbitrary code is being executed and that code may be expensive. It’s a visual indicator that something different is going on.<br>

### Ownership and Functions

The mechanics of passing a value to a function are similar to those when assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does.<br>

```Rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

If we tried to use `s` after the call to takes_ownership, Rust would throw a compile-time error.

### Return Values And Scope

Returning values can also transfer ownership. Here is an example of a function that returns some value.<br>

```Rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by `drop` unless ownership of the data has been moved to another variable.
