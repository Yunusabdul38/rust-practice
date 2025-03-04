pub fn find_largest_in_slice(slice: &[i32]) -> Option<i32> {
    // Your code here...
    let k = slice.iter().max();

    match k {
        Some(l) => Some(*l),
        None => None,
    }
}
