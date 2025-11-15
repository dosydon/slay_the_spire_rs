use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect};
use crate::battle::{events::{BattleEvent, EventListener}, target::Entity};

/// Combust Power Listener
/// At the end of your turn, deal 6 damage to ALL enemies.
#[derive(Debug)]
pub struct CombustListener {
    owner: Entity,
    damage: u32,
}

impl CombustListener {
    pub fn new(owner: Entity, damage: u32) -> Self {
        CombustListener {
            owner,
            damage,
        }
    }
}

impl EventListener for CombustListener {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<Effect> {
        match event {
            BattleEvent::EndOfTurn { entity } if *entity == self.owner => {
                // Lose 1 HP and deal damage to all enemies at end of player's turn
                vec![
                    Effect::AttackAllEnemies {
                        amount: self.damage,
                        num_attacks: 1,
                    },
                    Effect::LoseHp(1),
                ]
            }
            _ => vec![]
        }
    }

    fn is_active(&self) -> bool {
        true // Combust is always active once played
    }

    fn get_owner(&self) -> Entity {
        self.owner
    }
}

/// Combust - Power Card
/// Cost: 1
/// Effect: At the end of your turn, lose 1 HP and deal 5 damage to ALL enemies.
pub fn combust() -> Card {
    Card::new(CardEnum::Combust, 1, CardType::Power, vec![
        Effect::ActivateCombust(5),
    ], false, true)
}

/// Combust+ (Upgraded)
/// Cost: 1
/// Effect: At the end of your turn, lose 1 HP and deal 7 damage to ALL enemies.
pub fn combust_upgraded() -> Card {
    Card::new(CardEnum::Combust, 1, CardType::Power, vec![
        Effect::ActivateCombust(7),
    ], true, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combust_creation() {
        let card = combust();

        assert_eq!(card.get_name(), "Combust");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Power);
        assert_eq!(card.get_effects().len(), 1);
        assert_eq!(card.get_effects()[0], Effect::ActivateCombust(5));
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_combust_upgraded_creation() {
        let card = combust_upgraded();

        assert_eq!(card.get_name(), "Combust+");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Power);
        assert_eq!(card.get_effects().len(), 1);
        assert_eq!(card.get_effects()[0], Effect::ActivateCombust(7));
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_combust_listener_creation() {
        let listener = CombustListener::new(Entity::Player, 5);
        assert!(listener.is_active());
        assert_eq!(listener.get_owner(), Entity::Player);
    }

    #[test]
    fn test_combust_triggers_on_end_of_turn() {
        let mut listener = CombustListener::new(Entity::Player, 5);

        let end_turn_event = BattleEvent::EndOfTurn {
            entity: Entity::Player,
        };

        let effects = listener.on_event(&end_turn_event);
        assert_eq!(effects.len(), 2);
        assert_eq!(effects[0], Effect::AttackAllEnemies {
            amount: 5,
            num_attacks: 1,
        });
        assert_eq!(effects[1], Effect::LoseHp(1));
        assert!(listener.is_active()); // Still active after triggering
    }

    #[test]
    fn test_combust_does_not_trigger_on_other_events() {
        let mut listener = CombustListener::new(Entity::Player, 5);

        let damage_event = BattleEvent::DamageTaken {
            target: Entity::Player,
            amount: 5,
            source: Entity::Enemy(0),
        };

        let effects = listener.on_event(&damage_event);
        assert_eq!(effects.len(), 0);
        assert!(listener.is_active());
    }

    #[test]
    fn test_combust_triggers_multiple_times() {
        let mut listener = CombustListener::new(Entity::Player, 5);

        let end_turn_event = BattleEvent::EndOfTurn {
            entity: Entity::Player,
        };

        // First end of turn
        let effects1 = listener.on_event(&end_turn_event);
        assert_eq!(effects1.len(), 2);
        assert_eq!(effects1[0], Effect::AttackAllEnemies {
            amount: 5,
            num_attacks: 1,
        });
        assert_eq!(effects1[1], Effect::LoseHp(1));

        // Second end of turn should also trigger
        let effects2 = listener.on_event(&end_turn_event);
        assert_eq!(effects2.len(), 2);
        assert_eq!(effects2[0], Effect::AttackAllEnemies {
            amount: 5,
            num_attacks: 1,
        });
        assert_eq!(effects2[1], Effect::LoseHp(1));

        assert!(listener.is_active()); // Always active
    }

    #[test]
    fn test_combust_only_triggers_for_owner() {
        let mut listener = CombustListener::new(Entity::Player, 5);

        // Enemy end of turn should not trigger
        let enemy_end_turn_event = BattleEvent::EndOfTurn {
            entity: Entity::Enemy(0),
        };

        let effects = listener.on_event(&enemy_end_turn_event);
        assert_eq!(effects.len(), 0);

        // Player end of turn should trigger
        let player_end_turn_event = BattleEvent::EndOfTurn {
            entity: Entity::Player,
        };

        let effects = listener.on_event(&player_end_turn_event);
        assert_eq!(effects.len(), 2);
        assert_eq!(effects[0], Effect::AttackAllEnemies {
            amount: 5,
            num_attacks: 1,
        });
        assert_eq!(effects[1], Effect::LoseHp(1));
    }

    #[test]
    fn test_combust_different_damage_values() {
        let mut normal_listener = CombustListener::new(Entity::Player, 5);
        let mut upgraded_listener = CombustListener::new(Entity::Player, 7);

        let end_turn_event = BattleEvent::EndOfTurn {
            entity: Entity::Player,
        };

        let normal_effects = normal_listener.on_event(&end_turn_event);
        let upgraded_effects = upgraded_listener.on_event(&end_turn_event);

        assert_eq!(normal_effects.len(), 2);
        assert_eq!(normal_effects[0], Effect::AttackAllEnemies {
            amount: 5,
            num_attacks: 1,
        });
        assert_eq!(normal_effects[1], Effect::LoseHp(1));

        assert_eq!(upgraded_effects.len(), 2);
        assert_eq!(upgraded_effects[0], Effect::AttackAllEnemies {
            amount: 7,
            num_attacks: 1,
        });
        assert_eq!(upgraded_effects[1], Effect::LoseHp(1));
    }

    #[test]
    fn test_combust_hp_loss_integration() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::battle::action::Action;
        use crate::enemies::red_louse::RedLouse;
        use crate::enemies::enemy_enum::EnemyEnum;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let initial_enemy_hp = red_louse.get_hp();
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create battle with Combust in hand, starting with 15 HP
        let deck = Deck::new(vec![combust()]);
        let mut battle = Battle::new(deck, global_info, 15, 80, enemies, &mut rng);

        let initial_hp = battle.get_player().battle_info.get_hp();
        assert_eq!(initial_hp, 15);

        // Play Combust to activate the power
        let combust_idx = 0;
        let result = battle.play_card(combust_idx, Entity::Player);
        assert!(result.is_ok());

        // Verify Combust is active
        let powers = battle.get_powers();
        assert_eq!(powers.len(), 1);
        assert_eq!(powers[0].get_name(), "Combust");

        // End turn to trigger Combust effect (lose 1 HP and damage enemies)
        let result = battle.eval_action(Action::EndTurn, &mut rng);
        assert!(result.is_ok());

        // Verify player lost 1 HP from Combust + damage from enemy attack (6)
        // Starting from 15 HP: 15 - 1 (Combust) - 6 (Red Louse attack) = 8 HP
        let final_hp = battle.get_player().battle_info.get_hp();
        assert_eq!(final_hp, 8);

        // Verify enemies took damage
        let enemy_hp = battle.get_enemies()[0].battle_info.get_hp();
        assert_eq!(enemy_hp, initial_enemy_hp.saturating_sub(5));

        // End another turn to verify repeated HP loss
        let result = battle.eval_action(Action::EndTurn, &mut rng);
        assert!(result.is_ok());

        // Verify player lost another 1 HP from Combust + damage from enemy attack (6)
        // Total: 8 - 1 (Combust) - 6 (Red Louse attack) = 1 HP
        let final_hp = battle.get_player().battle_info.get_hp();
        assert_eq!(final_hp, 1);

        // Verify enemies took more damage
        let enemy_hp = battle.get_enemies()[0].battle_info.get_hp();
        assert_eq!(enemy_hp, initial_enemy_hp.saturating_sub(10));
    }
}