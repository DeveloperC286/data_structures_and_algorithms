use proptest::prelude::*;
use rstest::rstest;

use super::*;

#[rstest(
    to_sort_vector,
    expected_sorted_vector,
    case(
        vec![5, 7, -2],
        vec![-2, 5, 7]
    ),
    case(
        vec![3, -1],
        vec![-1, 3]
    ),
    case(
        vec![1],
        vec![1]
    ),
    case(
        vec![],
        vec![]
    ),
)]
fn test_heapsort_edgecases(to_sort_vector: Vec<i32>, expected_sorted_vector: Vec<i32>) {
    assert_eq!(expected_sorted_vector, heapsort(to_sort_vector));
}

proptest! {
    #[test]
    fn test_heapsort(to_sort_vector in prop::collection::vec(-1000000i32..1000000, 1..1000)) {
        //Given
        let expected_length = to_sort_vector.len();

        //When
        let returned_sorted_vector = heapsort(to_sort_vector);

        //Then
        assert_eq!(expected_length, returned_sorted_vector.len());
        // Check the next one is equal or larger, aka all are in order.
        for i in 1..returned_sorted_vector.len() {
            if returned_sorted_vector[i-1] > returned_sorted_vector[i] {
                panic!("returned_sorted_vector[{}] {} > returned_sorted_vector[{}] {}",i -1, returned_sorted_vector[i -1], i, returned_sorted_vector[i]);
            }
        }
    }
}
