pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut left = s.chars().collect::<Vec<_>>();
    left.sort();
    let mut right = t.chars().collect::<Vec<_>>();
    right.sort();
    left.eq(&right)
}
