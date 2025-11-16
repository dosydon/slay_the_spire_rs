use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect};

pub fn intimidate() -> Card {
    Card::new(CardEnum::Intimidate, 0, CardType::Skill, vec![
        Effect::ApplyWeakAll(1),
        Effect::Exhaust
    ], false, true)
}

pub fn intimidate_upgraded() -> Card {
    Card::new(CardEnum::Intimidate, 0, CardType::Skill, vec![
        Effect::ApplyWeakAll(2),
        Effect::Exhaust
    ], true, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intimidate_basic() {
        let card = intimidate();
        assert_eq!(card.get_name(), "Intimidate");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert!(!card.is_upgraded());

        let effects = card.get_effects();
        assert_eq!(effects.len(), 2);
        assert!(effects.contains(&Effect::ApplyWeakAll(1)));
        assert!(effects.contains(&Effect::Exhaust));
    }

    #[test]
    fn test_intimidate_upgraded() {
        let card = intimidate_upgraded();
        assert_eq!(card.get_name(), "Intimidate+");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert!(card.is_upgraded());

        let effects = card.get_effects();
        assert_eq!(effects.len(), 2);
        assert!(effects.contains(&Effect::ApplyWeakAll(2)));
        assert!(effects.contains(&Effect::Exhaust));
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::battle::{Battle, enemy_in_battle::EnemyInBattle, action::Action, target::Entity};
    use crate::enemies::{red_louse::RedLouse, enemy_enum::EnemyEnum};
    use crate::game::{global_info::GlobalInfo, deck::Deck, enemy::EnemyTrait};

    #[test]
    fn test_intimidate_applies_weak_to_all_enemies() {
        let mut deck_cards = vec![intimidate()];
        for _ in 0..4 {
            deck_cards.push(crate::cards::ironclad::strike::strike());
        }
        let deck = Deck::new(deck_cards);

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        // Create battle with three enemies
        let red_louse1 = RedLouse::instantiate(&mut rng, &global_info);
        let red_louse2 = RedLouse::instantiate(&mut rng, &global_info);
        let red_louse3 = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse1)),
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse2)),
            EnemyInBattle::new(EnemyEnum::RedLouse(red_louse3))
        ];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);

        battle.start_of_player_turn(&mut rng);

        // Verify no enemies are weak initially
        assert!(!battle.get_enemies()[0].battle_info.is_weak());
        assert!(!battle.get_enemies()[1].battle_info.is_weak());
        assert!(!battle.get_enemies()[2].battle_info.is_weak());

        // Find and play Intimidate
        let hand = battle.get_hand();
        let intimidate_idx = hand.iter().position(|card| card.get_name() == "Intimidate");
        assert!(intimidate_idx.is_some(), "Intimidate card should be in hand");

        let action = Action::PlayCard(intimidate_idx.unwrap(), Entity::None);
        let result = battle.eval_action(action, &mut rng);
        assert!(result.is_ok(), "Playing Intimidate should succeed");

        // Verify all enemies are weak
        assert!(battle.get_enemies()[0].battle_info.is_weak());
        assert!(battle.get_enemies()[1].battle_info.is_weak());
        assert!(battle.get_enemies()[2].battle_info.is_weak());
    }

    #[test]
    fn test_intimidate_upgraded_applies_more_weak() {
        let mut deck_cards = vec![intimidate_upgraded()];
        for _ in 0..4 {
            deck_cards.push(crate::cards::ironclad::strike::strike());
        }
        let deck = Deck::new(deck_cards);

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);

        battle.start_of_player_turn(&mut rng);

        // Play upgraded Intimidate
        let hand = battle.get_hand();
        let intimidate_idx = hand.iter().position(|card| card.get_name() == "Intimidate+");
        assert!(intimidate_idx.is_some());

        let action = Action::PlayCard(intimidate_idx.unwrap(), Entity::None);
        let result = battle.eval_action(action, &mut rng);
        assert!(result.is_ok());

        // Verify enemy has weak (upgraded applies 2 stacks)
        assert!(battle.get_enemies()[0].battle_info.is_weak());
    }
}
