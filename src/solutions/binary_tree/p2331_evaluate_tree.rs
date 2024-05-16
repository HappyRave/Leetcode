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
        let root = TreeNode::new(1);
        let root = Some(Rc::new(RefCell::new(root)));
        assert!(Solution::evaluate_tree(root));
    }

    #[test]
    fn test_2() {
        let root = TreeNode::new(0);
        let root = Some(Rc::new(RefCell::new(root)));
        assert!(!Solution::evaluate_tree(root));
    }

    #[test]
    fn test_3() {
        let root = TreeNode::new(2);
        let left = TreeNode::new(1);
        let right = TreeNode::new(0);
        let root = Some(Rc::new(RefCell::new(root)));
        root.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(left)));
        root.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(right)));
        assert!(Solution::evaluate_tree(root));
    }

    #[test]
    fn test_4() {
        let root = TreeNode::new(2);
        let left = TreeNode::new(1);
        let right = TreeNode::new(1);
        let root = Some(Rc::new(RefCell::new(root)));
        root.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(left)));
        root.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(right)));
        assert!(Solution::evaluate_tree(root));
    }

    #[test]
    fn test_6() {
        let root = TreeNode::new(2);
        let left = TreeNode::new(1);
        let right = TreeNode::new(2);
        let right_left = TreeNode::new(0);
        let right_right = TreeNode::new(1);
        let root = Some(Rc::new(RefCell::new(root)));
        root.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(left)));
        root.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(right)));
        root.as_ref()
            .unwrap()
            .borrow()
            .right
            .as_ref()
            .unwrap()
            .borrow_mut()
            .left = Some(Rc::new(RefCell::new(right_left)));
        root.as_ref()
            .unwrap()
            .borrow()
            .right
            .as_ref()
            .unwrap()
            .borrow_mut()
            .right = Some(Rc::new(RefCell::new(right_right)));
        assert!(Solution::evaluate_tree(root));
    }
}
