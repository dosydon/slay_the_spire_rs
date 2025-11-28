use crate::game::{card::Card, effect::Effect, card_type::CardType, card_enum::CardEnum, effect::Condition};

/// Sentinel - Uncommon Skill Card
/// Cost: 1 (0 when upgraded)
/// Effect: Gain 5 (8+) Block. Whenever this card is Exhausted, gain 2 (3+) Energy.
pub fn sentinel() -> Card {
    Card::new_with_on_exhaust(
        CardEnum::Sentinel,
        1,
        CardType::Skill,
        vec![
            Effect::GainDefense { amount: 5 },
        ],
        false, // not upgraded
        Condition::True,
        vec![
            Effect::GainEnergy { amount: 2 },
        ],
    )
}

/// Sentinel+ (Upgraded version)
/// Cost: 0
/// Effect: Gain 8 Block. Whenever this card is Exhausted, gain 3 Energy.
pub fn sentinel_upgraded() -> Card {
    Card::new_with_on_exhaust(
        CardEnum::Sentinel,
        0,
        CardType::Skill,
        vec![
            Effect::GainDefense { amount: 8 },
        ],
        true,  // upgraded
        Condition::True,
        vec![
            Effect::GainEnergy { amount: 3 },
        ],
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

        assert_eq!(effects.len(), 1);
        match &effects[0] {
            Effect::GainDefense { amount } => {
                assert_eq!(*amount, 5);
            }
            _ => panic!("Expected GainDefense effect"),
        }

        // Check on_exhaust effects
        let on_exhaust = card.get_on_exhaust();
        assert!(on_exhaust.is_some(), "Sentinel should have on_exhaust effects");
        let on_exhaust_effects = on_exhaust.unwrap();
        assert_eq!(on_exhaust_effects.len(), 1);
        match &on_exhaust_effects[0] {
            Effect::GainEnergy { amount } => {
                assert_eq!(*amount, 2);
            }
            _ => panic!("Expected GainEnergy on exhaust effect"),
        }
    }

    #[test]
    fn test_sentinel_upgraded_effects() {
        let card = sentinel_upgraded();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 1);
        match &effects[0] {
            Effect::GainDefense { amount } => {
                assert_eq!(*amount, 8);
            }
            _ => panic!("Expected GainDefense effect"),
        }

        // Check on_exhaust effects
        let on_exhaust = card.get_on_exhaust();
        assert!(on_exhaust.is_some(), "Sentinel+ should have on_exhaust effects");
        let on_exhaust_effects = on_exhaust.unwrap();
        assert_eq!(on_exhaust_effects.len(), 1);
        match &on_exhaust_effects[0] {
            Effect::GainEnergy { amount } => {
                assert_eq!(*amount, 3);
            }
            _ => panic!("Expected GainEnergy on exhaust effect"),
        }
    }

    #[test]
    fn test_sentinel_battle_integration() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        let deck = Deck::new(vec![sentinel()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, vec![enemy], &mut rng);

        let initial_block = battle.get_player().get_block();
        let initial_energy = battle.get_player().get_energy();

        // Play Sentinel - it should NOT be exhausted on play, just gains block
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());

        // Should gain block only, activates the listener
        assert_eq!(battle.get_player().get_block(), initial_block + 5, "Should gain 5 block");
        assert_eq!(battle.get_player().get_energy(), initial_energy - 1, "Cost -1, no energy gain yet");

        // Sentinel should go to discard pile, not exhausted
        let discard = battle.cards.get_discard_pile();
        assert!(discard.iter().any(|card| card.get_name() == "Sentinel"), "Sentinel should be in discard pile");
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

        // Play Sentinel+ - it should NOT be exhausted on play, just gains block
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());

        // Should gain more block, costs 0, no energy gain yet
        assert_eq!(battle.get_player().get_block(), initial_block + 8, "Should gain 8 block");
        assert_eq!(battle.get_player().get_energy(), initial_energy, "Cost 0, no energy gain yet");

        // Sentinel+ should go to discard pile, not exhausted
        let discard = battle.cards.get_discard_pile();
        assert!(discard.iter().any(|card| card.get_name() == "Sentinel+"), "Sentinel+ should be in discard pile");
    }

    #[test]
    fn test_sentinel_with_corruption() {
        use crate::cards::ironclad::corruption::corruption;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        // Create battle with Corruption and Sentinel
        let deck = Deck::new(vec![corruption(), sentinel()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, vec![enemy], &mut rng);

        // Give player enough energy
        battle.get_player_mut().battle_info.energy = 10;

        // Play Corruption first
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok(), "Should be able to play Corruption");

        let initial_block = battle.get_player().get_block();
        let initial_energy = battle.get_player().get_energy();

        // Play Sentinel - Corruption will exhaust it, triggering Sentinel's energy gain
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok(), "Should be able to play Sentinel");

        // Should gain block from Sentinel, cost 0 with Corruption, gain 2 energy from exhaust
        assert_eq!(battle.get_player().get_block(), initial_block + 5, "Should gain 5 block");
        assert_eq!(battle.get_player().get_energy(), initial_energy + 2, "Cost 0 (Corruption), gain +2 energy from exhaust");

        // Sentinel should be in exhausted pile (due to Corruption)
        let exhausted_cards = battle.cards.get_exhausted();
        assert!(exhausted_cards.iter().any(|card| card.get_name() == "Sentinel"), "Sentinel should be exhausted by Corruption");
    }

    #[test]
    fn test_sentinel_cost_reduction() {
        let normal_card = sentinel();
        let upgraded_card = sentinel_upgraded();

        assert_eq!(normal_card.get_cost(), 1, "Sentinel should cost 1 energy");
        assert_eq!(upgraded_card.get_cost(), 0, "Sentinel+ should cost 0 energy");
    }
}