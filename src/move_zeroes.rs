pub fn move_zeroes(nums: &mut Vec<i32>) {
    // move the zero to the end basicallly
    // how to swap?
    let mut non_zero_index = 0;
    // Move all non-zero elements to the front
    for i in 0..nums.len() {
        if nums[i] != 0 {
            nums.swap(non_zero_index, i);
            non_zero_index += 1;
        }
    }
}
