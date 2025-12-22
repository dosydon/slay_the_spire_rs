use crate::game::{card::Card, effect::{Effect, Condition}, card_type::CardType, card_enum::CardEnum, card::Rarity};
use crate::battle::{battle_events::{BattleEvent, EventListener}, target::Entity};

/// Metallicize Listener
/// At the end of each turn, gain Block
#[derive(Debug)]
pub struct MetallicizeListener {
    owner: Entity,
    block_amount: u32,
    is_active: bool,
}

impl MetallicizeListener {
    pub fn new(owner: Entity, block_amount: u32) -> Self {
        MetallicizeListener {
            owner,
            block_amount,
            is_active: true,
        }
    }

    /// Deactivate this listener (used when Lagavulin wakes up)
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }
}

impl EventListener for MetallicizeListener {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<Effect> {
        match event {
            BattleEvent::EndOfTurn { entity } if *entity == self.owner && self.is_active => {
                // At end of turn, gain Block
                vec![Effect::GainDefense { amount: self.block_amount }]
            }
            _ => vec![]
        }
    }

    fn is_active(&self) -> bool {
        self.is_active
    }

    fn get_owner(&self) -> Entity {
        self.owner
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Metallicize - Power Card
/// Cost: 1 (1 when upgraded)
/// Effect: At the end of your turn, gain 3 Block.
pub fn metallicize() -> Card {
    Card::new(CardEnum::Metallicize, 1, CardType::Power, vec![
        Effect::ActivateMetallicize { amount: 3 },
    ], Rarity::Uncommon)
        .set_play_condition(Condition::True)
}

pub fn metallicize_upgraded() -> Card {
    Card::new(CardEnum::Metallicize, 1, CardType::Power, vec![
        Effect::ActivateMetallicize { amount: 4 },
    ], Rarity::Uncommon)
        .set_upgraded(true)
        .set_play_condition(Condition::True)
}

#[cfg(test)]
mod tests {
    use crate::game::PlayerRunState;

    use super::*;

    #[test]
    fn test_metallicize_creation() {
        let card = metallicize();

        assert_eq!(card.get_name(), "Metallicize");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Power);
        assert_eq!(card.get_effects().len(), 1);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_metallicize_upgraded_creation() {
        let card = metallicize_upgraded();

        assert_eq!(card.get_name(), "Metallicize+");
        assert_eq!(card.get_cost(), 1);  // Cost stays the same
        assert_eq!(card.get_card_type(), &CardType::Power);
        assert_eq!(card.get_effects().len(), 1);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_metallicize_listener_creation() {
        let listener = MetallicizeListener::new(Entity::Player, 3);
        assert!(listener.is_active());
        assert_eq!(listener.get_owner(), Entity::Player);
    }

    #[test]
    fn test_metallicize_triggers_on_end_of_turn() {
        let mut listener = MetallicizeListener::new(Entity::Player, 3);

        let end_turn_event = BattleEvent::EndOfTurn {
            entity: Entity::Player,
        };

        let effects = listener.on_event(&end_turn_event);
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0], Effect::GainDefense { amount: 3 });
        assert!(listener.is_active()); // Still active after triggering
    }

    #[test]
    fn test_metallicize_does_not_trigger_on_enemy_turn() {
        let mut listener = MetallicizeListener::new(Entity::Player, 3);

        let enemy_turn_event = BattleEvent::EndOfTurn {
            entity: Entity::Enemy(0),
        };

        let effects = listener.on_event(&enemy_turn_event);
        assert_eq!(effects.len(), 0);
        assert!(listener.is_active());
    }

    #[test]
    fn test_metallicize_block_amounts() {
        let mut base_listener = MetallicizeListener::new(Entity::Player, 3);
        let mut upgraded_listener = MetallicizeListener::new(Entity::Player, 4);

        let end_turn_event = BattleEvent::EndOfTurn {
            entity: Entity::Player,
        };

        let base_effects = base_listener.on_event(&end_turn_event);
        let upgraded_effects = upgraded_listener.on_event(&end_turn_event);

        assert_eq!(base_effects.len(), 1);
        assert_eq!(base_effects[0], Effect::GainDefense { amount: 3 });

        assert_eq!(upgraded_effects.len(), 1);
        assert_eq!(upgraded_effects[0], Effect::GainDefense { amount: 4 });
    }

    #[test]
    fn test_metallicize_only_triggers_for_owner() {
        let mut listener = MetallicizeListener::new(Entity::Player, 3);

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
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0], Effect::GainDefense { amount: 3 });
    }

    #[test]
    fn test_metallicize_power_activation() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::red_louse::RedLouse;
        use crate::enemies::enemy_enum::EnemyEnum;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create battle with Metallicize in hand
        let deck = Deck::new(vec![metallicize()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        // Play Metallicize (power card targets player)
        let metallicize_idx = 0;
        let result = battle.play_card(metallicize_idx, Entity::Player);
        assert!(result.is_ok());

        // Verify Metallicize was added to powers collection
        let powers = battle.get_powers();
        assert_eq!(powers.len(), 1);
        assert_eq!(powers[0].get_name(), "Metallicize");

        // Verify Metallicize did NOT go to discard pile (power cards stay in play)
        let discard = battle.cards.get_discard_pile();
        assert!(!discard.iter().any(|card| card.get_name() == "Metallicize"));
    }

    #[test]
    fn test_metallicize_cost_stays_same() {
        let metallicize_card = metallicize();
        assert_eq!(metallicize_card.get_cost(), 1, "Metallicize should cost 1 energy");

        let metallicize_plus = metallicize_upgraded();
        assert_eq!(metallicize_plus.get_cost(), 1, "Metallicize+ should also cost 1 energy");
    }
}