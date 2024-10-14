use crate::solutions::Solution;

impl Solution {
    pub fn max_kelements(nums: Vec<i32>, mut k: i32) -> i64 {
        let mut heap = std::collections::BinaryHeap::from(nums);
        let mut res = 0_i64;
        while k > 0 {
            let num = heap.pop().unwrap();
            res += num as i64;
            heap.push((num + 2) / 3);
            k -= 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_kelements() {
        let nums = vec![10, 10, 10, 10, 10];
        let k = 5;
        let res = 50;
        assert_eq!(Solution::max_kelements(nums, k), res);
    }

    #[test]
    fn test_max_kelements_2() {
        let nums = vec![1, 10, 3, 3, 3];
        let k = 3;
        let res = 17;
        assert_eq!(Solution::max_kelements(nums, k), res);
    }

    #[test]
    fn test_max_kelements_3() {
        let nums = vec![
            756902131, 995414896, 95906472, 149914376, 387433380, 848985151,
        ];
        let k = 6;
        let res = 3603535575;
        assert_eq!(Solution::max_kelements(nums, k), res);
    }
}
