use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect, card::Rarity};

/// Entrench - Skill Card
/// Cost: 2
/// Effect: Double your current Block.
pub fn entrench() -> Card {
    Card::new(CardEnum::Entrench, 2, CardType::Skill, vec![
        Effect::DoubleBlock,
    ], false, true,
        Rarity::Common)
}

/// Entrench+ (Upgraded)
/// Cost: 1
/// Effect: Double your current Block.
pub fn entrench_upgraded() -> Card {
    Card::new(CardEnum::Entrench, 1, CardType::Skill, vec![
        Effect::DoubleBlock,
    ], true, true,
        Rarity::Common)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entrench_creation() {
        let card = entrench();

        assert_eq!(card.get_name(), "Entrench");
        assert_eq!(card.get_cost(), 2);
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert_eq!(card.get_effects().len(), 1);
        assert_eq!(card.get_effects()[0], Effect::DoubleBlock);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_entrench_upgraded_creation() {
        let card = entrench_upgraded();

        assert_eq!(card.get_name(), "Entrench+");
        assert_eq!(card.get_cost(), 1); // Upgraded cost is 1
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert_eq!(card.get_effects().len(), 1);
        assert_eq!(card.get_effects()[0], Effect::DoubleBlock);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_entrench_battle_integration() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::red_louse::RedLouse;
        use crate::enemies::enemy_enum::EnemyEnum;
        use crate::cards::ironclad::defend::defend;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create battle with Defend and Entrench in hand
        let deck = Deck::new(vec![defend(), entrench()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        // Play Defend first to gain some block
        let defend_idx = 0;
        let result = battle.play_card(defend_idx, Entity::Player);
        assert!(result.is_ok());

        let block_after_defend = battle.get_player().get_block();
        assert!(block_after_defend > 0);

        // Play Entrench to double the block
        let entrench_idx = 0; // Now at index 0 after Defend was played
        let result = battle.play_card(entrench_idx, Entity::Player);
        assert!(result.is_ok());

        let block_after_entrench = battle.get_player().get_block();
        assert_eq!(block_after_entrench, block_after_defend * 2);

        // Verify Entrench went to discard pile (not a power card)
        let discard = battle.cards.get_discard_pile();
        assert_eq!(discard.len(), 2); // Defend and Entrench
        assert!(discard.iter().any(|card| card.get_name() == "Entrench"));
    }

    #[test]
    fn test_entrench_upgraded_battle_integration() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::red_louse::RedLouse;
        use crate::enemies::enemy_enum::EnemyEnum;
        use crate::cards::ironclad::defend::defend;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create battle with Defend and Entrench+ in hand
        let deck = Deck::new(vec![defend(), entrench_upgraded()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        // Play Defend first to gain some block
        let defend_idx = 0;
        let result = battle.play_card(defend_idx, Entity::Player);
        assert!(result.is_ok());

        let block_after_defend = battle.get_player().get_block();
        assert!(block_after_defend > 0);

        // Play Entrench+ to double the block (costs only 1 energy)
        let entrench_idx = 0; // Now at index 0 after Defend was played
        let result = battle.play_card(entrench_idx, Entity::Player);
        assert!(result.is_ok());

        let block_after_entrench = battle.get_player().get_block();
        assert_eq!(block_after_entrench, block_after_defend * 2);

        // Verify Entrench+ went to discard pile
        let discard = battle.cards.get_discard_pile();
        assert_eq!(discard.len(), 2); // Defend and Entrench+
        assert!(discard.iter().any(|card| card.get_name() == "Entrench+"));
    }

    #[test]
    fn test_entrench_with_zero_block() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::red_louse::RedLouse;
        use crate::enemies::enemy_enum::EnemyEnum;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create battle with Entrench in hand but no prior block
        let deck = Deck::new(vec![entrench()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        let initial_block = battle.get_player().get_block();
        assert_eq!(initial_block, 0);

        // Play Entrench with zero block
        let entrench_idx = 0;
        let result = battle.play_card(entrench_idx, Entity::Player);
        assert!(result.is_ok());

        let block_after_entrench = battle.get_player().get_block();
        // Zero block doubled should still be zero
        assert_eq!(block_after_entrench, 0);
    }

    #[test]
    fn test_multiple_entrench() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::red_louse::RedLouse;
        use crate::enemies::enemy_enum::EnemyEnum;
        use crate::cards::ironclad::defend::defend;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create battle with Defend and two Entrench+ cards (cost less energy)
        let deck = Deck::new(vec![defend(), entrench_upgraded(), entrench_upgraded()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        // Play Defend first
        let defend_idx = 0;
        let result = battle.play_card(defend_idx, Entity::Player);
        assert!(result.is_ok());

        let block_after_defend = battle.get_player().get_block();

        // Play first Entrench
        let entrench_idx = 0;
        let result = battle.play_card(entrench_idx, Entity::Player);
        assert!(result.is_ok());

        let block_after_first_entrench = battle.get_player().get_block();
        assert_eq!(block_after_first_entrench, block_after_defend * 2);

        // Play second Entrench (upgraded, costs 1)
        let entrench_idx = 0;
        let result = battle.play_card(entrench_idx, Entity::Player);
        assert!(result.is_ok());

        let block_after_second_entrench = battle.get_player().get_block();
        assert_eq!(block_after_second_entrench, block_after_first_entrench * 2);
        assert_eq!(block_after_second_entrench, block_after_defend * 4);
    }
}