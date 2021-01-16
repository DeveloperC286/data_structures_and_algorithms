use proptest::prelude::*;
use rand::Rng;

use super::*;

#[test]
fn test_empty_vector() {
    // Given
    let searching = vec![];
    let searching_for = 2;

    // When/Then
    assert_eq!(None, binary_search(searching, searching_for));
}

#[test]
fn test_single_vector() {
    // Given
    let searching = vec![2];
    let searching_for = 2;

    // When/Then
    assert_eq!(Some(0), binary_search(searching, searching_for));
}

#[test]
fn test_in_sorted_vector() {
    // Given
    let searching = vec![-7, -1, 0, 2, 3, 5, 8, 11, 23, 31];
    let searching_for = 3;

    // When/Then
    assert_eq!(Some(4), binary_search(searching, searching_for));
}

#[test]
fn test_double_vector() {
    // Given
    let searching = vec![3, 4];
    let searching_for = 3;

    // When/Then
    assert_eq!(Some(0), binary_search(searching, searching_for));
}

#[test]
fn test_not_in_sorted_vector() {
    // Given
    let searching = vec![-7, -1, 0, 2, 3, 5, 8, 11, 23, 31];
    let searching_for = 4;

    // When/Then
    assert_eq!(None, binary_search(searching, searching_for));
}

proptest! {
    #[test]
    fn test_binary_search_finds_it(mut searching in prop::collection::vec(-1000000i32..1000000, 1..10000)) {
        // Given
        searching.sort_unstable();
        searching.dedup();

        let mut rng = rand::thread_rng();
        let expected_index = rng.gen_range(0..searching.len());
        let searching_for = searching[expected_index];

        // When/Then
        assert_eq!(Some(expected_index), binary_search(searching, searching_for));
    }

    #[test]
    fn test_binary_search_does_not_find_it(mut searching in prop::collection::vec(-1000000i32..1000000, 1..10000)) {
        // Given
        searching.sort_unstable();
        searching.dedup();

        let mut rng = rand::thread_rng();
        let mut searching_for = rng.gen_range(-1000000i32..1000000);

        while searching.contains(&searching_for) {
            searching_for = rng.gen_range(-1000000i32..1000000);
        }

        // When/Then
        assert_eq!(None, binary_search(searching, searching_for));
    }
}
