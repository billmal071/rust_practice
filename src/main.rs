// use median_and_mode::median;
// use median_and_mode::mode;

// use reverse_string::reverse_string;
// use anagram::anagram;
// use disemvowel::disemvowel;
use roman_numerals::num_as_roman;

use crate::largest_num::largest_num;

/* mod prime_number; */
/* mod factorial; */
// mod reverse_string;
// mod anagram;
// mod disemvowel;
pub mod largest_num;
mod roman_numerals;
// mod median_and_mode;
/* mod pyg_latin; */

fn main() {
    // prime_number::is_prime_number(1);
    // prime_number::is_prime_number(30);

    // println!("{}", factorial::factorial(4));
    // println!("{}", factorial::factorial(1));
    // println!("{}", factorial::factorial(50));

    // reverse_string("Hello world!!");
    // reverse_string("Let's go home, yes?");
    //
    // anagram(&String::from("Madam"));
    // anagram(&String::from("Able was I 'ere I saw Elba"));

    // println!("{}", disemvowel("This website is for losers LOL!"));
    // let mut even_median = vec![1, 2, 3, 4, 5];
    // let mut odd_numbers = vec![1, 2, 3, 4, 5, 6];
    // println!("{}", median(&mut even_median));
    // println!("{}", median(&mut odd_numbers));
    //
    // let mode1 = vec![1, 1, 1, 2, 2, 2, 3, 3, 4, 4, 5];
    // let mode2 = vec![1, 2, 3, 3, 3, 3, 4, 4, 5, 5];
    // println!("{:?}", mode(&mode1));
    // println!("{:?}", mode(&mode2));

    println!("{:?}", num_as_roman(123));

    println!("largest number is {}", largest_num(&vec![1, 5, 2, 8, 0]));
    println!(
        "largest number is {}",
        largest_num(&vec![2000, 5, 102, 80, 100000])
    );
    println!(
        "largest number is {}",
        largest_num(&vec![10, 50, 20, 8, 40])
    );
    println!(
        "largest number is {}",
        largest_num(&vec![11, 15, 12, 18, 0])
    );
}
