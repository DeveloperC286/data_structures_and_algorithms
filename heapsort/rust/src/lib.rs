use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn heapsort(to_sort: Vec<i32>) -> Vec<i32> {
    let mut min_heap = BinaryHeap::new();

    for i in to_sort {
        min_heap.push(Reverse(i));
    }

    let mut sorted = vec![];

    while !min_heap.is_empty() {
        sorted.push(min_heap.pop().unwrap().0);
    }

    sorted
}

#[cfg(test)]
extern crate proptest;

#[cfg(test)]
mod tests;
