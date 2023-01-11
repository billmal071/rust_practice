/* mod prime_number; */
mod factorial;

fn main() {
    // prime_number::is_prime_number(1);
    // prime_number::is_prime_number(30);
    println!("{}", factorial::factorial(4));
    println!("{}", factorial::factorial(1));
    println!("{}", factorial::factorial(50));
}
