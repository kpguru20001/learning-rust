// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

fn get_array() -> Vec<i32> {
    let mut v = Vec::new();
    for i in 1..=1001 {
        v.push(i)
    }
    return v;
}

fn main() {
    let a = get_array();

    if a.len() >= 100 {
        println!("Wow, that's a big array! {}",a.len());
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}
