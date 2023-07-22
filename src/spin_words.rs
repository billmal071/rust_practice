/**
* Spin only the words that are more than 5
*/
pub fn spin_words(words: &str) -> String {
    let mut spun_words: Vec<&str> = words.split(' ').collect();
    let mut result = String::new();

    // for word in spun_words {
    //     println!("{}", word);
    //     if word.len() >= 5 {
    //         result.push_str(&format!("{}", word.chars().rev().collect::<String>()));
    //     } else {
    //         result.push_str(&format!("{}", word));
    //     }
    // }
    // result

    spun_words
        .iter()
        .map(|word| {
            if word.len() >= 5 {
                word.chars().rev().collect::<String>()
            } else {
                word.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}
