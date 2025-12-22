use crate::game::{card::{Card, Rarity, CardClass}, effect::{Effect, Condition}, card_type::CardType, card_enum::CardEnum};

/// Immolate - Rare Attack Card
/// Cost: 2 (2 when upgraded)
/// Effect: Deal 21 damage to ALL enemies. Add a Burn to your discard pile.
pub fn immolate() -> Card {
    Card::new(CardEnum::Immolate, 2, CardClass::IronClad(Rarity::Uncommon, CardType::Attack), vec![
            Effect::AttackAllEnemies { amount: 21, num_attacks: 1 },
            Effect::AddCardToDiscard(CardEnum::Burn),
        ])
        .set_play_condition(Condition::True)
}

/// Immolate+ (Upgraded version)
/// Cost: 2 (2 when upgraded)
/// Effect: Deal 28 damage to ALL enemies. Add a Burn to your discard pile.
pub fn immolate_upgraded() -> Card {
    Card::new(CardEnum::Immolate, 2, CardClass::IronClad(Rarity::Uncommon, CardType::Attack), vec![
            Effect::AttackAllEnemies { amount: 28, num_attacks: 1 },
            Effect::AddCardToDiscard(CardEnum::Burn),
        ])
        .set_play_condition(Condition::True)
        .set_upgraded(true)
}

#[cfg(test)]
mod tests {
    use crate::game::PlayerRunState;

    use super::*;

    #[test]
    fn test_immolate_creation() {
        let card = immolate();

        assert_eq!(card.get_name(), "Immolate");
        assert_eq!(card.get_cost(), 2);
        assert_eq!(card.get_card_type(), CardType::Attack);
        assert_eq!(card.get_effects().len(), 2);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_immolate_upgraded_creation() {
        let card = immolate_upgraded();

        assert_eq!(card.get_name(), "Immolate+");
        assert_eq!(card.get_cost(), 2);
        assert_eq!(card.get_card_type(), CardType::Attack);
        assert_eq!(card.get_effects().len(), 2);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_immolate_effects() {
        let card = immolate();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 2);
        assert_eq!(effects[0], Effect::AttackAllEnemies { amount: 21, num_attacks: 1 });
        assert_eq!(effects[1], Effect::AddCardToDiscard(CardEnum::Burn));
    }

    #[test]
    fn test_immolate_upgraded_effects() {
        let card = immolate_upgraded();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 2);
        assert_eq!(effects[0], Effect::AttackAllEnemies { amount: 28, num_attacks: 1 });
        assert_eq!(effects[1], Effect::AddCardToDiscard(CardEnum::Burn));
    }

    #[test]
    fn test_immolate_cost_stays_same() {
        let immolate_card = immolate();
        assert_eq!(immolate_card.get_cost(), 2, "Immolate should cost 2 energy");

        let immolate_plus = immolate_upgraded();
        assert_eq!(immolate_plus.get_cost(), 2, "Immolate+ should also cost 2 energy");
    }

    #[test]
    fn test_immolate_damage_upgrade() {
        let base_card = immolate();
        let upgraded_card = immolate_upgraded();

        let base_effects = base_card.get_effects();
        let upgraded_effects = upgraded_card.get_effects();

        // Check damage amounts
        assert_eq!(base_effects[0], Effect::AttackAllEnemies { amount: 21, num_attacks: 1 });
        assert_eq!(upgraded_effects[0], Effect::AttackAllEnemies { amount: 28, num_attacks: 1 });

        // Both should add Burn to discard
        assert_eq!(base_effects[1], Effect::AddCardToDiscard(CardEnum::Burn));
        assert_eq!(upgraded_effects[1], Effect::AddCardToDiscard(CardEnum::Burn));
    }

    #[test]
    fn test_immolate_battle_integration() {
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

        // Create battle with Immolate in hand
        let deck = Deck::new(vec![immolate()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        // Check initial enemy HP
        let initial_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();
        let initial_discard_size = battle.cards.discard_pile_size();

        // Play Immolate targeting an enemy (AOE hits all)
        let immolate_idx = 0;
        let result = battle.play_card(immolate_idx, Entity::Enemy(0));
        assert!(result.is_ok());

        // Verify damage was dealt to all enemies
        let final_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();
        assert_eq!(final_enemy_hp, initial_enemy_hp - 21);

        // Verify Burn was added to discard pile (and possibly other cards from Gremlin Nob behavior)
        let final_discard_size = battle.cards.discard_pile_size();
        assert!(final_discard_size >= initial_discard_size + 1);

        let discard = battle.cards.get_discard_pile();
        assert!(discard.iter().any(|card| card.get_name() == "Burn"));
    }

    #[test]
    fn test_immolate_upgraded_battle_integration() {
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

        // Create battle with Immolate+ in hand
        let deck = Deck::new(vec![immolate_upgraded()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        // Check initial enemy HP
        let initial_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();

        // Play Immolate+ targeting an enemy (AOE hits all)
        let immolate_idx = 0;
        let result = battle.play_card(immolate_idx, Entity::Enemy(0));
        assert!(result.is_ok());

        // Verify increased damage was dealt
        let final_enemy_hp = battle.get_enemies()[0].battle_info.get_hp();
        assert_eq!(final_enemy_hp, initial_enemy_hp - 28);
    }

    #[test]
    fn test_immolate_multiple_enemies() {
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
        let gremlin_nob1 = GremlinNob::instantiate(&mut rng, &global_info);
        let gremlin_nob2 = GremlinNob::instantiate(&mut rng, &global_info);
        let enemies = vec![
            EnemyInBattle::new(EnemyEnum::GremlinNob(gremlin_nob1)),
            EnemyInBattle::new(EnemyEnum::GremlinNob(gremlin_nob2)),
        ];

        // Create battle with Immolate in hand
        let deck = Deck::new(vec![immolate()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        // Check initial enemy HP
        let initial_hp1 = battle.get_enemies()[0].battle_info.get_hp();
        let initial_hp2 = battle.get_enemies()[1].battle_info.get_hp();

        // Play Immolate
        let immolate_idx = 0;
        let result = battle.play_card(immolate_idx, Entity::Enemy(0));
        assert!(result.is_ok());

        // Verify damage was dealt to ALL enemies
        let final_hp1 = battle.get_enemies()[0].battle_info.get_hp();
        let final_hp2 = battle.get_enemies()[1].battle_info.get_hp();
        assert_eq!(final_hp1, initial_hp1 - 21);
        assert_eq!(final_hp2, initial_hp2 - 21);
    }
}