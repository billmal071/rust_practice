// keep dividing by 2 until you get o
pub fn num_to_binary(num: i64) -> i64 {
    (format!("{num:b}")).parse::<i64>().unwrap()
}

pub fn count_bit(n: i64) -> u32 {
    let binary = num_to_binary(n);
    println!("binary of {} is {}", n, binary);
    let bin = binary
        .to_string()
        .chars()
        .filter(|x| x.to_digit(10).unwrap() != 0)
        .collect::<String>()
        .len();
    println!("number of ones(1) is: {}", bin);
    // n.count_ones()
    bin.try_into().unwrap()
}
