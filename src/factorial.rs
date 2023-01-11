pub fn factorial(num: i64) -> i64 {
    if num == 1 || num == 0 {
        return 1;
    }
    return num * factorial(num - 1);
}
