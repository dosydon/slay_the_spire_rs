use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::BattleEffect, card::{Rarity, CardClass}};

pub fn strike() -> Card {
    Card::new(CardEnum::Strike, 1, CardClass::IronClad(Rarity::Basic, CardType::Attack), vec![
        BattleEffect::AttackToTarget { amount: 6, num_attacks: 1, strength_multiplier: 1 }
    ])
}

pub fn strike_upgraded() -> Card {
    Card::new(CardEnum::Strike, 1, CardClass::IronClad(Rarity::Basic, CardType::Attack), vec![
        BattleEffect::AttackToTarget { amount: 9, num_attacks: 1, strength_multiplier: 1 }
    ])
        .set_upgraded(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strike_basic() {
        let card = strike();
        assert_eq!(card.get_name(), "Strike");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), CardType::Attack);
        
        let effects = card.get_effects();
        assert_eq!(effects.len(), 1);
        match &effects[0] {
            BattleEffect::AttackToTarget { amount, num_attacks, strength_multiplier } => {
                assert_eq!(*amount, 6);
                assert_eq!(*num_attacks, 1);
                assert_eq!(*strength_multiplier, 1);
            }
            _ => panic!("Expected AttackToTarget effect"),
        }
    }

    #[test]
    fn test_strike_upgraded() {
        let card = strike_upgraded();
        assert_eq!(card.get_name(), "Strike+");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), CardType::Attack);
        
        let effects = card.get_effects();
        assert_eq!(effects.len(), 1);
        match &effects[0] {
            BattleEffect::AttackToTarget { amount, num_attacks, strength_multiplier } => {
                assert_eq!(*amount, 9); // +3 damage over regular Strike
                assert_eq!(*num_attacks, 1);
                assert_eq!(*strength_multiplier, 1);
            }
            _ => panic!("Expected AttackToTarget effect"),
        }
    }
}