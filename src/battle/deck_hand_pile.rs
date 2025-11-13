use crate::game::{card::Card, deck::Deck};

#[derive(Debug, Clone)]
pub struct DeckHandPile {
    deck: Deck,
    hand: Vec<Card>,
    discard_pile: Vec<Card>,
    exhausted: Vec<Card>,
}

impl DeckHandPile {
    pub fn new(deck: Deck) -> Self {
        DeckHandPile {
            deck,
            hand: Vec::new(),
            discard_pile: Vec::new(),
            exhausted: Vec::new(),
        }
    }
    
    pub fn draw_card(&mut self) -> Option<Card> {
        // If deck is empty, shuffle discard pile into deck
        if self.is_deck_empty() && !self.discard_pile.is_empty() {
            self.shuffle_discard_into_deck();
        }
        
        // Draw from deck if available
        if let Some(card) = self.deck.draw_card() {
            self.hand.push(card.clone());
            Some(card)
        } else {
            None
        }
    }
    
    /// Draw n cards into hand, returns the number of cards actually drawn
    pub(in crate::battle) fn draw_n(&mut self, n: usize) -> usize {
        let mut cards_drawn = 0;
        for _ in 0..n {
            if self.draw_card().is_some() {
                cards_drawn += 1;
            } else {
                break;
            }
        }
        cards_drawn
    }
    
    pub(in crate::battle) fn discard_card_from_hand(&mut self, hand_index: usize) -> Option<Card> {
        if hand_index < self.hand.len() {
            let card = self.hand.remove(hand_index);
            self.discard_pile.push(card.clone());
            Some(card)
        } else {
            None
        }
    }
    
    pub(in crate::battle) fn discard_entire_hand(&mut self) {
        while !self.hand.is_empty() {
            let card = self.hand.remove(0);
            self.discard_pile.push(card);
        }
    }
    
    pub(in crate::battle) fn shuffle_discard_into_deck(&mut self) {
        // Move all cards from discard pile to deck
        while let Some(card) = self.discard_pile.pop() {
            self.deck.add_card(card);
        }
        
        // Shuffle the deck
        let mut rng = rand::rng();
        self.deck.shuffle(&mut rng);
    }
    
    pub(in crate::battle) fn add_card_to_hand(&mut self, card: Card) {
        self.hand.push(card);
    }
    
    pub(in crate::battle) fn add_card_to_deck(&mut self, card: Card) {
        self.deck.add_card(card);
    }
    
    pub(in crate::battle) fn add_card_to_discard(&mut self, card: Card) {
        self.discard_pile.push(card);
    }
    
    pub(in crate::battle) fn exhaust_card_from_hand(&mut self, hand_index: usize) -> Option<Card> {
        if hand_index < self.hand.len() {
            let card = self.hand.remove(hand_index);
            self.exhausted.push(card.clone());
            Some(card)
        } else {
            None
        }
    }
    
    pub(in crate::battle) fn exhaust_card(&mut self, card: Card) {
        self.exhausted.push(card);
    }
    
    // Play card from hand (removes from hand, adds to discard pile, returns the card)
    pub(in crate::battle) fn play_card_from_hand(&mut self, hand_index: usize) -> Option<Card> {
        if hand_index < self.hand.len() {
            let card = self.hand.remove(hand_index);
            self.discard_pile.push(card.clone());
            Some(card)
        } else {
            None
        }
    }
    
    // Getters
    pub fn get_hand(&self) -> &Vec<Card> {
        &self.hand
    }
    
    pub fn get_deck(&self) -> &Deck {
        &self.deck
    }
    
    pub fn get_discard_pile(&self) -> &Vec<Card> {
        &self.discard_pile
    }
    
    pub fn get_exhausted(&self) -> &Vec<Card> {
        &self.exhausted
    }
    
    pub fn hand_size(&self) -> usize {
        self.hand.len()
    }
    
    pub fn deck_size(&self) -> usize {
        self.deck.size()
    }
    
    pub fn discard_pile_size(&self) -> usize {
        self.discard_pile.len()
    }
    
    pub fn exhausted_size(&self) -> usize {
        self.exhausted.len()
    }
    
    pub fn total_cards(&self) -> usize {
        self.hand.len() + self.deck.size() + self.discard_pile.len() + self.exhausted.len()
    }
    
    pub fn cards_in_play(&self) -> usize {
        self.hand.len() + self.deck.size() + self.discard_pile.len()
    }
    
    // Helper methods
    fn is_deck_empty(&self) -> bool {
        self.deck.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::ironclad::{strike::strike, defend::defend};

    #[test]
    fn test_deck_hand_pile_creation() {
        let cards = vec![strike(), defend(), strike(), defend(), strike()];
        let deck = Deck::new(cards);
        let original_deck_size = deck.size();
        let deck_hand_pile = DeckHandPile::new(deck.clone());
        
        // Initial hand should be empty, deck should have all cards, discard empty
        assert_eq!(deck_hand_pile.hand_size(), 0);
        assert_eq!(deck_hand_pile.deck_size(), original_deck_size);
        assert_eq!(deck_hand_pile.discard_pile_size(), 0);
        assert_eq!(deck_hand_pile.total_cards(), original_deck_size);
        
        // Original deck should be unchanged
        assert_eq!(deck.size(), original_deck_size);
    }
    
    #[test]
    fn test_discard_card() {
        let cards = vec![strike(), defend(), strike()];
        let deck = Deck::new(cards);
        let mut deck_hand_pile = DeckHandPile::new(deck);
        
        // Draw cards into hand first
        deck_hand_pile.draw_n(3);
        
        // Discard a card from hand
        let discarded = deck_hand_pile.discard_card_from_hand(0);
        assert!(discarded.is_some());
        assert_eq!(deck_hand_pile.hand_size(), 2);
        assert_eq!(deck_hand_pile.discard_pile_size(), 1);
    }
    
    #[test]
    fn test_play_card() {
        let cards = vec![strike(), defend(), strike()];
        let deck = Deck::new(cards);
        let mut deck_hand_pile = DeckHandPile::new(deck);
        
        // Draw some cards into hand first
        deck_hand_pile.draw_n(2);
        
        let initial_hand_size = deck_hand_pile.hand_size();
        let played_card = deck_hand_pile.play_card_from_hand(0);
        
        assert!(played_card.is_some());
        assert_eq!(deck_hand_pile.hand_size(), initial_hand_size - 1);
        // Playing a card now puts it in discard pile automatically
        assert_eq!(deck_hand_pile.discard_pile_size(), 1);
    }
    
    #[test]
    fn test_discard_hand() {
        let cards = vec![strike(), defend(), strike(), defend(), strike()];
        let deck = Deck::new(cards);
        let mut deck_hand_pile = DeckHandPile::new(deck);
        
        // Draw cards into hand first
        deck_hand_pile.draw_n(5);
        
        deck_hand_pile.discard_entire_hand();
        assert_eq!(deck_hand_pile.hand_size(), 0);
        assert_eq!(deck_hand_pile.discard_pile_size(), 5);
    }
    
    #[test]
    fn test_deck_reshuffling_when_empty() {
        let cards = vec![strike(), defend()];
        let deck = Deck::new(cards);
        let mut deck_hand_pile = DeckHandPile::new(deck);
        
        // Draw cards into hand first
        deck_hand_pile.draw_n(2);
        
        // Initial state: deck is empty (all cards drawn), hand has 2 cards, discard empty
        assert_eq!(deck_hand_pile.deck_size(), 0);
        assert_eq!(deck_hand_pile.hand_size(), 2);
        assert_eq!(deck_hand_pile.discard_pile_size(), 0);
        
        // Discard all cards from hand
        deck_hand_pile.discard_entire_hand();
        assert_eq!(deck_hand_pile.deck_size(), 0);
        assert_eq!(deck_hand_pile.hand_size(), 0);
        assert_eq!(deck_hand_pile.discard_pile_size(), 2);
        
        // Try to draw a card - should reshuffle discard into deck and then draw
        let drawn_card = deck_hand_pile.draw_card();
        assert!(drawn_card.is_some());
        assert_eq!(deck_hand_pile.hand_size(), 1);
        assert_eq!(deck_hand_pile.discard_pile_size(), 0);
        assert_eq!(deck_hand_pile.deck_size(), 1); // One card left in deck after drawing
        
        // Draw the second card
        let second_card = deck_hand_pile.draw_card();
        assert!(second_card.is_some());
        assert_eq!(deck_hand_pile.hand_size(), 2);
        assert_eq!(deck_hand_pile.discard_pile_size(), 0);
        assert_eq!(deck_hand_pile.deck_size(), 0); // Deck is empty again
        
        // Try to draw again - should return None (no cards left anywhere)
        let third_card = deck_hand_pile.draw_card();
        assert!(third_card.is_none());
        assert_eq!(deck_hand_pile.hand_size(), 2); // Hand unchanged
    }
    
    #[test]
    fn test_simulate_multiple_turns_with_small_deck() {
        // Test with exactly 10 cards to reproduce the issue
        let cards = vec![
            strike(), defend(), strike(), defend(), strike(),
            defend(), strike(), defend(), strike(), defend()
        ];
        let deck = Deck::new(cards);
        let mut deck_hand_pile = DeckHandPile::new(deck);
        
        // Draw initial hand of 5 cards
        deck_hand_pile.draw_n(5);
        
        // Initial state: 5 cards in hand, 5 in deck, 0 in discard
        assert_eq!(deck_hand_pile.hand_size(), 5);
        assert_eq!(deck_hand_pile.deck_size(), 5);
        assert_eq!(deck_hand_pile.discard_pile_size(), 0);
        
        // Turn 1: Discard hand, then draw new hand (5 cards)
        deck_hand_pile.discard_entire_hand();
        assert_eq!(deck_hand_pile.hand_size(), 0);
        assert_eq!(deck_hand_pile.deck_size(), 5);
        assert_eq!(deck_hand_pile.discard_pile_size(), 5);
        
        // Draw 5 cards for new turn
        let drawn_cards = deck_hand_pile.draw_n(5);
        assert_eq!(drawn_cards, 5, "Should draw 5 cards on turn 1");
        assert_eq!(deck_hand_pile.hand_size(), 5);
        assert_eq!(deck_hand_pile.deck_size(), 0); // Deck empty  
        assert_eq!(deck_hand_pile.discard_pile_size(), 5); // Discard pile has the cards we discarded
        
        // Turn 2: Discard hand again, then draw new hand
        deck_hand_pile.discard_entire_hand();
        assert_eq!(deck_hand_pile.hand_size(), 0);
        assert_eq!(deck_hand_pile.deck_size(), 0);
        assert_eq!(deck_hand_pile.discard_pile_size(), 10); // Now have 10 cards in discard (5 from each turn)
        
        let drawn_cards = deck_hand_pile.draw_n(5);
        assert_eq!(drawn_cards, 5, "Should draw 5 cards on turn 2");
        assert_eq!(deck_hand_pile.hand_size(), 5);
        assert_eq!(deck_hand_pile.deck_size(), 5); // Remaining 5 cards after drawing 5 of the 10 reshuffled
        assert_eq!(deck_hand_pile.discard_pile_size(), 0); // Discard was reshuffled into deck
        
        // Turn 3: This should work fine now
        deck_hand_pile.discard_entire_hand();
        assert_eq!(deck_hand_pile.hand_size(), 0);
        assert_eq!(deck_hand_pile.deck_size(), 5); // Still 5 cards in deck from turn 2
        assert_eq!(deck_hand_pile.discard_pile_size(), 5); // 5 cards just discarded
        
        let drawn_cards = deck_hand_pile.draw_n(5);
        
        
        assert_eq!(drawn_cards, 5, "Should draw 5 cards on turn 3");
        assert_eq!(deck_hand_pile.hand_size(), 5);
    }

    #[test]
    fn test_exhaust_card_from_hand() {
        let deck = Deck::new(vec![strike(), defend(), strike()]);
        let mut deck_hand_pile = DeckHandPile::new(deck);
        deck_hand_pile.draw_n(3);
        
        assert_eq!(deck_hand_pile.hand_size(), 3);
        assert_eq!(deck_hand_pile.exhausted_size(), 0);
        
        // Exhaust the first card
        let exhausted_card = deck_hand_pile.exhaust_card_from_hand(0);
        assert!(exhausted_card.is_some());
        assert_eq!(deck_hand_pile.hand_size(), 2);
        assert_eq!(deck_hand_pile.exhausted_size(), 1);
        
        // Exhaust an invalid index should return None
        let invalid_exhaust = deck_hand_pile.exhaust_card_from_hand(5);
        assert!(invalid_exhaust.is_none());
        assert_eq!(deck_hand_pile.hand_size(), 2);
        assert_eq!(deck_hand_pile.exhausted_size(), 1);
    }

    #[test] 
    fn test_exhaust_card_directly() {
        let deck = Deck::new(vec![strike()]);
        let mut deck_hand_pile = DeckHandPile::new(deck);
        
        assert_eq!(deck_hand_pile.exhausted_size(), 0);
        
        let card_to_exhaust = defend();
        deck_hand_pile.exhaust_card(card_to_exhaust);
        
        assert_eq!(deck_hand_pile.exhausted_size(), 1);
        assert_eq!(deck_hand_pile.get_exhausted()[0].get_name(), "Defend");
    }

    #[test]
    fn test_total_cards_with_exhausted() {
        let deck = Deck::new(vec![strike(), defend(), strike(), defend(), strike()]);
        let mut deck_hand_pile = DeckHandPile::new(deck);
        
        // Initial total should be 5
        assert_eq!(deck_hand_pile.total_cards(), 5);
        assert_eq!(deck_hand_pile.cards_in_play(), 5);
        
        // Draw 2 cards
        deck_hand_pile.draw_n(2);
        assert_eq!(deck_hand_pile.total_cards(), 5); // Still 5 total
        assert_eq!(deck_hand_pile.cards_in_play(), 5); // Still 5 in play
        
        // Exhaust one card from hand
        deck_hand_pile.exhaust_card_from_hand(0);
        assert_eq!(deck_hand_pile.total_cards(), 5); // Still 5 total
        assert_eq!(deck_hand_pile.cards_in_play(), 4); // Now only 4 in play
        
        // Discard one card
        deck_hand_pile.discard_card_from_hand(0);
        assert_eq!(deck_hand_pile.total_cards(), 5); // Still 5 total
        assert_eq!(deck_hand_pile.cards_in_play(), 4); // Still 4 in play
        
        // Check counts
        assert_eq!(deck_hand_pile.hand_size(), 0);
        assert_eq!(deck_hand_pile.deck_size(), 3);
        assert_eq!(deck_hand_pile.discard_pile_size(), 1);
        assert_eq!(deck_hand_pile.exhausted_size(), 1);
    }
}