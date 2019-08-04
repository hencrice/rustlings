// test4.rs
// This test covers the sections:
// - Modules
// - Macros

// Write a macro that passes the test! No hints this time, you can do it!

macro_rules! my_macro {
    ($val:expr) => {
        {
            let mut owned_string: String = "Hello ".to_owned();
            owned_string.push_str($val);
            owned_string
        }
    };
}

fn main() {
    if my_macro!("world!") != "Hello world!" {
        panic!("Oh no! Wrong output!");
    }
}
