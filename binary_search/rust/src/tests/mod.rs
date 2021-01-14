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
fn test_in_sorted_array() {
    // Given
    let searching = vec![-7, -1 , 0, 2, 3, 5 ,8, 11, 23, 31];
    let searching_for = 3;

    // When/Then
    assert_eq!(Some(4), binary_search(searching, searching_for));
}

#[test]
fn test_not_in_sorted_array() {
    // Given
    let searching = vec![-7, -1 , 0, 2, 3, 5 ,8, 11, 23, 31];
    let searching_for = 4;

    // When/Then
    assert_eq!(None, binary_search(searching, searching_for));
}
