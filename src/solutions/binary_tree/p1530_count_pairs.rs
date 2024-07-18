use std::cell::RefCell;
use std::rc::Rc;

use crate::solutions::Solution;

use super::TreeNode;
impl Solution {
    pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, distance: i32, count: &mut i32) -> Vec<i32> {
            match node {
                Some(node) => {
                    let n = node.borrow();
                    if n.left.is_none() && n.right.is_none() {
                        return vec![1];
                    }
                    let left = dfs(n.left.clone(), distance, count);
                    let right = dfs(n.right.clone(), distance, count);
                    *count += left
                        .iter()
                        .flat_map(|&l| right.iter().map(move |&r| l + r))
                        .filter(|&sum| sum <= distance)
                        .count() as i32;
                    left.iter()
                        .chain(right.iter())
                        .map(|&x| x + 1)
                        .filter(|&x| x < distance)
                        .collect()
                }
                None => Vec::new(),
            }
        }

        let mut count = 0;
        dfs(root, distance, &mut count);
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_pairs() {
        let tree = TreeNode::from_string("1,2,3,null,4");
        let root = tree.clone();
        let distance = 3;
        let result = Solution::count_pairs(root, distance);
        let expected = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_pairs_2() {
        let tree = TreeNode::from_string("1,2,3,4,5,6,7");
        let root = tree.clone();
        let distance = 3;
        let result = Solution::count_pairs(root, distance);
        let expected = 2;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_pairs_3() {
        let tree = TreeNode::from_string("7,1,4,6,null,5,3,null,null,null,null,null,2");
        let root = tree.clone();
        let distance = 3;
        let result = Solution::count_pairs(root, distance);
        let expected = 1;
        assert_eq!(result, expected);
    }
}
