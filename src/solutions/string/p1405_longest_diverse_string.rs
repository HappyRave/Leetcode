use std::collections::BinaryHeap;

use crate::solutions::Solution;

impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut total = a + b + c;
        let mut res = String::with_capacity((total) as usize);
        let mut heap = BinaryHeap::new();

        if a > 0 {
            heap.push((a, 'a'));
        }
        if b > 0 {
            heap.push((b, 'b'));
        }
        if c > 0 {
            heap.push((c, 'c'));
        }

        let mut last = None;

        while let Some((mut count, ch)) = heap.pop() {
            res.push(ch);
            count -= 1;
            total -= 1;

            if count > (total - count) * 2 {
                res.push(ch);
                count -= 1;
                total -= 1;
            }

            if let Some(last) = last.take() {
                heap.push(last);
            }

            if count > 0 {
                last = Some((count, ch));
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_diverse_string() {
        assert_eq!(
            Solution::longest_diverse_string(1, 1, 7),
            "ccbccacc".to_string()
        );
    }

    #[test]
    fn test_longest_diverse_string_2() {
        assert_eq!(
            Solution::longest_diverse_string(7, 1, 0),
            "aabaa".to_string()
        );
    }
}
