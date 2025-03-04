pub fn update_slice(slice: &mut [i32], indices: &[usize], value: i32) {
    // Implement your logic here
    for item in indices.iter() {
        if &slice.len() > item {
            slice[*item] = value;
        } else {
            print!("index {item} is out of scope")
        }
    }
}
