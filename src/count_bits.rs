// keep dividing by 2 until you get o
pub fn num_to_binary(num: i64) -> i64 {
    //  (x >> n) & 1 to convert to binary
    (format!("{num:b}")).matches('1').count() as i64
}

pub fn count_bit(n: i64) -> u32 {
    let binary = num_to_binary(n);
    println!("binary of {} is {}", n, binary);
    let bin = binary.to_string().chars().filter(|&x| x != '0').count() as u32;
    println!("number of ones(1) is: {}", bin);
    // n.count_ones()
    bin
}
