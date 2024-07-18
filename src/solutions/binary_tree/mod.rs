use std::{cell::RefCell, rc::Rc};

pub mod p1038_bst_to_gst;
pub mod p1110_del_nodes;
pub mod p1325_remove_leaf_nodes;
pub mod p1382_balance_bst;
pub mod p1530_count_pairs;
pub mod p2096_get_directions;
pub mod p2196_create_binary_tree;
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

    fn size(&self) -> i32 {
        fn size(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match node {
                Some(node) => {
                    let node = node.borrow();
                    1 + size(&node.left) + size(&node.right)
                }
                None => 0,
            }
        }
        size(&self.wrap())
    }

    fn height(&self) -> i32 {
        fn height(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match node {
                Some(node) => {
                    let node = node.borrow();
                    1 + height(&node.left).max(height(&node.right))
                }
                None => 0,
            }
        }
        height(&self.wrap())
    }

    fn is_balanced(&self) -> bool {
        fn is_balanced(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match node {
                Some(node) => {
                    let node = node.borrow();
                    let left = is_balanced(&node.left);
                    let right = is_balanced(&node.right);
                    if left == -1 || right == -1 || (left - right).abs() > 1 {
                        -1
                    } else {
                        1 + left.max(right)
                    }
                }
                None => 0,
            }
        }
        is_balanced(&self.wrap()) != -1
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
        is_valid(&self.wrap(), i64::MIN, i64::MAX)
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
        pretty_print(&self.wrap(), "".to_string(), false);
    }

    fn from_string(s: &str) -> Option<Rc<RefCell<TreeNode>>> {
        let mut iter = s.split(',');
        fn next_val(iter: &mut std::str::Split<char>) -> Option<i32> {
            iter.next().and_then(|val| val.parse::<i32>().ok())
        }
        fn next_node_ref(iter: &mut std::str::Split<char>) -> Option<Rc<RefCell<TreeNode>>> {
            next_val(iter).map(|val| Rc::new(RefCell::new(TreeNode::new(val))))
        }
        let root = next_node_ref(&mut iter);
        let mut queue = std::collections::VecDeque::new();
        if let Some(root) = root.as_ref() {
            queue.push_back(Rc::clone(root));
        }
        while let Some(node) = queue.pop_front() {
            if let Some(val) = next_val(&mut iter) {
                let left = Rc::new(RefCell::new(TreeNode::new(val)));
                node.borrow_mut().left = Some(Rc::clone(&left));
                queue.push_back(left);
            }
            if let Some(val) = next_val(&mut iter) {
                let right = Rc::new(RefCell::new(TreeNode::new(val)));
                node.borrow_mut().right = Some(Rc::clone(&right));
                queue.push_back(right);
            }
        }
        root
    }

    fn wrap(&self) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(self.clone())))
    }
}

trait BinaryTreeExt {
    fn into_tree(self) -> Option<Rc<RefCell<TreeNode>>>;
}

impl BinaryTreeExt for Vec<Option<i32>> {
    fn into_tree(self) -> Option<Rc<RefCell<TreeNode>>> {
        let str = self
            .into_iter()
            .map(|v| v.map(|v| v.to_string()).unwrap_or("null".to_string()))
            .collect::<Vec<String>>()
            .join(",");
        TreeNode::from_string(&str)
    }
}

trait OptionTreeNodeExt {
    fn get_val(&self) -> i32;
    fn get_left(&self) -> Option<Rc<RefCell<TreeNode>>>;
    fn get_right(&self) -> Option<Rc<RefCell<TreeNode>>>;
    fn get_size(&self) -> i32;
    fn get_height(&self) -> i32;
    fn is_none(&self) -> bool;
    fn is_balanced(&self) -> bool;
    fn is_search_tree(&self) -> bool;
    fn pretty_print(&self);
}

impl OptionTreeNodeExt for Option<Rc<RefCell<TreeNode>>> {
    fn get_val(&self) -> i32 {
        self.as_ref().unwrap().borrow().val
    }

    fn get_left(&self) -> Option<Rc<RefCell<TreeNode>>> {
        self.as_ref().unwrap().borrow().left.clone()
    }

    fn get_right(&self) -> Option<Rc<RefCell<TreeNode>>> {
        self.as_ref().unwrap().borrow().right.clone()
    }

    fn get_size(&self) -> i32 {
        self.as_ref().unwrap().borrow().size()
    }

    fn get_height(&self) -> i32 {
        self.as_ref().unwrap().borrow().height()
    }

    fn is_none(&self) -> bool {
        self.is_none()
    }

    fn is_balanced(&self) -> bool {
        self.as_ref().unwrap().borrow().is_balanced()
    }

    fn is_search_tree(&self) -> bool {
        self.as_ref().unwrap().borrow().is_search_tree()
    }

    fn pretty_print(&self) {
        self.as_ref().unwrap().borrow().pretty_print();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_node() {
        let tree = TreeNode::from_string("1");
        assert_eq!(tree.get_size(), 1);
        assert_eq!(tree.get_height(), 1);
        assert_eq!(tree.get_val(), 1);
        assert!(tree.get_left().is_none());
        assert!(tree.get_right().is_none());
    }

    #[test]
    fn test_tree_node_2() {
        let tree = TreeNode::from_string("1,2,3");
        assert_eq!(tree.get_size(), 3);
        assert_eq!(tree.get_height(), 2);
        assert_eq!(tree.get_val(), 1);
        assert_eq!(tree.get_left().get_val(), 2);
        assert_eq!(tree.get_right().get_val(), 3);
        assert!(tree.get_left().get_left().is_none());
        assert!(tree.get_left().get_right().is_none());
        assert!(tree.get_right().get_left().is_none());
        assert!(tree.get_right().get_right().is_none());
    }

    #[test]
    fn test_tree_node_3() {
        let tree = TreeNode::from_string("1,2,3,null,4");
        assert_eq!(tree.get_size(), 4);
        assert_eq!(tree.get_height(), 3);
        assert_eq!(tree.get_val(), 1);
        assert_eq!(tree.get_left().get_val(), 2);
        assert_eq!(tree.get_right().get_val(), 3);
        assert!(tree.get_left().get_left().is_none());
        assert_eq!(tree.get_left().get_right().get_val(), 4);
        assert!(tree.get_right().get_left().is_none());
        assert!(tree.get_right().get_right().is_none());
    }

    #[test]
    fn test_tree_node_4() {
        let tree = TreeNode::from_string("1,null,3,null,5");
        assert_eq!(tree.get_size(), 3);
        assert_eq!(tree.get_height(), 3);
        assert_eq!(tree.get_val(), 1);
        assert!(tree.get_left().is_none());
        assert_eq!(tree.get_right().get_val(), 3);
        assert!(tree.get_right().get_left().is_none());
        assert_eq!(tree.get_right().get_right().get_val(), 5);
    }

    #[test]
    fn test_tree_node_5() {
        let tree = TreeNode::from_string("1,2,3,null,5,null,7,null,9");
        assert_eq!(tree.get_size(), 6);
        assert_eq!(tree.get_height(), 4);
        assert_eq!(tree.get_val(), 1);
        assert_eq!(tree.get_left().get_val(), 2);
        assert_eq!(tree.get_right().get_val(), 3);
        assert!(tree.get_left().get_left().is_none());
        assert_eq!(tree.get_left().get_right().get_val(), 5);
        assert!(tree.get_right().get_left().is_none());
        assert_eq!(tree.get_right().get_right().get_val(), 7);
        assert!(tree.get_right().get_right().get_left().is_none());
        assert_eq!(tree.get_left().get_right().get_right().get_val(), 9);
    }

    #[test]
    fn test_tree_node_6() {
        let tree = TreeNode::from_string("1,2,3,null,5,null,7,null,null,null,9");
        assert_eq!(tree.get_size(), 6);
        assert_eq!(tree.get_height(), 4);
        assert_eq!(tree.get_val(), 1);
        assert_eq!(tree.get_left().get_val(), 2);
        assert_eq!(tree.get_right().get_val(), 3);
        assert!(tree.get_left().get_left().is_none());
        assert_eq!(tree.get_left().get_right().get_val(), 5);
        assert!(tree.get_right().get_left().is_none());
        assert_eq!(tree.get_right().get_right().get_val(), 7);
        assert!(tree.get_right().get_right().get_left().is_none());
        assert_eq!(tree.get_right().get_right().get_right().get_val(), 9);
    }

    #[test]
    fn test_tree_node_is_balanced() {
        let tree = TreeNode::from_string("3,9,20,null,null,15,7");
        assert!(tree.is_balanced());
    }

    #[test]
    fn test_tree_node_is_balanced_2() {
        let tree = TreeNode::from_string("1,2,2,3,3,null,null,4,4");
        assert!(!tree.is_balanced());
    }

    #[test]
    fn test_tree_node_is_balanced_3() {
        let tree = TreeNode::from_string("1");
        assert!(tree.is_balanced());
    }

    #[test]
    fn test_tree_node_is_search_tree() {
        let tree = TreeNode::from_string("2,1,3");
        assert!(tree.is_search_tree());
    }

    #[test]
    fn test_tree_node_is_search_tree_2() {
        let tree = TreeNode::from_string("5,1,4,null,null,3,6");
        assert!(!tree.is_search_tree());
    }
}
