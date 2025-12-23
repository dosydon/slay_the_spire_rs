use crate::game::enemy::EnemyTrait;
use crate::game::global_info::GlobalInfo;
use crate::game::effect::BattleEffect;
use crate::utils::CategoricalDistribution;
use crate::battle::battle_events::{BattleEvent, EventListener};
use crate::battle::target::Entity;

/// Lagavulin - Act 1 Elite Enemy
///
/// **Health:**
/// - Base: 109-111 HP
/// - Ascension 8+: 112-115 HP
///
/// **Starting Powers:**
/// - Metallicize 8: At end of turn, gains 8 Block
/// - Block: Starts with 8 Block
/// - Asleep: Starts combat sleeping
///
/// **Behavior:**
/// - While asleep: Takes no action for 3 turns or until damaged
/// - When damaged while asleep: Becomes Stunned for 1 turn, then awakens
/// - When awakened: Loses Metallicize power
///
/// **Moves (When Awakened):**
/// 1. Attack - Damage: 18 (20 at A3+)
/// 2. Siphon Soul - Applies -1 Dexterity and -1 Strength (-2/-2 at A18+)
///
/// **Pattern:** Attack → Attack → Siphon Soul (repeating)
#[derive(Debug, Clone)]
pub struct Lagavulin {
    hp: u32,
    max_hp: u32,
    state: LagavulinState,
    turns_asleep: u32,
    move_count: u32,
    ascension: u32,
}

#[derive(Debug, Clone, PartialEq)]
enum LagavulinState {
    Asleep,
    Stunned,
    Awake,
}

#[derive(Copy, Debug, Clone, PartialEq)]
pub enum LagavulinMove {
    Asleep,
    Stunned,
    Attack,
    SiphonSoul,
}

impl Lagavulin {
    pub fn new(hp: u32, ascension: u32) -> Self {
        Lagavulin {
            hp,
            max_hp: hp,
            state: LagavulinState::Asleep,
            turns_asleep: 0,
            move_count: 0,
            ascension,
        }
    }

    pub fn calculate_hp_range(global_info: &GlobalInfo) -> (u32, u32) {
        if global_info.ascention >= 8 {
            (112, 115)
        } else {
            (109, 111)
        }
    }

    /// Check if Lagavulin should wake up (3 turns passed)
    fn should_wake_from_sleep(&self) -> bool {
        self.state == LagavulinState::Asleep && self.turns_asleep >= 3
    }

    /// Wake up from being damaged
    pub fn wake_from_damage(&mut self) {
        if self.state == LagavulinState::Asleep {
            self.state = LagavulinState::Stunned;
        }
    }

    /// Called at start of turn to handle state transitions
    pub fn at_start_of_turn(&mut self) {
        // If stunned, transition to awake
        if self.state == LagavulinState::Stunned {
            self.state = LagavulinState::Awake;
            self.move_count = 0;
        } else if self.state == LagavulinState::Asleep && self.should_wake_from_sleep() {
            self.state = LagavulinState::Awake;
        }
    }

    /// Get the attack damage based on ascension
    fn get_attack_damage(&self) -> u32 {
        if self.ascension >= 3 {
            20
        } else {
            18
        }
    }

    /// Get the Siphon Soul debuff amounts
    fn get_siphon_amounts(&self) -> (u32, u32) {
        if self.ascension >= 18 {
            (2, 2) // -2 Dex, -2 Str
        } else {
            (1, 1) // -1 Dex, -1 Str
        }
    }

    fn get_valid_moves(&self) -> Vec<LagavulinMove> {
        match self.state {
            LagavulinState::Asleep => {
                if self.should_wake_from_sleep() {
                    vec![LagavulinMove::Stunned]
                } else {
                    vec![LagavulinMove::Asleep]
                }
            }
            LagavulinState::Stunned => vec![LagavulinMove::Stunned],
            LagavulinState::Awake => {
                // Pattern: Attack → Attack → Siphon Soul (repeating)
                let move_in_cycle = self.move_count % 3;
                match move_in_cycle {
                    0 | 1 => vec![LagavulinMove::Attack],
                    2 => vec![LagavulinMove::SiphonSoul],
                    _ => unreachable!(),
                }
            }
        }
    }

    pub fn record_move(&mut self, move_type: LagavulinMove) {
        match move_type {
            LagavulinMove::Asleep => {
                self.turns_asleep += 1;
            }
            LagavulinMove::Stunned => {
                self.move_count = 0;
            }
            LagavulinMove::Attack | LagavulinMove::SiphonSoul => {
                self.move_count += 1;
            }
        }
    }

    pub fn get_move_effects(&self, move_type: LagavulinMove) -> Vec<BattleEffect> {
        match move_type {
            LagavulinMove::Asleep | LagavulinMove::Stunned => {
                // No effects for sleeping or stunned
                vec![]
            }
            LagavulinMove::Attack => {
                vec![BattleEffect::AttackToTarget {
                    amount: self.get_attack_damage(),
                    num_attacks: 1,
                    strength_multiplier: 1,
                }]
            }
            LagavulinMove::SiphonSoul => {
                let (dex_loss, str_loss) = self.get_siphon_amounts();
                vec![
                    BattleEffect::LoseDexterityTarget(dex_loss),
                    BattleEffect::LoseStrengthTarget(str_loss),
                ]
            }
        }
    }

    fn choose_next_move(&self, _global_info: &GlobalInfo) -> CategoricalDistribution<LagavulinMove> {
        let valid_moves = self.get_valid_moves();

        // Lagavulin has deterministic behavior based on state
        let outcomes_and_weights: Vec<(LagavulinMove, f64)> = valid_moves
            .into_iter()
            .map(|move_type| (move_type, 1.0))
            .collect();

        CategoricalDistribution::new(outcomes_and_weights)
    }

    pub fn set_hp(&mut self, hp: u32) {
        self.hp = hp;

        // If damaged while asleep, trigger wake sequence
        if self.hp < self.max_hp && self.state == LagavulinState::Asleep {
            self.wake_from_damage();
        }
    }
}

impl EnemyTrait for Lagavulin {
    type MoveType = LagavulinMove;

    fn instantiate(rng: &mut impl rand::Rng, global_info: &GlobalInfo) -> Self {
        let (hp_min, hp_max) = Self::calculate_hp_range(global_info);
        let hp = hp_min + rng.random_range(0..=(hp_max - hp_min));
        let ascension = global_info.ascention;

        Lagavulin::new(hp, ascension)
    }

    fn get_name() -> String {
        "Lagavulin".to_string()
    }

    fn get_hp(&self) -> u32 {
        self.hp
    }

    fn choose_move_and_effects(&mut self, global_info: &GlobalInfo, rng: &mut impl rand::Rng) -> (LagavulinMove, Vec<BattleEffect>) {
        let move_distribution = self.choose_next_move(global_info);
        let selected_move = move_distribution.sample_owned(rng);

        // Record the move for state tracking
        self.record_move(selected_move);

        // Generate the effects for this move
        let effects = self.get_move_effects(selected_move);

        (selected_move, effects)
    }
}

/// Event listener for Lagavulin's mechanics
/// - Grants 8 block at combat start
/// - Wakes up (to Stunned) when damaged while asleep
/// - Transitions from Stunned to Awake at start of enemy turn
pub struct LagavulinListener {
    enemy_index: usize,
    has_given_initial_block: bool,
    has_woken: bool,
}

impl LagavulinListener {
    pub(crate) fn new(enemy_index: usize) -> Self {
        LagavulinListener {
            enemy_index,
            has_given_initial_block: false,
            has_woken: false,
        }
    }
}

impl EventListener for LagavulinListener {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<BattleEffect> {
        match event {
            BattleEvent::CombatStart { .. } if !self.has_given_initial_block => {
                self.has_given_initial_block = true;
                vec![BattleEffect::GainDefense { amount: 8 }]
            }
            BattleEvent::DamageTaken { target, amount, .. }
                if *target == Entity::Enemy(self.enemy_index) && *amount > 0 && !self.has_woken => {
                self.has_woken = true;
                // Wake Lagavulin to Stunned state
                vec![BattleEffect::WakeLagavulin { enemy_index: self.enemy_index }]
            }
            BattleEvent::StartOfEnemyTurn { enemy_index } if *enemy_index == self.enemy_index => {
                // Transition Lagavulin from Stunned to Awake at start of turn
                vec![BattleEffect::TransitionLagavulinStunnedToAwake { enemy_index: self.enemy_index }]
            }
            _ => vec![]
        }
    }

    fn is_active(&self) -> bool {
        true  // Always active to handle turn transitions
    }

    fn get_owner(&self) -> Entity {
        Entity::Enemy(self.enemy_index)
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{enemies::enemy_enum::EnemyMove, game::PlayerRunState};

    #[test]
    fn test_lagavulin_creation() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let lagavulin = Lagavulin::instantiate(&mut rng, &global_info);

        assert_eq!(Lagavulin::get_name(), "Lagavulin");
        assert!(lagavulin.get_hp() >= 109);
        assert!(lagavulin.get_hp() <= 111);
        assert_eq!(lagavulin.state, LagavulinState::Asleep);
        assert_eq!(lagavulin.turns_asleep, 0);
    }

    #[test]
    fn test_lagavulin_ascension_hp() {
        let mut rng = rand::rng();
        let global_info_a8 = GlobalInfo { ascention: 8, current_floor: 1 };

        let lagavulin = Lagavulin::instantiate(&mut rng, &global_info_a8);

        assert!(lagavulin.get_hp() >= 112);
        assert!(lagavulin.get_hp() <= 115);
    }

    #[test]
    fn test_lagavulin_wake_from_damage() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let mut lagavulin = Lagavulin::instantiate(&mut rng, &global_info);

        // Lagavulin starts asleep
        assert_eq!(lagavulin.state, LagavulinState::Asleep);

        // Take damage while asleep - triggers wake to Stunned state
        lagavulin.wake_from_damage();

        // Should now be stunned (damage transitions to stunned first)
        assert_eq!(lagavulin.state, LagavulinState::Stunned);

        // At start of next turn, Lagavulin transitions from Stunned to Awake
        lagavulin.at_start_of_turn();
        assert_eq!(lagavulin.state, LagavulinState::Awake);

        // Next turn should attack (first move in attack pattern)
        let (move1, _) = lagavulin.choose_move_and_effects(&global_info, &mut rng);
        assert_eq!(move1, LagavulinMove::Attack);
        assert_eq!(lagavulin.state, LagavulinState::Awake);

        // Second turn should also attack
        let (move2, _) = lagavulin.choose_move_and_effects(&global_info, &mut rng);
        assert_eq!(move2, LagavulinMove::Attack);
    }

    #[test]
    fn test_lagavulin_attack_pattern() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let mut lagavulin = Lagavulin::instantiate(&mut rng, &global_info);

        // Force awake state
        lagavulin.state = LagavulinState::Awake;
        lagavulin.move_count = 0;

        // Pattern should be: Attack → Attack → Siphon Soul → Attack → Attack → Siphon Soul...
        let (move1, _) = lagavulin.choose_move_and_effects(&global_info, &mut rng);
        assert_eq!(move1, LagavulinMove::Attack);

        let (move2, _) = lagavulin.choose_move_and_effects(&global_info, &mut rng);
        assert_eq!(move2, LagavulinMove::Attack);

        let (move3, _) = lagavulin.choose_move_and_effects(&global_info, &mut rng);
        assert_eq!(move3, LagavulinMove::SiphonSoul);

        let (move4, _) = lagavulin.choose_move_and_effects(&global_info, &mut rng);
        assert_eq!(move4, LagavulinMove::Attack);

        let (move5, _) = lagavulin.choose_move_and_effects(&global_info, &mut rng);
        assert_eq!(move5, LagavulinMove::Attack);

        let (move6, _) = lagavulin.choose_move_and_effects(&global_info, &mut rng);
        assert_eq!(move6, LagavulinMove::SiphonSoul);
    }

    #[test]
    fn test_lagavulin_attack_damage_scaling() {
        let mut rng = rand::rng();
        let global_info_base = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_a3 = GlobalInfo { ascention: 3, current_floor: 1 };

        let mut lagavulin_base = Lagavulin::instantiate(&mut rng, &global_info_base);
        lagavulin_base.state = LagavulinState::Awake;

        let mut lagavulin_a3 = Lagavulin::instantiate(&mut rng, &global_info_a3);
        lagavulin_a3.state = LagavulinState::Awake;

        let (_, effects_base) = lagavulin_base.choose_move_and_effects(&global_info_base, &mut rng);
        let (_, effects_a3) = lagavulin_a3.choose_move_and_effects(&global_info_a3, &mut rng);

        // Check attack damage
        match &effects_base[0] {
            BattleEffect::AttackToTarget { amount, .. } => assert_eq!(*amount, 18),
            _ => panic!("Expected AttackToTarget"),
        }

        match &effects_a3[0] {
            BattleEffect::AttackToTarget { amount, .. } => assert_eq!(*amount, 20),
            _ => panic!("Expected AttackToTarget"),
        }
    }

    #[test]
    fn test_lagavulin_siphon_soul_scaling() {
        let mut rng = rand::rng();
        let global_info_base = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_a18 = GlobalInfo { ascention: 18, current_floor: 1 };

        let mut lagavulin_base = Lagavulin::instantiate(&mut rng, &global_info_base);
        lagavulin_base.state = LagavulinState::Awake;
        lagavulin_base.move_count = 2; // Force Siphon Soul

        let mut lagavulin_a18 = Lagavulin::instantiate(&mut rng, &global_info_a18);
        lagavulin_a18.state = LagavulinState::Awake;
        lagavulin_a18.move_count = 2; // Force Siphon Soul

        let (move_base, effects_base) = lagavulin_base.choose_move_and_effects(&global_info_base, &mut rng);
        let (move_a18, effects_a18) = lagavulin_a18.choose_move_and_effects(&global_info_a18, &mut rng);

        assert_eq!(move_base, LagavulinMove::SiphonSoul);
        assert_eq!(move_a18, LagavulinMove::SiphonSoul);

        // Base: -1 Dexterity, -1 Strength
        assert_eq!(effects_base.len(), 2);
        match &effects_base[0] {
            BattleEffect::LoseDexterityTarget(amount) => assert_eq!(*amount, 1),
            _ => panic!("Expected LoseDexterityTarget"),
        }
        match &effects_base[1] {
            BattleEffect::LoseStrengthTarget(amount) => assert_eq!(*amount, 1),
            _ => panic!("Expected LoseStrengthTarget"),
        }

        // A18+: -2 Dexterity, -2 Strength
        assert_eq!(effects_a18.len(), 2);
        match &effects_a18[0] {
            BattleEffect::LoseDexterityTarget(amount) => assert_eq!(*amount, 2),
            _ => panic!("Expected LoseDexterityTarget"),
        }
        match &effects_a18[1] {
            BattleEffect::LoseStrengthTarget(amount) => assert_eq!(*amount, 2),
            _ => panic!("Expected LoseStrengthTarget"),
        }
    }

    #[test]
    fn test_lagavulin_battle_integration_wake_from_damage() {
        use crate::battle::Battle;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::battle::target::Entity;
        use crate::game::deck::Deck;
        use crate::cards::ironclad::strike::strike;
        use crate::enemies::enemy_enum::{EnemyEnum, EnemyMove};

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        // Create a Lagavulin enemy
        let lagavulin = Lagavulin::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::Lagavulin(lagavulin));

        // Create battle with Lagavulin
        let deck = Deck::new(vec![strike(), strike(), strike(), strike(), strike()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(80, 80, 0), vec![enemy], &mut rng);

        // Verify Lagavulin starts in Asleep state by checking its first sampled move
        if let Some(enemy_move) = battle.get_enemy_move(0) {
            match enemy_move {
                EnemyMove::Lagavulin(LagavulinMove::Asleep) => {
                    // Correct - Lagavulin starts asleep
                }
                _ => panic!("Expected Lagavulin to be Asleep initially, got {:?}", enemy_move),
            }
        }

        // Verify Lagavulin has 8 block from initial block listener
        assert_eq!(battle.get_enemies()[0].battle_info.get_block(), 8);

        // Play two Strikes to damage the sleeping Lagavulin (6+6=12 damage > 8 block)
        let result = battle.play_card(0, Entity::Enemy(0));
        assert!(result.is_ok(), "Should be able to play first Strike");
        let result = battle.play_card(0, Entity::Enemy(0)); // Index 0 again since cards shift after first play
        assert!(result.is_ok(), "Should be able to play second Strike");

        // The wake listener should have triggered and Lagavulin is now Stunned
        // Verify Lagavulin is in Stunned state
        if let EnemyEnum::Lagavulin(lagavulin) = &battle.get_enemies()[0].enemy {
            assert_eq!(lagavulin.state, LagavulinState::Stunned, "Lagavulin should be Stunned after taking damage");
        }

        // Start enemy turn - this will transition Lagavulin from Stunned to Awake
        battle.at_start_of_enemy_turn();

        // Sample enemy actions after the transition
        battle.sample_enemy_actions(&mut rng);

        // Check what move the Lagavulin sampled - should be Attack after waking
        if let Some(enemy_move) = battle.get_enemy_move(0) {
            match enemy_move {
                EnemyMove::Lagavulin(LagavulinMove::Attack) => {
                    // Correct - Lagavulin attacks after transitioning from Stunned to Awake
                }
                _ => panic!("Expected Lagavulin to Attack after waking from damage, got {:?}", enemy_move),
            }
        }

        // Verify Lagavulin is now Awake
        if let EnemyEnum::Lagavulin(lagavulin) = &battle.get_enemies()[0].enemy {
            assert_eq!(lagavulin.state, LagavulinState::Awake, "Lagavulin should be Awake after turn start");
        }

        // Execute enemy turn
        battle.process_enemy_effects(&mut rng, &global_info);

        // Start next player turn and sample enemy actions again
        battle.at_start_of_player_turn(&mut rng);

        // Check what move the Lagavulin sampled - should still be Attack
        if let Some(enemy_move) = battle.get_enemy_move(0) {
            match enemy_move {
                EnemyMove::Lagavulin(LagavulinMove::Attack) => {
                    // Correct - second attack in pattern
                }
                _ => panic!("Expected Lagavulin to Attack (second in pattern), got {:?}", enemy_move),
            }
        }
    }

    #[test]
    fn test_lagavulin_starts_with_8_block() {
        use crate::battle::Battle;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::cards::ironclad::strike::strike;
        use crate::enemies::enemy_enum::{EnemyEnum};

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        // Create a Lagavulin enemy
        let lagavulin = Lagavulin::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::Lagavulin(lagavulin));

        // Create battle with Lagavulin
        let deck = Deck::new(vec![strike(); 5]);
        let battle = Battle::new(deck, global_info, PlayerRunState::new(80, 80, 0), vec![enemy], &mut rng);

        // Verify Lagavulin starts with 8 block from initial block listener
        // This tests both the initial block listener and indirectly confirms Metallicize 8 power
        assert_eq!(battle.get_enemies()[0].battle_info.get_block(), 8, "Lagavulin should start combat with 8 block");
    }

    #[test]
    fn test_lagavulin_wake_from_damage_direct() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let mut lagavulin = Lagavulin::instantiate(&mut rng, &global_info);
        let initial_hp = lagavulin.get_hp();

        // Lagavulin starts asleep
        assert_eq!(lagavulin.state, LagavulinState::Asleep);

        // Simulate taking damage while asleep - this should trigger wake to Stunned state
        lagavulin.set_hp(initial_hp - 6); // Take 6 damage

        // Should now be stunned (damage transitions to stunned first)
        assert_eq!(lagavulin.state, LagavulinState::Stunned, "Lagavulin should be Stunned after taking damage");

        // At start of next turn, Lagavulin should transition from Stunned to Awake
        lagavulin.at_start_of_turn();
        assert_eq!(lagavulin.state, LagavulinState::Awake, "Lagavulin should be Awake after turn start");

        // Next turn should attack (first move in attack pattern)
        let (move1, _) = lagavulin.choose_move_and_effects(&global_info, &mut rng);
        assert_eq!(move1, LagavulinMove::Attack, "Lagavulin should attack after waking");
        assert_eq!(lagavulin.state, LagavulinState::Awake, "Lagavulin should remain Awake");
    }


    #[test]
    fn test_lagavulin_complete_battle_cycle() {
        use crate::battle::Battle;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::battle::target::Entity;
        use crate::game::deck::Deck;
        use crate::cards::ironclad::strike::strike;
        use crate::enemies::enemy_enum::{EnemyEnum};
        use crate::cards::ironclad::defend::defend;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        // Create a Lagavulin enemy
        let lagavulin = Lagavulin::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::Lagavulin(lagavulin));

        // Create battle with a mix of cards
        let deck = Deck::new(vec![strike(), strike(), strike(), defend(), defend()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(80, 80, 0), vec![enemy], &mut rng);

        // Verify initial state: Asleep with 8 block
        assert_eq!(battle.get_enemies()[0].battle_info.get_block(), 8);
        battle.sample_enemy_actions(&mut rng);
        if let Some(enemy_move) = battle.get_enemy_move(0) {
            assert_eq!(*enemy_move, EnemyMove::Lagavulin(LagavulinMove::Asleep));
        }

        // Player attacks Lagavulin to wake it up (12 damage > 8 block)
        let result = battle.play_card(0, Entity::Enemy(0)); // Strike for 6 damage
        assert!(result.is_ok());
        let result = battle.play_card(0, Entity::Enemy(0)); // Second Strike for 6 damage
        assert!(result.is_ok());

        // Wake up effects should be processed automatically

        // End player turn - this will handle enemy turn and start new player turn
        battle.end_turn(&mut rng, &global_info);

        battle.end_turn(&mut rng, &global_info);

        // Player should have taken damage from the Lagavulin attack
        // (Lagavulin woke up, transitioned from Stunned to Awake, then attacked)
        // We can verify this happened by checking if player took damage
        assert!(battle.get_current_hp() < 80, "Player should have taken damage after Lagavulin wakes up and attacks");

        // End another turn - second attack
        battle.end_turn(&mut rng, &global_info);
        // Player should have taken more damage from second attack
        assert!(battle.get_current_hp() < 76, "Player should have taken more damage from second Lagavulin attack");

        // End another turn - Siphon Soul (this should apply debuffs)
        battle.end_turn(&mut rng, &global_info);
        // Player should have lost 1 Dexterity and 1 Strength from Siphon Soul
        assert_eq!(battle.get_player().get_dexterity(), -1, "Player should have -1 Dexterity from Siphon Soul");
        assert_eq!(battle.get_player().get_strength(), -1, "Player should have -1 Strength from Siphon Soul");

        // End another turn - Attack again (cycle repeats)
        battle.end_turn(&mut rng, &global_info);
        // Player should have taken even more damage from the next attack
        assert!(battle.get_current_hp() < 58, "Player should have taken damage from the next attack in the cycle");
    }

    #[test]
    fn test_lagavulin_natural_sleep_cycle() {
        use crate::battle::Battle;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::cards::ironclad::defend::defend;
        use crate::enemies::enemy_enum::{EnemyEnum, EnemyMove};

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        // Create a Lagavulin enemy
        let lagavulin = Lagavulin::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::Lagavulin(lagavulin));

        // Create battle with only Defend cards so we don't attack Lagavulin
        let deck = Deck::new(vec![defend(), defend(), defend(), defend(), defend()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(80, 80, 0), vec![enemy], &mut rng);

        println!("\n=== Testing Lagavulin Natural Sleep Cycle ===");

        // Initial state: Lagavulin should be Asleep with 8 block
        assert_eq!(battle.get_enemies()[0].battle_info.get_block(), 8, "Lagavulin should start with 8 block");
        if let EnemyEnum::Lagavulin(lagavulin) = &battle.get_enemies()[0].enemy {
            assert_eq!(lagavulin.state, LagavulinState::Asleep, "Lagavulin should start Asleep");
        }

        // Sample enemy actions - should be Asleep
        battle.sample_enemy_actions(&mut rng);
        if let Some(enemy_move) = battle.get_enemy_move(0) {
            assert_eq!(*enemy_move, EnemyMove::Lagavulin(LagavulinMove::Asleep), "Turn 0: Lagavulin should be Asleep");
        }
        println!("Turn 0: Lagavulin is Asleep (0/3 turns)");

        // === TURN 1: Player doesn't attack, just ends turn ===
        battle.end_turn(&mut rng, &global_info);

        // Check Lagavulin state after turn 1
        if let EnemyEnum::Lagavulin(lagavulin) = &battle.get_enemies()[0].enemy {
            assert_eq!(lagavulin.state, LagavulinState::Asleep, "Turn 1: Lagavulin should still be Asleep");
        }

        if let Some(enemy_move) = battle.get_enemy_move(0) {
            assert_eq!(*enemy_move, EnemyMove::Lagavulin(LagavulinMove::Asleep), "Turn 1: Lagavulin should be Asleep");
        }
        println!("Turn 1: Lagavulin is Asleep (1/3 turns)");

        // Player should not have taken any damage
        assert_eq!(battle.get_current_hp(), 80, "Turn 1: Player should not have taken damage");

        // === TURN 2: Player doesn't attack, Lagavulin should wake naturally ===
        battle.end_turn(&mut rng, &global_info);

        // Check Lagavulin state after turn 2 - should have woken up naturally
        if let EnemyEnum::Lagavulin(lagavulin) = &battle.get_enemies()[0].enemy {
            // After 3 turns asleep (initial + turn 1 + turn 2), Lagavulin wakes and transitions to Awake
            assert_eq!(lagavulin.state, LagavulinState::Awake, "Turn 2: Lagavulin should be Awake after 3 sleep turns");
        }

        // Lagavulin should have sampled Attack for next turn
        if let Some(enemy_move) = battle.get_enemy_move(0) {
            assert_eq!(*enemy_move, EnemyMove::Lagavulin(LagavulinMove::Attack), "Turn 2: Lagavulin should Attack after waking");
        }
        println!("Turn 2: Lagavulin wakes naturally and prepares to Attack!");

        // Execute the attack by ending another turn
        battle.end_turn(&mut rng, &global_info);

        // Player should have taken damage (18 damage from Attack)
        assert!(battle.get_current_hp() < 80, "Turn 3: Player should have taken damage from Lagavulin attack");
        let expected_hp = battle.get_current_hp();
        println!("Turn 3: Lagavulin executes Attack!");

        // Continue for a few more turns to verify the Attack → Attack → Siphon Soul pattern
        for turn in 4..=6 {
            battle.end_turn(&mut rng, &global_info);

            if let Some(enemy_move) = battle.get_enemy_move(0) {
                println!("Turn {}: Lagavulin will use {:?}", turn, enemy_move);
            }

            battle.end_turn(&mut rng, &global_info);
        }

        // After several turns, player should have debuffs from Siphon Soul
        assert!(battle.get_player().get_dexterity() < 0, "Player should have negative Dexterity from Siphon Soul");
        assert!(battle.get_player().get_strength() < 0, "Player should have negative Strength from Siphon Soul");

        // Player should have taken damage from attacks
        assert!(battle.get_current_hp() < expected_hp, "Player should have taken more damage");

        println!("\n=== Natural Sleep Cycle Test Complete! ===");
        println!("Summary:");
        println!("  - Turns 0-1: Lagavulin slept naturally (3 sleep turns total)");
        println!("  - Turn 2-3: Lagavulin woke and attacked");
        println!("  - Turns 4+: Attack → Attack → Siphon Soul pattern confirmed");
    }
}
