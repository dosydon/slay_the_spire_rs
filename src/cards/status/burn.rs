use crate::game::{card::Card, effect::Effect, card_type::CardType, card_enum::CardEnum};
use crate::battle::{events::{BattleEvent, EventListener}, target::Entity};

/// Burn Listener
/// Deals 2 damage to the player at the end of turn
#[derive(Debug)]
pub struct BurnListener {
    owner: Entity,
    damage: u32,
    is_active: bool,
}

impl BurnListener {
    pub fn new(owner: Entity, damage: u32) -> Self {
        BurnListener {
            owner,
            damage,
            is_active: true,
        }
    }
}

impl EventListener for BurnListener {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<Effect> {
        match event {
            BattleEvent::EndOfTurn { entity } if *entity == self.owner && self.is_active => {
                // At end of turn, deal damage to owner
                vec![Effect::LoseHp(self.damage)]
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
}

pub fn burn() -> Card {
    Card::new(CardEnum::Burn, 1, CardType::Status, vec![
        Effect::ActivateBurn { damage: 2 }, // Activate Burn listener for end-of-turn damage
    ], false, true) // unplayable
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_burn_creation() {
        let card = burn();

        assert_eq!(card.get_name(), "Burn");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Status);
        assert_eq!(card.get_effects().len(), 1);
        assert!(matches!(card.get_effects()[0], Effect::LoseHp(2)));
        assert!(!card.is_upgraded());
        assert!(!card.is_playable()); // Status cards are not playable
    }

    #[test]
    fn test_burn_effect() {
        let card = burn();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0], Effect::LoseHp(2));
    }

    #[test]
    fn test_burn_upgraded_name() {
        let card = burn();
        assert_eq!(card.get_card_enum().upgraded_name(), "Burn+");
    }
}