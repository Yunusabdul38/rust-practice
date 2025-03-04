pub fn is_prime(n: u32) -> bool {
    let is_even = n % 2;
    if n == 2 || n == 1 {
        true
    } else if n < 2 {
        false
    } else if is_even == 0 {
        false
    } else {
        true
    }
}
