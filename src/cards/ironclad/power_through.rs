use crate::game::{card::{Card, Rarity}, card_type::CardType, card_enum::CardEnum, effect::Effect};

pub fn power_through() -> Card {
    Card::new(
        CardEnum::PowerThrough,
        1,
        CardType::Skill,
        vec![
            Effect::AddCardToHand(CardEnum::Wound),
            Effect::AddCardToHand(CardEnum::Wound),
            Effect::GainDefense { amount: 15 },
        ],
        false, // not upgraded
        true,  // playable
        Rarity::Uncommon)
}

pub fn power_through_upgraded() -> Card {
    Card::new(
        CardEnum::PowerThrough,
        1,
        CardType::Skill,
        vec![
            Effect::AddCardToHand(CardEnum::Wound),
            Effect::AddCardToHand(CardEnum::Wound),
            Effect::GainDefense { amount: 20 },
        ],
        true,  // upgraded
        true,  // playable
        Rarity::Uncommon)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_through_creation() {
        let card = power_through();
        assert_eq!(card.get_name(), "Power Through");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert!(!card.is_upgraded());
        assert_eq!(card.get_effects().len(), 3);
        assert!(card.is_playable());
    }

    #[test]
    fn test_power_through_upgraded_creation() {
        let card = power_through_upgraded();
        assert_eq!(card.get_name(), "Power Through+");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert!(card.is_upgraded());
        assert_eq!(card.get_effects().len(), 3);
        assert!(card.is_playable());
    }

    #[test]
    fn test_power_through_effects() {
        let card = power_through();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 3);
        // Should add 2 Wounds and gain 15 Block
        assert_eq!(effects[0], Effect::AddCardToHand(CardEnum::Wound));
        assert_eq!(effects[1], Effect::AddCardToHand(CardEnum::Wound));
        assert_eq!(effects[2], Effect::GainDefense { amount: 15 });
    }

    #[test]
    fn test_power_through_upgraded_effects() {
        let card = power_through_upgraded();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 3);
        // Should add 2 Wounds and gain 20 Block
        assert_eq!(effects[0], Effect::AddCardToHand(CardEnum::Wound));
        assert_eq!(effects[1], Effect::AddCardToHand(CardEnum::Wound));
        assert_eq!(effects[2], Effect::GainDefense { amount: 20 });
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::battle::Battle;
    use crate::battle::target::Entity;
    use crate::battle::enemy_in_battle::EnemyInBattle;
    use crate::game::deck::Deck;
    use crate::game::global_info::GlobalInfo;
    use crate::game::enemy::EnemyTrait;
    use crate::enemies::jaw_worm::JawWorm;
    use crate::enemies::enemy_enum::EnemyEnum;

    #[test]
    fn test_power_through_adds_wounds_and_gains_block() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm))];

        let deck = Deck::new(vec![power_through()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        let initial_hand_size = battle.get_hand().len();
        let initial_block = battle.get_player().get_block();

        // Play Power Through
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());

        // Verify 2 Wounds were added to hand (net increase of 1 since Power Through was removed)
        let final_hand_size = battle.get_hand().len();
        assert_eq!(final_hand_size, initial_hand_size + 1); // -1 (Power Through) +2 (Wounds)

        // Verify hand contains 2 Wounds
        let wounds_in_hand = battle.get_hand().iter().filter(|c| c.get_name() == "Wound").count();
        assert_eq!(wounds_in_hand, 2);

        // Verify player gained 15 block
        let final_block = battle.get_player().get_block();
        assert_eq!(final_block, initial_block + 15);
    }

    #[test]
    fn test_power_through_upgraded_higher_block() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm))];

        let deck = Deck::new(vec![power_through_upgraded()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        let initial_block = battle.get_player().get_block();

        // Play Power Through+
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());

        // Verify player gained 20 block (upgraded version)
        let final_block = battle.get_player().get_block();
        assert_eq!(final_block, initial_block + 20);

        // Verify still adds 2 Wounds
        let wounds_in_hand = battle.get_hand().iter().filter(|c| c.get_name() == "Wound").count();
        assert_eq!(wounds_in_hand, 2);
    }

    #[test]
    fn test_power_through_costs_one_energy() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm))];

        let deck = Deck::new(vec![power_through()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        let initial_energy = battle.get_player().get_energy();

        // Play Power Through
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());

        // Verify energy was consumed
        assert_eq!(battle.get_player().get_energy(), initial_energy - 1);
    }

    #[test]
    fn test_power_through_wound_is_unplayable() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm))];

        let deck = Deck::new(vec![power_through()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        // Play Power Through
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());

        // Verify Wounds in hand are unplayable (should be Status cards)
        let hand = battle.get_hand();
        let wounds: Vec<_> = hand.iter().filter(|c| c.get_name() == "Wound").collect();
        assert_eq!(wounds.len(), 2);

        for wound in wounds {
            assert_eq!(wound.get_card_type(), &CardType::Status);
        }
    }

    #[test]
    fn test_power_through_is_skill_card() {
        let card = power_through();
        assert_eq!(card.get_card_type(), &CardType::Skill);
    }

    #[test]
    fn test_power_through_upgraded_is_skill_card() {
        let card = power_through_upgraded();
        assert_eq!(card.get_card_type(), &CardType::Skill);
    }
}