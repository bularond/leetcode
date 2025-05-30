use std::collections::VecDeque;

impl Solution {
    pub fn highest_peak(mut is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = is_water.len();
        let m = is_water[0].len();
        let mut bfs = VecDeque::<(usize, usize, i32)>::new();
    
        for i in 0..n {
            for j in 0..m {
                if is_water[i][j] == 1 {
                    is_water[i][j] = 0;
                    bfs.push_back((i, j, 0));
                } else {
                    is_water[i][j] = -1;
                }
            }
        }
    
        while !bfs.is_empty() {
            let (i, j, val) = bfs.pop_front().unwrap();
            if i > 0 && is_water[i - 1][j] == -1 {
                is_water[i - 1][j] = val + 1;
                bfs.push_back((i - 1, j, val + 1));
            }
            if i < n - 1 && is_water[i + 1][j] == -1 {
                is_water[i + 1][j] = val + 1;
                bfs.push_back((i + 1, j, val + 1));
            }
            if j > 0 && is_water[i][j - 1] == -1 {
                is_water[i][j - 1] = val + 1;
                bfs.push_back((i, j - 1, val + 1));
            }
            if j < m - 1 && is_water[i][j + 1] == -1 {
                is_water[i][j + 1] = val + 1;
                bfs.push_back((i, j + 1, val + 1));
            }
        }
    
        is_water
    }
}