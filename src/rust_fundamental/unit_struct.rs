// Define a struct named `Logger`
pub struct Logger;
// Implement an associated function `log_message`
impl Logger {
    pub fn log_message(message: &str) {
        println!("{message}");
    }
}
// That accepts a `&str` and prints the output.

// Example usage:
pub fn main() {
    Logger::log_message("Hello, World!");
}
