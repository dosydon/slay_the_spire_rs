use crate::game::{card::Card, deck::Deck};
use rand::Rng;

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

    /// Draw initial hand with innate cards (only used at the start of combat)
    /// Innate cards are added to hand from deck or discard, then draw additional cards up to n
    pub(in crate::battle) fn draw_initial_hand(&mut self, n: usize) -> usize {
        // First, ensure all innate cards are in hand
        self.ensure_innate_cards_in_hand();

        // Then draw additional cards up to n
        let current_hand_size = self.hand_size();
        let cards_to_draw = n.saturating_sub(current_hand_size);
        self.draw_n(cards_to_draw)
    }

    /// Ensure all innate cards from deck are in hand
    /// This is called only at the start of combat
    fn ensure_innate_cards_in_hand(&mut self) {
        // Collect indices of innate cards not currently in hand
        let innate_indices: Vec<usize> = (0..self.deck.size())
            .filter(|&i| {
                if let Some(card) = self.deck.get_card(i) {
                    card.is_innate()
                } else {
                    false
                }
            })
            .collect();

        // Move innate cards from deck to hand
        for i in innate_indices.into_iter().rev() {
            if let Some(card) = self.deck.remove_card(i) {
                self.hand.push(card);
            }
        }

        // Check discard pile for innate cards
        let innate_discard_indices: Vec<usize> = (0..self.discard_pile.len())
            .filter(|&i| self.discard_pile[i].is_innate())
            .collect();

        // Move innate cards from discard to hand
        for i in innate_discard_indices.into_iter().rev() {
            let card = self.discard_pile.remove(i);
            self.hand.push(card);
        }
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

    pub(in crate::battle) fn replace_card_in_hand(&mut self, hand_index: usize, new_card: Card) {
        if hand_index < self.hand.len() {
            self.hand[hand_index] = new_card;
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
    
    pub(in crate) fn add_card_to_hand(&mut self, card: Card) {
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
    
    /// Look at the top card of the draw pile without removing it
    pub(in crate) fn peek_top_card(&mut self) -> Option<Card> {
        // If deck is empty, shuffle discard pile into deck
        if self.is_deck_empty() && !self.discard_pile.is_empty() {
            self.shuffle_discard_into_deck();
        }

        // Return top card without removing it
        self.deck.peek_top_card()
    }

    /// Put a card on top of the draw pile
    pub(in crate::battle) fn put_card_on_top_of_deck(&mut self, card: Card) {
        self.deck.put_card_on_top(card);
    }

    /// Draw top card from deck without adding to hand
    pub(in crate::battle) fn draw_top_card(&mut self) -> Option<Card> {
        // If deck is empty, shuffle discard pile into deck
        if self.is_deck_empty() && !self.discard_pile.is_empty() {
            self.shuffle_discard_into_deck();
        }

        self.deck.draw_card()
    }

    /// Put a random card from discard pile on top of draw pile
    pub(in crate::battle) fn put_random_discard_on_top(&mut self) -> bool {
        if self.discard_pile.is_empty() {
            return false;
        }

        let mut rng = rand::rng();
        let random_index = rng.random_range(0..self.discard_pile.len());
        let card = self.discard_pile.remove(random_index);
        self.put_card_on_top_of_deck(card);
        true
    }

    /// Remove a card from discard pile at specific index
    pub(in crate::battle) fn remove_from_discard_pile(&mut self, index: usize) -> Option<Card> {
        if index < self.discard_pile.len() {
            Some(self.discard_pile.remove(index))
        } else {
            None
        }
    }

    /// Remove a card from exhausted pile at specific index
    pub(in crate::battle) fn remove_card_from_exhausted(&mut self, index: usize) -> Option<Card> {
        if index < self.exhausted.len() {
            Some(self.exhausted.remove(index))
        } else {
            None
        }
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

    // Remove card from hand without adding to discard (for power cards)
    pub(in crate::battle) fn remove_card_from_hand(&mut self, hand_index: usize) -> Option<Card> {
        if hand_index < self.hand.len() {
            Some(self.hand.remove(hand_index))
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

    /// Clear temporary cost modifications from all cards in hand (called at end of turn)
    pub fn clear_hand_modified_costs(&mut self) {
        for card in &mut self.hand {
            card.clear_modified_cost();
        }
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

    /// Upgrade all cards in hand for the rest of combat
    pub(in crate) fn upgrade_all_cards_in_hand(&mut self) {
        // Process each card in hand and upgrade if not already upgraded
        for i in 0..self.hand.len() {
            let card = &mut self.hand[i];
            if !card.is_upgraded() {
                let upgraded_card = card.clone().upgrade();
                *card = upgraded_card;
            }
        }
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

    #[test]
    fn test_innate_cards_in_starting_hand() {
        // Create a deck with one innate card
        let innate_card = strike().set_innate(true);
        let cards = vec![
            innate_card.clone(),
            defend(),
            strike(),
            defend(),
            strike()
        ];
        let deck = Deck::new(cards);
        let mut deck_hand_pile = DeckHandPile::new(deck);

        // Draw initial hand - should have innate card + 4 other cards
        deck_hand_pile.draw_initial_hand(5);
        assert_eq!(deck_hand_pile.hand_size(), 5);

        // Verify innate card is in hand
        let hand = deck_hand_pile.get_hand();
        assert!(hand.iter().any(|c| c.is_innate()), "Innate card should be in starting hand");
    }

    #[test]
    fn test_innate_cards_discarded_at_end_of_turn() {
        // Create a deck with one innate card
        let innate_card = strike().set_innate(true);
        let cards = vec![innate_card.clone(), defend()];
        let deck = Deck::new(cards);
        let mut deck_hand_pile = DeckHandPile::new(deck);

        // Draw initial hand
        deck_hand_pile.draw_initial_hand(2);
        assert_eq!(deck_hand_pile.hand_size(), 2);
        assert!(deck_hand_pile.get_hand().iter().any(|c| c.is_innate()), "Innate card should be in starting hand");

        // Discard entire hand - innate card should go to discard pile
        deck_hand_pile.discard_entire_hand();
        assert_eq!(deck_hand_pile.hand_size(), 0);
        assert_eq!(deck_hand_pile.discard_pile_size(), 2); // Both cards in discard
    }

    #[test]
    fn test_innate_cards_drawn_naturally_from_discard() {
        // Create a deck with one innate card
        let innate_card = strike().set_innate(true);
        let cards = vec![innate_card.clone(), defend()];
        let deck = Deck::new(cards);
        let mut deck_hand_pile = DeckHandPile::new(deck);

        // Draw initial hand
        deck_hand_pile.draw_initial_hand(2);
        assert_eq!(deck_hand_pile.hand_size(), 2);

        // Discard entire hand
        deck_hand_pile.discard_entire_hand();
        assert_eq!(deck_hand_pile.hand_size(), 0);
        assert_eq!(deck_hand_pile.discard_pile_size(), 2);

        // Draw normally - innate card should be drawn like any other card
        let drawn = deck_hand_pile.draw_n(2);
        assert_eq!(drawn, 2);
        assert_eq!(deck_hand_pile.hand_size(), 2);
        assert_eq!(deck_hand_pile.discard_pile_size(), 0);

        // Verify innate card is in hand (it was drawn from discard pile)
        let hand = deck_hand_pile.get_hand();
        assert!(hand.iter().any(|c| c.is_innate()), "Innate card should be drawn from discard");
    }

    #[test]
    fn test_writhe_innate_behavior() {
        // Create a deck with Writhe curse (which is innate)
        let writhe = crate::cards::curse::writhe();
        let cards = vec![writhe.clone(), strike(), defend()];
        let deck = Deck::new(cards);
        let mut deck_hand_pile = DeckHandPile::new(deck);

        // Draw initial hand - Writhe should be in hand
        deck_hand_pile.draw_initial_hand(3);
        assert_eq!(deck_hand_pile.hand_size(), 3);

        // Verify Writhe is in hand
        let hand = deck_hand_pile.get_hand();
        assert!(hand.iter().any(|c| c.get_name() == "Writhe"), "Writhe should be in starting hand");

        // Verify Writhe properties
        let writhe_in_hand = hand.iter().find(|c| c.get_name() == "Writhe").unwrap();
        assert!(writhe_in_hand.is_innate(), "Writhe should be innate");
        assert!(!writhe_in_hand.is_playable(), "Writhe should be unplayable");
        assert!(writhe_in_hand.is_removable(), "Writhe should be removable");

        // Discard entire hand - Writhe should go to discard
        deck_hand_pile.discard_entire_hand();
        assert_eq!(deck_hand_pile.hand_size(), 0);
        assert_eq!(deck_hand_pile.discard_pile_size(), 3);
    }
}