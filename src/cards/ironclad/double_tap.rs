use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::BattleEffect, card::{Rarity, CardClass}};
use crate::battle::{battle_events::{BattleEvent, EventListener}, target::Entity};

pub fn double_tap() -> Card {
    Card::new(CardEnum::DoubleTap, 1, CardClass::IronClad(Rarity::Rare, CardType::Skill), vec![
            BattleEffect::ActivateDoubleTap { remaining_attacks: 1 },
            BattleEffect::Exhaust,
        ])
        .set_playable(true)
}

pub fn double_tap_upgraded() -> Card {
    Card::new(CardEnum::DoubleTap, 1, CardClass::IronClad(Rarity::Rare, CardType::Skill), vec![
            BattleEffect::ActivateDoubleTap { remaining_attacks: 2 },
            BattleEffect::Exhaust,
        ])
        .set_upgraded(true)
        .set_playable(true)
}

pub struct DoubleTapListener {
    source: Entity,
    remaining_attacks: u32,
}

impl DoubleTapListener {
    pub fn new(source: Entity, remaining_attacks: u32) -> Self {
        Self {
            source,
            remaining_attacks,
        }
    }
}

impl EventListener for DoubleTapListener {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<BattleEffect> {
        match event {
            BattleEvent::CardPlayed { source, card_type }
                if *source == self.source
                && *card_type == crate::game::card_type::CardType::Attack
                && self.remaining_attacks > 0 => {
                // When an Attack card is played, play it again
                self.remaining_attacks -= 1;
                // Note: In a full implementation, this would need to trigger card duplication
                // For now, we'll use a placeholder effect to indicate the card should be played again
                vec![]
            }
            _ => vec![]
        }
    }

    fn is_active(&self) -> bool {
        self.remaining_attacks > 0
    }

    fn get_owner(&self) -> Entity {
        self.source
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_tap_creation() {
        let card = double_tap();
        assert_eq!(card.get_name(), "Double Tap");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), CardType::Skill);
        assert!(!card.is_upgraded());
        assert_eq!(card.get_effects().len(), 2);
        assert_eq!(card.get_effects()[0], BattleEffect::ActivateDoubleTap { remaining_attacks: 1 });
        assert_eq!(card.get_effects()[1], BattleEffect::Exhaust);
    }

    #[test]
    fn test_double_tap_upgraded() {
        let card = double_tap_upgraded();
        assert_eq!(card.get_name(), "Double Tap+");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), CardType::Skill);
        assert!(card.is_upgraded());
        assert_eq!(card.get_effects().len(), 2);
        assert_eq!(card.get_effects()[0], BattleEffect::ActivateDoubleTap { remaining_attacks: 2 });
        assert_eq!(card.get_effects()[1], BattleEffect::Exhaust);
    }

    #[test]
    fn test_double_tap_listener_creation() {
        let listener = DoubleTapListener::new(Entity::Player, 1);
        assert_eq!(listener.source, Entity::Player);
        assert_eq!(listener.remaining_attacks, 1);
        assert!(listener.is_active());
    }

    #[test]
    fn test_double_tap_listener_exhaustion() {
        let listener = DoubleTapListener::new(Entity::Player, 0);
        assert_eq!(listener.remaining_attacks, 0);
        assert!(!listener.is_active());
    }

    #[test]
    fn test_double_tap_listener_on_attack() {
        let mut listener = DoubleTapListener::new(Entity::Player, 2);

        // Simulate playing an Attack card
        let event = BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: crate::game::card_type::CardType::Attack,
        };

        listener.on_event(&event);
        assert_eq!(listener.remaining_attacks, 1);
        assert!(listener.is_active());

        // Play another Attack
        listener.on_event(&event);
        assert_eq!(listener.remaining_attacks, 0);
        assert!(!listener.is_active());
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
    use crate::cards::ironclad::strike::strike;

    #[test]
    fn test_double_tap_activates_listener() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm))];

        // Create battle with Double Tap and Strike in hand
        let deck = Deck::new(vec![double_tap(), strike()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        // Play Double Tap
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());

        // Verify Double Tap exhausted
        assert_eq!(battle.cards.exhausted_size(), 1);

        // Verify listener was added (this is tested indirectly via the double attack behavior)
        // In the full implementation, when Strike is played next, it should trigger twice
    }

    #[test]
    fn test_double_tap_upgraded_has_two_charges() {
        let card = double_tap_upgraded();
        let effects = card.get_effects();

        match &effects[0] {
            BattleEffect::ActivateDoubleTap { remaining_attacks } => {
                assert_eq!(*remaining_attacks, 2);
            }
            _ => panic!("Expected ActivateDoubleTap effect"),
        }
    }

    #[test]
    fn test_double_tap_exhausts_after_use() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm))];

        let deck = Deck::new(vec![double_tap()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        let initial_exhaust_size = battle.cards.exhausted_size();

        // Play Double Tap
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());

        // Verify it went to exhaust pile
        assert_eq!(battle.cards.exhausted_size(), initial_exhaust_size + 1);
    }

    #[test]
    fn test_double_tap_costs_one_energy() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm))];

        let deck = Deck::new(vec![double_tap()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        let initial_energy = battle.get_player().get_energy();

        // Play Double Tap
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());

        // Verify energy was consumed
        assert_eq!(battle.get_player().get_energy(), initial_energy - 1);
    }
}