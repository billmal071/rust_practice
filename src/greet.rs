/// Write a method that takes one argument as name and then greets that name, capitalized and ends with an exclamation point.
/// "riley" --> "Hello Riley!"
/// "JACK"  --> "Hello Jack!"
///
/// # Arguments
///
/// * `name` - a string name
///
/// # Returns
///
/// a string
pub fn greet(name: &str) -> String {
    let first_letter = String::from(&name[0..1]).to_uppercase();
    let rest = String::from(&name[1..]).to_lowercase();
    format!("Hello {first_letter}{rest}!")
}
