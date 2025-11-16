use crate::battle::events::{BattleEvent, EventListener};
use crate::battle::target::Entity;
use crate::game::effect::Effect;

#[derive(Debug)]
pub struct AnchorRelic {
    used: bool,
    owner: Entity,
}

impl AnchorRelic {
    pub fn new(owner: Entity) -> Self {
        AnchorRelic {
            used: false,
            owner,
        }
    }
}

impl EventListener for AnchorRelic {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<Effect> {
        match event {
            BattleEvent::CombatStart { player } if !self.used && *player == self.owner => {
                self.used = true;
                vec![Effect::GainDefense(10)]
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battle::target::Entity;

    #[test]
    fn test_anchor_relic_creation() {
        let player = Entity::Player;
        let anchor = AnchorRelic::new(player);
        assert!(anchor.is_active());
        assert_eq!(anchor.get_owner(), Entity::Player);
    }

    #[test]
    fn test_anchor_triggers_on_combat_start() {
        let player = Entity::Player;
        let mut anchor = AnchorRelic::new(player);

        let combat_start_event = BattleEvent::CombatStart { player };
        let effects = anchor.on_event(&combat_start_event);

        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0], Effect::GainDefense(10));
        assert!(!anchor.is_active()); // Used up
    }

    #[test]
    fn test_anchor_only_triggers_once() {
        let player = Entity::Player;
        let mut anchor = AnchorRelic::new(player);

        let combat_start_event = BattleEvent::CombatStart { player };

        // First combat start triggers block gain
        anchor.on_event(&combat_start_event);

        // Second combat start should not trigger
        let effects = anchor.on_event(&combat_start_event);
        assert_eq!(effects.len(), 0);
    }

    #[test]
    fn test_anchor_only_triggers_for_owner() {
        let player = Entity::Player;
        let enemy = Entity::Enemy(0);
        let mut anchor = AnchorRelic::new(player);

        // Combat start for enemy should not trigger
        let enemy_combat_start = BattleEvent::CombatStart { player: enemy };
        let effects = anchor.on_event(&enemy_combat_start);
        assert_eq!(effects.len(), 0);
        assert!(anchor.is_active());

        // Combat start for player should trigger
        let player_combat_start = BattleEvent::CombatStart { player };
        let effects = anchor.on_event(&player_combat_start);
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0], Effect::GainDefense(10));
        assert!(!anchor.is_active());
    }

    #[test]
    fn test_anchor_gives_exactly_10_block_to_player() {
        use crate::cards::ironclad::strike::strike;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::battle::Battle;
        use crate::enemies::red_louse::RedLouse;
        use crate::enemies::enemy_enum::EnemyEnum;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::relics::Relic;

        let player = Entity::Player;
        let anchor = AnchorRelic::new(player);

        // Create a battle context
        let deck = Deck::new(vec![strike()]);
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create battle with the anchor relic listener
        let relics = vec![Relic::Anchor];
        let battle = Battle::new_with_relics(deck, global_info, 50, 80, enemies, relics, &mut rng);

        // Verify player now has exactly 10 block (CombatStart event already emitted)
        let final_block = battle.get_player().battle_info.get_block();
        assert_eq!(final_block, 10, "Player should have exactly 10 block after Anchor relic activation");
    }
}