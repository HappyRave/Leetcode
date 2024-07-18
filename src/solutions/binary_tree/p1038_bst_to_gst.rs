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
    use super::*;

    #[test]
    fn test_bst_to_gst() {
        let tree = TreeNode::from_string("4,1,6,0,2,5,7,null,null,null,3,null,null,null,8");
        let res = TreeNode::from_string("30,36,21,36,35,26,15,null,null,null,33,null,null,null,8");
        assert_eq!(Solution::bst_to_gst(tree), res);
    }

    #[test]
    fn test_bst_to_gst_2() {
        let tree = TreeNode::from_string("0,null,1");
        let res = TreeNode::from_string("1,null,1");
        assert_eq!(Solution::bst_to_gst(tree), res);
    }
}
