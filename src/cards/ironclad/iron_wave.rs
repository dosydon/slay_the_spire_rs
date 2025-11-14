use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect};

pub fn iron_wave() -> Card {
    Card::new(CardEnum::IronWave, 1, CardType::Attack, vec![
        Effect::GainDefense(5),
        Effect::AttackToTarget { amount: 5, num_attacks: 1 }
    ], false)
}

pub fn iron_wave_upgraded() -> Card {
    Card::new(CardEnum::IronWave, 1, CardType::Attack, vec![
        Effect::GainDefense(8),
        Effect::AttackToTarget { amount: 8, num_attacks: 1 }
    ], true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::{card::Card, card_type::CardType};

    #[test]
    fn test_iron_wave_basic() {
        let card = iron_wave();
        assert_eq!(card.get_name(), "IronWave");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(*card.get_card_type(), CardType::Attack);
        assert!(!card.is_upgraded());
        assert_eq!(card.get_effects().len(), 2);
    }

    #[test]
    fn test_iron_wave_effects() {
        let card = iron_wave();
        let effects = card.get_effects();

        // Should have 2 effects: GainDefense(5) and AttackToTarget { amount: 5, num_attacks: 1 }
        assert_eq!(effects.len(), 2);
        assert!(effects.contains(&Effect::GainDefense(5)));
        assert!(effects.contains(&Effect::AttackToTarget { amount: 5, num_attacks: 1 }));
    }

    #[test]
    fn test_iron_wave_upgraded() {
        let card = iron_wave_upgraded();
        assert_eq!(card.get_name(), "IronWave+");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(*card.get_card_type(), CardType::Attack);
        assert!(card.is_upgraded());
        assert_eq!(card.get_effects().len(), 2);
    }

    #[test]
    fn test_iron_wave_upgraded_effects() {
        let card = iron_wave_upgraded();
        let effects = card.get_effects();

        // Should have 2 effects: GainDefense(8) and AttackToTarget { amount: 8, num_attacks: 1 }
        assert_eq!(effects.len(), 2);
        assert!(effects.contains(&Effect::GainDefense(8)));
        assert!(effects.contains(&Effect::AttackToTarget { amount: 8, num_attacks: 1 }));
    }

    #[test]
    fn test_iron_wave_upgrade_name() {
        let card = iron_wave();
        assert_eq!(CardEnum::IronWave.upgraded_name(), "IronWave+");
    }

    #[test]
    fn test_iron_wave_enum_consistency() {
        let card = iron_wave();
        assert_eq!(card.get_card_enum(), CardEnum::IronWave);
        assert_eq!(CardEnum::IronWave.name(), "IronWave");
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
    fn test_iron_wave_battle_integration() {
        let mut deck_cards = vec![iron_wave()];
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

        battle.start_of_player_turn(&mut rng);

        let initial_player_block = battle.get_player().battle_info.get_block();
        let initial_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();

        // Find and play IronWave
        let hand = battle.get_hand();
        let iron_wave_idx = hand.iter().position(|card| card.get_name() == "IronWave");
        assert!(iron_wave_idx.is_some(), "IronWave card should be in hand");

        // Play IronWave targeting the enemy using eval_action
        let action = Action::PlayCard(iron_wave_idx.unwrap(), Entity::Enemy(0));
        let result = battle.eval_action(action, &mut rng);
        assert!(result.is_ok());

        // Check effects: player should gain 5 block, enemy should take 5 damage
        assert_eq!(battle.get_player().battle_info.get_block(), initial_player_block + 5);
        assert_eq!(battle.get_enemies()[0].battle_info.get_hp(), initial_enemy_hp - 5);
    }

    #[test]
    fn test_iron_wave_upgraded_battle_integration() {
        let mut deck_cards = vec![iron_wave_upgraded()];
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

        battle.start_of_player_turn(&mut rng);

        let initial_player_block = battle.get_player().battle_info.get_block();
        let initial_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();

        // Find and play upgraded IronWave
        let hand = battle.get_hand();
        let iron_wave_idx = hand.iter().position(|card| card.get_name() == "IronWave+");
        assert!(iron_wave_idx.is_some(), "IronWave+ card should be in hand");

        // Play upgraded IronWave targeting the enemy using eval_action
        let action = Action::PlayCard(iron_wave_idx.unwrap(), Entity::Enemy(0));
        let result = battle.eval_action(action, &mut rng);
        assert!(result.is_ok());

        // Check effects: player should gain 8 block, enemy should take 8 damage
        assert_eq!(battle.get_player().battle_info.get_block(), initial_player_block + 8);
        assert_eq!(battle.get_enemies()[0].battle_info.get_hp(), initial_enemy_hp - 8);
    }
}