use std::{cmp::Reverse, collections::BinaryHeap};

use crate::solutions::Solution;

impl Solution {
    pub fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
        let mut available_chairs = (0..times.len()).map(Reverse).collect::<BinaryHeap<_>>();
        let mut unavailable_chairs = BinaryHeap::<Reverse<(i32, usize)>>::new();
        let mut times = times.into_iter().enumerate().collect::<Vec<_>>();
        times.sort_unstable_by_key(|x| x.1[0]);
        for (i, time) in times {
            while let Some(&Reverse((end_time, chair))) = unavailable_chairs.peek() {
                if end_time > time[0] {
                    break;
                }
                unavailable_chairs.pop();
                available_chairs.push(Reverse(chair));
            }
            let chair = available_chairs.pop().unwrap().0;
            if i as i32 == target_friend {
                return chair as i32;
            }
            unavailable_chairs.push(Reverse((time[1], chair)));
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_chair() {
        let times = vec![vec![1, 4], vec![2, 3], vec![4, 6]];
        let target_friend = 1;
        let res = 1;
        assert_eq!(Solution::smallest_chair(times, target_friend), res);
    }

    #[test]
    fn test_smallest_chair_2() {
        let times = vec![vec![3, 10], vec![1, 5], vec![2, 6]];
        let target_friend = 0;
        let res = 2;
        assert_eq!(Solution::smallest_chair(times, target_friend), res);
    }
}
