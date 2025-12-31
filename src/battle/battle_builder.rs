use crate::{
    battle::Battle,
    game::{
        card::Card,
        deck::Deck,
        global_info::GlobalInfo,
        player_run_state::PlayerRunState,
    },
    battle::enemy_in_battle::EnemyInBattle,
    enemies::enemy_enum::EnemyEnum,
    cards::ironclad::starter_deck::starter_deck,
    potion::PotionInventory,
};

/// Builder for creating Battle instances with customizable parameters
pub struct BattleBuilder {
    deck: Option<Vec<Card>>,
    enemies: Option<Vec<EnemyInBattle>>,
    current_hp: Option<u32>,
    max_hp: Option<u32>,
    gold: Option<u32>,
    ascension: Option<u32>,
    floor: Option<u32>,
    seed: Option<u64>,
    potions: Option<PotionInventory>,
}

impl BattleBuilder {
    /// Create a new BattleBuilder with default values
    pub fn new() -> Self {
        BattleBuilder {
            deck: None,
            enemies: None,
            current_hp: None,
            max_hp: None,
            gold: None,
            ascension: None,
            floor: None,
            seed: None,
            potions: None,
        }
    }

    /// Set the deck to use in the battle
    pub fn with_deck(mut self, deck: Vec<Card>) -> Self {
        self.deck = Some(deck);
        self
    }

    /// Add a single card to the deck
    pub fn add_card(mut self, card: Card) -> Self {
        if let Some(ref mut deck) = self.deck {
            deck.push(card);
        } else {
            self.deck = Some(vec![card]);
        }
        self
    }

    /// Set the enemies to fight
    pub fn with_enemies(mut self, enemies: Vec<EnemyInBattle>) -> Self {
        self.enemies = Some(enemies);
        self
    }

    /// Add a single enemy to the encounter
    pub fn add_enemy(mut self, enemy: EnemyEnum) -> Self {
        let enemy_in_battle = EnemyInBattle::new(enemy);
        if let Some(ref mut enemies) = self.enemies {
            enemies.push(enemy_in_battle);
        } else {
            self.enemies = Some(vec![enemy_in_battle]);
        }
        self
    }

    /// Set an encounter by providing multiple enemies
    pub fn with_encounter(mut self, enemies: Vec<EnemyEnum>) -> Self {
        self.enemies = Some(
            enemies
                .into_iter()
                .map(EnemyInBattle::new)
                .collect()
        );
        self
    }

    /// Set the player's current HP
    pub fn with_current_hp(mut self, hp: u32) -> Self {
        self.current_hp = Some(hp);
        self
    }

    /// Set the player's max HP
    pub fn with_max_hp(mut self, max_hp: u32) -> Self {
        self.max_hp = Some(max_hp);
        self
    }

    /// Set both current and max HP
    pub fn with_hp(mut self, current_hp: u32, max_hp: u32) -> Self {
        self.current_hp = Some(current_hp);
        self.max_hp = Some(max_hp);
        self
    }

    /// Set the player's gold
    pub fn with_gold(mut self, gold: u32) -> Self {
        self.gold = Some(gold);
        self
    }

    /// Set the ascension level
    pub fn with_ascension(mut self, ascension: u32) -> Self {
        self.ascension = Some(ascension);
        self
    }

    /// Set the current floor
    pub fn with_floor(mut self, floor: u32) -> Self {
        self.floor = Some(floor);
        self
    }

    /// Set the random seed for reproducible battles
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Set the potion inventory
    pub fn with_potions(mut self, potions: PotionInventory) -> Self {
        self.potions = Some(potions);
        self
    }

    /// Build the Battle instance
    pub fn build(self) -> Battle {
        use rand::SeedableRng;

        // Use provided deck or default to starter deck
        let deck = if let Some(cards) = self.deck {
            Deck::new(cards)
        } else {
            starter_deck()
        };

        // Use provided enemies or default to empty (will panic if trying to start battle with no enemies)
        let enemies = self.enemies.unwrap_or_default();

        // Create global info
        let global_info = GlobalInfo {
            ascention: self.ascension.unwrap_or(0),
            current_floor: self.floor.unwrap_or(1),
        };

        // Create player run state
        let current_hp = self.current_hp.unwrap_or(80);
        let max_hp = self.max_hp.unwrap_or(80);
        let gold = self.gold.unwrap_or(100);
        let player_state = PlayerRunState::new(current_hp, max_hp, gold);

        // Create battle with shuffle using the appropriate RNG
        if let Some(seed) = self.seed {
            let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
            Battle::new_with_shuffle(deck, global_info, player_state, enemies, &mut rng)
        } else {
            let mut rng = rand::rng();
            Battle::new_with_shuffle(deck, global_info, player_state, enemies, &mut rng)
        }
    }
}

impl Default for BattleBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enemies::jaw_worm::JawWorm;
    use crate::cards::ironclad::strike::strike;

    #[test]
    fn test_builder_with_defaults() {
        let battle = BattleBuilder::new()
            .add_enemy(EnemyEnum::JawWorm(JawWorm::new(40, false)))
            .build();

        assert_eq!(battle.get_current_hp(), 80);
        assert_eq!(battle.get_max_hp(), 80);
    }

    #[test]
    fn test_builder_with_custom_hp() {
        let battle = BattleBuilder::new()
            .with_hp(50, 100)
            .add_enemy(EnemyEnum::JawWorm(JawWorm::new(40, false)))
            .build();

        assert_eq!(battle.get_current_hp(), 50);
        assert_eq!(battle.get_max_hp(), 100);
    }

    #[test]
    fn test_builder_with_seed() {
        let battle1 = BattleBuilder::new()
            .with_seed(12345)
            .add_enemy(EnemyEnum::JawWorm(JawWorm::new(40, false)))
            .build();

        let battle2 = BattleBuilder::new()
            .with_seed(12345)
            .add_enemy(EnemyEnum::JawWorm(JawWorm::new(40, false)))
            .build();

        // With the same seed, initial states should be identical
        assert_eq!(battle1.get_current_hp(), battle2.get_current_hp());
    }

    #[test]
    fn test_builder_with_custom_deck() {
        let custom_deck = vec![
            strike(),
            strike(),
        ];

        let battle = BattleBuilder::new()
            .with_deck(custom_deck)
            .add_enemy(EnemyEnum::JawWorm(JawWorm::new(40, false)))
            .build();

        // Battle should be created successfully
        assert!(!battle.is_battle_over());
    }

    #[test]
    fn test_builder_add_multiple_enemies() {
        let battle = BattleBuilder::new()
            .add_enemy(EnemyEnum::JawWorm(JawWorm::new(40, false)))
            .add_enemy(EnemyEnum::JawWorm(JawWorm::new(40, false)))
            .build();

        assert_eq!(battle.get_enemies().len(), 2);
    }

    #[test]
    fn test_builder_with_encounter() {
        let encounter = vec![
            EnemyEnum::JawWorm(JawWorm::new(40, false)),
            EnemyEnum::JawWorm(JawWorm::new(40, false)),
        ];

        let battle = BattleBuilder::new()
            .with_encounter(encounter)
            .build();

        assert_eq!(battle.get_enemies().len(), 2);
    }
}
