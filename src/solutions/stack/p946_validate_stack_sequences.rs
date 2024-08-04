use crate::solutions::Solution;

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack = vec![];
        let mut i = 0;
        for x in pushed {
            stack.push(x);
            while !stack.is_empty() && stack.last().unwrap() == &popped[i] {
                stack.pop();
                i += 1;
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_stack_sequences() {
        let pushed = vec![1, 2, 3, 4, 5];
        let popped = vec![4, 5, 3, 2, 1];
        assert!(Solution::validate_stack_sequences(pushed, popped));
    }

    #[test]
    fn test_validate_stack_sequences_2() {
        let pushed = vec![1, 2, 3, 4, 5];
        let popped = vec![4, 3, 5, 1, 2];
        assert!(!Solution::validate_stack_sequences(pushed, popped));
    }
}
