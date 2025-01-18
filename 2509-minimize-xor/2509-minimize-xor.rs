impl Solution {
    pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
        let num1: usize = num1 as usize;
        let num2: usize = num2 as usize;

        let mut count = num2.count_ones() as usize;
        let mut mask = 1 << 31;
        let mut result = 0;

        while mask > 0 && count > 0 {
            if num1 & mask != 0 {
                result |= mask;
                count -= 1;
            }
            mask >>= 1;
        }
        
        mask = 1;
        while mask <= 1 << 31 && count > 0 {
            if result & mask == 0 {
                result |= mask;
                count -= 1;
            }
            mask <<= 1;
        }

        result as i32
    }
}