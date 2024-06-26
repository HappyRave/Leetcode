use std::{cell::RefCell, rc::Rc};

pub mod p1325_remove_leaf_nodes;
pub mod p1382_balance_bst;
pub mod p2331_evaluate_tree;
pub mod p979_distribute_coins;
pub mod p993_is_cousins;

#[derive(Debug, PartialEq, Eq, Clone)]
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

    fn is_balanced(&self) -> bool {
        fn height(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match node {
                Some(node) => {
                    let node = node.borrow();
                    let left = height(&node.left);
                    let right = height(&node.right);
                    if left == -1 || right == -1 || (left - right).abs() > 1 {
                        -1
                    } else {
                        1 + left.max(right)
                    }
                }
                None => 0,
            }
        }
        height(&Some(Rc::new(RefCell::new(self.clone())))) != -1
    }

    fn is_search_tree(&self) -> bool {
        fn is_valid(node: &Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
            match node {
                Some(node) => {
                    let node = node.borrow();
                    let val = node.val as i64;
                    if val <= min || val >= max {
                        return false;
                    }
                    is_valid(&node.left, min, val) && is_valid(&node.right, val, max)
                }
                None => true,
            }
        }
        is_valid(
            &Some(Rc::new(RefCell::new(self.clone()))),
            i64::MIN,
            i64::MAX,
        )
    }

    fn pretty_print(&self) {
        fn pretty_print(node: &Option<Rc<RefCell<TreeNode>>>, prefix: String, is_left: bool) {
            if let Some(node) = node {
                let node = node.borrow();
                println!(
                    "{}{}{}",
                    prefix,
                    if is_left { "├── " } else { "└── " },
                    node.val
                );
                pretty_print(
                    &node.left,
                    format!("{}{}", prefix, if is_left { "│   " } else { "    " }),
                    true,
                );
                pretty_print(
                    &node.right,
                    format!("{}{}", prefix, if is_left { "│   " } else { "    " }),
                    false,
                );
            }
        }
        pretty_print(
            &Some(Rc::new(RefCell::new(self.clone()))),
            "".to_string(),
            false,
        );
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

    #[test]
    fn test_tree_node_is_balanced() {
        let tree = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        let root = tree.into_tree();
        assert!(root.as_ref().unwrap().borrow().is_balanced());
    }

    #[test]
    fn test_tree_node_is_balanced_2() {
        let tree = vec![
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(3),
            None,
            None,
            Some(4),
            Some(4),
        ];
        let root = tree.into_tree();
        assert!(!root.as_ref().unwrap().borrow().is_balanced());
    }

    #[test]
    fn test_tree_node_is_balanced_3() {
        let tree = vec![Some(1)];
        let root = tree.into_tree();
        assert!(root.as_ref().unwrap().borrow().is_balanced());
    }

    #[test]
    fn test_tree_node_is_search_tree() {
        let tree = vec![Some(2), Some(1), Some(3)];
        let root = tree.into_tree();
        assert!(root.as_ref().unwrap().borrow().is_search_tree());
    }

    #[test]
    fn test_tree_node_is_search_tree_2() {
        let tree = vec![Some(5), Some(1), Some(4), None, None, Some(3), Some(6)];
        let root = tree.into_tree();
        assert!(!root.as_ref().unwrap().borrow().is_search_tree());
    }
}
