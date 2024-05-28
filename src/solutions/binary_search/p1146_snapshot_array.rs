struct SnapshotArray {
    snaps: Vec<Vec<(usize, i32)>>,
    snap_id: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

impl SnapshotArray {
    fn new(length: i32) -> Self {
        Self {
            snaps: vec![vec![(0, 0)]; length as usize],
            snap_id: 0,
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        let index = index as usize;
        let snap_id = self.snap_id;
        if self.snaps[index].last().unwrap().0 == snap_id {
            self.snaps[index].last_mut().unwrap().1 = val;
        } else {
            self.snaps[index].push((snap_id, val));
        }
    }

    fn snap(&mut self) -> i32 {
        self.snap_id += 1;
        self.snap_id as i32 - 1
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let index = index as usize;
        let snap_id = snap_id as usize;
        let snaps = &self.snaps[index];
        match snaps.binary_search_by(|(s_id, _)| s_id.cmp(&snap_id)) {
            Ok(i) => snaps[i].1,
            Err(i) => snaps[i - 1].1,
        }
    }
}

/**
 * Your SnapshotArray object will be instantiated and called as such:
 * let obj = SnapshotArray::new(length);
 * obj.set(index, val);
 * let ret_2: i32 = obj.snap();
 * let ret_3: i32 = obj.get(index, snap_id);
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1146_snapshot_array_t1() {
        let mut obj = SnapshotArray::new(3);
        obj.set(0, 5);
        assert_eq!(obj.snap(), 0);
        obj.set(0, 6);
        assert_eq!(obj.get(0, 0), 5);
    }

    #[test]
    fn p1146_snapshot_array_t2() {
        let mut obj = SnapshotArray::new(3);
        obj.set(0, 5);
        obj.set(1, 7);
        obj.snap();
        obj.snap();
        obj.set(0, 6);
        assert_eq!(obj.get(0, 0), 5);
        obj.set(1, 8);
        assert_eq!(obj.get(1, 1), 7);
        assert_eq!(obj.get(1, 2), 8);
    }
}
