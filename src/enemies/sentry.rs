use crate::{game::{effect::BattleEffect, enemy::EnemyTrait, global_info::GlobalInfo}, battle::{battle_events::{BattleEvent, EventListener}, target::Entity}};
use std::any::Any;
use serde::{Serialize, Deserialize};

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SentryMove {
    Bolt,  // Debuff intent - adds Dazed cards
    Beam,  // Attack intent - deals damage
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Sentry {
    hp: u32,
    ascension: u32,
    turn_count: u32,
    starts_with_bolt: bool,  // true = Bolt first (outer sentries), false = Beam first (middle sentry)
}

impl Sentry {
    /// Create a new Sentry
    pub fn new(hp: u32, ascension: u32, starts_with_bolt: bool) -> Self {
        Sentry {
            hp,
            ascension,
            turn_count: 0,
            starts_with_bolt,
        }
    }

    /// Calculate HP range based on ascension
    pub fn calculate_hp_range(ascension: u32) -> (u32, u32) {
        if ascension >= 8 {
            (39, 45)  // Ascension 8+: 39-45 HP
        } else {
            (38, 42)  // Base: 38-42 HP
        }
    }

    /// Calculate Bolt dazed card count based on ascension
    pub fn calculate_bolt_dazed_count(ascension: u32) -> u32 {
        if ascension >= 18 {
            3  // Ascension 18+: 3 Dazed cards
        } else {
            2  // Base: 2 Dazed cards
        }
    }

    /// Calculate Beam damage based on ascension
    pub fn calculate_beam_damage(ascension: u32) -> u32 {
        if ascension >= 3 {
            10  // Ascension 3+: 10 damage
        } else {
            9   // Base: 9 damage
        }
    }

    /// Get current turn move based on starting pattern and turn count
    pub fn get_current_move(&self) -> SentryMove {
        if self.starts_with_bolt {
            // Starts with Bolt: Bolt → Beam → Bolt (repeating)
            if self.turn_count % 2 == 0 {
                SentryMove::Bolt
            } else {
                SentryMove::Beam
            }
        } else {
            // Starts with Beam: Beam → Bolt → Beam (repeating)
            if self.turn_count % 2 == 0 {
                SentryMove::Beam
            } else {
                SentryMove::Bolt
            }
        }
    }

    /// Get the effects for a specific move
    pub fn get_move_effects(&self, move_type: SentryMove) -> Vec<BattleEffect> {
        match move_type {
            SentryMove::Bolt => {
                // Add Dazed cards to discard pile
                let dazed_count = Self::calculate_bolt_dazed_count(self.ascension);
                vec![BattleEffect::AddStatusToDiscard {
                    status_card: crate::game::card_enum::CardEnum::Dazed
                }; dazed_count as usize]
            }
            SentryMove::Beam => {
                // Deal damage to player
                let damage = Self::calculate_beam_damage(self.ascension);
                vec![BattleEffect::AttackToTarget {
                    amount: damage,
                    num_attacks: 1,
                    strength_multiplier: 0,
                }]
            }
        }
    }

    /// Increment turn count and get current move
    pub fn next_turn(&mut self) -> SentryMove {
        let current_move = self.get_current_move();
        self.turn_count += 1;
        current_move
    }

    /// Get valid moves for current turn
    pub fn get_valid_moves(&self) -> Vec<SentryMove> {
        vec![self.get_current_move()]
    }

    /// Create multiple Sentries with proper position patterns
    /// Returns 3 Sentries: Left (Bolt first), Middle (Beam first), Right (Bolt first)
    pub fn create_sentry_group(rng: &mut impl rand::Rng, ascension: u32) -> Vec<Self> {
        let (min_hp, max_hp) = Self::calculate_hp_range(ascension);

        // Create 3 Sentries with alternating patterns
        let hp1 = min_hp + rng.random_range(0..=(max_hp - min_hp));
        let hp2 = min_hp + rng.random_range(0..=(max_hp - min_hp));
        let hp3 = min_hp + rng.random_range(0..=(max_hp - min_hp));

        let sentry1 = Self::new(hp1, ascension, true);   // Left: Bolt first
        let sentry2 = Self::new(hp2, ascension, false);  // Middle: Beam first
        let sentry3 = Self::new(hp3, ascension, true);   // Right: Bolt first

        vec![sentry1, sentry2, sentry3]
    }
}

impl EnemyTrait for Sentry {
    type MoveType = SentryMove;

    fn instantiate(rng: &mut impl rand::Rng, global_info: &GlobalInfo) -> Self {
        let ascension = global_info.ascention;

        // Randomly decide if this Sentry starts with Bolt or Beam
        // In practice, Sentries usually appear in groups of 3 with specific patterns,
        // but for single instantiation, we randomize
        let starts_with_bolt = rng.random_range(0..2) == 0;

        let (min_hp, max_hp) = Self::calculate_hp_range(ascension);
        let hp = min_hp + rng.random_range(0..=(max_hp - min_hp));

        Self::new(hp, ascension, starts_with_bolt)
    }

    fn get_name() -> String {
        "Sentry".to_string()
    }

    fn get_hp(&self) -> u32 {
        self.hp
    }

    fn choose_move_and_effects(&mut self, _global_info: &GlobalInfo, _rng: &mut impl rand::Rng) -> (Self::MoveType, Vec<BattleEffect>) {
        let move_type = self.next_turn();
        let effects = self.get_move_effects(move_type);
        (move_type, effects)
    }
}

/// Event listener for Sentry enemies
/// Grants 1 Artifact at combat start
#[derive(Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct SentryListener {
    enemy_index: usize,
    has_given_artifact: bool,
}

impl SentryListener {
    pub fn new(enemy_index: usize) -> Self {
        SentryListener {
            enemy_index,
            has_given_artifact: false,
        }
    }
}

impl EventListener for SentryListener {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<BattleEffect> {
        match event {
            BattleEvent::CombatStart { .. } if !self.has_given_artifact => {
                self.has_given_artifact = true;
                vec![BattleEffect::GainArtifact { amount: 1 }]
            }
            _ => vec![],
        }
    }

    fn is_active(&self) -> bool {
        true
    }

    fn get_owner(&self) -> Entity {
        Entity::Enemy(self.enemy_index)
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
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
    use crate::battle::Battle;
    use crate::battle::enemy_in_battle::EnemyInBattle;
    use crate::game::PlayerRunState;
    use crate::game::deck::Deck;
    use crate::cards::ironclad::uppercut::uppercut;
    use crate::enemies::enemy_enum::EnemyEnum;
    use crate::battle::target::Entity;

    #[test]
    fn test_sentry_starts_with_artifact() {
        use crate::game::deck::Deck;
        use crate::cards::ironclad::strike::strike;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        // Create a Sentry in an actual battle to trigger the listener
        let sentry = Sentry::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::Sentry(sentry))];

        let deck = Deck::new(vec![strike()]);
        let battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        // Verify Sentry starts with 1 Artifact after combat start event
        assert_eq!(battle.get_enemies()[0].battle_info.get_artifact(), 1);
    }

    #[test]
    fn test_sentry_artifact_blocks_uppercut_debuffs() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        // Create a Sentry enemy
        let sentry = Sentry::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::Sentry(sentry))];

        // Create battle with Uppercut in hand
        let deck = Deck::new(vec![uppercut()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        // Verify Sentry starts with 1 Artifact
        let initial_artifact = battle.get_enemies()[0].battle_info.get_artifact();
        assert_eq!(initial_artifact, 1, "Sentry should start with 1 Artifact");

        let initial_hp = battle.get_enemies()[0].get_current_hp();
        let initial_weak = battle.get_enemies()[0].get_weak();
        let initial_vulnerable = battle.get_enemies()[0].get_vulnerable();

        // Play Uppercut targeting the Sentry
        let result = battle.play_card(0, Entity::Enemy(0));
        assert!(result.is_ok());

        // Verify Sentry took damage (Artifact doesn't block damage)
        let final_hp = battle.get_enemies()[0].get_current_hp();
        assert_eq!(final_hp, initial_hp.saturating_sub(13), "Sentry should take damage");

        // Verify one Artifact charge was consumed (Uppercut applies Weak first)
        let final_artifact = battle.get_enemies()[0].battle_info.get_artifact();
        assert_eq!(final_artifact, 0, "Sentry should have consumed 1 Artifact");

        // Verify first debuff (Weak) was blocked but second (Vulnerable) was applied
        let final_weak = battle.get_enemies()[0].get_weak();
        let final_vulnerable = battle.get_enemies()[0].get_vulnerable();
        assert_eq!(final_weak, initial_weak, "Weak should be blocked by Artifact");
        assert_eq!(final_vulnerable, initial_vulnerable + 1, "Vulnerable should be applied after Artifact consumed");
    }

    #[test]
    fn test_sentry_group_all_have_artifact() {
        use crate::game::deck::Deck;
        use crate::cards::ironclad::strike::strike;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        // Create a group of 3 Sentries in a battle
        let sentries = Sentry::create_sentry_group(&mut rng, global_info.ascention);
        assert_eq!(sentries.len(), 3);

        let enemies: Vec<EnemyInBattle> = sentries
            .into_iter()
            .map(|s| EnemyInBattle::new(EnemyEnum::Sentry(s)))
            .collect();

        let deck = Deck::new(vec![strike()]);
        let battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        // Each Sentry should have 1 Artifact after combat start event
        for (i, _) in battle.get_enemies().iter().enumerate() {
            assert_eq!(
                battle.get_enemies()[i].battle_info.get_artifact(),
                1,
                "Sentry {} should start with 1 Artifact",
                i
            );
        }
    }

    #[test]
    fn test_sentry_listener() {
        use crate::battle::battle_events::BattleEvent;

        // Create a SentryListener for enemy index 0
        let mut listener = SentryListener::new(0);

        // Initially hasn't given artifact
        assert!(!listener.has_given_artifact);

        // On CombatStart, should return GainArtifact effect
        let combat_start_event = BattleEvent::CombatStart {
            player: Entity::Player,
        };
        let effects = listener.on_event(&combat_start_event);

        // Should return exactly one GainArtifact effect
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0], BattleEffect::GainArtifact { amount: 1 });

        // Should mark as having given artifact
        assert!(listener.has_given_artifact);

        // Subsequent CombatStart events should not grant artifact again
        let effects2 = listener.on_event(&combat_start_event);
        assert_eq!(effects2.len(), 0);

        // Other events should not trigger anything
        let other_event = BattleEvent::StartOfPlayerTurn;
        let effects3 = listener.on_event(&other_event);
        assert_eq!(effects3.len(), 0);

        // Listener should be active
        assert!(listener.is_active());

        // Owner should be Enemy(0)
        assert_eq!(listener.get_owner(), Entity::Enemy(0));
    }
}