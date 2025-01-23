impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut horizontal = vec![0; n];
        let mut vertical = vec![0; m];
    
        for (x, line) in grid.iter().enumerate() {
            for (y, val) in line.iter().enumerate() {
                if *val == 1 {
                    horizontal[x] += 1;
                    vertical[y] += 1;
                }
            }
        }
    
        let mut res = 0;
        for (x, line) in grid.iter().enumerate() {
            for (y, val) in line.iter().enumerate() {
                if *val == 1 && (horizontal[x] != 1 || vertical[y] != 1) {
                    res += 1;
                }
            }
        }
    
        res
    }
}