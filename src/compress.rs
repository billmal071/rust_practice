use std::collections::HashMap;

pub fn compress(chars: &mut Vec<char>) -> i32 {
    let mut stores = HashMap::new();
    let mut result = Vec::new();
    
    for n in chars.iter() {
        let count = stores.entry(n).or_insert(0);
        *count += 1;
    }
    println!("{stores:?}");
   
    for (key, value) in &stores  {
       if *value > 9 {
           result.push(**key);
           result.extend(value.to_string().chars().collect::<Vec<char>>());
       } else if *value == 1 {
           result.push(**key);
       } else {
           result.extend([**key, *value as u8 as char]);
       } 
    }
    
    result.len() as i32
}

