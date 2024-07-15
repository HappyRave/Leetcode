use std::cell::RefCell;
use std::rc::Rc;

use crate::solutions::Solution;

use super::TreeNode;
impl Solution {
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut map: std::collections::HashMap<i32, Rc<RefCell<TreeNode>>> =
            std::collections::HashMap::new();
        let mut children: std::collections::HashSet<i32> = std::collections::HashSet::new();

        for description in &descriptions {
            let (parent, child, is_left) = (description[0], description[1], description[2] == 1);

            children.insert(child);

            let parent_node = map
                .entry(parent)
                .or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(parent))))
                .clone();
            let child_node = map
                .entry(child)
                .or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(child))))
                .clone();

            if is_left {
                parent_node.borrow_mut().left = Some(child_node);
            } else {
                parent_node.borrow_mut().right = Some(child_node);
            }
        }

        // Find the root node: the only node not in `children`
        map.into_iter().find_map(|(val, node)| {
            if !children.contains(&val) {
                Some(node)
            } else {
                None
            }
        })
    }

    pub fn create_binary_tree2(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut map: std::collections::HashMap<i32, Rc<RefCell<TreeNode>>> =
            std::collections::HashMap::new();
        let mut children = std::collections::HashSet::new();

        for description in &descriptions {
            let parent = description[0];
            let child = description[1];
            let is_left = description[2] == 1;

            children.insert(child);

            let parent_node = match map.get(&parent) {
                Some(node) => Rc::clone(node),
                None => Rc::new(RefCell::new(TreeNode::new(parent))),
            };

            let child_node = match map.get(&child) {
                Some(node) => Rc::clone(node),
                None => Rc::new(RefCell::new(TreeNode::new(child))),
            };

            if is_left {
                parent_node.borrow_mut().left = Some(child_node.clone());
                map.entry(child)
                    .and_modify(|node| {
                        *node = Rc::clone(parent_node.borrow().left.as_ref().unwrap())
                    })
                    .or_insert(Rc::clone(parent_node.borrow().left.as_ref().unwrap()));
            } else {
                parent_node.borrow_mut().right = Some(child_node.clone());
                map.entry(child)
                    .and_modify(|node| {
                        *node = Rc::clone(parent_node.borrow().right.as_ref().unwrap())
                    })
                    .or_insert(Rc::clone(parent_node.borrow().right.as_ref().unwrap()));
            }

            map.entry(parent)
                .and_modify(|node| *node = Rc::clone(&parent_node))
                .or_insert(parent_node);
        }

        for description in descriptions {
            if !children.contains(&description[0]) {
                return Some(Rc::clone(map.get(&description[0]).unwrap()));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::binary_tree::BinaryTreeExt;

    use super::*;

    #[test]
    fn test_create_binary_tree() {
        let descriptions = vec![
            vec![20, 15, 1],
            vec![20, 17, 0],
            vec![50, 20, 1],
            vec![50, 80, 0],
            vec![80, 19, 1],
        ];
        let answer = vec![Some(50), Some(20), Some(80), Some(15), Some(17), Some(19)].into_tree();
        let solution = Solution::create_binary_tree(descriptions);
        assert_eq!(solution, answer);
    }

    #[test]
    fn test_create_binary_tree_2() {
        let descriptions = vec![vec![1, 2, 1], vec![2, 3, 0], vec![3, 4, 1]];
        let answer = vec![Some(1), Some(2), None, None, Some(3), Some(4)].into_tree();
        let solution = Solution::create_binary_tree(descriptions);
        assert_eq!(solution, answer);
    }
}
