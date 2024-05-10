use crate::solutions::Solution;

use super::ListNode;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode { val: 0, next: None };
        let (mut l1, mut l2, mut current, mut carry) = (l1, l2, &mut dummy, 0);

        while l1.is_some() || l2.is_some() {
            let sum = carry
                + l1.as_ref().map_or(0, |node| node.val)
                + l2.as_ref().map_or(0, |node| node.val);
            carry = sum / 10;
            current.next = Some(Box::new(ListNode::new(sum % 10)));
            current = current.next.as_mut().unwrap();
            l1 = l1.and_then(|node| node.next);
            l2 = l2.and_then(|node| node.next);
        }

        if carry > 0 {
            current.next = Some(Box::new(ListNode::new(carry)));
        }

        dummy.next
    }
}

#[cfg(test)]
mod tests {

    use crate::solutions::linked_lists::ListExt;

    use super::*;

    #[test]
    fn test_add_two_numbers() {
        let list1 = vec![2, 4, 3].into_list();
        let list2 = vec![5, 6, 4].into_list();
        let expected = vec![7, 0, 8].into_list();
        let result = Solution::add_two_numbers(list1, list2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_add_two_numbers_2() {
        let list1 = vec![0].into_list();
        let list2 = vec![0].into_list();
        let expected = vec![0].into_list();
        let result = Solution::add_two_numbers(list1, list2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_add_two_numbers_3() {
        let list1 = vec![9, 9, 9, 9, 9, 9, 9].into_list();
        let list2 = vec![9, 9, 9, 9].into_list();
        let expected = vec![8, 9, 9, 9, 0, 0, 0, 1].into_list();
        let result = Solution::add_two_numbers(list1, list2);
        assert_eq!(result, expected);
    }
}
