use std::collections::HashSet;

impl Solution {
    pub fn find_the_prefix_common_array(mut a: Vec<i32>, mut b: Vec<i32>) -> Vec<i32> {
        let n: i32 = a.len() as i32;
        let mut c = Vec::<i32>::with_capacity(n as usize);
        let mut ans = 0;
        for i in 0..a.len() {
            let mut a_value = a[i];
            let mut b_value = b[i];
            a_value = if a_value - 1 >= n {
                a_value - n
            } else {
                a_value
            };
            b_value = if b_value - 1 >= n {
                b_value - n
            } else {
                b_value
            };

            if b[a_value as usize - 1] - 1 >= n{
                ans += 1;
            }
            if a[b_value as usize - 1] - 1 >= n{
                ans += 1;
            }
            if a_value == b_value {
                ans += 1;
            }

            a[a_value as usize - 1] += n;
            b[b_value as usize - 1] += n;

            c.push(ans);
        }

        c
    }
}
