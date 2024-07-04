use crate::solutions::Solution;

use super::ListNode;

impl Solution {
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = head;
        let mut result = None;
        let mut tail = &mut result;
        let mut sum = 0;
        while let Some(mut node) = current {
            if node.val == 0 {
                if sum > 0 {
                    let mut new_node = Box::new(ListNode::new(sum));
                    sum = 0;
                    new_node.next = None;
                    *tail = Some(new_node);
                    tail = &mut tail.as_mut().unwrap().next;
                }
            } else {
                sum += node.val;
            }
            current = node.next.take();
        }
        result
    }
}

#[cfg(test)]
mod tests {

    use crate::solutions::linked_lists::ListExt;

    use super::*;

    #[test]
    fn test_merge_nodes() {
        let head = vec![0, 3, 1, 0, 4, 5, 2, 0].into_list();
        let solution = vec![4, 11].into_list();
        let result = Solution::merge_nodes(head);
        assert_eq!(result, solution);
    }

    #[test]
    fn test_merge_nodes_2() {
        let head = vec![0, 1, 0, 3, 0, 2, 2, 0].into_list();
        let solution = vec![1, 3, 4].into_list();
        let result = Solution::merge_nodes(head);
        assert_eq!(result, solution);
    }
}
