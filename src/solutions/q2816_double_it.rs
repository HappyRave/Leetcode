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
    pub fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode { val: 0, next: head };
        let mut current = &mut dummy;

        while let Some(node) = current.next.as_mut() {
            node.val *= 2;
            if node.val >= 10 {
                node.val -= 10;
                current.val += 1;
            }
            current = node;
        }

        if dummy.val == 0 {
            dummy.next
        } else {
            Some(Box::new(dummy))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_it() {
        let mut list = ListNode::new(1);
        list.next = Some(Box::new(ListNode::new(8)));
        list.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(9)));

        let result = Solution::double_it(Some(Box::new(list)));

        let mut expected = ListNode::new(3);
        expected.next = Some(Box::new(ListNode::new(7)));
        expected.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(8)));

        assert_eq!(result, Some(Box::new(expected)));
    }

    #[test]
    fn test_double_it_2() {
        let mut list = ListNode::new(9);
        list.next = Some(Box::new(ListNode::new(9)));
        list.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(9)));

        let result = Solution::double_it(Some(Box::new(list)));

        let mut expected = ListNode::new(1);
        expected.next = Some(Box::new(ListNode::new(9)));
        expected.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(9)));
        expected.next.as_mut().unwrap().next.as_mut().unwrap().next =
            Some(Box::new(ListNode::new(8)));

        assert_eq!(result, Some(Box::new(expected)));
    }
}
