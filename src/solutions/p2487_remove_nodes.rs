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
    pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            Some(mut node) => match Self::remove_nodes(node.next) {
                Some(next) if next.val > node.val => Some(next),
                next => {
                    node.next = next;
                    Some(node)
                }
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_nodes() {
        // [5,2,13,3,8]
        let node8 = ListNode::new(8);
        let mut node3 = ListNode::new(3);
        node3.next = Some(Box::new(node8));
        let mut node13 = ListNode::new(13);
        node13.next = Some(Box::new(node3));
        let mut node2 = ListNode::new(2);
        node2.next = Some(Box::new(node13));
        let mut node5 = ListNode::new(5);
        node5.next = Some(Box::new(node2));

        let head = node5;

        // [13,8]
        let node8 = ListNode::new(8);
        let mut node13 = ListNode::new(13);
        node13.next = Some(Box::new(node8));

        let solution = node13;

        let result = Solution::remove_nodes(Some(Box::new(head)));
        assert_eq!(*result.unwrap().as_ref(), solution);
    }
}
