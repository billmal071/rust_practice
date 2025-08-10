pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let mut count = 0;
    // select adjacent pair with the minimum sim in nums, if multiple pairs exist, select the first
    // one
    if nums.len() < 2 {
        return 0;
    }
    if nums.len() == 2 {
        if nums[0] == nums[1] {
            return 1;
        } else {
            return 0;
        }
    }
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] == nums[j] {
                count += 1;
                nums.remove(j);
                nums.remove(i);
                return count + minimum_pair_removal(nums);
            }
        }
    }
    return count;
}
