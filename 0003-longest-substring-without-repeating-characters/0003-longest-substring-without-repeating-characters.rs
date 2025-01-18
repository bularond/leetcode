impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut counter: [i32; 128] = [0; 128];
        let bs = s.as_bytes();
        let n = s.len();
        let (mut left, mut right) = (0, 0);
        let mut ans = 0;

        while right < n {
            let mut pos = 0;
            while right < n && counter[pos] <= 1 {
                pos = bs[right] as usize;
                counter[pos] += 1;
                right += 1;
            }
            if ans < right - left - 1 {
                ans = right - left - 1;
            }

            while left < right && counter[pos] > 1 {
                counter[bs[left] as usize] -= 1;
                left += 1;
            }
            if ans < right - left {
                ans = right - left;
            }
        }

        ans as i32
    }
}