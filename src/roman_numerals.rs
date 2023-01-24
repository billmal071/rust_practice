use std::collections::HashMap;

pub fn num_as_roman(num: i32) -> String {
  let roman_numerals: HashMap<u8, &str> = HashMap::from_iter(
    vec![(1, "I"), (4, "IV"), (5, "V"), (9, "IX"), (10, "X"), (40, "IL"), (50, "L"), (90, "XC"), (100, "C"), (400, "CD"), (500, "D"), (900, "CM"), (1000, "M")]
      .into_iter()
  );

  num.to_string()
}