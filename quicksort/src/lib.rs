#[macro_use]
extern crate log;

pub fn quicksort(to_sort: Vec<i32>) -> Vec<i32> {
    let ending_index = to_sort.len() as i32 - 1;
    return quicksort_internal(to_sort, 0, ending_index);
}

fn quicksort_internal(mut to_sort: Vec<i32>, starting_index: i32, ending_index: i32) -> Vec<i32> {
    if starting_index < ending_index {
        let (pivot_point_index, partitioned_to_sort) =
            sort_partition(to_sort, starting_index, ending_index);
        to_sort = partitioned_to_sort;

        to_sort = quicksort_internal(to_sort, starting_index, pivot_point_index - 1);
        to_sort = quicksort_internal(to_sort, pivot_point_index + 1, ending_index);
    }

    return to_sort;
}

fn sort_partition(
    mut to_sort: Vec<i32>,
    starting_index: i32,
    ending_index: i32,
) -> (i32, Vec<i32>) {
    trace!(
        "starting_index {}, ending_index {}.",
        starting_index,
        ending_index
    );
    debug!("Partitioning {:?}.", to_sort);
    let pivot = to_sort[ending_index as usize];
    let mut i = starting_index as i32 - 1;

    for j in starting_index..ending_index {
        if to_sort[j as usize] < pivot {
            i += 1;

            let temp = to_sort[i as usize];
            to_sort[i as usize] = to_sort[j as usize];
            to_sort[j as usize] = temp;
            trace!("[{}] & [{}] switched {:?}", i, j, to_sort);
        }
    }

    i += 1;
    let temp = to_sort[i as usize];
    to_sort[i as usize] = to_sort[ending_index as usize];
    to_sort[ending_index as usize] = temp;

    debug!("Finished partitioning {:?}.", to_sort);
    return (i, to_sort);
}

#[cfg(test)]
extern crate pretty_env_logger;

#[cfg(test)]
extern crate proptest;

#[cfg(test)]
mod tests;
