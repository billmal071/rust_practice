fn max_product_of_three_numbers(nums: &Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort();
    let n = nums.len();
    let a = nums[n - 1] * nums[n - 2] * nums[n - 3];
    let b = nums[0] * nums[1] * nums[n - 1];
    if a > b {
        a
    } else {
        b
    }
}
