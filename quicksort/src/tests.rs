use super::*;

use proptest::prelude::*;
use rstest::rstest;
use std::sync::Once;

static INIT: Once = Once::new();

/// Setup function that is only run once, even if called multiple times.
fn setup() {
    INIT.call_once(|| {
        pretty_env_logger::init();
    });
}

#[rstest(
    to_sort_vector,
    expected_sorted_vector,
    case(
        vec![1],
        vec![1]
    ),
    case(
        vec![],
        vec![]
    ),
)]
fn test_quicksort_edgecases(to_sort_vector: Vec<i32>, expected_sorted_vector: Vec<i32>) {
    setup();
    assert_eq!(expected_sorted_vector, quicksort(to_sort_vector));
}

proptest! {
    #[test]
    fn test_quicksort(to_sort_vector in prop::collection::vec(0i32..10000, 1..10000)) {
        let returned_sorted_vector = quicksort(to_sort_vector);

        // Check the next one is equal or larger, aka all are in order.
        for i in 1..returned_sorted_vector.len() {
            if returned_sorted_vector[i-1] > returned_sorted_vector[i] {
                panic!("returned_sorted_vector[{}] {} > returned_sorted_vector[{}] {}",i -1, returned_sorted_vector[i -1], i, returned_sorted_vector[i]);
            }
        }
    }
}
