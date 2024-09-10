use crate::solutions::Solution;

use super::ListNode;

impl Solution {
    pub fn insert_greatest_common_divisors(
        mut head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        fn gcd(mut a: i32, mut b: i32) -> i32 {
            while b != 0 {
                let t = b;
                b = a % b;
                a = t;
            }
            a
        }

        let mut current = &mut head;
        while let Some(ref mut node) = current {
            if node.next.is_none() {
                break;
            }
            let gcd = gcd(node.val, node.next.as_ref().unwrap().val);
            let mut new_node = Box::new(ListNode::new(gcd));
            new_node.next = node.next.take();
            node.next = Some(new_node);

            current = &mut node.next.as_mut().unwrap().next;
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::linked_lists::ListExt;

    use super::*;

    #[test]
    fn test_insert_greatest_common_divisors() {
        let head = vec![18, 6, 10, 3].into_list();
        let result = vec![18, 6, 6, 2, 10, 1, 3].into_list();
        assert_eq!(Solution::insert_greatest_common_divisors(head), result);
    }

    #[test]
    fn test_insert_greatest_common_divisors_2() {
        let head = vec![7].into_list();
        let result = vec![7].into_list();
        assert_eq!(Solution::insert_greatest_common_divisors(head), result);
    }
}
