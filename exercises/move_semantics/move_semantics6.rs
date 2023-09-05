// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.



fn main() {
    let data = "Rust is great!".to_string();

    string_uppercase(&data); // Pass a reference to data
}

// Takes ownership and returns the last char
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Borrows the data and converts it to uppercase
fn string_uppercase(data: &String) {
    let upper = data.to_uppercase();
    println!("{}", upper);
}
