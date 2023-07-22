use std::collections::HashMap;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut container: HashMap<i32, i32> = HashMap::new();
    for n in nums {
        if container.insert(n, n).is_some() {
            return true
        }
    }
    false
}
