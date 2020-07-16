// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is. 
// Execute `rustlings hint primitive_types3` for hints!

fn main() {
    let a = ["Hello there!"; 10];
    let s: String = a.len().to_string();

    for phrase in a.iter() {
        println!("{}", phrase);
    }

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
