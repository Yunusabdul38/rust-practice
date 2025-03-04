pub struct Person {
    // Define fields here
    // Read the description
    pub _name: String,
    pub age: u8,
}

pub fn is_adult(user: &Person) -> bool {
    if user.age >= 18 {
        true
    } else {
        false
    }
} // Finish the function
