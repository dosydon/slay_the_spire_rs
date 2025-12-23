use crate::battle::battle_events::{BattleEvent, EventListener};
use crate::battle::target::Entity;
use crate::game::effect::BattleEffect;

#[derive(Debug)]
pub struct BloodVialRelic {
    used: bool,
    owner: Entity,
}

impl BloodVialRelic {
    pub fn new(owner: Entity) -> Self {
        BloodVialRelic {
            used: false,
            owner,
        }
    }
}

impl EventListener for BloodVialRelic {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<BattleEffect> {
        match event {
            BattleEvent::CombatStart { player } if !self.used && *player == self.owner => {
                self.used = true;
                vec![BattleEffect::Heal(2)]
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
    use crate::{game::PlayerRunState};

    use super::*;

    #[test]
    fn test_blood_vial_relic_creation() {
        let player = Entity::Player;
        let blood_vial = BloodVialRelic::new(player);
        assert!(blood_vial.is_active());
        assert_eq!(blood_vial.get_owner(), Entity::Player);
    }

    #[test]
    fn test_blood_vial_triggers_on_combat_start() {
        let player = Entity::Player;
        let mut blood_vial = BloodVialRelic::new(player);

        let combat_start_event = BattleEvent::CombatStart { player };
        let effects = blood_vial.on_event(&combat_start_event);

        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0], BattleEffect::Heal(2));
        assert!(!blood_vial.is_active()); // Used up
    }

    #[test]
    fn test_blood_vial_only_triggers_once() {
        let player = Entity::Player;
        let mut blood_vial = BloodVialRelic::new(player);

        let combat_start_event = BattleEvent::CombatStart { player };

        // First combat start triggers healing
        blood_vial.on_event(&combat_start_event);

        // Second combat start should not trigger
        let effects = blood_vial.on_event(&combat_start_event);
        assert_eq!(effects.len(), 0);
    }

    #[test]
    fn test_blood_vial_only_triggers_for_owner() {
        let player = Entity::Player;
        let enemy = Entity::Enemy(0);
        let mut blood_vial = BloodVialRelic::new(player);

        // Combat start for enemy should not trigger
        let enemy_combat_start = BattleEvent::CombatStart { player: enemy };
        let effects = blood_vial.on_event(&enemy_combat_start);
        assert_eq!(effects.len(), 0);
        assert!(blood_vial.is_active());

        // Combat start for player should trigger
        let player_combat_start = BattleEvent::CombatStart { player };
        let effects = blood_vial.on_event(&player_combat_start);
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0], BattleEffect::Heal(2));
        assert!(!blood_vial.is_active());
    }

    #[test]
    fn test_blood_vial_heals_exactly_2_hp_to_player() {
        use crate::cards::ironclad::strike::strike;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::battle::Battle;
        use crate::enemies::red_louse::RedLouse;
        use crate::enemies::enemy_enum::EnemyEnum;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::relics::Relic;


        // Create a battle context
        let deck = Deck::new(vec![strike()]);
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create battle with the blood vial relic listener
        let player_state = PlayerRunState::new_with_relics(48, 80, 0, vec![Relic::BloodVial]);
        let battle = Battle::new(deck, global_info, player_state, enemies, &mut rng);

        // Verify player now has exactly 50 HP (48 + 2 from Blood Vial)
        let final_hp = battle.get_player().battle_info.get_hp();
        assert_eq!(final_hp, 50, "Player should have exactly 50 HP after Blood Vial activation");
    }
}