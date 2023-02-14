/**
*The goal of this exercise is to convert a string to a new string where each character in the new string is "(" if that character appears only once in the original string, or ")" if that character appears more than once in the original string. Ignore capitalization when determining if a character is a duplicate.
*/
pub fn duplicate_encode(word: &str) -> String {
    let word = &word.to_lowercase();
    let mut result = String::from("");
    for c in word.chars() {
        if word.chars().filter(|x| x == &c).count() >= 2 {
           result.push_str(")") 
        } else {
            result.push_str("(")
        }
    }
    result
}

// try with match later, 2, 1, then error or something
