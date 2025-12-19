use crate::game::{card::Card, effect::Effect, card_type::CardType, card_enum::CardEnum, card::Rarity};

/// Pummel - Uncommon Attack Card
/// Cost: 1
/// Effect: Deal 2 damage 4 times. Exhaust.
pub fn pummel() -> Card {
    Card::new(
        CardEnum::Pummel,
        1,
        CardType::Attack,
        vec![
            Effect::AttackToTarget {
                amount: 2,
                num_attacks: 4,
                strength_multiplier: 1,
            },
            Effect::Exhaust,
        ],
        false, // not upgraded
        true,  // playable
        Rarity::Uncommon
    )
}

/// Pummel+ (Upgraded version)
/// Cost: 1
/// Effect: Deal 2 damage 5 times. Exhaust.
pub fn pummel_upgraded() -> Card {
    Card::new(
        CardEnum::Pummel,
        1,
        CardType::Attack,
        vec![
            Effect::AttackToTarget {
                amount: 2,
                num_attacks: 5, // upgraded version deals damage 5 times instead of 4
                strength_multiplier: 1,
            },
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
    fn test_pummel_creation() {
        let card = pummel();
        assert_eq!(card.get_name(), "Pummel");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert_eq!(card.get_effects().len(), 2); // AttackToTarget and Exhaust
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_pummel_upgraded_creation() {
        let card = pummel_upgraded();
        assert_eq!(card.get_name(), "Pummel+");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert_eq!(card.get_effects().len(), 2); // AttackToTarget and Exhaust
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_pummel_effect() {
        let card = pummel();
        let effects = card.get_effects();
        assert_eq!(effects.len(), 2);
        match &effects[0] {
            Effect::AttackToTarget { amount, num_attacks, strength_multiplier } => {
                assert_eq!(*amount, 2);
                assert_eq!(*num_attacks, 4);
                assert_eq!(*strength_multiplier, 1);
            }
            _ => panic!("Expected AttackToTarget effect"),
        }
        assert_eq!(effects[1], Effect::Exhaust);
    }

    #[test]
    fn test_pummel_upgraded_effect() {
        let card = pummel_upgraded();
        let effects = card.get_effects();
        assert_eq!(effects.len(), 2);
        match &effects[0] {
            Effect::AttackToTarget { amount, num_attacks, strength_multiplier } => {
                assert_eq!(*amount, 2);
                assert_eq!(*num_attacks, 5); // upgraded version hits 5 times
                assert_eq!(*strength_multiplier, 1);
            }
            _ => panic!("Expected AttackToTarget effect"),
        }
        assert_eq!(effects[1], Effect::Exhaust);
    }

    #[test]
    fn test_pummel_battle_integration() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::gremlin_nob::GremlinNob;
        use crate::enemies::enemy_enum::EnemyEnum;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let gremlin_nob = GremlinNob::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::GremlinNob(gremlin_nob))];

        // Create battle with Pummel in hand
        let deck = Deck::new(vec![pummel()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        // Check initial enemy HP
        let initial_hp = battle.get_enemies()[0].get_current_hp();

        // Play Pummel
        let pummel_idx = 0;
        let result = battle.play_card(pummel_idx, Entity::Enemy(0));
        assert!(result.is_ok());

        // Pummel should deal 2 damage 4 times = 8 total damage
        let final_hp = battle.get_enemies()[0].get_current_hp();
        assert_eq!(final_hp, initial_hp - 8);

        // Pummel should go to exhaust pile (not discard)
        assert_eq!(battle.cards.get_exhausted().len(), 1);
        assert_eq!(battle.cards.get_discard_pile().len(), 0);
    }

    #[test]
    fn test_pummel_upgraded_battle_integration() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::gremlin_nob::GremlinNob;
        use crate::enemies::enemy_enum::EnemyEnum;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let gremlin_nob = GremlinNob::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::GremlinNob(gremlin_nob))];

        // Create battle with Pummel+ in hand
        let deck = Deck::new(vec![pummel_upgraded()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        // Check initial enemy HP
        let initial_hp = battle.get_enemies()[0].get_current_hp();

        // Play Pummel+
        let pummel_idx = 0;
        let result = battle.play_card(pummel_idx, Entity::Enemy(0));
        assert!(result.is_ok());

        // Pummel+ should deal 2 damage 5 times = 10 total damage
        let final_hp = battle.get_enemies()[0].get_current_hp();
        assert_eq!(final_hp, initial_hp - 10);

        // Pummel+ should go to exhaust pile (not discard)
        assert_eq!(battle.cards.get_exhausted().len(), 1);
        assert_eq!(battle.cards.get_discard_pile().len(), 0);
    }

    #[test]
    fn test_pummel_with_strength() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::gremlin_nob::GremlinNob;
        use crate::enemies::enemy_enum::EnemyEnum;
        use crate::cards::ironclad::inflame::inflame;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let gremlin_nob = GremlinNob::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::GremlinNob(gremlin_nob))];

        // Create battle with Inflame and Pummel in hand
        let deck = Deck::new(vec![inflame(), pummel()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        // Draw both cards
        battle.cards.draw_card();
        battle.cards.draw_card();

        // Play Inflame to gain 2 Strength
        let inflame_idx = battle.cards.get_hand().iter()
            .position(|c| c.get_name() == "Inflame").unwrap();
        let result = battle.play_card(inflame_idx, Entity::Player);
        assert!(result.is_ok());

        // Check player has 2 Strength
        assert_eq!(battle.get_player().battle_info.get_strength(), 2);

        // Check initial enemy HP
        let initial_hp = battle.get_enemies()[0].get_current_hp();

        // Play Pummel with Strength bonus
        let pummel_idx = battle.cards.get_hand().iter()
            .position(|c| c.get_name() == "Pummel").unwrap();
        let result = battle.play_card(pummel_idx, Entity::Enemy(0));
        assert!(result.is_ok());

        // Pummel should deal (2 + 2) damage 4 times = 4 damage 4 times = 16 total damage
        let final_hp = battle.get_enemies()[0].get_current_hp();
        assert_eq!(final_hp, initial_hp - 16);
    }

    #[test]
    fn test_pummel_upgraded_name() {
        let card = pummel();
        assert_eq!(card.get_card_enum().upgraded_name(), "Pummel+");
    }
}