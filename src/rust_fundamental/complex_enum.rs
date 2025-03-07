pub enum Animal {
    // Define the Animal variants here
    Dog,
    Cat(String),
    Bird { species: String, can_fly: bool },
}

pub fn describe_animal(animal: &Animal) -> String {
    // Your code here...
    match animal {
        Animal::Dog => "A friendly dog.".to_string(),
        Animal::Cat(a) => {
            let m = a.clone();
            let fmt = format!("A cat named {m}.");
            //"".to_string() + a.clone() + ".".to_string()
            fmt
        }
        Animal::Bird { species, can_fly } => {
            let fly;
            match can_fly {
                true => {
                    fly = "can".to_string();
                }
                _ => {
                    fly = "cannot".to_string();
                }
            }
            let fmt = format!("A {species} that {fly} fly.");
            fmt
        }
    }
}

// Example use case
pub fn test_complex_enum() {
    let dog = Animal::Dog;
    assert_eq!(describe_animal(&dog), "A friendly dog.");

    let cat = Animal::Cat("Whiskers".to_string());
    assert_eq!(describe_animal(&cat), "A cat named Whiskers.");

    let bird = Animal::Bird {
        species: "Penguin".to_string(),
        can_fly: false,
    };
    assert_eq!(describe_animal(&bird), "A Penguin that cannot fly.");
}
