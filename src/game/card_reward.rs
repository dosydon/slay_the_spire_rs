use crate::game::{card::Card, card_enum::CardEnum};
use rand::seq::SliceRandom;
use rand::prelude::IndexedRandom;

/// Card reward pool for generating random card rewards
pub struct CardRewardPool {
    /// All available cards for rewards (excluding basic strike/defend which shouldn't be rewards)
    available_cards: Vec<CardEnum>,
}

impl CardRewardPool {
    /// Create a new card reward pool with all currently implemented cards
    pub fn new() -> Self {
        let available_cards = vec![
            // Ironclad Common Cards (excluding basic cards)
            CardEnum::Bash,
            CardEnum::Cleave,
            CardEnum::Clothesline,
            CardEnum::Flex,
            CardEnum::HeavyBlade,
            CardEnum::IronWave,
            CardEnum::PerfectedStrike,
            CardEnum::PommelStrike,
            CardEnum::ShrugItOff,
            CardEnum::Thunderclap,
            CardEnum::TwinStrike,
            CardEnum::WildStrike,
            // Status cards typically aren't rewards (exclude Slimed, Wound)
        ];

        Self { available_cards }
    }

    /// Generate 3 random card reward options
    pub fn generate_reward_options(&self, rng: &mut impl rand::Rng) -> Vec<Card> {
        let mut options = Vec::new();
        let mut used_cards = Vec::new();

        // Generate 3 unique cards
        while options.len() < 3 && used_cards.len() < self.available_cards.len() {
            let random_card = self.available_cards.choose(rng).unwrap();

            // Ensure we don't have duplicate options
            if !used_cards.contains(&random_card) {
                used_cards.push(random_card);
                options.push(self.create_card_from_enum(*random_card));
            }
        }

        // If we couldn't get 3 unique cards, pad with random cards (allowing duplicates)
        while options.len() < 3 {
            let random_card = self.available_cards.choose(rng).unwrap();
            options.push(self.create_card_from_enum(*random_card));
        }

        options
    }

    /// Create a card from a CardEnum using the appropriate factory function
    fn create_card_from_enum(&self, card_enum: CardEnum) -> Card {
        match card_enum {
            CardEnum::Strike => crate::cards::ironclad::strike::strike(),
            CardEnum::Defend => crate::cards::ironclad::defend::defend(),
            CardEnum::Bash => crate::cards::ironclad::bash::bash(),
            CardEnum::Cleave => crate::cards::ironclad::cleave::cleave(),
            CardEnum::Clothesline => crate::cards::ironclad::clothesline::clothesline(),
            CardEnum::Embrace => crate::cards::ironclad::embrace::embrace(),
            CardEnum::Flex => crate::cards::ironclad::flex::flex(),
            CardEnum::Inflame => crate::cards::ironclad::inflame::inflame(),
            CardEnum::HeavyBlade => crate::cards::ironclad::heavy_blade::heavy_blade(),
            CardEnum::IronWave => crate::cards::ironclad::iron_wave::iron_wave(),
            CardEnum::PerfectedStrike => crate::cards::ironclad::perfected_strike::perfected_strike(),
            CardEnum::PommelStrike => crate::cards::ironclad::pommel_strike::pommel_strike(),
            CardEnum::ShrugItOff => crate::cards::ironclad::shrug_it_off::shrug_it_off(),
            CardEnum::Thunderclap => crate::cards::ironclad::thunderclap::thunderclap(),
            CardEnum::TwinStrike => crate::cards::ironclad::twin_strike::twin_strike(),
            CardEnum::WildStrike => crate::cards::ironclad::wild_strike::wild_strike(),
            CardEnum::Slimed => crate::cards::status::slimed::slimed(),
            CardEnum::Wound => crate::cards::status::wound::wound(),
        }
    }
}

impl Default for CardRewardPool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn test_card_reward_pool_creation() {
        let pool = CardRewardPool::new();
        assert!(!pool.available_cards.is_empty());
        assert!(pool.available_cards.len() >= 12); // At least the implemented Ironclad cards
    }

    #[test]
    fn test_generate_reward_options() {
        let pool = CardRewardPool::new();
        let mut rng = StdRng::seed_from_u64(42);

        let options = pool.generate_reward_options(&mut rng);
        assert_eq!(options.len(), 3);

        // Verify all options are valid cards
        for option in &options {
            assert!(option.get_cost() <= 3); // Cards shouldn't have absurd costs
        }
    }

    #[test]
    fn test_generate_multiple_times() {
        let pool = CardRewardPool::new();
        let mut rng = StdRng::seed_from_u64(42);

        // Generate rewards multiple times to ensure randomness
        let options1 = pool.generate_reward_options(&mut rng);
        let options2 = pool.generate_reward_options(&mut rng);

        // Should be different (most likely, given randomness)
        assert_ne!(options1, options2);
    }

    #[test]
    fn test_no_duplicates_in_single_reward() {
        let pool = CardRewardPool::new();
        let mut rng = StdRng::seed_from_u64(42);

        let options = pool.generate_reward_options(&mut rng);

        // Check for duplicates in a single reward set
        let mut card_names = Vec::new();
        for option in &options {
            let name = option.get_name();
            assert!(!card_names.contains(&name), "Found duplicate card: {}", name);
            card_names.push(name);
        }
    }
}