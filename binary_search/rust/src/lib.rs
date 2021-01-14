pub fn binary_search(searching: Vec<i32>, searching_for: i32) -> Option<usize> {
    if searching.is_empty() {
        return None;
    }

    let lower_bound = 0;
    let upper_bound = searching.len() - 1;

    if searching_for < searching[lower_bound] {
        return None;
    }

    if searching_for > searching[upper_bound] {
        return None;
    }

    return binary_search_internal(searching, searching_for, lower_bound, upper_bound);
}

fn binary_search_internal(
    searching: Vec<i32>,
    searching_for: i32,
    lower_bound: usize,
    upper_bound: usize,
) -> Option<usize> {
    let middle_bound = (lower_bound + upper_bound) / 2;

    if searching[middle_bound] == searching_for {
        return Some(middle_bound);
    } else {
        if lower_bound == upper_bound {
            return None;
        }

        if searching[middle_bound] > searching_for {
            return binary_search_internal(searching, searching_for, middle_bound, upper_bound);
        } else {
            return binary_search_internal(searching, searching_for, lower_bound, middle_bound);
        }
    }
}

#[cfg(test)]
mod tests;
