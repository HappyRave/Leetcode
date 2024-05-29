// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

use crate::solutions::Solution;

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut left = 1;
        let mut right = n;
        while left < right {
            let mid = left + (right - left) / 2;
            if self.isBadVersion(mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }

    #[allow(non_snake_case)]
    fn isBadVersion(&self, version: i32) -> bool {
        version >= 4
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::Solution;

    #[test]
    fn test_first_bad_version() {
        let solution = Solution {};
        assert_eq!(solution.first_bad_version(5), 4);
    }

    #[test]
    fn test_first_bad_version_2() {
        let solution = Solution {};
        assert_eq!(solution.first_bad_version(1_000_000), 4);
    }
}
