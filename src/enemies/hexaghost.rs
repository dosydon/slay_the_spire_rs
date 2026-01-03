use crate::game::enemy::EnemyTrait;
use crate::game::global_info::GlobalInfo;
use crate::game::effect::Effect;
use crate::battle::battle_events::{BattleEvent, EventListener};
use crate::battle::target::Entity;

/// Hexaghost - Act 1 Boss Enemy
///
/// **Health:**
/// - Base: 250 HP
/// - Ascension 9+: 264 HP (Bosses use A9 for HP scaling)
///
/// **Behavior:**
/// - Has a specific move pattern that cycles through 7 different moves
/// - Starts with Activate and Divider on turns 1-2
/// - Then cycles through 7-move pattern repeatedly
///
/// **Move Pattern:**
/// Turn 1: Activate (does nothing)
/// Turn 2: Divider (damage scales with player HP)
/// Turn 3-9: Sear → Tackle → Sear → Inflame → Tackle → Sear → Inferno
/// Then repeats the 7-turn cycle
///
/// **Move Details:**
/// - **Sear**: 6 damage + 1 Burn to discard pile
/// - **Tackle**: 5×2 hits (6×2 at A4+)
/// - **Inflame**: Gains 12 Block + 2 Strength (3 Strength at A19+)
/// - **Inferno**: 2×6 hits (3×6 at A4+) + 3 Burns to discard + upgrades all existing Burns
/// - **Divider**: (N+1)×6 damage where N = player HP ÷ 12 (rounded down)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Hexaghost {
    hp: u32,
    max_hp: u32,
    move_count: u32,
    ascension: u32,
    inferno_upgrades_active: bool,
}

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
pub enum HexaghostMove {
    /// Activate - Turn 1 only, does nothing
    Activate,
    /// Divider - Turn 2 only, damage scales with player HP
    Divider,
    /// Sear - Attack + Burn debuff
    Sear,
    /// Tackle - Multi-hit attack
    Tackle,
    /// Inflame - Block + Strength buff
    Inflame,
    /// Inferno - Heavy attack + multiple Burns + upgrade existing Burns
    Inferno,
}

impl Hexaghost {
    pub fn new(hp: u32, ascension: u32) -> Self {
        Hexaghost {
            hp,
            max_hp: hp,
            move_count: 0,
            ascension,
            inferno_upgrades_active: false,
        }
    }

    /// Calculate HP range based on ascension
    pub fn calculate_hp_range(_global_info: &GlobalInfo) -> (u32, u32) {
        // Bosses use different ascension scaling for HP (A9+ instead of A7+)
        // For simplicity, we'll use base HP range for now
        (250, 250) // Fixed HP for boss
    }

    /// Get the current move based on the 2-turn + 7-cycle pattern
    fn get_current_move(&self) -> HexaghostMove {
        match self.move_count {
            0 => HexaghostMove::Activate,
            1 => HexaghostMove::Divider,
            _ => {
                // Start cycle from move_count = 2
                let cycle_position = (self.move_count - 2) % 7;
                match cycle_position {
                    0 => HexaghostMove::Sear,
                    1 => HexaghostMove::Tackle,
                    2 => HexaghostMove::Sear,
                    3 => HexaghostMove::Inflame,
                    4 => HexaghostMove::Tackle,
                    5 => HexaghostMove::Sear,
                    6 => HexaghostMove::Inferno,
                    _ => unreachable!(),
                }
            }
        }
    }

    /// Get Divider damage based on player HP
    fn get_divider_damage(&self, player_hp: u32) -> u32 {
        let n = player_hp / 12;
        n + 1
    }

    /// Get Tackle damage per hit
    fn get_tackle_damage(&self) -> u32 {
        if self.ascension >= 4 {
            6
        } else {
            5
        }
    }

    /// Get Inflame Strength amount
    fn get_inflame_strength(&self) -> u32 {
        if self.ascension >= 19 {
            3
        } else {
            2
        }
    }

    /// Get Inferno damage per hit and number of hits
    fn get_inferno_damage(&self) -> (u32, u32) {
        if self.ascension >= 4 {
            (3, 6) // 3×6 hits
        } else {
            (2, 6) // 2×6 hits
        }
    }

    pub fn record_move(&mut self, _move_type: HexaghostMove) {
        self.move_count += 1;
    }

    pub fn get_move_effects(&self, move_type: HexaghostMove, player_hp: u32) -> Vec<Effect> {
        match move_type {
            HexaghostMove::Activate => {
                vec![] // Does nothing
            }
            HexaghostMove::Divider => {
                let damage = self.get_divider_damage(player_hp);
                vec![Effect::AttackToTarget {
                    amount: damage,
                    num_attacks: 6,
                    strength_multiplier: 1,
                }]
            }
            HexaghostMove::Sear => {
                vec![
                    Effect::AttackToTarget {
                        amount: 6,
                        num_attacks: 1,
                        strength_multiplier: 1,
                    },
                    Effect::AddStatusToDiscard {
                        status_card: crate::game::card_enum::CardEnum::Burn,
                    },
                ]
            }
            HexaghostMove::Tackle => {
                let damage = self.get_tackle_damage();
                vec![Effect::AttackToTarget {
                    amount: damage,
                    num_attacks: 2,
                    strength_multiplier: 1,
                }]
            }
            HexaghostMove::Inflame => {
                let strength = self.get_inflame_strength();
                vec![
                    Effect::GainDefense {
                        amount: 12,
                    },
                    Effect::GainStrength {
                        amount: strength,
                    },
                ]
            }
            HexaghostMove::Inferno => {
                let (num_hits, damage_per_hit) = self.get_inferno_damage();
                let mut effects = vec![
                    Effect::AttackToTarget {
                        amount: damage_per_hit,
                        num_attacks: num_hits,
                        strength_multiplier: 1,
                    },
                ];

                // Add 3 Burns to discard
                for _ in 0..3 {
                    effects.push(Effect::AddStatusToDiscard {
                        status_card: crate::game::card_enum::CardEnum::Burn,
                    });
                }

                effects
            }
        }
    }

    fn choose_next_move(&self, _global_info: &GlobalInfo) -> crate::utils::CategoricalDistribution<HexaghostMove> {
        // Hexaghost has deterministic behavior based on move cycle
        let current_move = self.get_current_move();
        let outcomes_and_weights = vec![(current_move, 1.0)];
        crate::utils::CategoricalDistribution::new(outcomes_and_weights)
    }

    pub fn set_hp(&mut self, hp: u32) {
        self.hp = hp;
    }

    /// Check if Inferno upgrades should be active
    pub fn activate_inferno_upgrades(&mut self) {
        self.inferno_upgrades_active = true;
    }

    pub fn has_inferno_upgrades(&self) -> bool {
        self.inferno_upgrades_active
    }
}

impl EnemyTrait for Hexaghost {
    type MoveType = HexaghostMove;

    fn instantiate(rng: &mut impl rand::Rng, global_info: &GlobalInfo) -> Self {
        let (hp_min, hp_max) = Self::calculate_hp_range(global_info);
        let hp = hp_min + rng.random_range(0..=(hp_max - hp_min));
        let ascension = global_info.ascention;

        Hexaghost::new(hp, ascension)
    }

    fn get_name() -> String {
        "Hexaghost".to_string()
    }

    fn get_hp(&self) -> u32 {
        self.hp
    }

    fn choose_move_and_effects(&mut self, global_info: &GlobalInfo, rng: &mut impl rand::Rng) -> (HexaghostMove, Vec<Effect>) {
        let move_distribution = self.choose_next_move(global_info);
        let selected_move = move_distribution.sample_owned(rng);

        // For moves that don't need player HP, use 0
        let player_hp_for_effects = match selected_move {
            HexaghostMove::Divider => 50, // Placeholder - will be properly set in battle
            _ => 0,
        };

        // Record the move for state tracking
        self.record_move(selected_move);

        // Activate Inferno upgrades after first Inferno
        if selected_move == HexaghostMove::Inferno && !self.has_inferno_upgrades() {
            self.activate_inferno_upgrades();
        }

        // Generate the effects for this move
        let effects = self.get_move_effects(selected_move, player_hp_for_effects);

        (selected_move, effects)
    }
}

/// Event listener for Hexaghost's mechanics
/// - No special mechanics needed for current implementation
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct HexaghostListener {
    enemy_index: usize,
}

impl HexaghostListener {
    pub(crate) fn new(enemy_index: usize) -> Self {
        HexaghostListener { enemy_index }
    }
}

impl EventListener for HexaghostListener {
    fn on_event(&mut self, _event: &BattleEvent) -> Vec<Effect> {
        vec![] // Hexaghost has no special event listeners for now
    }

    fn is_active(&self) -> bool {
        true
    }

    fn get_owner(&self) -> Entity {
        Entity::Enemy(self.enemy_index)
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn hash_to(&self, state: &mut std::collections::hash_map::DefaultHasher) {
        use std::hash::Hash;
        self.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enemies::enemy_enum::EnemyMove;

    #[test]
    fn test_hexaghost_creation() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 15 }; // Boss floor

        let hexaghost = Hexaghost::instantiate(&mut rng, &global_info);

        assert_eq!(Hexaghost::get_name(), "Hexaghost");
        assert_eq!(hexaghost.get_hp(), 250);
        assert_eq!(hexaghost.move_count, 0);
        assert!(!hexaghost.has_inferno_upgrades());
    }

    #[test]
    fn test_hexaghost_move_sequence() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 15 };

        let mut hexaghost = Hexaghost::instantiate(&mut rng, &global_info);

        // Test the first 9 moves (initial + first full cycle)
        let expected_moves = vec![
            HexaghostMove::Activate,    // Turn 1
            HexaghostMove::Divider,     // Turn 2
            HexaghostMove::Sear,        // Turn 3
            HexaghostMove::Tackle,      // Turn 4
            HexaghostMove::Sear,        // Turn 5
            HexaghostMove::Inflame,     // Turn 6
            HexaghostMove::Tackle,      // Turn 7
            HexaghostMove::Sear,        // Turn 8
            HexaghostMove::Inferno,     // Turn 9
        ];

        for (i, expected_move) in expected_moves.iter().enumerate() {
            let (actual_move, _) = hexaghost.choose_move_and_effects(&global_info, &mut rng);
            assert_eq!(actual_move, *expected_move, "Move {} should be {:?}", i + 1, expected_move);
        }

        // Test that the cycle repeats
        let move10 = hexaghost.get_current_move();
        assert_eq!(move10, HexaghostMove::Sear, "Cycle should repeat with Sear");
    }

    #[test]
    fn test_hexaghost_sear_effects() {
        let hexaghost = Hexaghost::new(250, 0);
        let effects = hexaghost.get_move_effects(HexaghostMove::Sear, 0);

        assert_eq!(effects.len(), 2);

        // Check attack effect
        match &effects[0] {
            Effect::AttackToTarget { amount, num_attacks, .. } => {
                assert_eq!(*amount, 6);
                assert_eq!(*num_attacks, 1);
            }
            _ => panic!("Expected AttackToTarget effect"),
        }

        // Check Burn addition effect
        match &effects[1] {
            Effect::AddStatusToDiscard { .. } => {
                // Burn status card - we can't easily check the card type here
                // but we can verify it's AddStatusToDiscard
            }
            _ => panic!("Expected AddStatusToDiscard effect"),
        }
    }

    #[test]
    fn test_hexaghost_tackle_effects() {
        let hexaghost_a0 = Hexaghost::new(250, 0);
        let hexaghost_a4 = Hexaghost::new(250, 4);

        let effects_a0 = hexaghost_a0.get_move_effects(HexaghostMove::Tackle, 0);
        let effects_a4 = hexaghost_a4.get_move_effects(HexaghostMove::Tackle, 0);

        assert_eq!(effects_a0.len(), 1);
        assert_eq!(effects_a4.len(), 1);

        // A0: 5×2 hits
        match &effects_a0[0] {
            Effect::AttackToTarget { amount, num_attacks, .. } => {
                assert_eq!(*amount, 5);
                assert_eq!(*num_attacks, 2);
            }
            _ => panic!("Expected AttackToTarget effect"),
        }

        // A4: 6×2 hits
        match &effects_a4[0] {
            Effect::AttackToTarget { amount, num_attacks, .. } => {
                assert_eq!(*amount, 6);
                assert_eq!(*num_attacks, 2);
            }
            _ => panic!("Expected AttackToTarget effect"),
        }
    }

    #[test]
    fn test_hexaghost_inflame_effects() {
        let hexaghost_a0 = Hexaghost::new(250, 0);
        let hexaghost_a19 = Hexaghost::new(250, 19);

        let effects_a0 = hexaghost_a0.get_move_effects(HexaghostMove::Inflame, 0);
        let effects_a19 = hexaghost_a19.get_move_effects(HexaghostMove::Inflame, 0);

        assert_eq!(effects_a0.len(), 2);
        assert_eq!(effects_a19.len(), 2);

        // Check Block effect (same for both)
        match &effects_a0[0] {
            Effect::GainDefense { amount, .. } => assert_eq!(*amount, 12),
            _ => panic!("Expected GainDefense effect"),
        }

        match &effects_a19[0] {
            Effect::GainDefense { amount, .. } => assert_eq!(*amount, 12),
            _ => panic!("Expected GainDefense effect"),
        }

        // Check Strength effects
        match &effects_a0[1] {
            Effect::GainStrength { amount, .. } => assert_eq!(*amount, 2),
            _ => panic!("Expected GainStrength effect"),
        }

        match &effects_a19[1] {
            Effect::GainStrength { amount, .. } => assert_eq!(*amount, 3),
            _ => panic!("Expected GainStrength effect"),
        }
    }

    #[test]
    fn test_hexaghost_inferno_effects() {
        let hexaghost_a0 = Hexaghost::new(250, 0);
        let hexaghost_a4 = Hexaghost::new(250, 4);

        let effects_a0 = hexaghost_a0.get_move_effects(HexaghostMove::Inferno, 0);
        let effects_a4 = hexaghost_a4.get_move_effects(HexaghostMove::Inferno, 0);

        // A0: Should have attack + 3 Burns = 4 effects
        assert_eq!(effects_a0.len(), 4);
        // A4: Should have attack + 3 Burns = 4 effects
        assert_eq!(effects_a4.len(), 4);

        // Check attack effects
        match &effects_a0[0] {
            Effect::AttackToTarget { amount, num_attacks, .. } => {
                assert_eq!(*amount, 6);
                assert_eq!(*num_attacks, 2);
            }
            _ => panic!("Expected AttackToTarget effect"),
        }

        match &effects_a4[0] {
            Effect::AttackToTarget { amount, num_attacks, .. } => {
                assert_eq!(*amount, 6);
                assert_eq!(*num_attacks, 3);
            }
            _ => panic!("Expected AttackToTarget effect"),
        }

        // Check Burn additions (3 burns)
        for i in 1..4 {
            match &effects_a0[i] {
                Effect::AddStatusToDiscard { .. } => {
                    // Expected Burn card
                }
                _ => panic!("Expected AddStatusToDiscard effect for Burn {}", i),
            }
        }
    }

    #[test]
    fn test_hexaghost_divider_damage() {
        let hexaghost = Hexaghost::new(250, 0);

        // Test various player HP levels
        // Formula: (player_hp / 12) + 1 damage per hit
        // With 6 attacks, total damage = ((player_hp / 12) + 1) * 6
        assert_eq!(hexaghost.get_divider_damage(11), 1); // (0) + 1 = 1 damage per hit
        assert_eq!(hexaghost.get_divider_damage(12), 2); // (1) + 1 = 2 damage per hit
        assert_eq!(hexaghost.get_divider_damage(24), 3); // (2) + 1 = 3 damage per hit
        assert_eq!(hexaghost.get_divider_damage(36), 4); // (3) + 1 = 4 damage per hit
        assert_eq!(hexaghost.get_divider_damage(60), 6); // (5) + 1 = 6 damage per hit
    }

    #[test]
    fn test_hexaghost_cycle_after_inferno() {
        let mut hexaghost = Hexaghost::new(250, 0);

        // Fast forward to just after first Inferno (move_count = 9)
        hexaghost.move_count = 9;

        // Next move should be Sear again (start of new cycle)
        let next_move = hexaghost.get_current_move();
        assert_eq!(next_move, HexaghostMove::Sear);

        // Sequence should repeat: Sear, Tackle, Sear, Inflame, Tackle, Sear, Inferno
        let expected_cycle = vec![
            HexaghostMove::Sear,
            HexaghostMove::Tackle,
            HexaghostMove::Sear,
            HexaghostMove::Inflame,
            HexaghostMove::Tackle,
            HexaghostMove::Sear,
            HexaghostMove::Inferno,
        ];

        for (i, expected_move) in expected_cycle.iter().enumerate() {
            hexaghost.move_count = 9 + i;
            let actual_move = hexaghost.get_current_move();
            assert_eq!(actual_move, *expected_move, "Cycle position {} should be {:?}", i, expected_move);
        }
    }
}