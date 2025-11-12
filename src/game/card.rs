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
    
    /// Upgrades this card to its improved version
    /// Returns a new Card instance with upgraded properties
    pub fn upgrade(self) -> Card {
        match self.name.as_str() {
            "Strike" => {
                Card::new("Strike+".to_string(), self.cost, self.card_type, vec![
                    Effect::AttackToTarget { amount: 9, num_attacks: 1 } // +3 damage
                ])
            }
            "Defend" => {
                Card::new("Defend+".to_string(), self.cost, self.card_type, vec![
                    Effect::GainDefense(8) // +3 block
                ])
            }
            "Bash" => {
                Card::new("Bash+".to_string(), self.cost, self.card_type, vec![
                    Effect::AttackToTarget { amount: 10, num_attacks: 1 }, // +2 damage
                    Effect::ApplyVulnerable(3) // +1 vulnerable duration
                ])
            }
            _ => {
                // For unknown cards, just add "+" to the name
                // In a real implementation, you might want to handle this differently
                Card::new(format!("{}+", self.name), self.cost, self.card_type, self.effects)
            }
        }
    }
    
    /// Checks if this card is already upgraded
    pub fn is_upgraded(&self) -> bool {
        self.name.ends_with('+')
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

    #[test]
    fn test_strike_upgrade() {
        let strike = Card::new("Strike".to_string(), 1, CardType::Attack, vec![Effect::AttackToTarget { amount: 6, num_attacks: 1 }]);
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
        let basic = Card::new("Strike".to_string(), 1, CardType::Attack, vec![Effect::AttackToTarget { amount: 6, num_attacks: 1 }]);
        let upgraded = Card::new("Strike+".to_string(), 1, CardType::Attack, vec![Effect::AttackToTarget { amount: 9, num_attacks: 1 }]);
        
        assert!(!basic.is_upgraded());
        assert!(upgraded.is_upgraded());
    }
}