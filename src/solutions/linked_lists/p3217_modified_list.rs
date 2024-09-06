use crate::solutions::Solution;

use super::ListNode;

impl Solution {
    pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let set = std::collections::HashSet::<i32>::from_iter(nums);
        let mut holder = Box::new(ListNode { val: 0, next: head });
        let mut current = &mut holder;
        while let Some(ref mut node) = current.next {
            if set.contains(&node.val) {
                current.next = node.next.take();
            } else {
                current = current.next.as_mut().unwrap();
            }
        }
        holder.next
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::linked_lists::ListExt;

    use super::*;

    #[test]
    fn test_modified_list() {
        let nums = vec![1, 2, 3];
        let head = vec![1, 2, 3, 4, 5].into_list();
        let res = vec![4, 5].into_list();
        assert_eq!(Solution::modified_list(nums, head), res);
    }

    #[test]
    fn test_modified_list_2() {
        let nums = vec![1];
        let head = vec![1, 2, 1, 2, 1, 2].into_list();
        let res = vec![2, 2, 2].into_list();
        assert_eq!(Solution::modified_list(nums, head), res);
    }

    #[test]
    fn test_modified_list_3() {
        let nums = vec![5];
        let head = vec![1, 2, 3, 4].into_list();
        let res = vec![1, 2, 3, 4].into_list();
        assert_eq!(Solution::modified_list(nums, head), res);
    }
}
