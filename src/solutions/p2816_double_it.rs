use super::{ListNode, Solution};

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
    use crate::solutions::ListExt;

    use super::*;

    #[test]
    fn test_double_it() {
        let list = vec![1, 8, 9].into_list();
        let expected = vec![3, 7, 8].into_list();
        let result = Solution::double_it(list);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_double_it_2() {
        let list = vec![9, 9, 9].into_list();
        let expected = vec![1, 9, 9, 8].into_list();
        let result = Solution::double_it(list);
        assert_eq!(result, expected);
    }
}
