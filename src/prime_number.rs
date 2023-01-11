pub fn is_prime_number(number: u64) {
    // numbers divisble by only one and itself.
    // loop through and check the numbers that can divide it, if a number apart from 1 and itself
    // can divide it, you can break and return false
    let mut condition: bool = true;
    if number < 1 {
        condition = false;
        println!("{number} is not a prime number");
    }
    if number == 1 {
        condition = false;
        println!("{number} is not a prime number");
    }
    // divide number by 2 since you reach half by then
    let dividend = number / 2;
    for num in 2..dividend {
        if number % num == 0 {
            condition = false;
            println!("{number} is not a prime number");
            break;
        }
    }
    if condition == false {
        println!("{number} is not a prime number {condition}");
    } else {
        println!("{number} is a prime number {condition}")
    }
}
