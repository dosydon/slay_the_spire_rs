use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect, card::Rarity};

pub fn pommel_strike() -> Card {
    Card::new(CardEnum::PommelStrike, 1, CardType::Attack, vec![
        Effect::AttackToTarget { amount: 9, num_attacks: 1, strength_multiplier: 1 },
        Effect::DrawCard { count: 1 }
    ], false, true, Rarity::Common)
}

pub fn pommel_strike_upgraded() -> Card {
    Card::new(CardEnum::PommelStrike, 1, CardType::Attack, vec![
        Effect::AttackToTarget { amount: 10, num_attacks: 1, strength_multiplier: 1 },
        Effect::DrawCard { count: 2 }
    ], true, true, Rarity::Common)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::card_type::CardType;

    #[test]
    fn test_pommel_strike_basic() {
        let card = pommel_strike();
        assert_eq!(card.get_name(), "PommelStrike");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(*card.get_card_type(), CardType::Attack);
        assert!(!card.is_upgraded());
        assert_eq!(card.get_effects().len(), 2);
    }

    #[test]
    fn test_pommel_strike_effects() {
        let card = pommel_strike();
        let effects = card.get_effects();

        // Should have 2 effects: AttackToTarget { amount: 9, num_attacks: 1 } and DrawCard(1)
        assert_eq!(effects.len(), 2);
        assert!(effects.contains(&Effect::AttackToTarget { amount: 9, num_attacks: 1, strength_multiplier: 1 }));
        assert!(effects.contains(&Effect::DrawCard { count: 1 }));
    }

    #[test]
    fn test_pommel_strike_upgraded() {
        let card = pommel_strike_upgraded();
        assert_eq!(card.get_name(), "PommelStrike+");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(*card.get_card_type(), CardType::Attack);
        assert!(card.is_upgraded());
        assert_eq!(card.get_effects().len(), 2);
    }

    #[test]
    fn test_pommel_strike_upgraded_effects() {
        let card = pommel_strike_upgraded();
        let effects = card.get_effects();

        // Should have 2 effects: AttackToTarget { amount: 10, num_attacks: 1 } and DrawCard(2)
        assert_eq!(effects.len(), 2);
        assert!(effects.contains(&Effect::AttackToTarget { amount: 10, num_attacks: 1, strength_multiplier: 1 }));
        assert!(effects.contains(&Effect::DrawCard { count: 2 }));
    }

    #[test]
    fn test_pommel_strike_upgrade_name() {
        let card = pommel_strike();
        assert_eq!(CardEnum::PommelStrike.upgraded_name(), "PommelStrike+");
    }

    #[test]
    fn test_pommel_strike_enum_consistency() {
        let card = pommel_strike();
        assert_eq!(card.get_card_enum(), CardEnum::PommelStrike);
        assert_eq!(CardEnum::PommelStrike.name(), "PommelStrike");
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
    fn test_pommel_strike_battle_integration() {
        let mut deck_cards = vec![pommel_strike()];
        for _ in 0..4 {
            deck_cards.push(crate::cards::ironclad::strike::strike());
        }
        let deck = Deck::new(deck_cards);

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        // Create battle with one enemy
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);

        battle.at_start_of_player_turn(&mut rng);

        let initial_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();
        let initial_hand_size = battle.get_hand().len();

        // Find and play PommelStrike
        let hand = battle.get_hand();
        let pommel_strike_idx = hand.iter().position(|card| card.get_name() == "PommelStrike");
        assert!(pommel_strike_idx.is_some(), "PommelStrike card should be in hand");

        // Play PommelStrike targeting the enemy using eval_action
        let action = Action::PlayCard(pommel_strike_idx.unwrap(), Entity::Enemy(0));
        let result = battle.eval_action(action, &mut rng);
        assert!(result.is_ok());

        // Check effects: enemy should take 9 damage, player should draw 1 card
        assert_eq!(battle.get_enemies()[0].battle_info.get_hp(), initial_enemy_hp - 9);
        assert_eq!(battle.get_hand().len(), initial_hand_size); // Played 1 card, drew 1 card, so same size
    }

    #[test]
    fn test_pommel_strike_upgraded_battle_integration() {
        let mut deck_cards = vec![pommel_strike_upgraded()];
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

        battle.at_start_of_player_turn(&mut rng);

        let initial_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();
        let initial_hand_size = battle.get_hand().len();

        // Find and play upgraded PommelStrike
        let hand = battle.get_hand();
        let pommel_strike_idx = hand.iter().position(|card| card.get_name() == "PommelStrike+");
        assert!(pommel_strike_idx.is_some(), "PommelStrike+ card should be in hand");

        // Play upgraded PommelStrike targeting the enemy using eval_action
        let action = Action::PlayCard(pommel_strike_idx.unwrap(), Entity::Enemy(0));
        let result = battle.eval_action(action, &mut rng);
        assert!(result.is_ok());

        // Check effects: enemy should take 10 damage
        assert_eq!(battle.get_enemies()[0].battle_info.get_hp(), initial_enemy_hp - 10);

        // Check that we drew at least 1 card (net gain after playing 1 card)
        // The exact hand size depends on deck size, so we just check that it's not smaller
        assert!(battle.get_hand().len() >= initial_hand_size);
    }
}