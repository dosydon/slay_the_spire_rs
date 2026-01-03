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
    enemies::cultist::Cultist,
    enemies::jaw_worm::JawWorm,
    cards::ironclad::starter_deck::starter_deck,
    potion::PotionInventory,
    relics::Relic,
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
    relics: Option<Vec<Relic>>,
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
            relics: None,
        }
    }

    /// Create a BattleBuilder for a named battle configuration
    ///
    /// # Arguments
    /// * `name` - The name of the battle (case-insensitive). Supported battles:
    ///   - "cultist": One Cultist with 50 HP and 3 ritual amount
    ///   - "cultist_orichalcum": Cultist battle with Orichalcum relic (gain 6 Block at end of turn if you have no Block)
    ///   - "jaw_worm" or "jawworm": One Jaw Worm with 40 HP (not angry)
    ///
    /// # Returns
    /// * `Ok(BattleBuilder)` - Builder configured with the named battle
    /// * `Err(String)` - Error message if the battle name is not recognized
    ///
    /// # Example
    /// ```
    /// let battle = BattleBuilder::from_name("cultist")?.build();
    /// let battle = BattleBuilder::from_name("jaw_worm")?.build();
    /// ```
    pub fn from_name(name: &str) -> Result<Self, String> {
        match name.to_lowercase().as_str() {
            "cultist" => {
                let cultist = Cultist::new(50, 3);
                Ok(BattleBuilder::new().add_enemy(EnemyEnum::Cultist(cultist)))
            }
            "cultist_orichalcum" => {
                let cultist = Cultist::new(50, 3);
                Ok(BattleBuilder::new()
                    .add_enemy(EnemyEnum::Cultist(cultist))
                    .add_relic(Relic::Orichalcum))
            }
            "jaw_worm" | "jawworm" => {
                let jaw_worm = JawWorm::new(40, false);
                Ok(BattleBuilder::new().add_enemy(EnemyEnum::JawWorm(jaw_worm)))
            }
            _ => Err(format!("Unknown battle name: {}", name))
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

    /// Set the starting relics
    pub fn with_relics(mut self, relics: Vec<Relic>) -> Self {
        self.relics = Some(relics);
        self
    }

    /// Add a single relic
    pub fn add_relic(mut self, relic: Relic) -> Self {
        if let Some(ref mut relics) = self.relics {
            relics.push(relic);
        } else {
            self.relics = Some(vec![relic]);
        }
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
        let relics = self.relics.unwrap_or_default();
        let potions = self.potions.unwrap_or_default();
        let player_state = PlayerRunState::new_with_relics_and_potions(
            current_hp,
            max_hp,
            gold,
            relics,
            potions,
        );

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

    #[test]
    fn test_from_name_cultist() {
        let battle = BattleBuilder::from_name("cultist")
            .expect("Should create Cultist battle")
            .build();

        // Should have exactly one enemy
        assert_eq!(battle.get_enemies().len(), 1);

        // Enemy should be a Cultist with 50 HP
        let enemy = &battle.get_enemies()[0];
        assert_eq!(enemy.get_name(), "Cultist");
        assert_eq!(enemy.get_current_hp(), 50);
    }

    #[test]
    fn test_from_name_case_insensitive() {
        // Test various capitalizations
        let battle1 = BattleBuilder::from_name("cultist").expect("lowercase should work");
        let battle2 = BattleBuilder::from_name("Cultist").expect("capitalized should work");
        let battle3 = BattleBuilder::from_name("CULTIST").expect("uppercase should work");
        let battle4 = BattleBuilder::from_name("CuLtIsT").expect("mixed case should work");

        // All should create battles successfully
        assert_eq!(battle1.build().get_enemies().len(), 1);
        assert_eq!(battle2.build().get_enemies().len(), 1);
        assert_eq!(battle3.build().get_enemies().len(), 1);
        assert_eq!(battle4.build().get_enemies().len(), 1);
    }

    #[test]
    fn test_from_name_unknown_battle() {
        let result = BattleBuilder::from_name("unknown_battle");
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err, "Unknown battle name: unknown_battle");
        }
    }

    #[test]
    fn test_from_name_with_customization() {
        let battle = BattleBuilder::from_name("cultist")
            .expect("Should create Cultist battle")
            .with_hp(50, 100)
            .with_gold(200)
            .with_ascension(5)
            .build();

        // Should have the customized player stats
        assert_eq!(battle.get_current_hp(), 50);
        assert_eq!(battle.get_max_hp(), 100);

        // Should still have the Cultist enemy
        assert_eq!(battle.get_enemies().len(), 1);
        assert_eq!(battle.get_enemies()[0].get_name(), "Cultist");
    }

    #[test]
    fn test_from_name_jaw_worm() {
        let battle = BattleBuilder::from_name("jaw_worm")
            .expect("Should create Jaw Worm battle")
            .build();

        // Should have exactly one enemy
        assert_eq!(battle.get_enemies().len(), 1);

        // Enemy should be a Jaw Worm with 40 HP
        let enemy = &battle.get_enemies()[0];
        assert_eq!(enemy.get_name(), "Jaw Worm");
        assert_eq!(enemy.get_current_hp(), 40);
    }

    #[test]
    fn test_from_name_jaw_worm_aliases() {
        // Test both spellings
        let battle1 = BattleBuilder::from_name("jaw_worm").expect("jaw_worm should work");
        let battle2 = BattleBuilder::from_name("jawworm").expect("jawworm should work");
        let battle3 = BattleBuilder::from_name("JawWorm").expect("JawWorm should work");
        let battle4 = BattleBuilder::from_name("JAW_WORM").expect("JAW_WORM should work");

        // All should create battles successfully with Jaw Worm
        assert_eq!(battle1.build().get_enemies()[0].get_name(), "Jaw Worm");
        assert_eq!(battle2.build().get_enemies()[0].get_name(), "Jaw Worm");
        assert_eq!(battle3.build().get_enemies()[0].get_name(), "Jaw Worm");
        assert_eq!(battle4.build().get_enemies()[0].get_name(), "Jaw Worm");
    }

    #[test]
    fn test_from_name_cultist_orichalcum() {
        let battle = BattleBuilder::from_name("cultist_orichalcum")
            .expect("Should create Cultist with Orichalcum battle")
            .build();

        // Should have exactly one enemy (Cultist)
        assert_eq!(battle.get_enemies().len(), 1);
        let enemy = &battle.get_enemies()[0];
        assert_eq!(enemy.get_name(), "Cultist");
        assert_eq!(enemy.get_current_hp(), 50);

        // Should have Orichalcum relic
        let relics = battle.get_relics();
        assert_eq!(relics.len(), 1);
        assert!(matches!(relics[0], Relic::Orichalcum));
    }

    #[test]
    fn test_builder_with_relics() {
        let relics = vec![Relic::Orichalcum];
        let battle = BattleBuilder::new()
            .with_relics(relics)
            .add_enemy(EnemyEnum::JawWorm(JawWorm::new(40, false)))
            .build();

        // Should have the relic
        let relics = battle.get_relics();
        assert_eq!(relics.len(), 1);
        assert!(matches!(relics[0], Relic::Orichalcum));
    }

    #[test]
    fn test_builder_add_relic() {
        let battle = BattleBuilder::new()
            .add_relic(Relic::Orichalcum)
            .add_enemy(EnemyEnum::JawWorm(JawWorm::new(40, false)))
            .build();

        // Should have the relic
        let relics = battle.get_relics();
        assert_eq!(relics.len(), 1);
        assert!(matches!(relics[0], Relic::Orichalcum));
    }
}
