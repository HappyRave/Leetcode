use std::collections::{BTreeMap, HashMap};

struct TimeMap {
    map: HashMap<String, BTreeMap<i32, String>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 *
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */
impl TimeMap {
    fn new() -> Self {
        TimeMap {
            map: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        let map = self.map.entry(key).or_insert(BTreeMap::new());
        map.insert(timestamp, value);
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(map) = self.map.get(&key) {
            if let Some((_, value)) = map.range(..=timestamp).next_back() {
                return value.clone();
            }
        }
        "".to_string()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_time_map() {
        let mut time_map = TimeMap::new();
        time_map.set("foo".to_string(), "bar".to_string(), 1);
        assert_eq!(time_map.get("foo".to_string(), 1), "bar".to_string());
        assert_eq!(time_map.get("foo".to_string(), 3), "bar".to_string());
        time_map.set("foo".to_string(), "bar2".to_string(), 4);
        assert_eq!(time_map.get("foo".to_string(), 4), "bar2".to_string());
        assert_eq!(time_map.get("foo".to_string(), 5), "bar2".to_string());
        //assert_eq!(time_map.get("foo".to_string(), 0), "".to_string());
    }
}
