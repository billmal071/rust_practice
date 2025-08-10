pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    let mut result = Vec::new();
    let mut inc_arr = [i32; 3];

    // take a range of 3, if i < i+1, j < k, then i < k. keep the variable true, when it is false, increase the begin
    // variable and make the true variable false
    // e.g [1, 4, 2, 0,  3, 6]
    for (index, value) in nums.iter().enumerate() {
        if *value < nums[index + 1] {
            
        }
    } 
}

pub fn contiguous_increasing_subsequence(nums: Vec<i32>) -> bool {
     let mut result = Vec::new();

        let mut idx = 0;
        let slice_size = 2;

        while idx + slice_size <= nums.len() {
            let slice = &nums[idx..idx + slice_size];
            if slice[0] < slice[1] {
                result.push(slice[0]);
            } else {
                result.clear();
            }
            println!("idx = {idx}, slice = {slice:?}, result = {result:?}");
            idx += 1;
        }
        println!("{:?}", result);
        !result.is_empty()
}
