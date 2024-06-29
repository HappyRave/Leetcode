use std::cell::RefCell;
use std::rc::Rc;

use crate::solutions::Solution;

use super::TreeNode;
impl Solution {
    pub fn bst_to_gst(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(node: &mut Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
            if let Some(n) = node {
                let mut n = n.borrow_mut();
                dfs(&mut n.right, sum);
                *sum += n.val;
                n.val = *sum;
                dfs(&mut n.left, sum);
            }
        }
        let mut sum = 0;
        dfs(&mut root, &mut sum);
        root
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::binary_tree::BinaryTreeExt;

    use super::*;

    #[test]
    fn test_bst_to_gst() {
        let tree = vec![
            Some(4),
            Some(1),
            Some(6),
            Some(0),
            Some(2),
            Some(5),
            Some(7),
            None,
            None,
            None,
            Some(3),
            None,
            None,
            None,
            Some(8),
        ];
        let root = tree.into_tree();
        let tree = vec![
            Some(30),
            Some(36),
            Some(21),
            Some(36),
            Some(35),
            Some(26),
            Some(15),
            None,
            None,
            None,
            Some(33),
            None,
            None,
            None,
            Some(8),
        ];
        let res = tree.into_tree();
        assert_eq!(Solution::bst_to_gst(root), res);
    }

    #[test]
    fn test_bst_to_gst_2() {
        let tree = vec![Some(0), None, Some(1)];
        let root = tree.into_tree();
        let tree = vec![Some(1), None, Some(1)];
        let res = tree.into_tree();
        assert_eq!(Solution::bst_to_gst(root), res);
    }
}
