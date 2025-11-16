use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect};

pub fn inflame() -> Card {
    Card::new(CardEnum::Inflame, 1, CardType::Power, vec![
        Effect::GainStrength(2),        // Gain 2 Strength permanently for this combat
    ], false, true)
}

pub fn inflame_upgraded() -> Card {
    Card::new(CardEnum::Inflame, 1, CardType::Power, vec![
        Effect::GainStrength(3),        // Gain 3 Strength permanently for this combat (+1)
    ], true, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn test_inflame_creation() {
        let card = inflame();

        assert_eq!(card.get_name(), "Inflame");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Power);
        assert_eq!(card.get_effects().len(), 1);
        assert_eq!(card.get_effects()[0], Effect::GainStrength(2));
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_inflame_upgraded_creation() {
        let card = inflame_upgraded();

        assert_eq!(card.get_name(), "Inflame+");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Power);
        assert_eq!(card.get_effects().len(), 1);
        assert_eq!(card.get_effects()[0], Effect::GainStrength(3));
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_inflame_gives_strength() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::red_louse::RedLouse;
        use crate::enemies::enemy_enum::EnemyEnum;
        use crate::cards::ironclad::strike::strike;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create battle with Inflame in hand
        let deck = Deck::new(vec![inflame(), strike()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        // Check initial strength
        let initial_strength = battle.get_player().battle_info.get_strength();
        assert_eq!(initial_strength, 0);

        // Play Inflame (assuming it's the first card in hand)
        let inflame_idx = 0;
        let result = battle.play_card(inflame_idx, Entity::Player);
        assert!(result.is_ok());

        // Check strength increased
        let final_strength = battle.get_player().battle_info.get_strength();
        assert_eq!(final_strength, 2);

        // Verify Inflame was added to powers collection
        let powers = battle.get_powers();
        assert_eq!(powers.len(), 1);
        assert_eq!(powers[0].get_name(), "Inflame");

        // Verify Inflame did NOT go to discard pile (power cards stay in play)
        let discard = battle.cards.get_discard_pile();
        assert!(!discard.iter().any(|card| card.get_name() == "Inflame"));
    }
}