#[derive(PartialEq, Debug)]
pub enum Card {
    // Define the Card variants here
    King,
    Queen,
    Jack,
    Numbered(u8, String),
}

pub fn card_description(card: &Card) -> String {
    // Your code here...
    match card {
        Card::Jack => "Jack".to_string(),
        Card::King => "King".to_string(),
        Card::Queen => "Queen".to_string(),
        Card::Numbered(a, b) => {
            let fmt_a = a.to_string();
            let fmt = format!("{fmt_a} of {b}");
            fmt
        }
    }
}
