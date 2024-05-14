use crate::solutions::Solution;

impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![];
        let mut start_map = std::collections::HashMap::new();
        let mut start_list = vec![];
        intervals.iter().enumerate().for_each(|(i, interval)| {
            start_map.insert(interval[0], i);
            start_list.push(interval[0]);
        });
        start_list.sort();
        intervals.iter().for_each(|interval| {
            let end = interval[1];
            let index = match start_list.binary_search(&end) {
                Ok(i) => i as i32,
                Err(i) => i as i32,
            };
            if index == start_list.len() as i32 {
                result.push(-1);
            } else {
                result.push(*start_map.get(&start_list[index as usize]).unwrap() as i32);
            }
        });
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_right_interval() {
        assert_eq!(Solution::find_right_interval(vec![vec![1, 2]]), vec![-1]);
    }

    #[test]
    fn test_find_right_interval_2() {
        assert_eq!(
            Solution::find_right_interval(vec![vec![3, 4], vec![2, 3], vec![1, 2]]),
            vec![-1, 0, 1]
        );
    }

    #[test]
    fn test_find_right_interval_3() {
        assert_eq!(
            Solution::find_right_interval(vec![vec![1, 4], vec![2, 3], vec![3, 4]]),
            vec![-1, 2, -1]
        );
    }
}
