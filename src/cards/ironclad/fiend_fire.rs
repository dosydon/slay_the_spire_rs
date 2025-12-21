use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect, card::Rarity};
use crate::battle::target::Entity;

pub fn fiend_fire() -> Card {
    Card::new(
        CardEnum::FiendFire,
        2,
        CardType::Attack,
        vec![
            Effect::Exhaust,
            Effect::ExhaustHandForDamage { damage_per_card: 7, target: Entity::Player },
        ],
        false, // not upgraded
        true,  // playable
        Rarity::Uncommon
    )
}

pub fn fiend_fire_upgraded() -> Card {
    Card::new(
        CardEnum::FiendFire,
        2,
        CardType::Attack,
        vec![
            Effect::Exhaust,
            Effect::ExhaustHandForDamage { damage_per_card: 10, target: Entity::Player },
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
    fn test_fiend_fire_creation() {
        let card = fiend_fire();
        assert_eq!(card.get_name(), "Fiend Fire");
        assert_eq!(card.get_cost(), 2);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert!(!card.is_upgraded());
        assert_eq!(card.get_effects().len(), 2);
        assert!(card.is_playable());
    }

    #[test]
    fn test_fiend_fire_upgraded_creation() {
        let card = fiend_fire_upgraded();
        assert_eq!(card.get_name(), "Fiend Fire+");
        assert_eq!(card.get_cost(), 2);
        assert_eq!(card.get_card_type(), &CardType::Attack);
        assert!(card.is_upgraded());
        assert_eq!(card.get_effects().len(), 2);
        assert!(card.is_playable());
    }

    #[test]
    fn test_fiend_fire_effects() {
        let card = fiend_fire();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 2);
        assert_eq!(effects[0], Effect::Exhaust);
        match &effects[1] {
            Effect::ExhaustHandForDamage { damage_per_card, target } => {
                assert_eq!(*damage_per_card, 7);
                assert_eq!(*target, Entity::Player);
            }
            _ => panic!("Expected ExhaustHandForDamage effect"),
        }
    }

    #[test]
    fn test_fiend_fire_upgraded_effects() {
        let card = fiend_fire_upgraded();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 2);
        assert_eq!(effects[0], Effect::Exhaust);
        match &effects[1] {
            Effect::ExhaustHandForDamage { damage_per_card, target } => {
                assert_eq!(*damage_per_card, 10);
                assert_eq!(*target, Entity::Player);
            }
            _ => panic!("Expected ExhaustHandForDamage effect"),
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::battle::Battle;
    use crate::battle::target::Entity;
    use crate::battle::enemy_in_battle::EnemyInBattle;
    use crate::game::PlayerRunState;
    use crate::game::deck::Deck;
    use crate::game::global_info::GlobalInfo;
    use crate::game::enemy::EnemyTrait;
    use crate::enemies::jaw_worm::JawWorm;
    use crate::enemies::enemy_enum::EnemyEnum;
    use crate::cards::ironclad::strike::strike;

    #[test]
    fn test_fiend_fire_exhausts_hand_and_deals_damage() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm))];

        // Create battle with Fiend Fire and 3 Strikes in hand (4 cards total)
        let deck = Deck::new(vec![fiend_fire(), strike(), strike(), strike()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        let initial_enemy_hp = battle.get_enemies()[0].get_current_hp();

        // Play Fiend Fire targeting enemy
        let fiend_fire_idx = battle.get_hand().iter().position(|c| c.get_name() == "Fiend Fire").unwrap();
        let result = battle.play_card(fiend_fire_idx, Entity::Enemy(0));
        assert!(result.is_ok());

        // Verify all remaining cards in hand were exhausted (3 Strikes)
        // After playing Fiend Fire, the hand should be empty
        assert_eq!(battle.get_hand().len(), 0);

        // Verify damage was dealt: 3 cards * 7 damage = 21 damage
        let final_enemy_hp = battle.get_enemies()[0].get_current_hp();
        assert_eq!(final_enemy_hp, initial_enemy_hp - 21);

        // Verify Fiend Fire itself was exhausted
        let exhaust_pile = battle.cards.get_exhausted();
        assert!(exhaust_pile.iter().any(|c| c.get_name() == "Fiend Fire"));
    }

    #[test]
    fn test_fiend_fire_upgraded_higher_damage() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm))];

        // Create battle with upgraded Fiend Fire and 2 Strikes
        let deck = Deck::new(vec![fiend_fire_upgraded(), strike(), strike()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        let initial_enemy_hp = battle.get_enemies()[0].get_current_hp();

        // Play Fiend Fire+ targeting enemy
        let fiend_fire_idx = battle.get_hand().iter().position(|c| c.get_name() == "Fiend Fire+").unwrap();
        let result = battle.play_card(fiend_fire_idx, Entity::Enemy(0));
        assert!(result.is_ok());

        // Verify damage was dealt: 2 cards * 10 damage = 20 damage
        let final_enemy_hp = battle.get_enemies()[0].get_current_hp();
        assert_eq!(final_enemy_hp, initial_enemy_hp - 20);
    }

    #[test]
    fn test_fiend_fire_with_empty_hand() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm))];

        // Create battle with only Fiend Fire
        let deck = Deck::new(vec![fiend_fire()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        let initial_enemy_hp = battle.get_enemies()[0].get_current_hp();

        // Play Fiend Fire with no other cards in hand
        let result = battle.play_card(0, Entity::Enemy(0));
        assert!(result.is_ok());

        // Verify no damage was dealt (0 cards * 7 damage = 0 damage)
        let final_enemy_hp = battle.get_enemies()[0].get_current_hp();
        assert_eq!(final_enemy_hp, initial_enemy_hp);

        // Verify hand is empty
        assert_eq!(battle.get_hand().len(), 0);

        // Verify Fiend Fire was exhausted
        let exhaust_pile = battle.cards.get_exhausted();
        assert!(exhaust_pile.iter().any(|c| c.get_name() == "Fiend Fire"));
    }

    #[test]
    fn test_fiend_fire_costs_two_energy() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm))];

        let deck = Deck::new(vec![fiend_fire()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        let initial_energy = battle.get_player().get_energy();

        // Play Fiend Fire
        let result = battle.play_card(0, Entity::Enemy(0));
        assert!(result.is_ok());

        // Verify energy was consumed (costs 2)
        assert_eq!(battle.get_player().get_energy(), initial_energy - 2);
    }

    #[test]
    fn test_fiend_fire_with_embrace() {
        use crate::cards::ironclad::embrace::embrace;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm))];

        // Create battle with Embrace, Fiend Fire, and many Strikes
        // Embrace will draw cards when cards are exhausted - need enough cards in deck
        let deck = Deck::new(vec![
            embrace(),
            fiend_fire(),
            strike(),
            strike(),
            strike(),
            strike(),
            strike(),
            strike(),
            strike(),
            strike(),
            strike(),
            strike(),
        ]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        // Give player extra energy since Embrace (2) + Fiend Fire (2) = 4 energy needed
        battle.get_player_mut().battle_info.gain_energy(1);

        // Play Embrace first
        let embrace_idx = battle.get_hand().iter().position(|c| c.get_name() == "Embrace").unwrap();
        let result = battle.play_card(embrace_idx, Entity::Player);
        assert!(result.is_ok());

        // Verify Embrace is active
        let powers = battle.get_powers();
        assert_eq!(powers.len(), 1);
        assert_eq!(powers[0].get_name(), "Embrace");

        let initial_deck_size = battle.cards.deck_size();
        let initial_hand_size = battle.get_hand().len();
        let initial_enemy_hp = battle.get_enemies()[0].get_current_hp();

        // Play Fiend Fire (should exhaust itself and remaining 3 cards in hand)
        let fiend_fire_idx = battle.get_hand().iter().position(|c| c.get_name() == "Fiend Fire").unwrap();
        let result = battle.play_card(fiend_fire_idx, Entity::Enemy(0));
        assert!(result.is_ok());

        // Verify damage was dealt: 3 cards * 7 damage = 21 damage
        let final_enemy_hp = battle.get_enemies()[0].get_current_hp();
        assert_eq!(final_enemy_hp, initial_enemy_hp - 21);

        // Verify all cards were exhausted (Fiend Fire + 3 other cards = 4 cards)
        let exhaust_pile = battle.cards.get_exhausted();
        assert_eq!(exhaust_pile.len(), 4);

        // Verify Embrace triggered and drew 4 cards (one for each exhaustion)
        // Hand had initial_hand_size - 4 exhausted cards + 4 drawn cards
        let final_hand_size = battle.get_hand().len();
        assert_eq!(final_hand_size, initial_hand_size); // Net effect: removed 4, drew 4

        // Verify deck decreased by 4 (4 cards drawn by Embrace)
        let final_deck_size = battle.cards.deck_size();
        assert_eq!(final_deck_size, initial_deck_size - 4);
    }

    #[test]
    fn test_fiend_fire_upgraded_costs_two_energy() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm))];

        let deck = Deck::new(vec![fiend_fire_upgraded()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        let initial_energy = battle.get_player().get_energy();

        // Play Fiend Fire+
        let result = battle.play_card(0, Entity::Enemy(0));
        assert!(result.is_ok());

        // Verify energy was consumed (upgraded still costs 2)
        assert_eq!(battle.get_player().get_energy(), initial_energy - 2);
    }

    #[test]
    fn test_fiend_fire_exhausts_in_correct_order() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm))];

        // Create battle with Fiend Fire and 2 Strikes
        let deck = Deck::new(vec![fiend_fire(), strike(), strike()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        // Play Fiend Fire
        let fiend_fire_idx = battle.get_hand().iter().position(|c| c.get_name() == "Fiend Fire").unwrap();
        let result = battle.play_card(fiend_fire_idx, Entity::Enemy(0));
        assert!(result.is_ok());

        // Verify exhaust pile contains exactly 3 cards (Fiend Fire + 2 Strikes)
        let exhaust_pile = battle.cards.get_exhausted();
        assert_eq!(exhaust_pile.len(), 3);

        // Verify Fiend Fire is in the exhaust pile
        assert!(exhaust_pile.iter().any(|c| c.get_name() == "Fiend Fire"));

        // Verify both Strikes are in the exhaust pile
        let strikes_count = exhaust_pile.iter().filter(|c| c.get_name() == "Strike").count();
        assert_eq!(strikes_count, 2);

        // Verify hand is empty
        assert_eq!(battle.get_hand().len(), 0);
    }
}