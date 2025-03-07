#[derive(PartialEq)]
pub enum TrafficLight {
    // Define enum variants here
    Red,
    Yellow,
    Green,
}

pub fn light_action(light: &TrafficLight) -> &'static str {
    // Your code here...
    if *light == TrafficLight::Green {
        "Go"
    } else if *light == TrafficLight::Yellow {
        "Caution"
    } else {
        "Stop"
    }
}
