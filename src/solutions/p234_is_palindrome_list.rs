use super::Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

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
    use super::*;

    #[test]
    fn test_is_palindrome_list() {
        let mut node1 = ListNode::new(1);
        let mut node2 = ListNode::new(2);
        let mut node3 = ListNode::new(2);
        let node4 = ListNode::new(1);
        node3.next = Some(Box::new(node4));
        node2.next = Some(Box::new(node3));
        node1.next = Some(Box::new(node2));
        assert_eq!(Solution::is_palindrome_list(Some(Box::new(node1))), true);

        let mut node1 = ListNode::new(1);
        let mut node2 = ListNode::new(2);
        let mut node3 = ListNode::new(3);
        let node4 = ListNode::new(1);
        node3.next = Some(Box::new(node4));
        node2.next = Some(Box::new(node3));
        node1.next = Some(Box::new(node2));
        assert_eq!(Solution::is_palindrome_list(Some(Box::new(node1))), false);
    }
}
