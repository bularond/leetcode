impl Solution {
    pub fn winning_player(x: i32, mut y: i32) -> String {
        if x.min(y / 4) % 2 == 0 {
            String::from("Bob")
        } else {
            String::from("Alice")
        }
    }
}