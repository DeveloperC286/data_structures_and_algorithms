pub fn selection_sort(mut to_sort: Vec<i32>) -> Vec<i32> {
    let mut sorted = vec![];

    while let Some(smallest_index) = get_smallest_index(&to_sort) {
        let smallest = to_sort.remove(smallest_index);
        sorted.push(smallest);
    }

    sorted
}

fn get_smallest_index(searching: &[i32]) -> Option<usize> {
    if !searching.is_empty() {
        let mut smallest_index: usize = 0;
        let mut smallest = searching.get(smallest_index).unwrap();

        for (index, item) in searching.iter().enumerate() {
            if item < smallest {
                smallest_index = index;
                smallest = item;
            }
        }

        return Some(smallest_index);
    }

    None
}

#[cfg(test)]
extern crate proptest;

#[cfg(test)]
mod tests;
