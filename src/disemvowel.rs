pub fn disemvowel(s: &str) -> String {
  let vowels = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
  let s = s
    .chars()
    .filter(|c| !vowels.contains(c))
    .collect::<String>();
  s.to_string()
}