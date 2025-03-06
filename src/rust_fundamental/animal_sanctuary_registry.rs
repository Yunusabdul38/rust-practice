use std::collections::HashMap;

type Collection = HashMap<String, Vec<String>>;

///  This function add an animal to a section in the registry. If the section does not exist, it should be created. If the animal is already in the section, it should not be added again
pub fn add_animal_to_section(animal: &str, section: &str, registry: &mut Collection) {
    // TODO: implement this function
    let data = registry.get(section);
    let mut m = Vec::new();
    match data {
        Some(d) => m = d.clone(),
        _ => (),
    }
    let l = m.iter().filter(|x| **x != animal.to_string());
    let mut b = Vec::new();
    for c in l {
        b.push(c.clone());
    }
    b.push(animal.to_string());
    registry.insert(section.to_string(), b);
}

///This function return a list of animals sorted alphabetically in a given section. If the section does not exist, it should return an empty list
pub fn get_animals_in_section(section: &str, registry: &Collection) -> Vec<String> {
    // TODO: implement this function
    match registry.get(section) {
        Some(t) => {
            let mut m = t.to_vec();
            m.sort();
            m
        }
        _ => vec![].to_vec(),
    }
}

///This function return a copy of the entire registry with all animals sorted alphabetically in each section
pub fn get_all_animals_sorted(registry: &Collection) -> Vec<String> {
    // TODO: implement this function
    let mut data = Vec::new();
    for (_key, value) in registry {
        for l in value {
            data.push(l.clone());
        }
    }
    data.sort();
    data
}
