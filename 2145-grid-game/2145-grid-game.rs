
impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let n = grid[0].len();
        let sum1: i64 = grid[0].iter().map(|x| *x as i64).sum();
    
        let mut cur1 = sum1;
        let mut cur2 = 0;
        let mut min = i64::MAX;
        for i in 0..n {
            cur1 -= grid[0][i] as i64;
            min = min.min(cur1.max(cur2));
            cur2 += grid[1][i] as i64;
        }
    
        min
    }
}
