use std::{cell::RefCell, rc::Rc};

pub mod p1325_remove_leaf_nodes;
pub mod p2331_evaluate_tree;
pub mod p979_distribute_coins;
pub mod p993_is_cousins;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

trait BinaryTreeExt {
    fn into_tree(self) -> Option<Rc<RefCell<TreeNode>>>;
}

impl BinaryTreeExt for Vec<Option<i32>> {
    fn into_tree(self) -> Option<Rc<RefCell<TreeNode>>> {
        let mut iter = self.into_iter();
        let root = iter
            .next()
            .and_then(|val| val.map(|val| Rc::new(RefCell::new(TreeNode::new(val)))));
        let mut queue = std::collections::VecDeque::new();
        if let Some(root) = root.as_ref() {
            queue.push_back(Rc::clone(root));
        }
        while let Some(node) = queue.pop_front() {
            if let Some(val) = iter.next().and_then(|v| v) {
                let left = Rc::new(RefCell::new(TreeNode::new(val)));
                node.borrow_mut().left = Some(Rc::clone(&left));
                queue.push_back(left);
            }
            if let Some(val) = iter.next().and_then(|v| v) {
                let right = Rc::new(RefCell::new(TreeNode::new(val)));
                node.borrow_mut().right = Some(Rc::clone(&right));
                queue.push_back(right);
            }
        }
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_node() {
        let tree = vec![Some(1)];
        let root = tree.into_tree();
        assert_eq!(root.as_ref().unwrap().borrow().val, 1);
        assert_eq!(root.as_ref().unwrap().borrow().left, None);
        assert_eq!(root.as_ref().unwrap().borrow().right, None);
    }

    #[test]
    fn test_tree_node_2() {
        let tree = vec![Some(1), Some(2), Some(3)];
        let root = tree.into_tree();
        assert_eq!(root.as_ref().unwrap().borrow().val, 1);
        assert_eq!(
            root.as_ref()
                .unwrap()
                .borrow()
                .left
                .as_ref()
                .unwrap()
                .borrow()
                .val,
            2
        );
        assert_eq!(
            root.as_ref()
                .unwrap()
                .borrow()
                .right
                .as_ref()
                .unwrap()
                .borrow()
                .val,
            3
        );
        assert_eq!(
            root.as_ref()
                .unwrap()
                .borrow()
                .left
                .as_ref()
                .unwrap()
                .borrow()
                .left,
            None
        );
        assert_eq!(
            root.as_ref()
                .unwrap()
                .borrow()
                .right
                .as_ref()
                .unwrap()
                .borrow()
                .left,
            None
        );
    }

    #[test]
    fn test_tree_node_3() {
        let tree = vec![Some(1), Some(2), Some(3), Some(2), None, Some(2), Some(4)];
        let root = tree.into_tree();
        assert_eq!(root.as_ref().unwrap().borrow().val, 1);
        assert_eq!(
            root.as_ref()
                .unwrap()
                .borrow()
                .left
                .as_ref()
                .unwrap()
                .borrow()
                .val,
            2
        );
        assert_eq!(
            root.as_ref()
                .unwrap()
                .borrow()
                .left
                .as_ref()
                .unwrap()
                .borrow()
                .right,
            None
        );
        assert_eq!(
            root.as_ref()
                .unwrap()
                .borrow()
                .right
                .as_ref()
                .unwrap()
                .borrow()
                .right
                .as_ref()
                .unwrap()
                .borrow()
                .val,
            4
        );
    }

    #[test]
    fn test_tree_node_4() {
        let tree = vec![Some(1), None, Some(3), None, Some(4)];
        let root = tree.into_tree();
        assert_eq!(root.as_ref().unwrap().borrow().val, 1);
        assert_eq!(root.as_ref().unwrap().borrow().left, None);
        assert_eq!(
            root.as_ref()
                .unwrap()
                .borrow()
                .right
                .as_ref()
                .unwrap()
                .borrow()
                .val,
            3
        );
        assert_eq!(
            root.as_ref()
                .unwrap()
                .borrow()
                .right
                .as_ref()
                .unwrap()
                .borrow()
                .left,
            None
        );
        assert_eq!(
            root.as_ref()
                .unwrap()
                .borrow()
                .right
                .as_ref()
                .unwrap()
                .borrow()
                .right
                .as_ref()
                .unwrap()
                .borrow()
                .val,
            4
        );
    }

    #[test]
    fn test_tree_node_5() {
        let tree = vec![Some(1), Some(2), Some(3), None, Some(4), None, Some(5)];
        let root = tree.into_tree();
        assert_eq!(root.as_ref().unwrap().borrow().val, 1);
        assert_eq!(
            root.as_ref()
                .unwrap()
                .borrow()
                .left
                .as_ref()
                .unwrap()
                .borrow()
                .val,
            2
        );
        assert_eq!(
            root.as_ref()
                .unwrap()
                .borrow()
                .right
                .as_ref()
                .unwrap()
                .borrow()
                .val,
            3
        );
        assert_eq!(
            root.as_ref()
                .unwrap()
                .borrow()
                .left
                .as_ref()
                .unwrap()
                .borrow()
                .left,
            None
        );
        assert_eq!(
            root.as_ref()
                .unwrap()
                .borrow()
                .left
                .as_ref()
                .unwrap()
                .borrow()
                .right
                .as_ref()
                .unwrap()
                .borrow()
                .val,
            4
        );
        assert_eq!(
            root.as_ref()
                .unwrap()
                .borrow()
                .left
                .as_ref()
                .unwrap()
                .borrow()
                .right
                .as_ref()
                .unwrap()
                .borrow()
                .right,
            None
        );
        assert_eq!(
            root.as_ref()
                .unwrap()
                .borrow()
                .right
                .as_ref()
                .unwrap()
                .borrow()
                .left,
            None
        );
        assert_eq!(
            root.as_ref()
                .unwrap()
                .borrow()
                .right
                .as_ref()
                .unwrap()
                .borrow()
                .right
                .as_ref()
                .unwrap()
                .borrow()
                .val,
            5
        );
    }
}
