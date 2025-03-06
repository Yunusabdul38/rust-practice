// 1. Define the struct
pub struct Counter {
    pub count: i32,
}

// 2. Implement the associated function and methods
impl Counter {
    pub fn new() -> Counter {
        Counter { count: 0 }
    }
    /// increase the current count value by 1
    pub fn increment(&mut self) {
        self.count += 1
    }
    pub fn decrement(&mut self) {
        self.count -= 1
    }
    pub fn get_count(&self) -> i32 {
        self.count
    }
}
