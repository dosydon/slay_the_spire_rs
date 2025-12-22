use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect, card::{Rarity, CardClass}};
use crate::battle::{target::Entity, battle_events::BattleEvent, battle_events::EventListener};

pub fn fire_breathing() -> Card {
    Card::new(CardEnum::FireBreathing, 1, CardClass::IronClad(Rarity::Uncommon, CardType::Power), vec![
            Effect::AddFireBreathing { damage_per_status: 6 },
        ])
        .set_playable(true)
}

pub fn fire_breathing_upgraded() -> Card {
    Card::new(CardEnum::FireBreathing, 1, CardClass::IronClad(Rarity::Uncommon, CardType::Power), vec![
            Effect::AddFireBreathing { damage_per_status: 10 },
        ])
        .set_upgraded(true)
        .set_playable(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fire_breathing_creation() {
        let card = fire_breathing();
        assert_eq!(card.get_name(), "Fire Breathing");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), CardType::Power);
        assert!(!card.is_upgraded());
        assert_eq!(card.get_effects().len(), 1);
        assert!(card.is_playable());
    }

    #[test]
    fn test_fire_breathing_upgraded_creation() {
        let card = fire_breathing_upgraded();
        assert_eq!(card.get_name(), "Fire Breathing+");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), CardType::Power);
        assert!(card.is_upgraded());
        assert_eq!(card.get_effects().len(), 1);
        assert!(card.is_playable());
    }

    #[test]
    fn test_fire_breathing_effects() {
        let card = fire_breathing();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 1);
        match &effects[0] {
            Effect::AddFireBreathing { damage_per_status } => {
                assert_eq!(*damage_per_status, 6);
            }
            _ => panic!("Expected AddFireBreathing effect"),
        }
    }

    #[test]
    fn test_fire_breathing_upgraded_effects() {
        let card = fire_breathing_upgraded();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 1);
        match &effects[0] {
            Effect::AddFireBreathing { damage_per_status } => {
                assert_eq!(*damage_per_status, 10);
            }
            _ => panic!("Expected AddFireBreathing effect"),
        }
    }
}

pub struct FireBreathingListener {
    source: Entity,
    damage_per_status: u32,
}

impl FireBreathingListener {
    pub fn new(source: Entity, damage_per_status: u32) -> Self {
        Self { source, damage_per_status }
    }
}

impl EventListener for FireBreathingListener {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<Effect> {
        match event {
            BattleEvent::CardDrawn { card_type: _, is_status_or_curse }
                if *is_status_or_curse => {
                vec![
                    Effect::AttackAllEnemies {
                        amount: self.damage_per_status,
                        num_attacks: 1,
                    }
                ]
            }
            _ => vec![]
        }
    }

    fn is_active(&self) -> bool {
        true
    }

    fn get_owner(&self) -> Entity {
        self.source
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::battle::Battle;
    use crate::battle::target::Entity;
    use crate::battle::enemy_in_battle::EnemyInBattle;
    use crate::game::PlayerRunState;
    use crate::game::deck::Deck;
    use crate::game::global_info::GlobalInfo;
    use crate::game::enemy::EnemyTrait;
    use crate::enemies::jaw_worm::JawWorm;
    use crate::enemies::enemy_enum::EnemyEnum;

    #[test]
    fn test_fire_breathing_is_power_card() {
        let card = fire_breathing();
        assert_eq!(card.get_card_type(), CardType::Power);
        assert_eq!(card.get_cost(), 1);
        assert!(!card.is_upgraded());
    }

    #[test]
    fn test_fire_breathing_upgraded_is_power_card() {
        let card = fire_breathing_upgraded();
        assert_eq!(card.get_card_type(), CardType::Power);
        assert_eq!(card.get_cost(), 1);
        assert!(card.is_upgraded());
    }

    #[test]
    fn test_fire_breathing_activates_listener() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm))];

        let deck = Deck::new(vec![fire_breathing()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        // Play Fire Breathing
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());

        // Verify the power card was played (checking it's in powers list would require access to internal state)
        // This test mainly verifies the card can be played successfully
    }

    #[test]
    fn test_fire_breathing_costs_one_energy() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm))];

        let deck = Deck::new(vec![fire_breathing()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        let initial_energy = battle.get_player().get_energy();

        // Play Fire Breathing
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());

        // Verify energy was consumed
        assert_eq!(battle.get_player().get_energy(), initial_energy - 1);
    }

    #[test]
    fn test_fire_breathing_listener_triggers_on_status_draw() {
        let mut listener = FireBreathingListener::new(Entity::Player, 6);

        // Simulate drawing a status card
        let event = BattleEvent::CardDrawn {
            card_type: CardType::Status,
            is_status_or_curse: true,
        };

        let effects = listener.on_event(&event);
        assert_eq!(effects.len(), 1);

        match &effects[0] {
            Effect::AttackAllEnemies { amount, num_attacks } => {
                assert_eq!(*amount, 6);
                assert_eq!(*num_attacks, 1);
            }
            _ => panic!("Expected AttackAllEnemies effect"),
        }
    }

    #[test]
    fn test_fire_breathing_upgraded_listener_higher_damage() {
        let mut listener = FireBreathingListener::new(Entity::Player, 10);

        // Simulate drawing a status card
        let event = BattleEvent::CardDrawn {
            card_type: CardType::Status,
            is_status_or_curse: true,
        };

        let effects = listener.on_event(&event);
        assert_eq!(effects.len(), 1);

        match &effects[0] {
            Effect::AttackAllEnemies { amount, num_attacks } => {
                assert_eq!(*amount, 10);
                assert_eq!(*num_attacks, 1);
            }
            _ => panic!("Expected AttackAllEnemies effect"),
        }
    }

    #[test]
    fn test_fire_breathing_listener_ignores_normal_cards() {
        let mut listener = FireBreathingListener::new(Entity::Player, 6);

        // Simulate drawing a normal Attack card
        let event = BattleEvent::CardDrawn {
            card_type: CardType::Attack,
            is_status_or_curse: false,
        };

        let effects = listener.on_event(&event);
        assert_eq!(effects.len(), 0);
    }

    #[test]
    fn test_fire_breathing_listener_is_always_active() {
        let listener = FireBreathingListener::new(Entity::Player, 6);
        assert!(listener.is_active());
    }
}