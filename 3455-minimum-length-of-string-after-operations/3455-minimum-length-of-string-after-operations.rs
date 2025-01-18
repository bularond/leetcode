impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let mut exist: i32 = 0;
        let mut odds: i32 = 0;
        for c in s.bytes() {
            let pos = c - b'a';
            exist |= 1 << pos;
            odds ^= 1 << pos;
        }
        odds ^= exist;
        (exist.count_ones() + odds.count_ones()) as i32
    }
}