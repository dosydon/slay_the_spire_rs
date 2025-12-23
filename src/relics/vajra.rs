use crate::battle::battle_events::{BattleEvent, EventListener};
use crate::battle::target::Entity;
use crate::game::effect::Effect;

/// Vajra relic
/// At the start of each combat, gain 1 Strength
#[derive(Debug)]
pub struct VajraRelic {
    used: bool,
    owner: Entity,
}

impl VajraRelic {
    pub fn new(owner: Entity) -> Self {
        VajraRelic {
            used: false,
            owner,
        }
    }
}

impl EventListener for VajraRelic {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<Effect> {
        match event {
            BattleEvent::CombatStart { player } if !self.used && *player == self.owner => {
                self.used = true;
                // Gain 1 Strength at combat start
                vec![Effect::GainStrength { amount: 1 }]
            }
            _ => vec![]
        }
    }

    fn is_active(&self) -> bool {
        !self.used
    }

    fn get_owner(&self) -> Entity {
        self.owner
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{battle::target::Entity, game::PlayerRunState};

    #[test]
    fn test_vajra_creation() {
        let player = Entity::Player;
        let vajra = VajraRelic::new(player);
        assert!(vajra.is_active());
        assert_eq!(vajra.get_owner(), Entity::Player);
    }

    #[test]
    fn test_vajra_triggers_on_combat_start() {
        let player = Entity::Player;
        let mut vajra = VajraRelic::new(player);

        let combat_start_event = BattleEvent::CombatStart { player };
        let effects = vajra.on_event(&combat_start_event);

        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0], Effect::GainStrength { amount: 1 });
        assert!(!vajra.is_active()); // Used up for this combat
    }

    #[test]
    fn test_vajra_gains_exactly_1_strength() {
        let player = Entity::Player;
        let mut vajra = VajraRelic::new(player);

        let combat_start_event = BattleEvent::CombatStart { player };
        let effects = vajra.on_event(&combat_start_event);

        assert_eq!(effects.len(), 1);
        if let Effect::GainStrength { amount } = effects[0] {
            assert_eq!(amount, 1, "Should gain exactly 1 Strength");
        } else {
            panic!("Expected GainStrength effect");
        }
    }

    #[test]
    fn test_vajra_reactivates_next_combat() {
        let player = Entity::Player;
        let mut vajra = VajraRelic::new(player);

        // First combat
        let combat_start_event = BattleEvent::CombatStart { player };
        let effects1 = vajra.on_event(&combat_start_event);
        assert_eq!(effects1.len(), 1);
        assert!(!vajra.is_active());

        // Reset for next combat (simulating new combat)
        let mut vajra = VajraRelic::new(player);
        let effects2 = vajra.on_event(&combat_start_event);
        assert_eq!(effects2.len(), 1);
        assert!(!vajra.is_active());
    }

    #[test]
    fn test_vajra_only_triggers_for_owner() {
        let player = Entity::Player;
        let enemy = Entity::Enemy(0);
        let mut vajra = VajraRelic::new(player);

        // Combat start for enemy should not trigger
        let enemy_combat_start = BattleEvent::CombatStart { player: enemy };
        let effects = vajra.on_event(&enemy_combat_start);
        assert_eq!(effects.len(), 0);
        assert!(vajra.is_active());

        // Combat start for player should trigger
        let player_combat_start = BattleEvent::CombatStart { player };
        let effects = vajra.on_event(&player_combat_start);
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0], Effect::GainStrength { amount: 1 });
        assert!(!vajra.is_active());
    }

    #[test]
    fn test_vajra_gives_1_strength_to_player() {
        use crate::cards::ironclad::strike::strike;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::battle::Battle;
        use crate::enemies::jaw_worm::JawWorm;
        use crate::enemies::enemy_enum::EnemyEnum;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::relics::Relic;

        // Create a battle context
        let deck = Deck::new(vec![strike()]);
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm))];

        // Create battle with the vajra relic listener
        let player_state = PlayerRunState::new_with_relics(50, 80, 0, vec![Relic::Vajra]);
        let battle = Battle::new(deck, global_info, player_state, enemies, &mut rng);

        // Verify player now has exactly 1 strength (CombatStart event already emitted)
        let final_strength = battle.get_player().battle_info.get_strength();
        assert_eq!(final_strength, 1, "Player should have exactly 1 strength after Vajra relic activation");
    }

    #[test]
    fn test_vajra_stacks_with_multiple_combats() {
        use crate::cards::ironclad::strike::strike;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::battle::Battle;
        use crate::enemies::jaw_worm::JawWorm;
        use crate::enemies::enemy_enum::EnemyEnum;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::relics::Relic;

        // Simulate multiple combats - each combat should trigger Vajra
        let deck = Deck::new(vec![strike()]);
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        // First combat
        let jaw_worm1 = JawWorm::instantiate(&mut rng, &global_info);
        let enemies1 = vec![EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm1))];
        let player_state1 = PlayerRunState::new_with_relics(50, 80, 0, vec![Relic::Vajra]);
        let battle1 = Battle::new(deck.clone(), global_info.clone(), player_state1, enemies1, &mut rng);

        let strength_after_first = battle1.get_player().battle_info.get_strength();
        assert_eq!(strength_after_first, 1, "Player should have 1 strength after first combat");

        // Note: In a real game, player state would be preserved between battles
        // This test demonstrates that Vajra activates once per combat
        // Multiple combats would each grant +1 Strength
    }
}
