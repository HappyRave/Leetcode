use crate::solutions::Solution;

// cannot use built-in functions

impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        fn merge_sort(nums: &mut Vec<i32>, start: usize, end: usize) {
            if start >= end {
                return;
            }
            let mid = start + (end - start) / 2;
            merge_sort(nums, start, mid);
            merge_sort(nums, mid + 1, end);
            let mut temp = vec![0; end - start + 1];
            let (mut i, mut j, mut k) = (start, mid + 1, 0);
            while i <= mid && j <= end {
                if nums[i] < nums[j] {
                    temp[k] = nums[i];
                    i += 1;
                } else {
                    temp[k] = nums[j];
                    j += 1;
                }
                k += 1;
            }
            while i <= mid {
                temp[k] = nums[i];
                i += 1;
                k += 1;
            }
            while j <= end {
                temp[k] = nums[j];
                j += 1;
                k += 1;
            }
            (0..temp.len()).for_each(|i| {
                nums[start + i] = temp[i];
            });
        }
        let n = nums.len();
        if n != 0 {
            merge_sort(&mut nums, 0, n - 1);
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_array() {
        assert_eq!(Solution::sort_array(vec![5, 2, 3, 1]), vec![1, 2, 3, 5]);
    }

    #[test]
    fn test_sort_array_2() {
        assert_eq!(Solution::sort_array(vec![]), vec![]);
    }

    #[test]
    fn test_sort_array_3() {
        assert_eq!(Solution::sort_array(vec![1]), vec![1]);
    }

    #[test]
    fn test_sort_array_4() {
        assert_eq!(
            Solution::sort_array(vec![5, 1, 1, 2, 0, 0]),
            vec![0, 0, 1, 1, 2, 5]
        );
    }

    #[test]
    fn test_sort_array_5() {
        let v = (0..=50000).collect::<Vec<i32>>();
        let answer = v.clone();
        assert_eq!(Solution::sort_array(v), answer);
    }
}
