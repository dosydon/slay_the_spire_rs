use crate::game::{card::Card, effect::Effect, card_type::CardType, card_enum::CardEnum};

/// Rampage - Uncommon Attack Card
/// Cost: 1
/// Effect: Deal 8 damage. Increases by 5 each use
pub fn rampage() -> Card {
    Card::new(
        CardEnum::Rampage,
        1,
        CardType::Attack,
        vec![
            Effect::AttackToTargetWithScaling {
                base_damage: 8,
                scaling: 5,
            },
        ],
        false, // not upgraded
        true,  // playable
    )
}

/// Rampage+ (Upgraded version)
/// Cost: 1
/// Effect: Deal 8 damage. Increases by 8 each use
pub fn rampage_upgraded() -> Card {
    Card::new(
        CardEnum::Rampage,
        1,
        CardType::Attack,
        vec![
            Effect::AttackToTargetWithScaling {
                base_damage: 8,
                scaling: 8, // upgraded version scales by 8 instead of 5
            },
        ],
        true,  // upgraded
        true,  // playable
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rampage_creation() {
        let card = rampage();
        assert_eq!(card.get_name(), "Rampage");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert_eq!(card.get_effects().len(), 1);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_rampage_upgraded_creation() {
        let card = rampage_upgraded();
        assert_eq!(card.get_name(), "Rampage+");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert_eq!(card.get_effects().len(), 1);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_rampage_effect() {
        let card = rampage();
        let effects = card.get_effects();
        assert_eq!(effects.len(), 1);
        match &effects[0] {
            Effect::AttackToTargetWithScaling { base_damage, scaling } => {
                assert_eq!(*base_damage, 8);
                assert_eq!(*scaling, 5);
            }
            _ => panic!("Expected AttackToTargetWithScaling effect"),
        }
    }

    #[test]
    fn test_rampage_upgraded_effect() {
        let card = rampage_upgraded();
        let effects = card.get_effects();
        assert_eq!(effects.len(), 1);
        match &effects[0] {
            Effect::AttackToTargetWithScaling { base_damage, scaling } => {
                assert_eq!(*base_damage, 8);
                assert_eq!(*scaling, 8); // upgraded version scales by 8
            }
            _ => panic!("Expected AttackToTargetWithScaling effect"),
        }
    }

    #[test]
    fn test_rampage_first_use() {
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

        // Create battle with Rampage in hand
        let deck = Deck::new(vec![rampage()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        // Check initial rampage damage scaling (should be 0)
        let initial_rampage_damage = battle.get_player().battle_info.get_rampage_damage();
        assert_eq!(initial_rampage_damage, 0);

        // Check initial enemy HP
        let initial_hp = battle.get_enemies()[0].get_current_hp();

        // Play Rampage
        let rampage_idx = 0;
        let result = battle.play_card(rampage_idx, Entity::Enemy(0));
        assert!(result.is_ok());

        // First Rampage should deal 8 damage (8 base + 0 scaling)
        let final_hp = battle.get_enemies()[0].get_current_hp();
        assert_eq!(final_hp, initial_hp - 8);

        // Rampage damage scaling should increase by 5
        let new_rampage_damage = battle.get_player().battle_info.get_rampage_damage();
        assert_eq!(new_rampage_damage, 5);
    }

    #[test]
    fn test_rampage_scaling() {
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

        // Create battle with two Rampage cards
        let deck = Deck::new(vec![rampage(), rampage()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        // Draw both cards
        battle.cards.draw_card();
        battle.cards.draw_card();

        // Check initial enemy HP
        let initial_hp = battle.get_enemies()[0].get_current_hp();

        // Play first Rampage
        let first_rampage_idx = battle.cards.get_hand().iter()
            .position(|c| c.get_name() == "Rampage").unwrap();
        let result = battle.play_card(first_rampage_idx, Entity::Enemy(0));
        assert!(result.is_ok());

        // First Rampage should deal 8 damage (8 base + 0 scaling)
        let hp_after_first = battle.get_enemies()[0].get_current_hp();
        assert_eq!(hp_after_first, initial_hp - 8);

        // Play second Rampage
        let second_rampage_idx = battle.cards.get_hand().iter()
            .position(|c| c.get_name() == "Rampage").unwrap();
        let result = battle.play_card(second_rampage_idx, Entity::Enemy(0));
        assert!(result.is_ok());

        // Second Rampage should deal 13 damage (8 base + 5 scaling from first use)
        let final_hp = battle.get_enemies()[0].get_current_hp();
        assert_eq!(final_hp, hp_after_first - 13);

        // Total scaling should be 10 (5 from each use)
        let final_rampage_damage = battle.get_player().battle_info.get_rampage_damage();
        assert_eq!(final_rampage_damage, 10);
    }

    #[test]
    fn test_rampage_upgraded_scaling() {
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

        // Create battle with two Rampage+ cards
        let deck = Deck::new(vec![rampage_upgraded(), rampage_upgraded()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        // Draw both cards
        battle.cards.draw_card();
        battle.cards.draw_card();

        // Check initial enemy HP
        let initial_hp = battle.get_enemies()[0].get_current_hp();

        // Play first Rampage+
        let first_rampage_idx = battle.cards.get_hand().iter()
            .position(|c| c.get_name() == "Rampage+").unwrap();
        let result = battle.play_card(first_rampage_idx, Entity::Enemy(0));
        assert!(result.is_ok());

        // First Rampage+ should deal 8 damage (8 base + 0 scaling)
        let hp_after_first = battle.get_enemies()[0].get_current_hp();
        assert_eq!(hp_after_first, initial_hp - 8);

        // Play second Rampage+
        let second_rampage_idx = battle.cards.get_hand().iter()
            .position(|c| c.get_name() == "Rampage+").unwrap();
        let result = battle.play_card(second_rampage_idx, Entity::Enemy(0));
        assert!(result.is_ok());

        // Second Rampage+ should deal 16 damage (8 base + 8 scaling from first use)
        let final_hp = battle.get_enemies()[0].get_current_hp();
        assert_eq!(final_hp, hp_after_first - 16);

        // Total scaling should be 16 (8 from each use)
        let final_rampage_damage = battle.get_player().battle_info.get_rampage_damage();
        assert_eq!(final_rampage_damage, 16);
    }

    #[test]
    fn test_rampage_with_strength() {
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

        // Create battle with Inflame and Rampage in hand
        let deck = Deck::new(vec![inflame(), rampage()]);
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

        // Play Rampage with Strength bonus
        let rampage_idx = battle.cards.get_hand().iter()
            .position(|c| c.get_name() == "Rampage").unwrap();
        let result = battle.play_card(rampage_idx, Entity::Enemy(0));
        assert!(result.is_ok());

        // Rampage should deal (8 + 2) damage + scaling = 10 damage total
        let final_hp = battle.get_enemies()[0].get_current_hp();
        assert_eq!(final_hp, initial_hp - 10);
    }

    #[test]
    fn test_rampage_upgraded_name() {
        let card = rampage();
        assert_eq!(card.get_card_enum().upgraded_name(), "Rampage+");
    }
}