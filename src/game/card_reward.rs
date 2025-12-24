use crate::game::{card::Card, card_enum::CardEnum};
use crate::utils::categorical_distribution::CategoricalDistribution;

/// Types of combat encounters for determining reward rarity
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CombatType {
    Normal,
    Elite,
    Boss,
}

/// Card rarity for categorizing cards
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
}

/// Card reward pool for generating random card rewards
/// Uses uniform distribution within each rarity tier and implements offset system
/// based on the actual Slay the Spire mechanics
pub struct CardRewardPool {
    /// Common cards - drawn with equal probability
    common_pool: Vec<CardEnum>,
    /// Uncommon cards - drawn with equal probability
    uncommon_pool: Vec<CardEnum>,
    /// Rare cards - drawn with equal probability
    rare_pool: Vec<CardEnum>,
    /// Rare card offset: starts at -5%, increases by 1% per common card rolled,
    /// resets to -5% when rare card is rolled. Capped at +40%.
    /// Negative offset decreases rare chance, positive offset decreases common chance.
    rare_offset_percent: i32,
}

impl CardRewardPool {
    /// Create a new card reward pool with all currently implemented cards
    /// Offset starts at -5% as per Slay the Spire mechanics
    ///
    /// This dynamically builds pools by iterating through all CardEnum values
    /// and categorizing them by rarity, filtering out non-rewardable cards
    /// (basic cards, status cards, and curse cards)
    pub fn new() -> Self {
        use crate::game::card_enum::CardEnum;

        let mut common_pool = Vec::new();
        let mut uncommon_pool = Vec::new();
        let mut rare_pool = Vec::new();

        // Dynamically iterate through all CardEnum values
        // Note: We need to explicitly list all cards here since CardEnum is not iterable
        // This is still better than manually maintaining separate pool lists
        let all_cards = [
            // Ironclad Cards
            CardEnum::Strike,
            CardEnum::Defend,
            CardEnum::Bash,
            CardEnum::BodySlam,
            CardEnum::Clash,
            CardEnum::Carnage,
            CardEnum::Cleave,
            CardEnum::Embrace,
            CardEnum::Flex,
            CardEnum::Inflame,
            CardEnum::Immolate,
            CardEnum::IronWave,
            CardEnum::PommelStrike,
            CardEnum::PowerThrough,
            CardEnum::ShrugItOff,
            CardEnum::TwinStrike,
            CardEnum::Clothesline,
            CardEnum::HeavyBlade,
            CardEnum::PerfectedStrike,
            CardEnum::Thunderclap,
            CardEnum::WildStrike,
            CardEnum::Combust,
            CardEnum::Disarm,
            CardEnum::Dropkick,
            CardEnum::FeelNoPain,
            CardEnum::Entrench,
            CardEnum::Bludgeon,
            CardEnum::Anger,
            CardEnum::SwordBoomerang,
            CardEnum::Hemokinesis,
            CardEnum::Armaments,
            CardEnum::Impervious,
            CardEnum::Brutality,
            CardEnum::Offering,
            CardEnum::Shockwave,
            CardEnum::Uppercut,
            CardEnum::Intimidate,
            CardEnum::SeeingRed,
            CardEnum::GhostlyArmor,
            CardEnum::Havoc,
            CardEnum::Headbutt,
            CardEnum::TrueGrit,
            CardEnum::Warcry,
            CardEnum::Corruption,
            CardEnum::LimitBreak,
            CardEnum::Metallicize,
            CardEnum::FlameBarrier,
            CardEnum::Rage,
            CardEnum::Rampage,
            CardEnum::RecklessCharge,
            CardEnum::SearingBlow,
            CardEnum::SeverSoul,
            CardEnum::SpotWeakness,
            CardEnum::Pummel,
            CardEnum::InfernalBlade,
            CardEnum::Evolve,
            CardEnum::Sentinel,
            CardEnum::Whirlwind,
            CardEnum::DemonForm,
            CardEnum::SecondWind,
            CardEnum::Rupture,
            CardEnum::DualWield,
            CardEnum::DoubleTap,
            CardEnum::Exhume,
            CardEnum::Feed,
            CardEnum::Reaper,
            CardEnum::FiendFire,
            CardEnum::FireBreathing,
            // Status Cards
            CardEnum::Slimed,
            CardEnum::Wound,
            CardEnum::Burn,
            CardEnum::Dazed,
            // Curse Cards
            CardEnum::AscendersCurse,
            CardEnum::Injury,
            CardEnum::Clumsy,
            CardEnum::Regret,
            CardEnum::Writhe,
            // Colorless Cards
            CardEnum::SwiftStrike,
            CardEnum::Finesse,
            CardEnum::FlashOfSteel,
            CardEnum::Blind,
            CardEnum::Trip,
            CardEnum::GoodInstincts,
            CardEnum::BandageUp,
            CardEnum::DeepBreath,
            CardEnum::MasterOfStrategy,
            CardEnum::DarkShackles,
            CardEnum::Impatience,
            CardEnum::PanicButton,
            CardEnum::Panacea,
            CardEnum::DramaticEntrance,
        ];

        // Categorize cards by rarity
        for card in all_cards {
            if let Some(rarity) = card.rarity() {
                match rarity {
                    Rarity::Common => common_pool.push(card),
                    Rarity::Uncommon => uncommon_pool.push(card),
                    Rarity::Rare => rare_pool.push(card),
                }
            }
            // Non-rewardable cards (None rarity) are filtered out automatically
        }

        Self {
            common_pool,
            uncommon_pool,
            rare_pool,
            rare_offset_percent: -5, // Start at -5% as per game mechanics
        }
    }

    /// Get the number of cards in the reward pool
    pub fn pool_size(&self) -> usize {
        self.common_pool.len() + self.uncommon_pool.len() + self.rare_pool.len()
    }

    /// Get cards by rarity as CardEnum vectors
    pub fn get_common_cards(&self) -> Vec<CardEnum> {
        self.common_pool.clone()
    }

    pub fn get_uncommon_cards(&self) -> Vec<CardEnum> {
        self.uncommon_pool.clone()
    }

    pub fn get_rare_cards(&self) -> Vec<CardEnum> {
        self.rare_pool.clone()
    }

    /// Get the current rare offset percentage
    pub fn get_rare_offset(&self) -> i32 {
        self.rare_offset_percent
    }

    /// Reset the rare offset to -5% (initial value)
    pub fn reset_rare_offset(&mut self) {
        self.rare_offset_percent = -5;
    }

    /// Calculate adjusted rarity probabilities based on offset
    /// Base probabilities are adjusted by the offset:
    /// - Negative offset: decreases rare chance (increases common)
    /// - Positive offset: decreases common chance (increases rare)
    /// Uncommon stays constant, and total always sums to 100%
    fn get_adjusted_probabilities(&self, combat_type: CombatType) -> (f64, f64, f64) {
        let (base_common, base_uncommon, base_rare) = match combat_type {
            CombatType::Normal => (60.0, 37.0, 3.0),
            CombatType::Elite => (50.0, 40.0, 10.0),
            CombatType::Boss => (0.0, 0.0, 100.0), // Boss is always 100% rare, unaffected by offset
        };

        // Boss combat ignores offset
        if combat_type == CombatType::Boss {
            return (base_common, base_uncommon, base_rare);
        }

        let offset = self.rare_offset_percent as f64;

        // Apply offset: negative decreases rare (increases common), positive increases rare (decreases common)
        let adjusted_rare = (base_rare + offset).max(0.0);
        let adjusted_common = (base_common - offset).max(0.0);
        let adjusted_uncommon = base_uncommon; // Uncommon stays constant

        // Normalize to ensure they sum to 100
        let total = adjusted_common + adjusted_uncommon + adjusted_rare;
        let norm_common = (adjusted_common / total) * 100.0;
        let norm_uncommon = (adjusted_uncommon / total) * 100.0;
        let norm_rare = (adjusted_rare / total) * 100.0;

        (norm_common, norm_uncommon, norm_rare)
    }

    /// Generate a single random card reward using rarity selection then uniform sampling
    /// Implements offset system: starts at -5%, increases by 1% per common card,
    /// resets to -5% when rare card is drawn. Capped at +40%.
    ///
    /// Normal Combat base: 60% Common, 37% Uncommon, 3% Rare
    /// Elite Combat base: 50% Common, 40% Uncommon, 10% Rare
    /// Boss Combat: 100% Rare (unaffected by offset)
    pub fn generate_single_reward(&mut self, rng: &mut impl rand::Rng, combat_type: CombatType) -> Option<Card> {
        // Get adjusted probabilities based on offset
        let (common_prob, uncommon_prob, rare_prob) = self.get_adjusted_probabilities(combat_type);

        // Build categorical distribution only with non-empty pools
        let mut rarity_options = Vec::new();
        if !self.common_pool.is_empty() && common_prob > 0.0 {
            rarity_options.push((Rarity::Common, common_prob));
        }
        if !self.uncommon_pool.is_empty() && uncommon_prob > 0.0 {
            rarity_options.push((Rarity::Uncommon, uncommon_prob));
        }
        if !self.rare_pool.is_empty() && rare_prob > 0.0 {
            rarity_options.push((Rarity::Rare, rare_prob));
        }

        if rarity_options.is_empty() {
            return None;
        }

        // Create categorical distribution for rarity sampling
        let rarity_dist = CategoricalDistribution::new(rarity_options);

        // Sample rarity
        let sampled_rarity = *rarity_dist.sample(rng);

        // Get the appropriate pool
        let pool = match sampled_rarity {
            Rarity::Common => &self.common_pool,
            Rarity::Uncommon => &self.uncommon_pool,
            Rarity::Rare => &self.rare_pool,
        };

        if pool.is_empty() {
            return None;
        }

        // Draw uniformly from the selected pool
        let card_dist = CategoricalDistribution::uniform(pool.clone());
        let selected_card_enum = card_dist.sample_owned(rng);

        // Update offset based on what was drawn
        match sampled_rarity {
            Rarity::Common => {
                // Increase offset by 1% per common card (capped at +40%)
                self.rare_offset_percent = (self.rare_offset_percent + 1).min(40);
            },
            Rarity::Rare => {
                // Reset offset to -5% when rare card is drawn
                self.rare_offset_percent = -5;
            },
            Rarity::Uncommon => {
                // Uncommon cards don't affect offset
            }
        }

        self.try_create_card_from_enum(selected_card_enum)
    }

    /// Generate 3 random card reward options using rarity selection then uniform sampling
    pub fn generate_reward_options(&mut self, rng: &mut impl rand::Rng) -> Vec<Card> {
        // Default to Normal combat type for standard card rewards
        self.generate_reward_options_with_combat_type(rng, CombatType::Normal)
    }

    /// Generate 3 random card reward options for specific combat type
    /// Note: Each card draw shares the same pity counter, so drawing 3 cards
    /// will increment the pity counter by up to 3 (unless a rare is drawn)
    pub fn generate_reward_options_with_combat_type(&mut self, rng: &mut impl rand::Rng, combat_type: CombatType) -> Vec<Card> {
        let mut options = Vec::new();
        let mut used_card_enums = Vec::new();

        // Generate 3 unique cards using rarity-based selection
        while options.len() < 3 {
            let card_option = self.generate_single_reward(rng, combat_type);

            if let Some(card) = card_option {
                let card_enum = card.get_card_enum();

                // Ensure we don't have duplicate options
                if !used_card_enums.contains(&card_enum) {
                    used_card_enums.push(card_enum);
                    options.push(card);
                }
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
            CardEnum::MasterOfStrategy => crate::cards::colorless::master_of_strategy::master_of_strategy(),
            CardEnum::DarkShackles => crate::cards::colorless::dark_shackles::dark_shackles(),
            CardEnum::Impatience => crate::cards::colorless::impatience::impatience(),
            CardEnum::PanicButton => crate::cards::colorless::panic_button::panic_button(),
            CardEnum::Panacea => crate::cards::colorless::panacea::panacea(),

            // Status/Curse Cards (should not be in reward pools but included for completeness)
            CardEnum::Slimed => crate::cards::status::slimed::slimed(),
            CardEnum::Wound => crate::cards::status::wound::wound(),
            CardEnum::Burn => crate::cards::status::burn::burn(),
            CardEnum::Dazed => crate::cards::status::dazed::dazed(),
            CardEnum::AscendersCurse => crate::cards::curse::ascenders_curse(), // For completeness, though not used in rewards
            CardEnum::Injury => crate::cards::curse::injury(), // For completeness, though not used in rewards
            CardEnum::Clumsy => crate::cards::curse::clumsy(), // For completeness, though not used in rewards
            CardEnum::Regret => crate::cards::curse::regret(), // For completeness, though not used in rewards
            CardEnum::Writhe => crate::cards::curse::writhe(), // For completeness, though not used in rewards
            CardEnum::DramaticEntrance => crate::cards::colorless::dramatic_entrance::dramatic_entrance(),
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
        assert!(pool.pool_size() >= 50); // Should have many Ironclad cards + colorless
        assert!(!pool.get_common_cards().is_empty());
        assert!(!pool.get_uncommon_cards().is_empty());
        assert!(!pool.get_rare_cards().is_empty());
    }

    #[test]
    fn test_pool_sizes() {
        let pool = CardRewardPool::new();

        let common_count = pool.get_common_cards().len();
        let uncommon_count = pool.get_uncommon_cards().len();
        let rare_count = pool.get_rare_cards().len();

        // Check that we have reasonable pool sizes
        // Updated to reflect actual card rarities defined in card implementations
        assert!(common_count >= 15, "Should have common cards: {}", common_count);
        assert!(uncommon_count >= 20, "Should have uncommon cards: {}", uncommon_count);
        assert!(rare_count >= 5, "Should have rare cards: {}", rare_count);
    }

    #[test]
    fn test_offset_system_initialization() {
        let pool = CardRewardPool::new();
        // Initial offset should be -5%
        assert_eq!(pool.get_rare_offset(), -5);
    }

    #[test]
    fn test_offset_increases_with_common_cards() {
        let mut pool = CardRewardPool::new();
        let mut rng = StdRng::seed_from_u64(12345);

        // Manually set offset to test increment
        pool.rare_offset_percent = 0;

        // Force a common draw by creating a pool with only common cards
        let mut test_pool = CardRewardPool::new();
        test_pool.uncommon_pool.clear();
        test_pool.rare_pool.clear();
        test_pool.rare_offset_percent = 0;

        let _ = test_pool.generate_single_reward(&mut rng, CombatType::Normal);

        // Offset should have increased by 1
        assert_eq!(test_pool.get_rare_offset(), 1);
    }

    #[test]
    fn test_offset_resets_on_rare_card() {
        let mut pool = CardRewardPool::new();
        let mut rng = StdRng::seed_from_u64(999);

        // Set offset to a high value
        pool.rare_offset_percent = 30;

        // Force a rare draw by using boss combat
        let _ = pool.generate_single_reward(&mut rng, CombatType::Boss);

        // Offset should reset to -5
        assert_eq!(pool.get_rare_offset(), -5);
    }

    #[test]
    fn test_offset_capped_at_40() {
        let mut pool = CardRewardPool::new();
        pool.rare_offset_percent = 39;

        let mut rng = StdRng::seed_from_u64(42);

        // Manually increment offset - simulating drawing common cards
        // Since we can't reliably force common draws without clearing pools,
        // we'll directly test the cap logic
        pool.rare_offset_percent = 40;

        // Create a pool with only common cards
        pool.uncommon_pool.clear();
        pool.rare_pool.clear();

        let _ = pool.generate_single_reward(&mut rng, CombatType::Normal);

        // Offset should stay at 40 (capped)
        assert_eq!(pool.get_rare_offset(), 40);
    }

    #[test]
    fn test_adjusted_probabilities_with_offset() {
        let mut pool = CardRewardPool::new();

        // At -5% offset, rare chance should be lower than base
        let (common, _, rare) = pool.get_adjusted_probabilities(CombatType::Normal);
        assert!(rare < 3.0, "Rare chance should be less than base 3% at -5 offset");
        assert!(common > 60.0, "Common chance should be greater than base 60% at -5 offset");

        // At +10% offset, rare chance should be higher than base
        pool.rare_offset_percent = 10;
        let (common, _, rare) = pool.get_adjusted_probabilities(CombatType::Normal);
        assert!(rare > 3.0, "Rare chance should be greater than base 3% at +10 offset");
        assert!(common < 60.0, "Common chance should be less than base 60% at +10 offset");
    }

    #[test]
    fn test_cards_by_rarity() {
        let pool = CardRewardPool::new();

        let common_cards = pool.get_common_cards();
        let uncommon_cards = pool.get_uncommon_cards();
        let rare_cards = pool.get_rare_cards();

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
        let mut pool = CardRewardPool::new();
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
        let mut pool = CardRewardPool::new();
        let mut rng = StdRng::seed_from_u64(42);

        // Generate rewards multiple times to ensure randomness
        let options1 = pool.generate_reward_options(&mut rng);
        let options2 = pool.generate_reward_options(&mut rng);

        // Should be different (most likely, given randomness)
        assert_ne!(options1, options2);
    }

    #[test]
    fn test_no_duplicates_in_single_reward() {
        let mut pool = CardRewardPool::new();
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
    fn test_uniform_distribution_within_rarity() {
        // Test that cards within the same rarity tier are drawn uniformly
        let mut pool = CardRewardPool::new();
        let mut rng = StdRng::seed_from_u64(999);

        // Force boss combat to only draw from rare pool
        let mut rare_card_counts = std::collections::HashMap::new();

        for _ in 0..100 {
            if let Some(card) = pool.generate_single_reward(&mut rng, CombatType::Boss) {
                let name = card.get_name();
                *rare_card_counts.entry(name).or_insert(0) += 1;
            }
        }

        // Should have drawn multiple different rare cards
        assert!(rare_card_counts.len() > 5, "Should draw various rare cards");

        // No single card should dominate (would indicate non-uniform distribution)
        for (name, count) in &rare_card_counts {
            assert!(*count < 50, "Card {} drawn {} times - distribution not uniform", name, count);
        }
    }

    #[test]
    fn test_no_basic_cards_in_pool() {
        let pool = CardRewardPool::new();
        let mut all_card_enums = Vec::new();
        all_card_enums.extend(pool.get_common_cards());
        all_card_enums.extend(pool.get_uncommon_cards());
        all_card_enums.extend(pool.get_rare_cards());

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