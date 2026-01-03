
use crate::game::card::Card;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

    pub fn insert_card(&mut self, index: usize, card: Card) {
        self.cards.insert(index, card);
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

    
    pub fn size(&self) -> usize {
        self.cards.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Look at the top card without removing it
    pub fn peek_top_card(&self) -> Option<Card> {
        if self.cards.is_empty() {
            None
        } else {
            Some(self.cards[0].clone())
        }
    }

    /// Put a card on top of the deck
    pub fn put_card_on_top(&mut self, card: Card) {
        self.cards.insert(0, card);
    }

    /// Get a reference to all cards in the deck
    pub fn get_cards(&self) -> &Vec<Card> {
        &self.cards
    }

    /// Get a reference to a specific card by index
    pub fn get_card(&self, index: usize) -> Option<&Card> {
        self.cards.get(index)
    }
}