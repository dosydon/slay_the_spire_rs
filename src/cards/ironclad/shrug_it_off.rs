use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::{Effect, Condition}};

pub fn shrug_it_off() -> Card {
    Card::new(CardEnum::ShrugItOff, 1, CardType::Skill, vec![
        Effect::GainDefense { amount: 8 },
        Effect::DrawCard { count: 1 }
    ], false, true)
}

pub fn shrug_it_off_upgraded() -> Card {
    Card::new(CardEnum::ShrugItOff, 1, CardType::Skill, vec![
        Effect::GainDefense { amount: 11 },
        Effect::DrawCard { count: 1 }
    ], true, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::card_type::CardType;

    #[test]
    fn test_shrug_it_off_basic() {
        let card = shrug_it_off();
        assert_eq!(card.get_name(), "ShrugItOff");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(*card.get_card_type(), CardType::Skill);
        assert!(!card.is_upgraded());
        assert_eq!(card.get_effects().len(), 2);
    }

    #[test]
    fn test_shrug_it_off_effects() {
        let card = shrug_it_off();
        let effects = card.get_effects();

        // Should have 2 effects: GainDefense(8) and DrawCard(1)
        assert_eq!(effects.len(), 2);
        assert!(effects.contains(&Effect::GainDefense { amount: 8 }));
        assert!(effects.contains(&Effect::DrawCard { count: 1 }));
    }

    #[test]
    fn test_shrug_it_off_upgraded() {
        let card = shrug_it_off_upgraded();
        assert_eq!(card.get_name(), "ShrugItOff+");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(*card.get_card_type(), CardType::Skill);
        assert!(card.is_upgraded());
        assert_eq!(card.get_effects().len(), 2);
    }

    #[test]
    fn test_shrug_it_off_upgraded_effects() {
        let card = shrug_it_off_upgraded();
        let effects = card.get_effects();

        // Should have 2 effects: GainDefense(11) and DrawCard(1)
        assert_eq!(effects.len(), 2);
        assert!(effects.contains(&Effect::GainDefense { amount: 11 }));
        assert!(effects.contains(&Effect::DrawCard { count: 1 }));
    }

    #[test]
    fn test_shrug_it_off_upgrade_name() {
        let card = shrug_it_off();
        assert_eq!(CardEnum::ShrugItOff.upgraded_name(), "ShrugItOff+");
    }

    #[test]
    fn test_shrug_it_off_enum_consistency() {
        let card = shrug_it_off();
        assert_eq!(card.get_card_enum(), CardEnum::ShrugItOff);
        assert_eq!(CardEnum::ShrugItOff.name(), "ShrugItOff");
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::battle::Battle;
    use crate::battle::action::Action;
    use crate::enemies::{red_louse::RedLouse, enemy_enum::EnemyEnum};
    use crate::game::{global_info::GlobalInfo, deck::Deck, enemy::EnemyTrait};
    use crate::battle::enemy_in_battle::EnemyInBattle;
    use crate::battle::target::Entity;

    #[test]
    fn test_shrug_it_off_battle_integration() {
        let mut deck_cards = vec![shrug_it_off()];
        for _ in 0..4 {
            deck_cards.push(crate::cards::ironclad::strike::strike());
        }
        // Add more cards to ensure there are enough to draw
        for _ in 0..5 {
            deck_cards.push(crate::cards::ironclad::defend::defend());
        }
        let deck = Deck::new(deck_cards);

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        // Create battle with one enemy
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);

        battle.start_of_player_turn(&mut rng);

        let initial_player_block = battle.get_player().battle_info.get_block();
        let initial_hand_size = battle.get_hand().len();

        // Find and play ShrugItOff
        let hand = battle.get_hand();
        let shrug_it_off_idx = hand.iter().position(|card| card.get_name() == "ShrugItOff");
        assert!(shrug_it_off_idx.is_some(), "ShrugItOff card should be in hand");

        // Play ShrugItOff (no target needed for Skill cards)
        let action = Action::PlayCard(shrug_it_off_idx.unwrap(), Entity::None);
        let result = battle.eval_action(action, &mut rng);
        assert!(result.is_ok());

        // Check effects: player should gain 8 block and draw 1 card
        assert_eq!(battle.get_player().battle_info.get_block(), initial_player_block + 8);
        // Hand size should be the same (played 1, drew 1)
        assert_eq!(battle.get_hand().len(), initial_hand_size);
    }

    #[test]
    fn test_shrug_it_off_upgraded_battle_integration() {
        let mut deck_cards = vec![shrug_it_off_upgraded()];
        for _ in 0..4 {
            deck_cards.push(crate::cards::ironclad::strike::strike());
        }
        // Add more cards to ensure there are enough to draw
        for _ in 0..5 {
            deck_cards.push(crate::cards::ironclad::defend::defend());
        }
        let deck = Deck::new(deck_cards);

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        // Create battle with one enemy
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);

        battle.start_of_player_turn(&mut rng);

        let initial_player_block = battle.get_player().battle_info.get_block();
        let initial_hand_size = battle.get_hand().len();

        // Find and play upgraded ShrugItOff
        let hand = battle.get_hand();
        let shrug_it_off_idx = hand.iter().position(|card| card.get_name() == "ShrugItOff+");
        assert!(shrug_it_off_idx.is_some(), "ShrugItOff+ card should be in hand");

        // Play upgraded ShrugItOff (no target needed for Skill cards)
        let action = Action::PlayCard(shrug_it_off_idx.unwrap(), Entity::None);
        let result = battle.eval_action(action, &mut rng);
        assert!(result.is_ok());

        // Check effects: player should gain 11 block and draw 1 card
        assert_eq!(battle.get_player().battle_info.get_block(), initial_player_block + 11);
        // Hand size should be the same (played 1, drew 1)
        assert_eq!(battle.get_hand().len(), initial_hand_size);
    }
}