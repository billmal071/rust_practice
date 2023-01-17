use std::cmp::Ordering;

pub fn anagram(word: &String) {
  let filtered_word = word
    .to_lowercase()
    .chars()
    .filter(|c| c.is_alphanumeric())
    .collect::<String>();

  let reversed_string = &filtered_word
    .chars()
    .rev()
    .collect::<String>();

  match String::from(&filtered_word).cmp(&reversed_string) {
    Ordering::Less => println!("{} doesn't match {}", &filtered_word, &reversed_string),
    Ordering::Equal => println!("{} matches {}", &filtered_word, &reversed_string),
    Ordering::Greater => println!("{} doesn't match {}", &filtered_word, &reversed_string)
  }
}