//! Caching utilities for holiday calculations

use chrono::NaiveDate;
use lru::LruCache;
use std::collections::HashSet;
use std::num::NonZeroUsize;
use std::sync::Mutex;

/// Thread-safe cache for holiday data with LRU eviction
pub struct HolidayCache {
    data: Mutex<LruCache<i32, HashSet<NaiveDate>>>,
}

impl HolidayCache {
    /// Create a new empty cache with default max entries (20 years)
    pub fn new() -> Self {
        Self::with_capacity(20)
    }

    /// Create a new cache with specified maximum entries
    pub fn with_capacity(max_entries: usize) -> Self {
        Self {
            data: Mutex::new(LruCache::new(NonZeroUsize::new(max_entries).unwrap())),
        }
    }

    /// Get cached holidays for a year, or compute and cache them
    pub fn get_or_compute<F>(&self, year: i32, compute: F) -> HashSet<NaiveDate>
    where
        F: FnOnce() -> HashSet<NaiveDate>,
    {
        let mut cache = self.data.lock().unwrap();

        if let Some(holidays) = cache.get(&year) {
            return holidays.clone();
        }

        let holidays = compute();
        cache.put(year, holidays.clone());
        holidays
    }

    /// Get cached holidays for a year if available
    pub fn get(&self, year: i32) -> Option<HashSet<NaiveDate>> {
        let mut cache = self.data.lock().unwrap();
        cache.get(&year).cloned()
    }

    /// Insert holidays for a year into the cache
    pub fn insert(&self, year: i32, holidays: HashSet<NaiveDate>) {
        let mut cache = self.data.lock().unwrap();
        cache.put(year, holidays);
    }

    /// Get the current number of cached entries
    pub fn len(&self) -> usize {
        self.data.lock().unwrap().len()
    }

    /// Check if the cache is empty
    pub fn is_empty(&self) -> bool {
        self.data.lock().unwrap().is_empty()
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        let cache = self.data.lock().unwrap();
        CacheStats {
            len: cache.len(),
            capacity: cache.cap().get(),
        }
    }

    /// Clear all cached entries
    pub fn clear(&self) {
        let mut cache = self.data.lock().unwrap();
        cache.clear();
    }
}

/// Cache statistics
#[derive(Debug, Clone, Copy)]
pub struct CacheStats {
    /// Current number of cached entries
    pub len: usize,
    /// Maximum capacity of the cache
    pub capacity: usize,
}

impl Default for HolidayCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_basic_operations() {
        let cache = HolidayCache::new();
        let year = 2025;
        let holidays = HashSet::from([
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2025, 12, 25).unwrap(),
        ]);

        // Test get_or_compute
        let cached = cache.get_or_compute(year, || holidays.clone());
        assert_eq!(cached.len(), 2);

        // Test basic functionality
        assert!(cached.contains(&NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()));
        assert!(!cached.contains(&NaiveDate::from_ymd_opt(2025, 1, 2).unwrap()));
    }

    #[test]
    fn test_cache_get_and_insert() {
        let cache = HolidayCache::new();
        let year = 2025;
        let holidays = HashSet::from([NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()]);

        // Initially empty
        assert!(cache.get(year).is_none());

        // Insert data
        cache.insert(year, holidays.clone());
        assert_eq!(cache.get(year), Some(holidays));
    }

    #[test]
    fn test_cache_stats() {
        let cache = HolidayCache::with_capacity(5);
        let stats = cache.stats();
        assert_eq!(stats.len, 0);
        assert_eq!(stats.capacity, 5);

        // Add some data
        cache.insert(2025, HashSet::new());
        let stats = cache.stats();
        assert_eq!(stats.len, 1);
        assert_eq!(stats.capacity, 5);
    }

    #[test]
    fn test_cache_clear() {
        let cache = HolidayCache::new();
        cache.insert(2025, HashSet::new());
        assert_eq!(cache.len(), 1);

        cache.clear();
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
    }

    #[test]
    fn test_cache_lru_eviction() {
        let cache = HolidayCache::with_capacity(3);

        // Fill cache to capacity
        for year in 2020..2023 {
            let holidays = HashSet::from([NaiveDate::from_ymd_opt(year, 1, 1).unwrap()]);
            let _cached = cache.get_or_compute(year, || holidays);
        }

        // Cache should be at capacity
        assert_eq!(cache.len(), 3);

        // Add one more entry - should evict the least recently used
        let holidays = HashSet::from([NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()]);
        let _cached = cache.get_or_compute(2023, || holidays);

        // Cache should still be at capacity
        assert_eq!(cache.len(), 3);

        // Access 2020 to make it recently used, then add 2024
        let _cached = cache.get_or_compute(2020, || {
            HashSet::from([NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()])
        });
        let holidays = HashSet::from([NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()]);
        let _cached = cache.get_or_compute(2024, || holidays);

        // Cache should still be at capacity
        assert_eq!(cache.len(), 3);
    }

    #[test]
    fn test_cache_concurrent_access() {
        use std::sync::Arc;
        use std::thread;

        let cache = Arc::new(HolidayCache::new());
        let mut handles = vec![];

        // Spawn multiple threads that access the cache concurrently
        for i in 0..10 {
            let cache_clone = Arc::clone(&cache);
            let handle = thread::spawn(move || {
                let year = 2020 + i;
                let holidays = HashSet::from([NaiveDate::from_ymd_opt(year, 1, 1).unwrap()]);
                let _cached = cache_clone.get_or_compute(year, || holidays);
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        // Verify cache is working
        let _cached = cache.get_or_compute(2029, || {
            HashSet::from([NaiveDate::from_ymd_opt(2029, 1, 1).unwrap()])
        });
    }
}
