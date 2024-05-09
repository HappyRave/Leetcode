use super::{ListNode, Solution};

impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if let Some(mut node) = head {
            let k = k - 1;
            Self::rotate_right(node.next, k)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::ListExt;

    use super::*;

    #[test]
    fn test_rotate_right() {
        let head = vec![1, 2, 3, 4, 5].into_list();
        let k = 2;
        let solution = vec![4, 5, 1, 2, 3].into_list();
        let result = Solution::rotate_right(head, k);
        assert_eq!(result, solution);

        let head = vec![0, 1, 2].into_list();
        let k = 4;
        let solution = vec![2, 0, 1].into_list();
        let result = Solution::rotate_right(head, k);
        assert_eq!(result, solution);

        let head = vec![1, 2].into_list();
        let k = 1;
        let solution = vec![2, 1].into_list();
        let result = Solution::rotate_right(head, k);
        assert_eq!(result, solution);
    }
}
