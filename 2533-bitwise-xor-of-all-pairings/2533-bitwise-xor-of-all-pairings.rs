impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut result = 0;
        if nums1.len() % 2 == 1 {
            for num in nums2.iter() {
                result ^= num;
            }
        }
        if nums2.len() % 2 == 1 {
            for num in nums1.iter() {
                result ^= num;
            }
        }
        result
    }
}