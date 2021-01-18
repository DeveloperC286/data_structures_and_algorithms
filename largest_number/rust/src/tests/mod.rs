use super::*;

use proptest::prelude::*;
use rstest::rstest;

#[rstest(
    searching,
    expected_largest_number,
    case(
        &[11, 5, 7],
        Some(11)
    ),
    case(
        &[-3, 7],
        Some(7)
    ),
    case(
        &[1],
        Some(1)
    ),
    case(
        &[],
        None
    ),
)]
fn test_largest_number_edgecases(searching: &[i32], expected_largest_number: Option<i32>) {
    assert_eq!(expected_largest_number, largest_number(searching));
}

proptest! {
    #[test]
    fn test_largest_number(mut searching in prop::collection::vec(-1000000i32..1000000, 1..10000)) {
        // When
        let largest_number = largest_number(&searching).unwrap();

        // Then
        searching.sort_unstable();
        assert_eq!(searching[searching.len() -1], largest_number);
    }
}
