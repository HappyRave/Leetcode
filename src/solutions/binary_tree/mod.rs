use std::{cell::RefCell, rc::Rc};

pub mod p1325_remove_leaf_nodes;
pub mod p2331_evaluate_tree;

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
        fn helper(
            tree: &mut Vec<Option<i32>>,
            index: usize,
            index_multiplier: usize,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if index >= tree.len() {
                return None;
            }
            if let Some(val) = tree[index] {
                let mut node = TreeNode::new(val);
                node.left = helper(tree, index_multiplier * index + 1, 2);
                let multiplier = if node.left.is_some() { 2 } else { 1 };
                node.right = helper(tree, index_multiplier * index + 2, multiplier);
                return Some(Rc::new(RefCell::new(node)));
            }
            None
        }
        helper(&mut self.clone(), 0, 2)
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
}
