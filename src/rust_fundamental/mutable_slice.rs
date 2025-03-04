pub fn transform_even_odd(slice: &mut [i32]) {
    // Your code here: iterate over the mutable slice and modify its elements.
    for item in slice.iter_mut() {
        let is_even = *item % 2;
        if is_even == 0 {
            *item *= 2;
        } else {
            *item -= 1;
        }
    }
}
