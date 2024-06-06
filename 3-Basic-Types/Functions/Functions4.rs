/*
Diverging Functions

Diverging functions never return to the caller, so they may be used in places where a value of any type is expected.



fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };

    Rather than returning a None, we use a diverging function instead
    never_return_fn()
}

IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! {

}

*/

fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    //taking arguments of type u8 and returning Option i32
    match tp {
        //match is similar to switch block.
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };

    // Rather than returning a None, we use a diverging function instead
    never_return_fn()
}

// IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! {
    panic!() // unimplemented() and todo() can also be used
}
