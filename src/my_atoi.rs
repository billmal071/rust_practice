pub fn my_atoi(s: String) -> i32 {
    let mut result = String::new();
    let mut sign = "+";
    for (idx, n) in s.trim().chars().enumerate() {
        if n.is_alphabetic() || n == '.' || n.is_whitespace() {
            break;
        }
        if idx != 0 && (n == '-' || n == '+') {
            break;
        }
        if result.is_empty() && n == '0' {
            continue;
        }
        if idx == 0 && n == '-' {
            sign = "-"; 
        }
        if n.is_ascii_digit() {
             result.push(n);   
        }
    }
    result.insert_str(0, sign);

    if result.len() > 11 {
        if result.trim().starts_with('-') {
            return i32::MIN;
        } else {
            return i32::MAX;
        }
    }
    
    match result.parse::<i128>().unwrap_or_default() {
        t if t > i32::MAX as i128 => i32::MAX,
        t if t < i32::MIN as i128 => i32::MIN,
        t => t as i32,
    }
}




fn parse_to_clamped_i32(s: String) -> i32 {
    let result: String = s
        .chars()
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| c.is_ascii_digit() || *c == '-')
        .collect();

    match result.parse::<i64>().unwrap_or_default() {
        t if t > i32::MAX as i64 => i32::MAX,
        t if t < i32::MIN as i64 => i32::MIN,
        t => t as i32,
    }
}
