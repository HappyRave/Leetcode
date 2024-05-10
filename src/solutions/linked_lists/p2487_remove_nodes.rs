use crate::solutions::Solution;

use super::ListNode;

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

    use crate::solutions::linked_lists::ListExt;

    use super::*;

    #[test]
    fn test_remove_nodes() {
        let head = vec![5, 2, 13, 3, 8].into_list();
        let solution = vec![13, 8].into_list();
        let result = Solution::remove_nodes(head);
        assert_eq!(result, solution);
    }
}
