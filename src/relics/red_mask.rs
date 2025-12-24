use crate::battle::battle_events::{BattleEvent, EventListener};
use crate::battle::target::Entity;
use crate::game::effect::BattleEffect;

/// Red Mask relic (Event relic)
/// At the start of each combat, apply 1 Weak to ALL enemies
#[derive(Debug)]
pub struct RedMaskRelic {
    used: bool,
    owner: Entity,
}

impl RedMaskRelic {
    pub fn new(owner: Entity) -> Self {
        RedMaskRelic {
            used: false,
            owner,
        }
    }
}

impl EventListener for RedMaskRelic {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<BattleEffect> {
        match event {
            BattleEvent::CombatStart { player } if !self.used && *player == self.owner => {
                self.used = true;
                // Apply 1 Weak to ALL enemies
                vec![BattleEffect::ApplyWeakAll { duration: 1 }]
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

    #[test]
    fn test_red_mask_creation() {
        let player = Entity::Player;
        let mask = RedMaskRelic::new(player);
        assert!(mask.is_active());
        assert_eq!(mask.get_owner(), Entity::Player);
    }

    #[test]
    fn test_red_mask_triggers_on_combat_start() {
        let player = Entity::Player;
        let mut mask = RedMaskRelic::new(player);

        let combat_start_event = BattleEvent::CombatStart { player };
        let effects = mask.on_event(&combat_start_event);

        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0], BattleEffect::ApplyWeakAll { duration: 1 });
        assert!(!mask.is_active()); // Used up for this combat
    }

    #[test]
    fn test_red_mask_reactivates_next_combat() {
        let player = Entity::Player;
        let mut mask = RedMaskRelic::new(player);

        // First combat
        let combat_start_event = BattleEvent::CombatStart { player };
        let effects1 = mask.on_event(&combat_start_event);
        assert_eq!(effects1.len(), 1);
        assert!(!mask.is_active());

        // Reset for next combat (simulating new combat)
        let mut mask = RedMaskRelic::new(player);
        let effects2 = mask.on_event(&combat_start_event);
        assert_eq!(effects2.len(), 1);
        assert!(!mask.is_active());
    }

    #[test]
    fn test_red_mask_only_triggers_for_owner() {
        let player = Entity::Player;
        let enemy = Entity::Enemy(0);
        let mut mask = RedMaskRelic::new(player);

        // Combat start for enemy should not trigger
        let enemy_combat_start = BattleEvent::CombatStart { player: enemy };
        let effects = mask.on_event(&enemy_combat_start);
        assert_eq!(effects.len(), 0);
        assert!(mask.is_active());

        // Combat start for player should trigger
        let player_combat_start = BattleEvent::CombatStart { player };
        let effects = mask.on_event(&player_combat_start);
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0], BattleEffect::ApplyWeakAll { duration: 1 });
        assert!(!mask.is_active());
    }

    #[test]
    fn test_red_mask_weak_duration() {
        let player = Entity::Player;
        let mut mask = RedMaskRelic::new(player);

        let combat_start_event = BattleEvent::CombatStart { player };
        let effects = mask.on_event(&combat_start_event);

        assert_eq!(effects.len(), 1);
        if let BattleEffect::ApplyWeakAll { duration } = effects[0] {
            assert_eq!(duration, 1, "Weak should last 1 turn");
        } else {
            panic!("Expected ApplyWeakAll effect");
        }
    }

    #[test]
    fn test_red_mask_applies_weak_to_all_enemies() {
        use crate::cards::ironclad::strike::strike;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::battle::Battle;
        use crate::enemies::jaw_worm::JawWorm;
        use crate::enemies::enemy_enum::EnemyEnum;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::relics::Relic;
        use crate::game::PlayerRunState;

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

        // Create battle with Red Mask relic
        let player_state = PlayerRunState::new_with_relics(50, 80, 0, vec![Relic::RedMask]);
        let battle = Battle::new(deck, global_info, player_state, enemies, &mut rng);

        // Verify all enemies have 1 Weak
        for enemy in battle.get_enemies() {
            assert_eq!(
                enemy.battle_info.get_weak_turns(),
                1,
                "Each enemy should have 1 Weak turn from Red Mask"
            );
        }
    }
}
