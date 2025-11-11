use crate::game::card_type::CardType;
use crate::game::effect::Effect;

#[derive(Debug, Clone, PartialEq)]
pub struct Card {
    name: String,
    cost: u32,
    card_type: CardType,
    effects: Vec<Effect>,
}

impl Card {
    pub fn new(name: String, cost: u32, card_type: CardType, effects: Vec<Effect>) -> Self {
        Card {
            name,
            cost,
            card_type,
            effects
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_cost(&self) -> u32 {
        self.cost
    }

    pub fn get_card_type(&self) -> &CardType {
        &self.card_type
    }

    pub fn get_effects(&self) -> &Vec<Effect> {
        &self.effects
    }

    pub fn cost(&self) -> u32 {
        self.cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_creation() {
        let card = Card::new("Strike".to_string(), 1, CardType::Attack, vec![Effect::AttackToTarget { amount: 6, num_attacks: 1 }]);
        assert_eq!(card.get_name(), "Strike");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(matches!(card.get_card_type(), CardType::Attack), true);
    }
}