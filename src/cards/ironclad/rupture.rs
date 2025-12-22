use crate::game::{card::Card, effect::Effect, card_type::CardType, card_enum::CardEnum, card::Rarity};
use crate::battle::{battle_events::{BattleEvent, EventListener}, target::Entity};

/// Rupture Listener
/// Gains 1 Strength whenever player loses HP
#[derive(Debug)]
pub struct RuptureListener {
    owner: Entity,
    is_active: bool,
}

impl RuptureListener {
    pub fn new(owner: Entity) -> Self {
        RuptureListener {
            owner,
            is_active: true,
        }
    }
}

impl EventListener for RuptureListener {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<Effect> {
        match event {
            BattleEvent::HpLostFromCard { target, amount } if *target == self.owner && *amount > 0 && self.is_active => {
                // When player loses HP from cards, gain 1 Strength per HP lost
                vec![Effect::GainStrength { amount: *amount }]
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

/// Rupture - Uncommon Power Card
/// Cost: 1
/// Effect: Whenever you lose HP, gain 1 Strength.
pub fn rupture() -> Card {
    Card::new(
        CardEnum::Rupture,
        1,
        CardType::Power,
        vec![Effect::ActivateRupture],
        Rarity::Uncommon
    )
        .set_playable(true)
}

/// Rupture+ (Upgraded version)
/// Cost: 1
/// Effect: Whenever you lose HP, gain 1 Strength.
pub fn rupture_upgraded() -> Card {
    Card::new(
        CardEnum::Rupture,
        1,
        CardType::Power,
        vec![Effect::ActivateRupture],
        Rarity::Uncommon
    )
        .set_upgraded(true)
        .set_playable(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rupture_creation() {
        let card = rupture();
        assert_eq!(card.get_name(), "Rupture");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Power);
        assert_eq!(card.get_effects().len(), 1);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_rupture_upgraded_creation() {
        let card = rupture_upgraded();
        assert_eq!(card.get_name(), "Rupture+");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Power);
        assert_eq!(card.get_effects().len(), 1);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_rupture_effect() {
        let card = rupture();
        let effects = card.get_effects();
        assert_eq!(effects.len(), 1);
        assert!(matches!(effects[0], Effect::ActivateRupture));
    }

    #[test]
    fn test_rupture_upgraded_effect() {
        let card = rupture_upgraded();
        let effects = card.get_effects();
        assert_eq!(effects.len(), 1);
        assert!(matches!(effects[0], Effect::ActivateRupture));
    }

    #[test]
    fn test_rupture_listener_creation() {
        let listener = RuptureListener::new(Entity::Player);
        assert_eq!(listener.get_owner(), Entity::Player);
        assert!(listener.is_active());
    }

    #[test]
    fn test_rupture_listener_hp_lost() {
        let mut listener = RuptureListener::new(Entity::Player);

        // When player loses 5 HP from cards, should gain 5 Strength
        let hp_loss_event = BattleEvent::HpLostFromCard {
            target: Entity::Player,
            amount: 5,
        };

        let effects = listener.on_event(&hp_loss_event);
        assert_eq!(effects.len(), 1);
        assert!(matches!(effects[0], Effect::GainStrength { amount: 5 }));
    }

    #[test]
    fn test_rupture_listener_hp_lost_small_amount() {
        let mut listener = RuptureListener::new(Entity::Player);

        // When player loses 1 HP from cards, should gain 1 Strength
        let hp_loss_event = BattleEvent::HpLostFromCard {
            target: Entity::Player,
            amount: 1,
        };

        let effects = listener.on_event(&hp_loss_event);
        assert_eq!(effects.len(), 1);
        assert!(matches!(effects[0], Effect::GainStrength { amount: 1 }));
    }

    
    #[test]
    fn test_rupture_listener_enemy_damage() {
        let mut listener = RuptureListener::new(Entity::Player);

        // When enemy takes damage, should not trigger
        let enemy_damage_event = BattleEvent::DamageTaken {
            target: Entity::Enemy(0),
            amount: 5,
            source: Entity::Player,
        };

        let effects = listener.on_event(&enemy_damage_event);
        assert_eq!(effects.len(), 0);
    }

    #[test]
    fn test_rupture_listener_zero_damage() {
        let mut listener = RuptureListener::new(Entity::Player);

        // When player takes zero damage, should not trigger
        let zero_damage_event = BattleEvent::DamageTaken {
            target: Entity::Player,
            amount: 0,
            source: Entity::Enemy(0),
        };

        let effects = listener.on_event(&zero_damage_event);
        assert_eq!(effects.len(), 0);
    }

    #[test]
    fn test_rupture_battle_integration() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::gremlin_nob::GremlinNob;
        use crate::enemies::enemy_enum::EnemyEnum;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let gremlin_nob = GremlinNob::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::GremlinNob(gremlin_nob))];

        // Create battle with Rupture in hand
        let deck = Deck::new(vec![rupture()]);
        let player_state = crate::game::player_run_state::PlayerRunState::new(50, 80, 0);
let mut battle = Battle::new(deck, global_info, player_state, enemies, &mut rng);

        // Check initial player strength
        let initial_strength = battle.get_player().battle_info.get_strength();
        assert_eq!(initial_strength, 0);

        // Play Rupture
        let rupture_idx = 0;
        let result = battle.play_card(rupture_idx, Entity::Player);
        assert!(result.is_ok());

        // Rupture should go to powers pile (not discard)
        assert_eq!(battle.get_powers().len(), 1);
    }

    #[test]
    fn test_rupture_upgraded_name() {
        let card = rupture();
        assert_eq!(card.get_card_enum().upgraded_name(), "Rupture+");
    }
}