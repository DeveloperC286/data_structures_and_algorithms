use super::*;

use insta::assert_json_snapshot;

#[test]
fn test_new_double_linked_list() {
    let double_linked_list = DoubleLinkedList::new();
}

#[test]
fn test_add_double_linked_list() {
    // Given
    let mut double_linked_list = DoubleLinkedList::new();
    let node = Node::new(5);

    // When
    double_linked_list.add(node);

    // Then
    assert_json_snapshot!(double_linked_list);
}

#[test]
fn test_multiple_add_double_linked_list() {
    // Given
    let mut double_linked_list = DoubleLinkedList::new();
    let node_1 = Node::new(5);
    let node_2 = Node::new(12);

    // When
    double_linked_list.add(node_1);
    double_linked_list.add(node_2);

    // Then
    assert_json_snapshot!(double_linked_list);
}
