use std::collections::HashMap;

pub fn median(numbers: &mut Vec<i32>) -> f64 {
  let mid = &numbers.len() / 2;
  numbers.sort();
  if &numbers.len() % 2 == 0 {
    ((numbers[mid - 1] + numbers[mid]) as f64 / 2.0) as f64
  } else {
    numbers[mid] as f64
  }
}

pub fn mode(numbers: &Vec<i32>) -> i32 {
  // let v = std::iter::repeat(5).take(5).collect::<Vec<i32>>();
  let mut hash_map: HashMap<i32, i32> = HashMap::new();
  for num in numbers {
    if hash_map.get(num).is_some() {
      hash_map.insert(*num, hash_map.get(num).copied().unwrap()+1);
    } else {
      hash_map.insert(*num, 1);
    };
  }
  *hash_map.values().max().unwrap_or(&0)
}