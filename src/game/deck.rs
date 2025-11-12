use rand::rngs::ThreadRng;

use crate::game::card::Card;

#[derive(Debug, Clone)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new(cards: Vec<Card>) -> Self {
        Deck { cards }
    }
    
    pub fn draw_card(&mut self) -> Option<Card> {
        if self.cards.is_empty() {
            None
        } else {
            Some(self.cards.remove(0))
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn remove_card(&mut self, index: usize) -> Option<Card> {
        if index < self.cards.len() {
            Some(self.cards.remove(index))
        } else {
            None
        }
    }

    pub fn shuffle(&mut self, rng: &mut impl rand::Rng) {
        use rand::seq::SliceRandom;
        self.cards.shuffle(rng);
    }

    pub fn initialize_game(&self) -> (Deck, Vec<Card>) {
        let mut deck = self.clone();
        let mut hand = Vec::new();

        // Take the first 5 cards as the initial hand (deterministic order)
        for _ in 0..5 {
            if let Some(card) = deck.draw_card() {
                hand.push(card);
            }
        }

        (deck, hand)
    }
}