use crate::game::{card::Card, effect::{Effect, Condition}, card_type::CardType, card_enum::CardEnum};

/// Sentinel - Uncommon Skill Card
/// Cost: 1 (0 when upgraded)
/// Effect: Gain 5 Block. If you have no Block, gain 2 Energy
pub fn sentinel() -> Card {
    Card::new_with_condition(
        CardEnum::Sentinel,
        1,
        CardType::Skill,
        vec![
            Effect::GainDefense { amount: 5 },
            Effect::GainEnergyIfNoBlock { amount: 2 },
        ],
        false, // not upgraded
        Condition::True,
    )
}

/// Sentinel+ (Upgraded version)
/// Cost: 0
/// Effect: Gain 8 Block. If you have no Block, gain 3 Energy
pub fn sentinel_upgraded() -> Card {
    Card::new_with_condition(
        CardEnum::Sentinel,
        0,
        CardType::Skill,
        vec![
            Effect::GainDefense { amount: 8 },
            Effect::GainEnergyIfNoBlock { amount: 3 },
        ],
        true,  // upgraded
        Condition::True,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battle::{Battle, target::Entity};
    use crate::battle::enemy_in_battle::EnemyInBattle;
    use crate::game::deck::Deck;
    use crate::game::global_info::GlobalInfo;
    use crate::game::enemy::EnemyTrait;
    use crate::enemies::red_louse::RedLouse;
    use crate::enemies::enemy_enum::EnemyEnum;

    #[test]
    fn test_sentinel_creation() {
        let card = sentinel();
        assert_eq!(card.get_name(), "Sentinel");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_sentinel_upgraded_creation() {
        let card = sentinel_upgraded();
        assert_eq!(card.get_name(), "Sentinel+");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_sentinel_effects() {
        let card = sentinel();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 2);
        match &effects[0] {
            Effect::GainDefense { amount } => {
                assert_eq!(*amount, 5);
            }
            _ => panic!("Expected GainDefense effect"),
        }

        match &effects[1] {
            Effect::GainEnergyIfNoBlock { amount } => {
                assert_eq!(*amount, 2);
            }
            _ => panic!("Expected GainEnergyIfNoBlock effect"),
        }
    }

    #[test]
    fn test_sentinel_upgraded_effects() {
        let card = sentinel_upgraded();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 2);
        match &effects[0] {
            Effect::GainDefense { amount } => {
                assert_eq!(*amount, 8);
            }
            _ => panic!("Expected GainDefense effect"),
        }

        match &effects[1] {
            Effect::GainEnergyIfNoBlock { amount } => {
                assert_eq!(*amount, 3);
            }
            _ => panic!("Expected GainEnergyIfNoBlock effect"),
        }
    }

    #[test]
    fn test_sentinel_battle_integration_with_block() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        let deck = Deck::new(vec![sentinel()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, vec![enemy], &mut rng);

        // Give player some block first
        battle.get_player_mut().battle_info.gain_block(3);
        let initial_block = battle.get_player().get_block();
        let initial_energy = battle.get_player().get_energy();

        // Play Sentinel with existing block
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());

        // Should gain block but no energy (since block > 0)
        assert_eq!(battle.get_player().get_block(), initial_block + 5);
        assert_eq!(battle.get_player().get_energy(), initial_energy - 1); // Normal cost
    }

    #[test]
    fn test_sentinel_battle_integration_no_block() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        let deck = Deck::new(vec![sentinel()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, vec![enemy], &mut rng);

        let initial_block = battle.get_player().get_block();
        let initial_energy = battle.get_player().get_energy();

        // Play Sentinel with no block
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());

        // Should gain block and energy (since block = 0)
        assert_eq!(battle.get_player().get_block(), initial_block + 5);
        assert_eq!(battle.get_player().get_energy(), initial_energy - 1 + 2); // Cost -1, gain +2
    }

    #[test]
    fn test_sentinel_upgraded_battle_integration() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        let deck = Deck::new(vec![sentinel_upgraded()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, vec![enemy], &mut rng);

        let initial_block = battle.get_player().get_block();
        let initial_energy = battle.get_player().get_energy();

        // Play Sentinel+ with no block
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());

        // Should gain more block and energy, costs 0
        assert_eq!(battle.get_player().get_block(), initial_block + 8);
        assert_eq!(battle.get_player().get_energy(), initial_energy + 3); // Cost 0, gain +3
    }

    #[test]
    fn test_sentinel_cost_reduction() {
        let normal_card = sentinel();
        let upgraded_card = sentinel_upgraded();

        assert_eq!(normal_card.get_cost(), 1, "Sentinel should cost 1 energy");
        assert_eq!(upgraded_card.get_cost(), 0, "Sentinel+ should cost 0 energy");
    }
}