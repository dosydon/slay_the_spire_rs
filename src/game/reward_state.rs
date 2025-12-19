//! Reward state types for post-combat rewards

/// Reward state after combat, containing various reward types
#[derive(Debug, Clone, PartialEq)]
pub struct RewardState {
    /// Gold reward earned from combat
    pub gold_reward: u32,
    /// Whether card selection reward is available (true after most combats)
    pub card_selection_available: bool,
    /// Whether the gold has been claimed
    pub gold_claimed: bool,
    /// Optional potion reward (40% chance in normal/elite combats)
    pub potion_reward: Option<crate::game::potion::Potion>,
    /// Whether the potion has been claimed
    pub potion_claimed: bool,
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
        }
    }

    /// Create a new reward state for boss combat (50-75 gold, card selection available, guaranteed potion)
    pub fn new_boss_combat(rng: &mut impl rand::Rng) -> Self {
        RewardState {
            gold_reward: rng.random_range(50..=75),
            card_selection_available: true,
            gold_claimed: false,
            potion_reward: Some(crate::game::potion::Potion::StrengthPotion), // Will be replaced when more potions are implemented
            potion_claimed: false,
        }
    }

    /// Roll for a potion drop with given probability (0.0 to 1.0)
    pub fn roll_potion_drop(rng: &mut impl rand::Rng, probability: f64) -> Option<crate::game::potion::Potion> {
        if rng.random::<f64>() < probability {
            Some(crate::game::potion::Potion::StrengthPotion) // Will be replaced when more potions are implemented
        } else {
            None
        }
    }

    /// Set the card selection availability
    pub fn set_card_selection_available(&mut self, available: bool) {
        self.card_selection_available = available;
    }

    /// Set the potion reward
    pub fn set_potion_reward(&mut self, potion: Option<crate::game::potion::Potion>) {
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
    pub fn claim_potion(&mut self) -> Option<crate::game::potion::Potion> {
        if !self.potion_claimed {
            self.potion_claimed = true;
            self.potion_reward.take()
        } else {
            None
        }
    }

    /// Check if all rewards have been claimed
    pub fn all_rewards_claimed(&self) -> bool {
        self.gold_claimed && self.potion_claimed
    }
}