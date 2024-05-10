use crate::solutions::Solution;

use super::ListNode;

impl Solution {
    pub fn is_palindrome_list(head: Option<Box<ListNode>>) -> bool {
        let mut values = Vec::new();
        let mut current = head;
        while let Some(node) = current {
            values.push(node.val);
            current = node.next;
        }
        let mut left = 0;
        let mut right = values.len() - 1;
        while left < right {
            if values[left] != values[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {

    use crate::solutions::linked_lists::ListExt;

    use super::*;

    #[test]
    fn test_is_palindrome_list() {
        assert!(Solution::is_palindrome_list(vec![1, 2, 2, 1].into_list()));
        assert!(!Solution::is_palindrome_list(vec![1, 2, 3, 1].into_list()));
    }
}
