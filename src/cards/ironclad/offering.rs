use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect, card::Rarity};

/// Offering - Rare Skill Card
/// Cost: 0
/// Effect: Lose 6 HP. Gain 2 Energy. Draw 3 cards. Exhaust.
pub fn offering() -> Card {
    Card::new(CardEnum::Offering, 0, CardType::Skill, vec![
        Effect::LoseHp(6),
        Effect::GainEnergy { amount: 2 },
        Effect::DrawCard { count: 3 },
        Effect::Exhaust,
    ], false, true,
        Rarity::Rare)
}

/// Offering+ (Upgraded)
/// Cost: 0
/// Effect: Lose 4 HP. Gain 2 Energy. Draw 5 cards. Exhaust.
pub fn offering_upgraded() -> Card {
    Card::new(CardEnum::Offering, 0, CardType::Skill, vec![
        Effect::LoseHp(4),
        Effect::GainEnergy { amount: 2 },
        Effect::DrawCard { count: 5 },
        Effect::Exhaust,
    ], true, true,
        Rarity::Rare)
}

#[cfg(test)]
mod tests {
    use crate::{battle::battle_action::BattleAction, game::PlayerRunState};

    use super::*;

    #[test]
    fn test_offering_creation() {
        let card = offering();

        assert_eq!(card.get_name(), "Offering");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert_eq!(card.get_effects().len(), 4);
        assert_eq!(card.get_effects()[0], Effect::LoseHp(6));
        assert_eq!(card.get_effects()[1], Effect::GainEnergy { amount: 2 });
        assert_eq!(card.get_effects()[2], Effect::DrawCard { count: 3 });
        assert_eq!(card.get_effects()[3], Effect::Exhaust);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_offering_upgraded_creation() {
        let card = offering_upgraded();

        assert_eq!(card.get_name(), "Offering+");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert_eq!(card.get_effects().len(), 4);
        assert_eq!(card.get_effects()[0], Effect::LoseHp(4));
        assert_eq!(card.get_effects()[1], Effect::GainEnergy { amount: 2 });
        assert_eq!(card.get_effects()[2], Effect::DrawCard { count: 5 });
        assert_eq!(card.get_effects()[3], Effect::Exhaust);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_offering_effect_values() {
        let normal_card = offering();
        let upgraded_card = offering_upgraded();

        let normal_effects = normal_card.get_effects();
        let upgraded_effects = upgraded_card.get_effects();

        // Both should have 4 effects
        assert_eq!(normal_effects.len(), 4);
        assert_eq!(upgraded_effects.len(), 4);

        assert_eq!(normal_effects[0], Effect::LoseHp(6));
        assert_eq!(upgraded_effects[0], Effect::LoseHp(4));

        assert_eq!(normal_effects[1], Effect::GainEnergy { amount: 2 });
        assert_eq!(upgraded_effects[1], Effect::GainEnergy { amount: 2 });
        assert_eq!(normal_effects[2], Effect::DrawCard { count: 3 });
        assert_eq!(upgraded_effects[2], Effect::DrawCard { count: 5 });
        assert_eq!(normal_effects[3], Effect::Exhaust);
        assert_eq!(upgraded_effects[3], Effect::Exhaust);
    }

    #[test]
    fn test_offering_upgraded_battle_integration() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::red_louse::RedLouse;
        use crate::enemies::enemy_enum::EnemyEnum;
        use crate::cards::ironclad::strike::strike;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create battle with Offering+ and cards in deck to draw
        let deck = Deck::new(vec![
            offering_upgraded(),
            strike(),
            strike(),
            strike(),
            strike(),
            strike(),  // Add extra cards to ensure enough cards to draw
            strike(),
            strike(),
            strike(),
            strike(),
        ]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        let initial_hp = battle.get_player().battle_info.get_current_hp();
        let initial_energy = battle.get_player().get_energy();
        let initial_hand_size = battle.get_hand().len();

        // Play Offering+ targeting the player
        let offering_idx = 0;
        battle.eval_action(BattleAction::PlayCard(offering_idx, Entity::None), &mut rng).unwrap();

        // Verify player lost 4 HP (upgraded version)
        let hp_after_offering = battle.get_player().battle_info.get_current_hp();
        assert_eq!(hp_after_offering, initial_hp - 4);

        // Verify player gained 2 energy
        let energy_after_offering = battle.get_player().get_energy();
        assert_eq!(energy_after_offering, initial_energy + 2);

        // Verify player drew 5 cards
        let hand_after_offering = battle.get_hand();
        assert_eq!(hand_after_offering.len(), initial_hand_size - 1 + 5); // -1 for offering+ played, +5 drawn

        // Verify Offering+ is exhausted
        let hand = battle.get_hand();
        let discard = battle.get_discard_pile();
        assert!(!hand.iter().any(|card| card.get_name() == "Offering+"));
        assert!(!discard.iter().any(|card| card.get_name() == "Offering+"));
    }
}