use crate::{cards::ironclad::{bash::bash, defend::defend, strike::strike}, game::deck::Deck};

pub fn starter_deck() -> Deck {
    let cards = vec![
        strike(),
        strike(),
        strike(),
        strike(),
        defend(),
        defend(),
        defend(),
        defend(),
        bash(),
    ];
    Deck::new(cards)
}