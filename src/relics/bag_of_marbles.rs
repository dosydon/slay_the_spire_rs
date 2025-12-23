use crate::battle::battle_events::{BattleEvent, EventListener};
use crate::battle::target::Entity;
use crate::game::effect::BattleEffect;

/// Bag of Marbles relic
/// At the start of each combat, apply 1 Vulnerable to ALL enemies
#[derive(Debug)]
pub struct BagOfMarblesRelic {
    used: bool,
    owner: Entity,
}

impl BagOfMarblesRelic {
    pub fn new(owner: Entity) -> Self {
        BagOfMarblesRelic {
            used: false,
            owner,
        }
    }
}

impl EventListener for BagOfMarblesRelic {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<BattleEffect> {
        match event {
            BattleEvent::CombatStart { player } if !self.used && *player == self.owner => {
                self.used = true;
                // Apply 1 Vulnerable to ALL enemies
                vec![BattleEffect::ApplyVulnerableAll { duration: 1 }]
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
    fn test_bag_of_marbles_creation() {
        let player = Entity::Player;
        let bag = BagOfMarblesRelic::new(player);
        assert!(bag.is_active());
        assert_eq!(bag.get_owner(), Entity::Player);
    }

    #[test]
    fn test_bag_of_marbles_triggers_on_combat_start() {
        let player = Entity::Player;
        let mut bag = BagOfMarblesRelic::new(player);

        let combat_start_event = BattleEvent::CombatStart { player };
        let effects = bag.on_event(&combat_start_event);

        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0], BattleEffect::ApplyVulnerableAll { duration: 1 });
        assert!(!bag.is_active()); // Used up for this combat
    }

    #[test]
    fn test_bag_of_marbles_reactivates_next_combat() {
        let player = Entity::Player;
        let mut bag = BagOfMarblesRelic::new(player);

        // First combat
        let combat_start_event = BattleEvent::CombatStart { player };
        let effects1 = bag.on_event(&combat_start_event);
        assert_eq!(effects1.len(), 1);
        assert!(!bag.is_active());

        // Reset for next combat (simulating new combat)
        let mut bag = BagOfMarblesRelic::new(player);
        let effects2 = bag.on_event(&combat_start_event);
        assert_eq!(effects2.len(), 1);
        assert!(!bag.is_active());
    }

    #[test]
    fn test_bag_of_marbles_applies_vulnerable_to_all_enemies() {
        use crate::cards::ironclad::strike::strike;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::battle::Battle;
        use crate::enemies::jaw_worm::JawWorm;
        use crate::enemies::enemy_enum::EnemyEnum;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::relics::Relic;

        // Create a battle with multiple enemies
        let deck = Deck::new(vec![strike()]);
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        // Create 3 enemies
        let jaw_worm1 = JawWorm::instantiate(&mut rng, &global_info);
        let jaw_worm2 = JawWorm::instantiate(&mut rng, &global_info);
        let jaw_worm3 = JawWorm::instantiate(&mut rng, &global_info);

        let enemies = vec![
            EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm1)),
            EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm2)),
            EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm3)),
        ];

        // Create battle with Bag of Marbles relic
        let player_state = PlayerRunState::new_with_relics(50, 80, 0, vec![Relic::BagOfMarbles]);
        let battle = Battle::new(deck, global_info, player_state, enemies, &mut rng);

        // Verify all enemies have 1 Vulnerable
        for enemy in battle.get_enemies() {
            assert_eq!(
                enemy.battle_info.get_vulnerable_turns(),
                1,
                "Each enemy should have 1 Vulnerable turn from Bag of Marbles"
            );
        }
    }

    #[test]
    fn test_bag_of_marbles_only_triggers_for_owner() {
        let player = Entity::Player;
        let enemy = Entity::Enemy(0);
        let mut bag = BagOfMarblesRelic::new(player);

        // Combat start for enemy should not trigger
        let enemy_combat_start = BattleEvent::CombatStart { player: enemy };
        let effects = bag.on_event(&enemy_combat_start);
        assert_eq!(effects.len(), 0);
        assert!(bag.is_active());

        // Combat start for player should trigger
        let player_combat_start = BattleEvent::CombatStart { player };
        let effects = bag.on_event(&player_combat_start);
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0], BattleEffect::ApplyVulnerableAll { duration: 1 });
        assert!(!bag.is_active());
    }

    #[test]
    fn test_bag_of_marbles_vulnerable_duration() {
        let player = Entity::Player;
        let mut bag = BagOfMarblesRelic::new(player);

        let combat_start_event = BattleEvent::CombatStart { player };
        let effects = bag.on_event(&combat_start_event);

        assert_eq!(effects.len(), 1);
        if let BattleEffect::ApplyVulnerableAll { duration } = effects[0] {
            assert_eq!(duration, 1, "Vulnerable should last 1 turn");
        } else {
            panic!("Expected ApplyVulnerableAll effect");
        }
    }
}
