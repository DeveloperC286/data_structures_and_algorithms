#[cfg(test)]
extern crate proptest;

pub fn largest_number(searching: &[i32]) -> Option<i32> {
    match searching.len() {
        0 => None,
        1 => Some(*searching.get(0).unwrap()),
        _ => {
            let top = *searching.get(0).unwrap();
            let largest_in_sub_vector = largest_number(&searching[1..]).unwrap();

            if top > largest_in_sub_vector {
                Some(top)
            } else {
                Some(largest_in_sub_vector)
            }
        }
    }
}

#[cfg(test)]
mod tests;
