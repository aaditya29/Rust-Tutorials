# Functions in Rust

In rust `fn` keyword is used to declare the function. We have already seen one of the most important functions in the language: the main function, which is the entry point of many programs.<br>

Rust code uses snake case as the conventional style for function and variable names, in which all letters are lowercase and underscores separate words.<br>
<b>For Example:</b>

```Rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

The curly brackets here tell the compiler where the function body begins and ends and we can call any function we’ve defined by entering its name followed by a set of parentheses.<br>

## Parameters In Functions

We can define functions to have parameters, which are special variables that are part of a function’s signature. When a function has parameters, you can provide it with concrete values for those parameters. Technically, the concrete values are called arguments, but in casual conversation, people tend to use the words parameter and argument interchangeably for either the variables in a function’s definition or the concrete values passed in when you call a function.

<b>Adding Parameters in Above Example</b>

```Rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
```

<b>Output:</b> `The value of x is: 5`
