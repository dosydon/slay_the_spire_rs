use crate::game::card::Card;

/// State for a shop visit
#[derive(Debug, Clone, PartialEq)]
pub struct ShopState {
    /// Cards available for purchase
    pub cards_for_sale: Vec<Card>,
    /// Prices for each card (index corresponds to cards_for_sale)
    pub card_prices: Vec<u32>,
}

impl ShopState {
    /// Create a new shop with the specified number of random cards for sale
    pub fn new(num_cards: usize, rng: &mut impl rand::Rng) -> Self {
        let card_pool = ShopCardPool::new();
        let (cards, prices) = card_pool.generate_shop_inventory(num_cards, rng);

        Self {
            cards_for_sale: cards,
            card_prices: prices,
        }
    }

    /// Get the number of cards available for sale
    pub fn card_count(&self) -> usize {
        self.cards_for_sale.len()
    }

    /// Get the card at the specified index
    pub fn get_card(&self, index: usize) -> Option<&Card> {
        self.cards_for_sale.get(index)
    }

    /// Get the price for the card at the specified index
    pub fn get_card_price(&self, index: usize) -> Option<u32> {
        self.card_prices.get(index).copied()
    }

    /// Purchase a card from the shop
    pub fn purchase_card(&mut self, index: usize) -> Option<Card> {
        if index < self.cards_for_sale.len() {
            self.card_prices.remove(index);
            Some(self.cards_for_sale.remove(index))
        } else {
            None
        }
    }
}

/// Pool of cards that can appear in the shop
pub struct ShopCardPool {
    available_cards: Vec<Card>,
}

impl ShopCardPool {
    /// Create a new shop card pool with all cards that can be sold
    pub fn new() -> Self {
        // For now, we'll use a simple approach with some basic cards
        // In a full implementation, this would be more comprehensive
        let mut available_cards = Vec::new();

        // Add Ironclad cards (basic attack/defend and some commons)
        available_cards.extend(vec![
            crate::cards::ironclad::strike::strike(),
            crate::cards::ironclad::defend::defend(),
            crate::cards::ironclad::bash::bash(),
            crate::cards::ironclad::armaments::armaments(),
            crate::cards::ironclad::body_slam::body_slam(),
            crate::cards::ironclad::clash::clash(),
            crate::cards::ironclad::cleave::cleave(),
            crate::cards::ironclad::flex::flex(),
            crate::cards::ironclad::heavy_blade::heavy_blade(),
            crate::cards::ironclad::iron_wave::iron_wave(),
            crate::cards::ironclad::pommel_strike::pommel_strike(),
            crate::cards::ironclad::shockwave::shockwave(),
            crate::cards::ironclad::sword_boomerang::sword_boomerang(),
            crate::cards::ironclad::thunderclap::thunderclap(),
            crate::cards::ironclad::twin_strike::twin_strike(),
            crate::cards::ironclad::uppercut::uppercut(),
            crate::cards::ironclad::whirlwind::whirlwind(),
            crate::cards::ironclad::warcry::warcry(),
            crate::cards::ironclad::true_grit::true_grit(),
        ]);

        Self {
            available_cards,
        }
    }

    /// Generate shop inventory with specified number of cards
    pub fn generate_shop_inventory(&self, num_cards: usize, rng: &mut impl rand::Rng) -> (Vec<Card>, Vec<u32>) {
        let mut cards = Vec::new();
        let mut prices = Vec::new();

        for _ in 0..num_cards {
            if let Some(card) = self.get_random_card(rng) {
                let price = self.calculate_card_price(&card, rng);
                cards.push(card);
                prices.push(price);
            }
        }

        (cards, prices)
    }

    /// Get a random card from the pool (could be upgraded)
    fn get_random_card(&self, rng: &mut impl rand::Rng) -> Option<Card> {
        if self.available_cards.is_empty() {
            return None;
        }

        let base_card = self.available_cards[rng.random_range(0..self.available_cards.len())].clone();

        // 20% chance for the card to be upgraded
        if rng.random_bool(0.2) && !base_card.is_upgraded() {
            Some(base_card.upgrade())
        } else {
            Some(base_card)
        }
    }

    /// Calculate price for a card based on its properties
    fn calculate_card_price(&self, card: &Card, rng: &mut impl rand::Rng) -> u32 {
        let base_price = match card.get_cost() {
            0 => 0,   // Zero cost cards are free
            1 => 30,  // 1-cost cards
            2 => 50,  // 2-cost cards
            3 => 75,  // 3-cost cards
            _ => 100, // Higher cost cards
        };

        let upgrade_bonus = if card.is_upgraded() { 25 } else { 0 };

        // Add some randomness (±20%)
        let variance = (base_price + upgrade_bonus) / 5;
        let random_adjustment = rng.random_range(0..=(variance * 2)) as i32 - variance as i32;

        ((base_price + upgrade_bonus) as i32 + random_adjustment).max(0) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shop_state_creation() {
        let mut rng = rand::rng();
        let shop = ShopState::new(5, &mut rng);

        assert_eq!(shop.card_count(), 5);
        assert_eq!(shop.cards_for_sale.len(), 5);
        assert_eq!(shop.card_prices.len(), 5);
    }

    #[test]
    fn test_get_card_and_price() {
        let mut rng = rand::rng();
        let shop = ShopState::new(3, &mut rng);

        // Test valid indices
        for i in 0..shop.card_count() {
            assert!(shop.get_card(i).is_some());
            assert!(shop.get_card_price(i).is_some());
        }

        // Test invalid index
        assert!(shop.get_card(shop.card_count()).is_none());
        assert!(shop.get_card_price(shop.card_count()).is_none());
    }

    #[test]
    fn test_purchase_card() {
        let mut rng = rand::rng();
        let mut shop = ShopState::new(3, &mut rng);
        let initial_count = shop.card_count();

        // Purchase first card
        let purchased_card = shop.purchase_card(0);
        assert!(purchased_card.is_some());
        assert_eq!(shop.card_count(), initial_count - 1);
        assert_eq!(shop.card_prices.len(), initial_count - 1);

        // Try to purchase with invalid index
        let invalid_purchase = shop.purchase_card(shop.card_count());
        assert!(invalid_purchase.is_none());
        assert_eq!(shop.card_count(), initial_count - 1);
    }

    #[test]
    fn test_card_pricing() {
        let mut rng = rand::rng();
        let shop = ShopState::new(10, &mut rng);

        for (i, card) in shop.cards_for_sale.iter().enumerate() {
            let price = shop.get_card_price(i).unwrap();

            // Prices should be reasonable (0-200 range)
            assert!(price <= 200);

            // Base pricing logic test - prices should be in reasonable ranges
            let base_price = match card.get_cost() {
                0 => 0,
                1 => 30,
                2 => 50,
                3 => 75,
                _ => 100,
            };

            let upgrade_bonus = if card.is_upgraded() { 25 } else { 0 };
            let expected_base = base_price + upgrade_bonus;

            // Price should be within ±20% of expected, but not negative
            let variance = expected_base / 5;
            let expected_min = if expected_base > 0 { expected_base - variance } else { 0 };

            assert!(price >= expected_min);
        }
    }
}