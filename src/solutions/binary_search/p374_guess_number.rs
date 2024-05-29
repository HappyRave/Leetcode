use crate::solutions::Solution;

/**
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return          -1 if num is higher than the picked number
 *                  1 if num is lower than the picked number
 *                  otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

fn guess(i: i32) -> i32 {
    let n = 6;
    match i.cmp(&n) {
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Greater => -1,
        std::cmp::Ordering::Equal => 0,
    }
}

impl Solution {
    #[allow(non_snake_case)]
    unsafe fn guessNumber(n: i32) -> i32 {
        let mut left = 1;
        let mut right = n;
        while left <= right {
            let mid = left + (right - left) / 2;
            match guess(mid) {
                -1 => right = mid - 1,
                1 => left = mid + 1,
                0 => return mid,
                _ => unreachable!(),
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guess_number() {
        unsafe {
            assert_eq!(Solution::guessNumber(10), 6);
        }
    }
}
