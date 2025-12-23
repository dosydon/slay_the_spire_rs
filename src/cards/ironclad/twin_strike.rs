use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::BattleEffect, card::{Rarity, CardClass}};

pub fn twin_strike() -> Card {
    Card::new(CardEnum::TwinStrike, 1, CardClass::IronClad(Rarity::Common, CardType::Attack), vec![
        BattleEffect::AttackToTarget { amount: 5, num_attacks: 2, strength_multiplier: 1 }
    ])
}

pub fn twin_strike_upgraded() -> Card {
    Card::new(CardEnum::TwinStrike, 1, CardClass::IronClad(Rarity::Common, CardType::Attack), vec![
        BattleEffect::AttackToTarget { amount: 7, num_attacks: 2, strength_multiplier: 1 }
    ])
        .set_upgraded(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::card_type::CardType;

    #[test]
    fn test_twin_strike_basic() {
        let card = twin_strike();
        assert_eq!(card.get_name(), "TwinStrike");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), CardType::Attack);
        assert!(!card.is_upgraded());
        assert_eq!(card.get_effects().len(), 1);
    }

    #[test]
    fn test_twin_strike_effects() {
        let card = twin_strike();
        let effects = card.get_effects();

        // Should have 1 effect: AttackToTarget { amount: 5, num_attacks: 2 }
        assert_eq!(effects.len(), 1);
        assert!(effects.contains(&BattleEffect::AttackToTarget { amount: 5, num_attacks: 2, strength_multiplier: 1 }));
    }

    #[test]
    fn test_twin_strike_upgraded() {
        let card = twin_strike_upgraded();
        assert_eq!(card.get_name(), "TwinStrike+");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), CardType::Attack);
        assert!(card.is_upgraded());
        assert_eq!(card.get_effects().len(), 1);
    }

    #[test]
    fn test_twin_strike_upgraded_effects() {
        let card = twin_strike_upgraded();
        let effects = card.get_effects();

        // Should have 1 effect: AttackToTarget { amount: 7, num_attacks: 2 }
        assert_eq!(effects.len(), 1);
        assert!(effects.contains(&BattleEffect::AttackToTarget { amount: 7, num_attacks: 2, strength_multiplier: 1 }));
    }

    #[test]
    fn test_twin_strike_enum_consistency() {
        let card = twin_strike();
        assert_eq!(card.get_card_enum(), CardEnum::TwinStrike);
        assert_eq!(CardEnum::TwinStrike.name(), "TwinStrike");
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::battle::Battle;
    use crate::battle::battle_action::BattleAction;
    use crate::enemies::{acid_slime_m::AcidSlimeM, enemy_enum::EnemyEnum};
    use crate::game::PlayerRunState;
    use crate::game::{global_info::GlobalInfo, deck::Deck};
    use crate::battle::enemy_in_battle::EnemyInBattle;
    use crate::battle::target::Entity;

    #[test]
    fn test_twin_strike_battle_integration() {
        let mut deck_cards = vec![twin_strike()];
        for _ in 0..4 {
            deck_cards.push(crate::cards::ironclad::strike::strike());
        }
        let deck = Deck::new(deck_cards);

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        // Create battle with one enemy
        let acid_slime = AcidSlimeM::new(30);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::AcidSlimeM(acid_slime))];
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(80, 80, 0), enemies, &mut rng);

        battle.at_start_of_player_turn(&mut rng);

        let initial_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();

        // Find and play TwinStrike
        let hand = battle.get_hand();
        let twin_strike_idx = hand.iter().position(|card| card.get_name() == "TwinStrike");
        assert!(twin_strike_idx.is_some(), "TwinStrike card should be in hand");

        // Play TwinStrike targeting the enemy
        let action = BattleAction::PlayCard(twin_strike_idx.unwrap(), Entity::Enemy(0));
        let result = battle.eval_action(action, &mut rng);
        assert!(result.is_ok());

        // Check effects: enemy should take exactly 10 damage (5 + 5 from Twin Strike)
        let actual_damage_dealt = initial_enemy_hp.saturating_sub(battle.get_enemies()[0].battle_info.get_hp());
        assert_eq!(actual_damage_dealt, 10, "Twin Strike should deal exactly 10 damage (5+5)");
    }

    #[test]
    fn test_twin_strike_upgraded_battle_integration() {
        let mut deck_cards = vec![twin_strike_upgraded()];
        for _ in 0..4 {
            deck_cards.push(crate::cards::ironclad::strike::strike());
        }
        let deck = Deck::new(deck_cards);

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        // Create battle with one enemy
        let acid_slime = AcidSlimeM::new(30);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::AcidSlimeM(acid_slime))];
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(80, 80, 0), enemies, &mut rng);

        battle.at_start_of_player_turn(&mut rng);

        let initial_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();

        // Find and play upgraded TwinStrike
        let hand = battle.get_hand();
        let twin_strike_idx = hand.iter().position(|card| card.get_name() == "TwinStrike+");
        assert!(twin_strike_idx.is_some(), "TwinStrike+ card should be in hand");

        // Play upgraded TwinStrike targeting the enemy
        let action = BattleAction::PlayCard(twin_strike_idx.unwrap(), Entity::Enemy(0));
        let result = battle.eval_action(action, &mut rng);
        assert!(result.is_ok());

        // Check effects: enemy should take exactly 14 damage (7 + 7 from upgraded Twin Strike)
        let actual_damage_dealt = initial_enemy_hp.saturating_sub(battle.get_enemies()[0].battle_info.get_hp());
        assert_eq!(actual_damage_dealt, 14, "Upgraded Twin Strike should deal exactly 14 damage (7+7)");
    }

    #[test]
    fn test_twin_strike_with_strength() {
        let mut deck_cards = vec![twin_strike()];
        for _ in 0..4 {
            deck_cards.push(crate::cards::ironclad::strike::strike());
        }
        let deck = Deck::new(deck_cards);

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        // Create battle with one enemy
        let acid_slime = AcidSlimeM::new(30);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::AcidSlimeM(acid_slime))];
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(80, 80, 0), enemies, &mut rng);

        battle.at_start_of_player_turn(&mut rng);

        // Give player 3 strength
        battle.get_player_mut().battle_info.gain_strength(3);

        let initial_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();

        // Find and play TwinStrike
        let hand = battle.get_hand();
        let twin_strike_idx = hand.iter().position(|card| card.get_name() == "TwinStrike");
        assert!(twin_strike_idx.is_some(), "TwinStrike card should be in hand");

        // Play TwinStrike targeting the enemy
        let action = BattleAction::PlayCard(twin_strike_idx.unwrap(), Entity::Enemy(0));
        let result = battle.eval_action(action, &mut rng);
        assert!(result.is_ok());

        // Check effects: each attack should deal 8 damage (5 base + 3 strength)
        // Two attacks = 16 total damage
        let actual_damage_dealt = initial_enemy_hp.saturating_sub(battle.get_enemies()[0].battle_info.get_hp());
        assert_eq!(actual_damage_dealt, 16, "Twin Strike with 3 strength should deal exactly 16 damage (8+8)");
    }

    #[test]
    fn test_upgraded_twin_strike_with_strength() {
        let mut deck_cards = vec![twin_strike_upgraded()];
        for _ in 0..4 {
            deck_cards.push(crate::cards::ironclad::strike::strike());
        }
        let deck = Deck::new(deck_cards);

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        // Create battle with one enemy
        let acid_slime = AcidSlimeM::new(30);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::AcidSlimeM(acid_slime))];
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(80, 80, 0), enemies, &mut rng);

        battle.at_start_of_player_turn(&mut rng);

        // Give player 2 strength
        battle.get_player_mut().battle_info.gain_strength(2);

        let initial_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();

        // Find and play upgraded TwinStrike
        let hand = battle.get_hand();
        let twin_strike_idx = hand.iter().position(|card| card.get_name() == "TwinStrike+");
        assert!(twin_strike_idx.is_some(), "TwinStrike+ card should be in hand");

        // Play upgraded TwinStrike targeting the enemy
        let action = BattleAction::PlayCard(twin_strike_idx.unwrap(), Entity::Enemy(0));
        let result = battle.eval_action(action, &mut rng);
        assert!(result.is_ok());

        // Check effects: each attack should deal 9 damage (7 base + 2 strength)
        // Two attacks = 18 total damage
        let actual_damage_dealt = initial_enemy_hp.saturating_sub(battle.get_enemies()[0].battle_info.get_hp());
        assert_eq!(actual_damage_dealt, 18, "Upgraded Twin Strike with 2 strength should deal exactly 18 damage (9+9)");
    }
}