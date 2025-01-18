use std::collections::{BinaryHeap, HashSet};
use std::cmp::Reverse;

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
    
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0, 0, 0)));
        let mut been: Vec<Vec<bool>> = vec![vec![false; n]; m];
    
        loop {
            let (cost, x, y) = heap.pop().unwrap().0;
            if (x, y) == (m - 1, n - 1) {
                return cost;
            }
            if been[x][y] {
                continue;
            }
            been[x][y] = true;
    
            let plus_y_cost = cost + if grid[x][y] == 1 { 0 } else { 1 };
            let minus_y_cost = cost + if grid[x][y] == 2 { 0 } else { 1 };
            let plus_x_cost = cost + if grid[x][y] == 3 { 0 } else { 1 };
            let minus_x_cost = cost + if grid[x][y] == 4 { 0 } else { 1 };
    
            if x > 0 && !been[x - 1][y] {
                heap.push(Reverse((minus_x_cost, x - 1, y)));
            }
            if x < m - 1 && !been[x + 1][y] {
                heap.push(Reverse((plus_x_cost, x + 1, y)));
            }
            if y > 0 && !been[x][y - 1] {
                heap.push(Reverse((minus_y_cost, x, y - 1)));
            }
            if y < n - 1 && !been[x][y + 1] {
                heap.push(Reverse((plus_y_cost, x, y + 1)));
            }
        }
    }
}