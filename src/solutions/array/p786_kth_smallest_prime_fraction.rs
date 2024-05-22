use crate::solutions::Solution;

impl Solution {
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let n = arr.len();
        let mut low = 0.0;
        let mut high = 1.0;
        let mut mid: f64;
        let mut p = 0;
        let mut q = 1;
        while high - low > 1e-9 {
            mid = (low + high) / 2.0;
            let mut count = 0;
            let mut j = 1;
            let mut max = 0.0;
            for i in 0..n {
                while j < n && arr[i] as f64 > arr[j] as f64 * mid {
                    j += 1;
                }
                if j == n {
                    break;
                }
                count += n - j;
                let val = arr[i] as f64 / arr[j] as f64;
                if val > max {
                    max = val;
                    p = i;
                    q = j;
                }
            }
            match (count as i32).cmp(&k) {
                std::cmp::Ordering::Equal => break,
                std::cmp::Ordering::Less => low = mid,
                std::cmp::Ordering::Greater => high = mid,
            }
        }
        vec![arr[p], arr[q]]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth_smallest_prime_fraction() {
        assert_eq!(
            Solution::kth_smallest_prime_fraction(vec![1, 2, 3, 5], 3),
            vec![2, 5]
        );
        assert_eq!(
            Solution::kth_smallest_prime_fraction(vec![1, 7], 1),
            vec![1, 7]
        );
    }
}
