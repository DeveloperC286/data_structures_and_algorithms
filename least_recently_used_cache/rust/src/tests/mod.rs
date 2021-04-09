use super::*;

#[test]
fn test_least_recently_used_cache_cache_keys_match_values() {
    // Given
    let mut least_recently_used_cache = LeastRecentlyUsedCache::new(2);

    // When/Then put
    least_recently_used_cache.put(1, 1);
    least_recently_used_cache.put(2, 2);
    assert_eq!(least_recently_used_cache.get(1), 1);

    // When/Then eviction.
    least_recently_used_cache.put(3, 3);
    assert_eq!(least_recently_used_cache.get(2), -1);

    // When/Then what is left in cache.
    least_recently_used_cache.put(4, 4);
    assert_eq!(least_recently_used_cache.get(1), -1);
    assert_eq!(least_recently_used_cache.get(3), 3);
    assert_eq!(least_recently_used_cache.get(4), 4);
}

#[test]
fn test_least_recently_used_cache_cache_keys_mismatch_values() {
    // Given
    let mut least_recently_used_cache = LeastRecentlyUsedCache::new(2);

    // When/Then put
    least_recently_used_cache.put(1, 0);
    least_recently_used_cache.put(2, 2);
    assert_eq!(least_recently_used_cache.get(1), 0);

    // When/Then eviction.
    least_recently_used_cache.put(3, 3);
    assert_eq!(least_recently_used_cache.get(2), -1);

    // When/Then what is left in cache.
    least_recently_used_cache.put(4, 4);
    assert_eq!(least_recently_used_cache.get(1), -1);
    assert_eq!(least_recently_used_cache.get(3), 3);
    assert_eq!(least_recently_used_cache.get(4), 4);
}

//Input
#[test]
fn test_least_recently_used_cache_cache_change_value() {
    // Given
    let mut least_recently_used_cache = LeastRecentlyUsedCache::new(2);

    // Whem/Then with nothing provided
    assert_eq!(least_recently_used_cache.get(2), -1);
    least_recently_used_cache.put(2, 6);

    // When/Then with nothing provided
    assert_eq!(least_recently_used_cache.get(1), -1);
    least_recently_used_cache.put(1, 5);

    // When/Then change value
    least_recently_used_cache.put(1, 2);
    assert_eq!(least_recently_used_cache.get(1), 2);
    assert_eq!(least_recently_used_cache.get(2), 6);
}
