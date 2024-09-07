use std::cell::RefCell;
use std::rc::Rc;

use crate::solutions::binary_tree::TreeNode;
use crate::solutions::Solution;

use super::ListNode;
impl Solution {
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let head = head.unwrap();
        let mut node = head.next;
        let mut list = vec![head.val];
        let mut lps = vec![0, 0];
        let mut last_lps = 0;
        while let Some(n) = node {
            list.push(n.val);

            if n.val == list[last_lps] {
                last_lps += 1;
            } else {
                last_lps = 0;
            }
            lps.push(last_lps);

            node = n.next;
        }

        let list = list;
        let lps = lps;
        let mut stack = vec![(0, root.unwrap())];

        while let Some((mut i, node)) = stack.pop() {
            let n = node.borrow();

            while n.val != list[i] && i != 0 {
                i = lps[i];
            }
            if n.val == list[i] {
                i += 1;
            }

            if i == list.len() {
                return true;
            }

            if let Some(m) = &n.left {
                stack.push((i, m.clone()));
            }
            if let Some(m) = &n.right {
                stack.push((i, m.clone()));
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::linked_lists::ListExt;

    use super::*;

    #[test]
    fn test_is_sub_path() {
        let head = vec![4, 2, 8].into_list();
        let root = TreeNode::from_string("1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3");
        assert!(Solution::is_sub_path(head, root));
    }

    #[test]
    fn test_is_sub_path_2() {
        let head = vec![1, 4, 2, 6].into_list();
        let root = TreeNode::from_string("1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3");
        assert!(Solution::is_sub_path(head, root));
    }

    #[test]
    fn test_is_sub_path_3() {
        let head = vec![1, 4, 2, 6, 8].into_list();
        let root = TreeNode::from_string("1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3");
        assert!(!Solution::is_sub_path(head, root));
    }
}
