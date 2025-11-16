
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
}