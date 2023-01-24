use median_and_mode::median;
use median_and_mode::mode;

// use reverse_string::reverse_string;
// use anagram::anagram;
// use disemvowel::disemvowel;
// use roman_numerals::num_as_roman;

/* mod prime_number; */
/* mod factorial; */
// mod reverse_string;
// mod anagram;
// mod disemvowel;
// mod roman_numerals;
mod median_and_mode;

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
  let mut even_median = vec![1, 2, 3, 4, 5];
  let mut odd_numbers = vec![1, 2, 3, 4, 5, 6];
  println!("{}", median(&mut even_median));
  println!("{}", median(&mut odd_numbers));

  let mode1 = vec![1, 1, 1, 2, 2, 2, 3, 3, 4, 4, 5];
  let mode2 = vec![1, 2, 3, 3, 3, 3, 4, 4, 5, 5];
  println!("{:?}", mode(&mode1));
  println!("{:?}", mode(&mode2));
}
