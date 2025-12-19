use crate::game::{card::{Card, Rarity}, card_type::CardType, card_enum::CardEnum, effect::Effect};

/// Reckless Charge - Common Attack Card
/// Cost: 0
/// Effect: Deal 7 damage. Add Dazed to discard pile
pub fn reckless_charge() -> Card {
    Card::new(
        CardEnum::RecklessCharge,
        0,
        CardType::Attack,
        vec![
            Effect::AttackToTarget { amount: 7, num_attacks: 1, strength_multiplier: 1 },
            Effect::AddStatusToDiscard { status_card: crate::game::card_enum::CardEnum::Dazed },
        ],
        false, // not upgraded
        true,  // playable
        Rarity::Uncommon)
}

/// Reckless Charge+ (Upgraded version)
/// Cost: 0
/// Effect: Deal 10 damage. Add Dazed to discard pile
pub fn reckless_charge_upgraded() -> Card {
    Card::new(
        CardEnum::RecklessCharge,
        0,
        CardType::Attack,
        vec![
            Effect::AttackToTarget { amount: 10, num_attacks: 1, strength_multiplier: 1 },
            Effect::AddStatusToDiscard { status_card: crate::game::card_enum::CardEnum::Dazed },
        ],
        true,  // upgraded
        true,  // playable
        Rarity::Uncommon)
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
    fn test_reckless_charge_creation() {
        let card = reckless_charge();
        assert_eq!(card.get_name(), "Reckless Charge");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_reckless_charge_upgraded_creation() {
        let card = reckless_charge_upgraded();
        assert_eq!(card.get_name(), "Reckless Charge+");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_reckless_charge_effects() {
        let card = reckless_charge();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 2);
        match &effects[0] {
            Effect::AttackToTarget { amount, num_attacks, strength_multiplier } => {
                assert_eq!(*amount, 7);
                assert_eq!(*num_attacks, 1);
                assert_eq!(*strength_multiplier, 1);
            }
            _ => panic!("Expected AttackToTarget effect"),
        }

        match &effects[1] {
            Effect::AddStatusToDiscard { status_card } => {
                assert_eq!(status_card.name(), "Dazed");
            }
            _ => panic!("Expected AddStatusToDiscard effect"),
        }
    }

    #[test]
    fn test_reckless_charge_upgraded_effects() {
        let card = reckless_charge_upgraded();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 2);
        match &effects[0] {
            Effect::AttackToTarget { amount, num_attacks, strength_multiplier } => {
                assert_eq!(*amount, 10);
                assert_eq!(*num_attacks, 1);
                assert_eq!(*strength_multiplier, 1);
            }
            _ => panic!("Expected AttackToTarget effect"),
        }
    }

    #[test]
    fn test_reckless_charge_battle_integration() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        let deck = Deck::new(vec![reckless_charge()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, vec![enemy], &mut rng);

        let initial_enemy_hp = battle.get_enemies()[0].get_current_hp();
        let initial_discard_size = battle.cards.discard_pile_size();

        // Play Reckless Charge
        let result = battle.play_card(0, Entity::Enemy(0));
        assert!(result.is_ok());

        // Verify damage dealt
        let final_enemy_hp = battle.get_enemies()[0].get_current_hp();
        assert_eq!(final_enemy_hp, initial_enemy_hp.saturating_sub(7));

        // Verify Dazed added to discard
        assert_eq!(battle.cards.discard_pile_size(), initial_discard_size + 2);
        let discard_cards = battle.cards.get_discard_pile();
        assert_eq!(discard_cards.last().unwrap().get_name(), "Dazed");
    }

    #[test]
    fn test_reckless_charge_upgraded_battle_integration() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::RedLouse(red_louse));

        let deck = Deck::new(vec![reckless_charge_upgraded()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, vec![enemy], &mut rng);

        let initial_enemy_hp = battle.get_enemies()[0].get_current_hp();

        // Play Reckless Charge+
        let result = battle.play_card(0, Entity::Enemy(0));
        assert!(result.is_ok());

        // Verify increased damage dealt
        let final_enemy_hp = battle.get_enemies()[0].get_current_hp();
        assert_eq!(final_enemy_hp, initial_enemy_hp - 10);
    }

    #[test]
    fn test_reckless_charge_zero_cost() {
        let normal_card = reckless_charge();
        let upgraded_card = reckless_charge_upgraded();

        assert_eq!(normal_card.get_cost(), 0, "Reckless Charge should cost 0 energy");
        assert_eq!(upgraded_card.get_cost(), 0, "Reckless Charge+ should also cost 0 energy");
    }
}