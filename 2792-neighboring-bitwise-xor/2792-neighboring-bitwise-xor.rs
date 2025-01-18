impl Solution {
    pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
        let mut res = 0;
        for i in derived {
            res ^= i;
        }
        res == 0
    }
}