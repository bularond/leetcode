impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        let m = mat[0].len();
    
        let mut positions = vec![(0, 0); n * m];
        for (x, line) in mat.iter().enumerate() {
            for (y, val) in line.iter().enumerate() {
                positions[(val - 1) as usize] = (x, y);
            }
        }
        let mut horisontal = vec![m; n];
        let mut vertical = vec![n; m];
    
        for (i, val) in arr.iter().enumerate() {
            let val = (val - 1) as usize;
            let (x, y) = positions[val];
            horisontal[x] -= 1;
            vertical[y] -= 1;
            if horisontal[x] == 0 || vertical[y] == 0 {
                return i as i32;
            }
        }
    
        unreachable!()
    }
}