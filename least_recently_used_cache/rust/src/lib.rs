use std::collections::{HashMap, VecDeque};

#[allow(dead_code)]
struct LeastRecentlyUsedCache {
    capacity: usize,
    cache: HashMap<i32, i32>,
    least_recently_used: VecDeque<i32>,
}

#[allow(dead_code)]
impl LeastRecentlyUsedCache {
    fn new(capacity: i32) -> Self {
        let capacity: usize = capacity as usize;
        LeastRecentlyUsedCache {
            capacity,
            cache: HashMap::with_capacity(capacity),
            least_recently_used: VecDeque::with_capacity(capacity),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(value) = self.cache.get(&key) {
            // We have a value, move the key to back of the least recently used.
            if let Some(least_recently_used_index) =
                self.least_recently_used.iter().position(|x| *x == key)
            {
                self.least_recently_used.remove(least_recently_used_index);
                self.least_recently_used.push_back(key);
            }

            return *value;
        }

        -1
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.cache.contains_key(&key) {
            // This key/value pair is already cached, remove it from the least recently used.
            if let Some(least_recently_used_index) =
                self.least_recently_used.iter().position(|x| *x == key)
            {
                self.least_recently_used.remove(least_recently_used_index);
            }
        } else if self.least_recently_used.len() == self.capacity {
            // Have not seen this key/value pair before, so remove the least recently used.
            if let Some(removing) = self.least_recently_used.pop_front() {
                self.cache.remove(&removing);
            }
        }

        self.least_recently_used.push_back(key);
        self.cache.insert(key, value);
    }
}

#[cfg(test)]
mod tests;
