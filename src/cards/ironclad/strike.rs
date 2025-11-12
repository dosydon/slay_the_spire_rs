use crate::game::{card::Card, card_type::CardType, effect::Effect};

pub fn strike() -> Card {
    Card::new("Strike".to_string(), 1, CardType::Attack, vec![
        Effect::AttackToTarget { amount: 6, num_attacks: 1 }
    ])
}

pub fn strike_upgraded() -> Card {
    Card::new("Strike+".to_string(), 1, CardType::Attack, vec![
        Effect::AttackToTarget { amount: 9, num_attacks: 1 }
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strike_basic() {
        let card = strike();
        assert_eq!(card.get_name(), "Strike");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        
        let effects = card.get_effects();
        assert_eq!(effects.len(), 1);
        match &effects[0] {
            Effect::AttackToTarget { amount, num_attacks } => {
                assert_eq!(*amount, 6);
                assert_eq!(*num_attacks, 1);
            }
            _ => panic!("Expected AttackToTarget effect"),
        }
    }

    #[test]
    fn test_strike_upgraded() {
        let card = strike_upgraded();
        assert_eq!(card.get_name(), "Strike+");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        
        let effects = card.get_effects();
        assert_eq!(effects.len(), 1);
        match &effects[0] {
            Effect::AttackToTarget { amount, num_attacks } => {
                assert_eq!(*amount, 9); // +3 damage over regular Strike
                assert_eq!(*num_attacks, 1);
            }
            _ => panic!("Expected AttackToTarget effect"),
        }
    }
}