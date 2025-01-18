use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());
        for (i, v) in nums.iter().enumerate() {
            map.insert(v, i);
        }
        for (i, v) in nums.iter().enumerate() {
            let op_s = map.get(&(target - v));
            if let Some(s) = op_s {
                if *s != i {
                    return vec![*s.min(&i) as i32, *s.max(&i) as i32];
                }
            }
        }

        panic!("unreachable")
    }
}