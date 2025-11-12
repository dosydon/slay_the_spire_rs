use crate::game::card_type::CardType;
use crate::game::card_enum::CardEnum;
use crate::game::effect::Effect;

#[derive(Debug, Clone, PartialEq)]
pub struct Card {
    card_enum: CardEnum,
    cost: u32,
    card_type: CardType,
    effects: Vec<Effect>,
    upgraded: bool,
}

impl Card {
    pub fn new(card_enum: CardEnum, cost: u32, card_type: CardType, effects: Vec<Effect>, upgraded: bool) -> Self {
        Card {
            card_enum,
            cost,
            card_type,
            effects,
            upgraded,
        }
    }

    pub fn get_name(&self) -> String {
        if self.upgraded {
            self.card_enum.upgraded_name()
        } else {
            self.card_enum.name().to_string()
        }
    }
    
    pub fn get_card_enum(&self) -> CardEnum {
        self.card_enum
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
    
    /// Upgrades this card to its improved version
    /// Returns a new Card instance with upgraded properties
    pub fn upgrade(self) -> Card {
        if self.upgraded {
            return self; // Already upgraded
        }
        
        // Delegate to individual card upgrade functions
        match self.card_enum {
            CardEnum::Strike => crate::cards::ironclad::strike::strike_upgraded(),
            CardEnum::Defend => crate::cards::ironclad::defend::defend_upgraded(),
            CardEnum::Bash => crate::cards::ironclad::bash::bash_upgraded(),
        }
    }
    
    /// Checks if this card is already upgraded
    pub fn is_upgraded(&self) -> bool {
        self.upgraded
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_creation() {
        let card = Card::new(CardEnum::Strike, 1, CardType::Attack, vec![Effect::AttackToTarget { amount: 6, num_attacks: 1 }], false);
        assert_eq!(card.get_name(), "Strike");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(matches!(card.get_card_type(), CardType::Attack), true);
    }

    #[test]
    fn test_strike_upgrade() {
        let strike = Card::new(CardEnum::Strike, 1, CardType::Attack, vec![Effect::AttackToTarget { amount: 6, num_attacks: 1 }], false);
        let upgraded = strike.upgrade();
        
        assert_eq!(upgraded.get_name(), "Strike+");
        assert_eq!(upgraded.get_cost(), 1); // Cost stays same
        assert_eq!(upgraded.get_card_type(), &CardType::Attack);
        assert!(upgraded.is_upgraded());
        
        // Check damage increased to 9
        match &upgraded.get_effects()[0] {
            Effect::AttackToTarget { amount, num_attacks } => {
                assert_eq!(*amount, 9);
                assert_eq!(*num_attacks, 1);
            }
            _ => panic!("Expected AttackToTarget effect"),
        }
    }

    #[test]
    fn test_is_upgraded() {
        let basic = Card::new(CardEnum::Strike, 1, CardType::Attack, vec![Effect::AttackToTarget { amount: 6, num_attacks: 1 }], false);
        let upgraded = Card::new(CardEnum::Strike, 1, CardType::Attack, vec![Effect::AttackToTarget { amount: 9, num_attacks: 1 }], true);
        
        assert!(!basic.is_upgraded());
        assert!(upgraded.is_upgraded());
    }
}