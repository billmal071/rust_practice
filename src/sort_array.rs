pub fn sort_array(arr: &[i32]) -> Vec<i32> {
    let mut odd_numbers: Vec<i32> = arr.iter().filter(|&x| x % 2 != 0).copied().collect();
    odd_numbers.sort();
    let mut odd_numbers_iter = odd_numbers.iter();
    arr.iter()
        .map(|&x| {
            if x % 2 != 0 {
                *odd_numbers_iter.next().unwrap()
            } else {
                x
            }
        })
        .collect()
}
