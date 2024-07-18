use std::cell::RefCell;
use std::rc::Rc;

use crate::solutions::Solution;

use super::TreeNode;

impl Solution {
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root {
            let node = node.borrow();
            if node.val == 0 {
                return false;
            }
            if node.val == 1 {
                return true;
            }
            let left = Solution::evaluate_tree(node.left.clone());
            let right = Solution::evaluate_tree(node.right.clone());
            return node.val == 2 && (left | right) || (left & right);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let root = TreeNode::from_string("1");
        assert!(Solution::evaluate_tree(root));
    }

    #[test]
    fn test_2() {
        let root = TreeNode::from_string("0");
        assert!(!Solution::evaluate_tree(root));
    }

    #[test]
    fn test_3() {
        let root = TreeNode::from_string("2,1,0");
        assert!(Solution::evaluate_tree(root));
    }

    #[test]
    fn test_4() {
        let root = TreeNode::from_string("2,1,1");
        assert!(Solution::evaluate_tree(root));
    }

    #[test]
    fn test_6() {
        let root = TreeNode::from_string("2,1,2,null,null0,1");
        assert!(Solution::evaluate_tree(root));
    }
}
