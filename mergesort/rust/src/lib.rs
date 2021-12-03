#[cfg(test)]
extern crate proptest;

pub fn mergesort(to_sort: Vec<i32>) -> Vec<i32> {
    let length = to_sort.len();
    //base cases of len == 0 and 1
    if length <= 1 {
        return to_sort;
    }

    // Split and sort.
    let first_half = to_sort[0..length / 2].to_vec();
    let second_half = to_sort[length / 2..].to_vec();
    let mut sorted_first_half = mergesort(first_half).into_iter().peekable();
    let mut sorted_second_half = mergesort(second_half).into_iter().peekable();

    // They are returned in sorted order so add the smallest then iter.
    let mut sorted = vec![];

    while sorted_first_half.peek().is_some() && sorted_second_half.peek().is_some() {
        // Can call unwrap as we already checked via .peek().is_some()
        let next_sorted_first_half = sorted_first_half.peek().unwrap();
        let next_sorted_second_half = sorted_second_half.peek().unwrap();

        if next_sorted_first_half < next_sorted_second_half {
            sorted.push(sorted_first_half.next().unwrap());
        } else {
            sorted.push(sorted_second_half.next().unwrap());
        }
    }

    // Edge cases where multiple are left in one iter after another is empty, will execute one of
    // these loops not both.
    for next_sorted in sorted_first_half {
        sorted.push(next_sorted);
    }

    for next_sorted in sorted_second_half {
        sorted.push(next_sorted);
    }

    sorted
}

#[cfg(test)]
mod tests;
