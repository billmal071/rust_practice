pub fn reverse_string(s: &str) {
    let reversed_string = s
      .to_lowercase()
      .chars()
      .filter(|c| c.is_alphanumeric())
      .rev()
      .collect::<String>();
    println!("Reversed chars: {}", reversed_string);
    let v: String = s
      .split(' ')
      .rev()
      .collect::<String>();
    println!("Reversed string: {}", &v);
}
