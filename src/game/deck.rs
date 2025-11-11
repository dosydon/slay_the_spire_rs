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
        self.cards.pop()
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

    pub fn initialize_game(&self, rng: &mut impl rand::Rng) -> (Deck, Vec<Card>) {
        let mut deck = self.clone();
        let mut hand = Vec::new();

        // Shuffle the deck
        use rand::seq::SliceRandom;
        deck.cards.shuffle(rng);

        // Draw the first 5 cards for the hand
        for _ in 0..5 {
            if let Some(card) = deck.cards.pop() {
                hand.push(card);
            }
        }

        (deck, hand)
    }
}