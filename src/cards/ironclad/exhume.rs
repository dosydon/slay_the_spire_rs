use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect, card::Rarity};

pub fn exhume() -> Card {
    Card::new(
        CardEnum::Exhume,
        1,
        CardType::Skill,
        vec![
            Effect::EnterSelectCardInExhaust, // Transition to select card from exhaust pile
            Effect::Exhaust,
        ],
        false, // not upgraded
        true,  // playable
        Rarity::Uncommon
    )
}

pub fn exhume_upgraded() -> Card {
    Card::new(
        CardEnum::Exhume,
        0, // costs 0 when upgraded
        CardType::Skill,
        vec![
            Effect::EnterSelectCardInExhaust, // Transition to select card from exhaust pile
            Effect::Exhaust,
        ],
        true,  // upgraded
        true,  // playable
        Rarity::Uncommon
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exhume_creation() {
        let card = exhume();
        assert_eq!(card.get_name(), "Exhume");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert!(!card.is_upgraded());
        assert_eq!(card.get_effects().len(), 2);
        assert!(card.is_playable());
    }

    #[test]
    fn test_exhume_upgraded_creation() {
        let card = exhume_upgraded();
        assert_eq!(card.get_name(), "Exhume+");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert!(card.is_upgraded());
        assert_eq!(card.get_effects().len(), 2);
        assert!(card.is_playable());
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::battle::Battle;
    use crate::battle::target::Entity;
    use crate::battle::enemy_in_battle::EnemyInBattle;
    use crate::battle::battle_action::BattleAction;
    use crate::battle::battle_state::BattleState;
    use crate::game::PlayerRunState;
    use crate::game::deck::Deck;
    use crate::game::global_info::GlobalInfo;
    use crate::game::enemy::EnemyTrait;
    use crate::enemies::jaw_worm::JawWorm;
    use crate::enemies::enemy_enum::EnemyEnum;
    use crate::cards::ironclad::seeing_red::seeing_red;

    #[test]
    fn test_exhume_enters_select_card_state() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm))];

        // Create battle with Exhume and Seeing Red (Seeing Red will be exhausted first)
        let deck = Deck::new(vec![seeing_red(), exhume()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        // Exhaust Seeing Red first by playing it
        let seeing_red_idx = battle.get_hand().iter().position(|c| c.get_name() == "Seeing Red").unwrap();
        let result = battle.play_card(seeing_red_idx, Entity::Player);
        assert!(result.is_ok());

        // Verify Seeing Red is in exhaust pile
        assert_eq!(battle.cards.exhausted_size(), 1);

        // Verify only Exhume remains in hand
        assert_eq!(battle.get_hand().len(), 1);

        // Play Exhume
        let exhume_idx = battle.get_hand().iter().position(|c| c.get_name() == "Exhume").unwrap();
        let result = battle.play_card(exhume_idx, Entity::Player);
        assert!(result.is_ok());

        // Verify battle state changed to SelectCardInExhaust
        assert_eq!(battle.get_battle_state(), BattleState::SelectCardInExhaust);

        // Select Seeing Red from exhaust pile
        let result = battle.eval_action(BattleAction::SelectCardInExhaust(0), &mut rng);
        assert!(result.is_ok());

        // Verify Seeing Red is now back in hand
        let hand = battle.get_hand();
        assert_eq!(hand.len(), 1);
        assert_eq!(hand[0].get_name(), "Seeing Red");
    }

    #[test]
    fn test_exhume_upgraded_costs_zero_energy() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm))];

        let deck = Deck::new(vec![exhume_upgraded()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        let initial_energy = battle.get_player().get_energy();

        // Play Exhume+
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());

        // Verify no energy was consumed (costs 0)
        assert_eq!(battle.get_player().get_energy(), initial_energy);
    }

    #[test]
    fn test_exhume_regular_costs_one_energy() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm))];

        let deck = Deck::new(vec![exhume()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        let initial_energy = battle.get_player().get_energy();

        // Play Exhume
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());

        // Verify energy was consumed
        assert_eq!(battle.get_player().get_energy(), initial_energy - 1);
    }

    #[test]
    fn test_exhume_exhausts_after_use() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm))];

        // Create battle with Seeing Red and Exhume
        let deck = Deck::new(vec![seeing_red(), exhume()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        // Exhaust Seeing Red first
        let seeing_red_idx = battle.get_hand().iter().position(|c| c.get_name() == "Seeing Red").unwrap();
        let result = battle.play_card(seeing_red_idx, Entity::Player);
        assert!(result.is_ok());

        assert_eq!(battle.cards.exhausted_size(), 1);

        // Play Exhume
        let exhume_idx = battle.get_hand().iter().position(|c| c.get_name() == "Exhume").unwrap();
        let result = battle.play_card(exhume_idx, Entity::Player);
        assert!(result.is_ok());

        // Battle state should be SelectCardInExhaust
        assert_eq!(battle.get_battle_state(), BattleState::SelectCardInExhaust);

        // Select Seeing Red from exhaust pile
        let result = battle.eval_action(BattleAction::SelectCardInExhaust(0), &mut rng);
        assert!(result.is_ok());

        // Verify Seeing Red is now back in hand
        let hand = battle.get_hand();
        assert!(hand.iter().any(|c| c.get_name() == "Seeing Red"), "Seeing Red should be returned to hand");

        // Verify Exhume itself is now in exhaust pile (along with any other cards)
        // The exhaust pile should contain Exhume now
        let exhaust_pile = battle.cards.get_exhausted();
        assert!(exhaust_pile.iter().any(|c| c.get_name() == "Exhume" || c.get_name() == "Exhume+"));
    }
}