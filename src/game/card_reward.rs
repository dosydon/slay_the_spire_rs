use crate::game::{card::Card, card_enum::CardEnum};

/// Card rarity for reward pools
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CardRarity {
    Common,
    Uncommon,
    Rare,
}

/// Card entry with weight for reward generation
#[derive(Debug, Clone)]
struct CardEntry {
    card_enum: CardEnum,
    rarity: CardRarity,
    weight: f32,
}

/// Card reward pool for generating random card rewards
pub struct CardRewardPool {
    /// All available cards for rewards with their weights
    available_cards: Vec<CardEntry>,
}

impl CardRewardPool {
    /// Create a new card reward pool with all currently implemented cards
    pub fn new() -> Self {
        let mut available_cards = Vec::new();

        // Ironclad Common Cards (weight: ~75% total pool)
        // These are the most frequently appearing rewards
        // Excluding basic cards (Strike, Defend) which should never be rewards
        let common_cards = vec![
            (CardEnum::Bash, 8.0),                 // Attack + Vulnerable
            (CardEnum::Cleave, 8.0),               // Attack all enemies
            (CardEnum::Clothesline, 8.0),         // Attack + Weak
            (CardEnum::Flex, 8.0),                 // Gain Strength
            (CardEnum::HeavyBlade, 8.0),          // High damage attack
            (CardEnum::IronWave, 8.0),            // Attack + Block
            (CardEnum::PerfectedStrike, 8.0),     // Strike synergy
            (CardEnum::PommelStrike, 8.0),        // Attack + Draw
            (CardEnum::ShrugItOff, 8.0),          // Block + Strength
            (CardEnum::Thunderclap, 8.0),         // Attack + Weak all
            (CardEnum::TwinStrike, 8.0),          // Attack twice
            (CardEnum::WildStrike, 8.0),          // Attack + Shuffle Wound
            (CardEnum::Anger, 8.0),               // Add Anger cards to draw pile
            (CardEnum::BodySlam, 8.0),            // Damage equal to Block
            (CardEnum::Combust, 8.0),             // Take damage, AoE damage on turn end
            (CardEnum::Dropkick, 8.0),            // Attack + Energy if enemy Vulnerable
            (CardEnum::Entrench, 8.0),            // Double Block
            (CardEnum::Havoc, 8.0),               // Play random Attack from draw pile
            (CardEnum::Headbutt, 8.0),            // Attack + Move card to top of draw pile
            (CardEnum::TrueGrit, 8.0),            // Block + add temporary Block card
            (CardEnum::Warcry, 8.0),              // Shuffle non-Attack cards into draw pile
            (CardEnum::RecklessCharge, 8.0),      // Add Vulnerable to self, draw card
            (CardEnum::SearingBlow, 8.0),         // Upgradeable attack
            (CardEnum::SeverSoul, 8.0),           // Attack + Exhaust + Energy
            (CardEnum::SpotWeakness, 8.0),        // High damage if enemy not Weak
            (CardEnum::Pummel, 8.0),              // Attack multiple times
            (CardEnum::InfernalBlade, 8.0),       // Low cost attack, channel Hot Feet
            (CardEnum::Evolve, 8.0),              // Transform cards in draw pile
            (CardEnum::Sentinel, 8.0),            // Block + retain
            (CardEnum::Whirlwind, 8.0),           // Multi-hit attack scaling with energy
            (CardEnum::SecondWind, 8.0),          // Exhaust damage cards, gain Block
            (CardEnum::Rupture, 8.0),             // Take damage, gain Strength
            (CardEnum::DualWield, 8.0),           // Duplicate attacks in hand
            (CardEnum::DoubleTap, 8.0),           // First card played twice
            (CardEnum::Feed, 8.0),                // Exhaust enemy card, heal
            (CardEnum::Reaper, 8.0),              // Lifesteal attack
            (CardEnum::FiendFire, 8.0),           // Exhaust attacks for damage
            (CardEnum::FireBreathing, 8.0),       // Passive damage when taking damage
        ];

        // Ironclad Uncommon Cards (weight: ~20% total pool)
        // These appear less frequently than commons but more than rares
        let uncommon_cards = vec![
            (CardEnum::Armaments, 4.0),            // Gain Block, optionally Upgrade all cards in hand
            (CardEnum::Bludgeon, 4.0),             // Attack scaling with Strength
            (CardEnum::Disarm, 4.0),               // Weak + discard cards from hand
            (CardEnum::GhostlyArmor, 4.0),         // Ethereal Block
            (CardEnum::Impervious, 4.0),           // Massive Block
            (CardEnum::PowerThrough, 4.0),         // Add temporary cards to hand, Block
            (CardEnum::Shockwave, 4.0),            // Weak all enemies
            (CardEnum::Uppercut, 4.0),             // Attack + Weak + draw
            (CardEnum::SeeingRed, 4.0),            // Energy + draw
            (CardEnum::FlameBarrier, 4.0),         // Block + passive damage
            (CardEnum::Metallicize, 4.0),          // Gain Plated Armor
            (CardEnum::Rage, 4.0),                 // Add Rage cards to deck
            (CardEnum::LimitBreak, 4.0),           // Gain Strength equal to max HP
            (CardEnum::DemonForm, 4.0),            // Power that increases Demon Form stacks
            (CardEnum::Exhume, 4.0),               // Revive exhausted card
            (CardEnum::FeelNoPain, 4.0),           // Add Block when taking damage
            (CardEnum::SwordBoomerang, 4.0),       // Attack multiple times
            (CardEnum::Clash, 4.0),                // High damage if only Attack cards in hand
        ];

        // Ironclad Rare Cards (weight: ~5% total pool)
        // These are the rarest and most powerful rewards
        let rare_cards = vec![
            (CardEnum::Carnage, 2.0),              // Massively powerful single-use attack
            (CardEnum::Corruption, 2.0),           // Powers become free, skills Exhaust
            (CardEnum::Immolate, 2.0),             // High damage AoE attack
            (CardEnum::Embrace, 2.0),              // Chaos power that randomizes effects
            (CardEnum::Inflame, 2.0),              // High Strength gain
            (CardEnum::Brutality, 2.0),            // Lose max HP, gain energy every turn
            (CardEnum::Offering, 2.0),             // Sacrifice HP for massive energy gain
            (CardEnum::Intimidate, 2.0),           // Apply Weak to all enemies, draw cards
            (CardEnum::Hemokinesis, 2.0),         // Pay HP for damage
            (CardEnum::Rampage, 2.0),              // Scalable multi-hit attack
        ];

        // Add all cards with their weights
        for (card_enum, weight) in common_cards {
            available_cards.push(CardEntry {
                card_enum,
                rarity: CardRarity::Common,
                weight,
            });
        }

        for (card_enum, weight) in uncommon_cards {
            available_cards.push(CardEntry {
                card_enum,
                rarity: CardRarity::Uncommon,
                weight,
            });
        }

        for (card_enum, weight) in rare_cards {
            available_cards.push(CardEntry {
                card_enum,
                rarity: CardRarity::Rare,
                weight,
            });
        }

        // Colorless Cards (these can appear as rewards for any character)
        // Weighted as uncommon-rarity cards, roughly 5% of total reward pool
        let colorless_cards = vec![
            (CardEnum::SwiftStrike, 3.0),          // Cheap attack
            (CardEnum::Finesse, 3.0),              // Block + Draw
            (CardEnum::FlashOfSteel, 3.0),        // Draw + play Attack from draw pile
            (CardEnum::Blind, 3.0),               // Apply Weak
            (CardEnum::Trip, 3.0),                // Vulnerable + Draw
            (CardEnum::GoodInstincts, 3.0),       // Gain Plated Armor + Draw
            (CardEnum::BandageUp, 3.0),           // Heal
            (CardEnum::DeepBreath, 3.0),          // Draw more cards if hand empty
        ];

        for (card_enum, weight) in colorless_cards {
            available_cards.push(CardEntry {
                card_enum,
                rarity: CardRarity::Uncommon, // Colorless are treated as uncommon
                weight,
            });
        }

        Self { available_cards }
    }

    /// Get the number of cards in the reward pool
    pub fn pool_size(&self) -> usize {
        self.available_cards.len()
    }

    /// Get cards by rarity
    pub fn get_cards_by_rarity(&self, rarity: CardRarity) -> Vec<CardEnum> {
        self.available_cards
            .iter()
            .filter(|entry| entry.rarity == rarity)
            .map(|entry| entry.card_enum)
            .collect()
    }

    /// Get rarity distribution as percentages
    pub fn get_rarity_distribution(&self) -> (f32, f32, f32) {
        let total_weight: f32 = self.available_cards.iter().map(|entry| entry.weight).sum();

        let common_weight: f32 = self.available_cards
            .iter()
            .filter(|entry| entry.rarity == CardRarity::Common)
            .map(|entry| entry.weight)
            .sum();

        let uncommon_weight: f32 = self.available_cards
            .iter()
            .filter(|entry| entry.rarity == CardRarity::Uncommon)
            .map(|entry| entry.weight)
            .sum();

        let rare_weight: f32 = self.available_cards
            .iter()
            .filter(|entry| entry.rarity == CardRarity::Rare)
            .map(|entry| entry.weight)
            .sum();

        (
            (common_weight / total_weight) * 100.0,
            (uncommon_weight / total_weight) * 100.0,
            (rare_weight / total_weight) * 100.0,
        )
    }

    /// Generate a single random card reward using weighted selection
    pub fn generate_single_reward(&self, rng: &mut impl rand::Rng) -> Option<Card> {
        let total_weight: f32 = self.available_cards.iter().map(|entry| entry.weight).sum();
        let random_value = rng.random_range(0.0..total_weight);
        let mut cumulative_weight = 0.0;

        if let Some(selected_entry) = self.available_cards.iter().find(|entry| {
            cumulative_weight += entry.weight;
            random_value <= cumulative_weight
        }) {
            self.try_create_card_from_enum(selected_entry.card_enum)
        } else {
            None
        }
    }

    /// Generate 3 random card reward options using weighted selection
    pub fn generate_reward_options(&self, rng: &mut impl rand::Rng) -> Vec<Card> {
        let mut options = Vec::new();
        let mut used_card_enums = Vec::new();

        // Create weighted distribution for card selection
        let total_weight: f32 = self.available_cards.iter().map(|entry| entry.weight).sum();

        // Generate 3 unique cards using weighted selection
        while options.len() < 3 && used_card_enums.len() < self.available_cards.len() {
            let random_value = rng.random_range(0.0..total_weight);
            let mut cumulative_weight = 0.0;

            // Select card based on weight
            let selected_entry = self.available_cards.iter().find(|entry| {
                cumulative_weight += entry.weight;
                random_value <= cumulative_weight
            }).unwrap_or(&self.available_cards[0]);

            // Ensure we don't have duplicate options
            if !used_card_enums.contains(&selected_entry.card_enum) {
                used_card_enums.push(selected_entry.card_enum);

                // Check if the card is implemented before creating it
                if let Some(card) = self.try_create_card_from_enum(selected_entry.card_enum) {
                    options.push(card);
                }
            }
        }

        // If we couldn't get 3 unique cards, pad with available cards (allowing duplicates)
        while options.len() < 3 {
            let random_value = rng.random_range(0.0..total_weight);
            let mut cumulative_weight = 0.0;

            let selected_entry = self.available_cards.iter().find(|entry| {
                cumulative_weight += entry.weight;
                random_value <= cumulative_weight
            }).unwrap_or(&self.available_cards[0]);

            if let Some(card) = self.try_create_card_from_enum(selected_entry.card_enum) {
                options.push(card);
            }
        }

        options
    }

    /// Try to create a card from a CardEnum, returning None if not implemented
    fn try_create_card_from_enum(&self, card_enum: CardEnum) -> Option<Card> {
        // Use catch_unwind to handle any panics from unimplemented cards
        std::panic::catch_unwind(|| self.create_card_from_enum(card_enum))
            .ok()
            .filter(|card| {
                // Filter out cards with invalid names or empty effects
                let name = card.get_name();
                !name.is_empty() && name != "UNIMPLEMENTED" && !name.contains("TODO")
            })
    }

    /// Create a card from a CardEnum using the appropriate factory function
    /// Returns a placeholder TODO card if the specific card is not implemented
    pub fn create_card_from_enum(&self, card_enum: CardEnum) -> Card {
        match card_enum {
            // Ironclad Common Cards
            CardEnum::Bash => crate::cards::ironclad::bash::bash(),
            CardEnum::Cleave => crate::cards::ironclad::cleave::cleave(),
            CardEnum::Clothesline => crate::cards::ironclad::clothesline::clothesline(),
            CardEnum::Flex => crate::cards::ironclad::flex::flex(),
            CardEnum::HeavyBlade => crate::cards::ironclad::heavy_blade::heavy_blade(),
            CardEnum::IronWave => crate::cards::ironclad::iron_wave::iron_wave(),
            CardEnum::PerfectedStrike => crate::cards::ironclad::perfected_strike::perfected_strike(),
            CardEnum::PommelStrike => crate::cards::ironclad::pommel_strike::pommel_strike(),
            CardEnum::ShrugItOff => crate::cards::ironclad::shrug_it_off::shrug_it_off(),
            CardEnum::Thunderclap => crate::cards::ironclad::thunderclap::thunderclap(),
            CardEnum::TwinStrike => crate::cards::ironclad::twin_strike::twin_strike(),
            CardEnum::WildStrike => crate::cards::ironclad::wild_strike::wild_strike(),
            CardEnum::Anger => crate::cards::ironclad::anger::anger(),
            CardEnum::BodySlam => crate::cards::ironclad::body_slam::body_slam(),
            CardEnum::Combust => crate::cards::ironclad::combust::combust(),
            CardEnum::Dropkick => crate::cards::ironclad::dropkick::dropkick(),
            CardEnum::Entrench => crate::cards::ironclad::entrench::entrench(),
            CardEnum::Havoc => crate::cards::ironclad::havoc::havoc(),
            CardEnum::Headbutt => crate::cards::ironclad::headbutt::headbutt(),
            CardEnum::TrueGrit => crate::cards::ironclad::true_grit::true_grit(),
            CardEnum::Warcry => crate::cards::ironclad::warcry::warcry(),
            CardEnum::RecklessCharge => crate::cards::ironclad::reckless_charge::reckless_charge(),
            CardEnum::SearingBlow => crate::cards::ironclad::searing_blow::searing_blow(),
            CardEnum::SeverSoul => crate::cards::ironclad::sever_soul::sever_soul(),
            CardEnum::SpotWeakness => crate::cards::ironclad::spot_weakness::spot_weakness(),
            CardEnum::Pummel => crate::cards::ironclad::pummel::pummel(),
            CardEnum::InfernalBlade => crate::cards::ironclad::infernal_blade::infernal_blade(),
            CardEnum::Evolve => crate::cards::ironclad::evolve::evolve(),
            CardEnum::Sentinel => crate::cards::ironclad::sentinel::sentinel(),
            CardEnum::Whirlwind => crate::cards::ironclad::whirlwind::whirlwind(),
            CardEnum::SecondWind => crate::cards::ironclad::second_wind::second_wind(),
            CardEnum::Rupture => crate::cards::ironclad::rupture::rupture(),
            CardEnum::DualWield => crate::cards::ironclad::dual_wield::dual_wield(),
            CardEnum::DoubleTap => crate::cards::ironclad::double_tap::double_tap(),
            CardEnum::Feed => crate::cards::ironclad::feed::feed(),
            CardEnum::Reaper => crate::cards::ironclad::reaper::reaper(),
            CardEnum::FiendFire => crate::cards::ironclad::fiend_fire::fiend_fire(),
            CardEnum::FireBreathing => crate::cards::ironclad::fire_breathing::fire_breathing(),

            // Ironclad Uncommon Cards
            CardEnum::Armaments => crate::cards::ironclad::armaments::armaments(),
            CardEnum::Bludgeon => crate::cards::ironclad::bludgeon::bludgeon(),
            CardEnum::Disarm => crate::cards::ironclad::disarm::disarm(),
            CardEnum::GhostlyArmor => crate::cards::ironclad::ghostly_armor::ghostly_armor(),
            CardEnum::Impervious => crate::cards::ironclad::impervious::impervious(),
            CardEnum::PowerThrough => crate::cards::ironclad::power_through::power_through(),
            CardEnum::Shockwave => crate::cards::ironclad::shockwave::shockwave(),
            CardEnum::Uppercut => crate::cards::ironclad::uppercut::uppercut(),
            CardEnum::SeeingRed => crate::cards::ironclad::seeing_red::seeing_red(),
            CardEnum::FlameBarrier => crate::cards::ironclad::flame_barrier::flame_barrier(),
            CardEnum::Metallicize => crate::cards::ironclad::metallicize::metallicize(),
            CardEnum::Rage => crate::cards::ironclad::rage::rage(),
            CardEnum::LimitBreak => crate::cards::ironclad::limit_break::limit_break(),
            CardEnum::DemonForm => crate::cards::ironclad::demon_form::demon_form(),
            CardEnum::Exhume => crate::cards::ironclad::exhume::exhume(),

            // Ironclad Rare Cards
            CardEnum::Carnage => crate::cards::ironclad::carnage::carnage(),
            CardEnum::Corruption => crate::cards::ironclad::corruption::corruption(),
            CardEnum::Immolate => crate::cards::ironclad::immolate::immolate(),
            CardEnum::Embrace => crate::cards::ironclad::embrace::embrace(),
            CardEnum::Inflame => crate::cards::ironclad::inflame::inflame(),
            CardEnum::Brutality => crate::cards::ironclad::brutality::brutality(),
            CardEnum::Offering => crate::cards::ironclad::offering::offering(),
            CardEnum::Intimidate => crate::cards::ironclad::intimidate::intimidate(),
            CardEnum::Hemokinesis => crate::cards::ironclad::hemokinesis::hemokinesis(),
            CardEnum::Rampage => crate::cards::ironclad::rampage::rampage(),

            // Ironclad Other Cards
            CardEnum::SwordBoomerang => crate::cards::ironclad::sword_boomerang::sword_boomerang(),
            CardEnum::FeelNoPain => crate::cards::ironclad::feel_no_pain::feel_no_pain(),
            CardEnum::Clash => crate::cards::ironclad::clash::clash(),

            // Basic cards (Strike/Defend should not be in reward pools)
            CardEnum::Strike => crate::cards::ironclad::strike::strike(),
            CardEnum::Defend => crate::cards::ironclad::defend::defend(),

            // Colorless Cards
            CardEnum::SwiftStrike => crate::cards::colorless::swift_strike::swift_strike(),
            CardEnum::Finesse => crate::cards::colorless::finesse::finesse(),
            CardEnum::FlashOfSteel => crate::cards::colorless::flash_of_steel::flash_of_steel(),
            CardEnum::Blind => crate::cards::colorless::blind::blind(),
            CardEnum::Trip => crate::cards::colorless::trip::trip(),
            CardEnum::GoodInstincts => crate::cards::colorless::good_instincts::good_instincts(),
            CardEnum::BandageUp => crate::cards::colorless::bandage_up::bandage_up(),
            CardEnum::DeepBreath => crate::cards::colorless::deep_breath::deep_breath(),

            // Status/Curse Cards (should not be in reward pools but included for completeness)
            CardEnum::Slimed => crate::cards::status::slimed::slimed(),
            CardEnum::Wound => crate::cards::status::wound::wound(),
            CardEnum::Burn => crate::cards::status::burn::burn(),
            CardEnum::Dazed => crate::cards::status::dazed::dazed(),
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
        assert!(pool.pool_size() >= 50); // Should have many Ironclad cards + colorless
    }

    #[test]
    fn test_rarity_distribution() {
        let pool = CardRewardPool::new();
        let (common_pct, uncommon_pct, rare_pct) = pool.get_rarity_distribution();

        // Check that we have a reasonable distribution
        assert!(common_pct > 70.0, "Common cards should be > 70%: {:.1}%", common_pct);
        assert!(uncommon_pct > 15.0, "Uncommon cards should be > 15%: {:.1}%", uncommon_pct);
        assert!(rare_pct > 2.0, "Rare cards should be > 2%: {:.1}%", rare_pct);

        // Should roughly sum to 100%
        let total = common_pct + uncommon_pct + rare_pct;
        assert!((total - 100.0).abs() < 0.1, "Distribution should sum to 100%: {:.1}%", total);
    }

    #[test]
    fn test_cards_by_rarity() {
        let pool = CardRewardPool::new();

        let common_cards = pool.get_cards_by_rarity(CardRarity::Common);
        let uncommon_cards = pool.get_cards_by_rarity(CardRarity::Uncommon);
        let rare_cards = pool.get_cards_by_rarity(CardRarity::Rare);

        // Should have cards of each rarity
        assert!(!common_cards.is_empty(), "Should have common cards");
        assert!(!uncommon_cards.is_empty(), "Should have uncommon cards");
        assert!(!rare_cards.is_empty(), "Should have rare cards");

        // Basic cards should not be in reward pool
        assert!(!common_cards.contains(&CardEnum::Strike), "Strike should not be in rewards");
        assert!(!common_cards.contains(&CardEnum::Defend), "Defend should not be in rewards");
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
            assert!(!option.get_name().is_empty());
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

    #[test]
    fn test_no_basic_cards_in_pool() {
        let pool = CardRewardPool::new();
        let all_card_enums: Vec<CardEnum> = pool.available_cards.iter().map(|entry| entry.card_enum).collect();

        // Basic cards should never be in reward pool
        assert!(!all_card_enums.contains(&CardEnum::Strike), "Strike should not be in reward pool");
        assert!(!all_card_enums.contains(&CardEnum::Defend), "Defend should not be in reward pool");

        // Status/Curse cards should not be in reward pool
        assert!(!all_card_enums.contains(&CardEnum::Wound), "Wound should not be in reward pool");
        assert!(!all_card_enums.contains(&CardEnum::Slimed), "Slimed should not be in reward pool");
        assert!(!all_card_enums.contains(&CardEnum::Dazed), "Dazed should not be in reward pool");
        assert!(!all_card_enums.contains(&CardEnum::Burn), "Burn should not be in reward pool");
    }
}