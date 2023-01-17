use reverse_string::reverse_string;
use anagram::anagram;

/* mod prime_number; */
/* mod factorial; */
mod reverse_string;
mod anagram;

fn main() {
    // prime_number::is_prime_number(1);
    // prime_number::is_prime_number(30);
    // println!("{}", factorial::factorial(4));
    // println!("{}", factorial::factorial(1));
    // println!("{}", factorial::factorial(50));
    reverse_string("Hello world!!");
    reverse_string("Let's go home, yes?");

    anagram(&String::from("Madam"));
    anagram(&String::from("Able was I 'ere I saw Elba"));
}
