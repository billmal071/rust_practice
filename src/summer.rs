/*Write a function that takes an array of numbers (integers for the tests) and a target number. It should find two different items in the array that, when added together, give the target value. The indices of these items should then be returned in a tuple / list (depending on your language) like so: (index1, index2).

For the purposes of this kata, some tests may have multiple answers; any valid solutions will be accepted.

The input will always be valid (numbers will be an array of length 2 or greater, and all of the items will be numbers; target will always be the sum of two different items from that array).

/// two_sum(&[1, 2, 3], 4) // return (0, 2) or (2, 0)
*/use std::collections::HashMap;

pub fn two_sum(numbers: &[i32], target: i32) -> (i32, i32) {
    let mut hashmap = HashMap::new();
    for (idx, num) in numbers.iter().enumerate() {
        let remainder = target - num;
        match hashmap.get(&remainder) {
            Some(&j) => return (idx as i32, j),
            None => {
                hashmap.insert(num, idx as i32);
            }
        }
    }
    (0, 0)
}
