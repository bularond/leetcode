fn sum(arr: &[i64], l: usize, r: usize) -> i64 {
    if r == 0 || l >= r {
        0
    } else if l == 0 {
        arr[r - 1]
    } else {
        arr[r - 1] - arr[l - 1]
    }
}

impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {    
        let n = grid[0].len();
        let mut first_sum = vec![0_i64; n];
        let mut second_sum = vec![0_i64; n];
        first_sum[0] = grid[0][0] as i64;
        second_sum[0] = grid[1][0] as i64;
        for i in 1..n {
            first_sum[i] = first_sum[i - 1] + grid[0][i] as i64;
            second_sum[i] = second_sum[i - 1] + grid[1][i] as i64;
        }
    
        let mut min = i64::MAX;
        for i in 0..n {
            min = min.min(sum(&first_sum, i + 1, n).max(sum(&second_sum, 0, i)));
        }
    
        min
    }
}
