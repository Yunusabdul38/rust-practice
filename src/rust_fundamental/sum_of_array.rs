pub fn sum_array(arr: &[i32]) -> i32 {
    // TODO: Implement the function here
    let mut b = 0;
    for element in arr.iter() {
        b = b + element
    }
    b
}
