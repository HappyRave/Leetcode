use crate::solutions::Solution;

impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let distinct = nums.iter().collect::<std::collections::HashSet<_>>().len();
        nums.iter()
            .enumerate()
            .fold(
                (0, 0, distinct, std::collections::HashMap::new()),
                |(mut left, mut count, distinct, mut map), (i, &num)| {
                    map.entry(num).and_modify(|v| *v += 1).or_insert(1);
                    while map.len() == distinct {
                        count += nums.len() - i;
                        map.entry(nums[left]).and_modify(|v| *v -= 1);
                        if map[&nums[left]] == 0 {
                            map.remove(&nums[left]);
                        }
                        left += 1;
                    }
                    (left, count, distinct, map)
                },
            )
            .1 as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::ArrayStringExt;

    use super::*;

    #[test]
    fn test_count_complete_subarrays() {
        let nums = "[1,3,1,2,2]".to_vec();
        let result = 4;
        assert_eq!(Solution::count_complete_subarrays(nums), result);
    }

    #[test]
    fn test_count_complete_subarrays_2() {
        let nums = "[5,5,5,5]".to_vec();
        let result = 10;
        assert_eq!(Solution::count_complete_subarrays(nums), result);
    }
}
