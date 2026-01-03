//! Reward state types for post-combat rewards and treasure chests

use serde::{Serialize, Deserialize};

/// Types of treasure chests
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChestType {
    /// Small chest: 75% Common, 25% Uncommon relic; 50% chance of 23-27 gold
    Small,
    /// Medium chest: 35% Common, 50% Uncommon, 15% Rare relic; 35% chance of 45-55 gold
    Medium,
    /// Large chest: 75% Uncommon, 25% Rare relic; 50% chance of 68-82 gold
    Large,
}

/// Relic rarity types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelicRarity {
    Common,
    Uncommon,
    Rare,
}

/// Reward state after combat or treasure chest, containing various reward types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RewardState {
    /// Gold reward earned from combat or chest
    pub gold_reward: u32,
    /// Whether card selection reward is available (true after most combats)
    pub card_selection_available: bool,
    /// Whether the gold has been claimed
    pub gold_claimed: bool,
    /// Optional potion reward (40% chance in normal/elite combats)
    pub potion_reward: Option<crate::potion::Potion>,
    /// Whether the potion has been claimed
    pub potion_claimed: bool,
    /// Optional relic reward (from treasure chests)
    pub relic_reward: Option<RelicRarity>,
    /// Whether the relic has been claimed
    pub relic_claimed: bool,
}

impl RewardState {
    /// Create a new reward state for normal combat (10-20 gold, card selection available, 40% potion chance)
    pub fn new_normal_combat(rng: &mut impl rand::Rng) -> Self {
        RewardState {
            gold_reward: rng.random_range(10..=20),
            card_selection_available: true,
            gold_claimed: false,
            potion_reward: Self::roll_potion_drop(rng, 0.4),
            potion_claimed: false,
            relic_reward: None,
            relic_claimed: false,
        }
    }

    /// Create a new reward state for elite combat (25-35 gold, card selection available, 40% potion chance)
    pub fn new_elite_combat(rng: &mut impl rand::Rng) -> Self {
        RewardState {
            gold_reward: rng.random_range(25..=35),
            card_selection_available: true,
            gold_claimed: false,
            potion_reward: Self::roll_potion_drop(rng, 0.4),
            potion_claimed: false,
            relic_reward: None,
            relic_claimed: false,
        }
    }

    /// Create a new reward state for boss combat (50-75 gold, card selection available, guaranteed potion)
    pub fn new_boss_combat(rng: &mut impl rand::Rng) -> Self {
        RewardState {
            gold_reward: rng.random_range(50..=75),
            card_selection_available: true,
            gold_claimed: false,
            potion_reward: Some(crate::potion::Potion::StrengthPotion), // Will be replaced when more potions are implemented
            potion_claimed: false,
            relic_reward: None,
            relic_claimed: false,
        }
    }

    /// Roll for a potion drop with given probability (0.0 to 1.0)
    pub fn roll_potion_drop(rng: &mut impl rand::Rng, probability: f64) -> Option<crate::potion::Potion> {
        if rng.random::<f64>() < probability {
            Some(crate::potion::Potion::StrengthPotion) // Will be replaced when more potions are implemented
        } else {
            None
        }
    }

    /// Set the card selection availability
    pub fn set_card_selection_available(&mut self, available: bool) {
        self.card_selection_available = available;
    }

    /// Set the potion reward
    pub fn set_potion_reward(&mut self, potion: Option<crate::potion::Potion>) {
        self.potion_reward = potion;
    }

    /// Claim the gold reward
    pub fn claim_gold(&mut self) -> u32 {
        if !self.gold_claimed {
            self.gold_claimed = true;
            self.gold_reward
        } else {
            0
        }
    }

    /// Claim the potion reward
    pub fn claim_potion(&mut self) -> Option<crate::potion::Potion> {
        if !self.potion_claimed {
            self.potion_claimed = true;
            self.potion_reward.take()
        } else {
            None
        }
    }

    /// Claim the relic reward
    pub fn claim_relic(&mut self) -> Option<RelicRarity> {
        if !self.relic_claimed {
            self.relic_claimed = true;
            self.relic_reward.take()
        } else {
            None
        }
    }

    /// Check if all rewards have been claimed
    pub fn all_rewards_claimed(&self) -> bool {
        self.gold_claimed && self.potion_claimed && self.relic_claimed
    }
}

impl ChestType {
    /// Sample a random chest type based on the wiki probabilities
    /// Small (50%), Medium (33%), Large (17%)
    pub fn sample(rng: &mut impl rand::Rng) -> Self {
        let roll = rng.random::<f64>();
        if roll < 0.50 {
            ChestType::Small
        } else if roll < 0.83 {  // 0.50 + 0.33 = 0.83
            ChestType::Medium
        } else {
            ChestType::Large
        }
    }

    /// Get the relic rarity for this chest type
    fn sample_relic_rarity(&self, rng: &mut impl rand::Rng) -> RelicRarity {
        let roll = rng.random::<f64>();
        match self {
            ChestType::Small => {
                // 75% Common, 25% Uncommon
                if roll < 0.75 {
                    RelicRarity::Common
                } else {
                    RelicRarity::Uncommon
                }
            }
            ChestType::Medium => {
                // 35% Common, 50% Uncommon, 15% Rare
                if roll < 0.35 {
                    RelicRarity::Common
                } else if roll < 0.85 {  // 0.35 + 0.50 = 0.85
                    RelicRarity::Uncommon
                } else {
                    RelicRarity::Rare
                }
            }
            ChestType::Large => {
                // 75% Uncommon, 25% Rare
                if roll < 0.75 {
                    RelicRarity::Uncommon
                } else {
                    RelicRarity::Rare
                }
            }
        }
    }

    /// Roll for gold reward based on chest type
    fn roll_gold(&self, rng: &mut impl rand::Rng) -> u32 {
        let (chance, min, max) = match self {
            ChestType::Small => (0.50, 23, 27),
            ChestType::Medium => (0.35, 45, 55),
            ChestType::Large => (0.50, 68, 82),
        };

        if rng.random::<f64>() < chance {
            rng.random_range(min..=max)
        } else {
            0
        }
    }

    /// Create a reward state for this chest type
    pub fn create_reward_state(&self, rng: &mut impl rand::Rng) -> RewardState {
        RewardState {
            gold_reward: self.roll_gold(rng),
            card_selection_available: false,  // Chests don't give card rewards
            gold_claimed: false,
            potion_reward: None,  // Chests don't give potions
            potion_claimed: false,
            relic_reward: Some(self.sample_relic_rarity(rng)),
            relic_claimed: false,
        }
    }
}