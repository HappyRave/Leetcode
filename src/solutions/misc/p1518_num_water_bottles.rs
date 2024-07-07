use crate::solutions::Solution;

impl Solution {
    pub fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
        let mut result = 0;
        let mut empty_bottles = 0;
        while num_bottles + empty_bottles >= num_exchange {
            result += num_bottles;
            empty_bottles += num_bottles;
            num_bottles = empty_bottles / num_exchange;
            empty_bottles %= num_exchange;
        }
        result + num_bottles
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_water_bottles() {
        assert_eq!(Solution::num_water_bottles(9, 3), 13);
    }

    #[test]
    fn test_num_water_bottles_2() {
        assert_eq!(Solution::num_water_bottles(15, 4), 19);
    }
}
