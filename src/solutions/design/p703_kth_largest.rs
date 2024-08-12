struct KthLargest {
    k: i32,
    heap: std::collections::BinaryHeap<std::cmp::Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut nums = nums;
        nums.sort_unstable();
        let heap = nums
            .into_iter()
            .rev()
            .take(k as usize)
            .map(std::cmp::Reverse)
            .collect::<std::collections::BinaryHeap<std::cmp::Reverse<i32>>>();
        KthLargest { k, heap }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(std::cmp::Reverse(val));
        if self.heap.len() > self.k as usize {
            self.heap.pop();
        }
        self.heap.peek().unwrap().0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth_largest() {
        let k = 3;
        let nums = vec![4, 5, 8, 2];
        let mut obj = KthLargest::new(k, nums);
        assert_eq!(obj.add(3), 4);
        assert_eq!(obj.add(5), 5);
        assert_eq!(obj.add(10), 5);
        assert_eq!(obj.add(9), 8);
        assert_eq!(obj.add(4), 8);
    }

    #[test]
    fn test_kth_largest_2() {
        let k = 2;
        let nums = vec![0];
        let mut obj = KthLargest::new(k, nums);
        assert_eq!(obj.add(-1), -1);
        assert_eq!(obj.add(1), 0);
        assert_eq!(obj.add(-2), 0);
        assert_eq!(obj.add(-4), 0);
        assert_eq!(obj.add(3), 1);
    }
}
