use std::collections::HashMap;


/// Time complexity: O(n)
/// Space complexity: O(n)
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());

        for i in 0..nums.len() {
            if let Some(k) = map.get(&(target - nums[i])) {
                return vec![*k as i32,  i as i32]
            }
            map.insert(nums[i], i);
        }
        
        return vec![]
    }
}
