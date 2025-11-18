use crate::game::{card::Card, effect::Effect, card_type::CardType, card_enum::CardEnum, effect::Condition};
use crate::battle::{target::Entity, events::BattleEvent, events::EventListener};

/// Demon Form - Rare Power Card
/// Cost: 3 (2 when upgraded)
/// Effect: At the start of your turn, gain 2 Strength. This card cannot be shuffled back into your draw pile.
pub fn demon_form() -> Card {
    Card::new_with_condition(
        CardEnum::DemonForm,
        3,
        CardType::Power,
        vec![
            Effect::ActivateDemonForm { strength_per_turn: 2 },
        ],
        false, // not upgraded
        Condition::True,
    )
}

/// Demon Form+ (Upgraded version)
/// Cost: 2
/// Effect: At the start of your turn, gain 3 Strength. This card cannot be shuffled back into your draw pile.
pub fn demon_form_upgraded() -> Card {
    Card::new_with_condition(
        CardEnum::DemonForm,
        2, // Costs 2 when upgraded
        CardType::Power,
        vec![
            Effect::ActivateDemonForm { strength_per_turn: 3 }, // Gain 3 Strength per turn when upgraded
        ],
        true,  // upgraded
        Condition::True,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battle::Battle;
    use crate::battle::target::Entity;
    use crate::battle::enemy_in_battle::EnemyInBattle;
    use crate::game::deck::Deck;
    use crate::game::global_info::GlobalInfo;
    use crate::game::enemy::EnemyTrait;
    use crate::enemies::red_louse::RedLouse;
    use crate::enemies::enemy_enum::EnemyEnum;

    #[test]
    fn test_demon_form_creation() {
        let card = demon_form();

        assert_eq!(card.get_name(), "Demon Form");
        assert_eq!(card.get_cost(), 3);
        assert_eq!(card.get_card_type(), &CardType::Power);
        assert_eq!(card.get_effects().len(), 1);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_demon_form_upgraded_creation() {
        let card = demon_form_upgraded();

        assert_eq!(card.get_name(), "Demon Form+");
        assert_eq!(card.get_cost(), 2);
        assert_eq!(card.get_card_type(), &CardType::Power);
        assert_eq!(card.get_effects().len(), 1);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_demon_form_effects() {
        let card = demon_form();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 1);
        match &effects[0] {
            Effect::ActivateDemonForm { strength_per_turn } => {
                assert_eq!(*strength_per_turn, 2);
            }
            _ => panic!("Expected ActivateDemonForm effect"),
        }
    }

    #[test]
    fn test_demon_form_upgraded_effects() {
        let card = demon_form_upgraded();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 1);
        match &effects[0] {
            Effect::ActivateDemonForm { strength_per_turn } => {
                assert_eq!(*strength_per_turn, 3); // Upgraded version gives 3 Strength per turn
            }
            _ => panic!("Expected ActivateDemonForm effect"),
        }
    }

    #[test]
    fn test_demon_form_cost_upgrade() {
        let base_card = demon_form();
        let upgraded_card = demon_form_upgraded();

        assert_eq!(base_card.get_cost(), 3, "Demon Form should cost 3 energy");
        assert_eq!(upgraded_card.get_cost(), 2, "Demon Form+ should cost 2 energy");
    }

    #[test]
    fn test_demon_form_upgrade_maintains_effects() {
        let base_card = demon_form();
        let upgraded_card = demon_form_upgraded();

        let base_effects = base_card.get_effects();
        let upgraded_effects = upgraded_card.get_effects();

        // Both should have ActivateDemonForm effect
        assert_eq!(base_effects.len(), 1);
        assert_eq!(upgraded_effects.len(), 1);

        // Base version gives 2 Strength per turn
        match &base_effects[0] {
            Effect::ActivateDemonForm { strength_per_turn } => {
                assert_eq!(*strength_per_turn, 2);
            }
            _ => panic!("Expected ActivateDemonForm effect"),
        }

        // Upgraded version gives 3 Strength per turn
        match &upgraded_effects[0] {
            Effect::ActivateDemonForm { strength_per_turn } => {
                assert_eq!(*strength_per_turn, 3);
            }
            _ => panic!("Expected ActivateDemonForm effect"),
        }
    }

    #[test]
    fn test_demon_form_battle_integration() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create battle with Demon Form in hand
        let deck = Deck::new(vec![demon_form()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        // Give player enough energy to play Demon Form
        battle.get_player_mut().battle_info.energy = 3;

        // Check initial strength
        let initial_strength = battle.get_player().battle_info.get_strength();
        assert_eq!(initial_strength, 0);

        // Play Demon Form
        let demon_form_idx = 0;
        let result = battle.play_card(demon_form_idx, Entity::Player);
        assert!(result.is_ok());

        // Demon Form should activate immediately but Strength gain happens at start of turn
        // For now, just verify the power was activated (no immediate Strength gain)
        let strength_after_play = battle.get_player().battle_info.get_strength();
        assert_eq!(strength_after_play, 0);

        // Demon Form should be in the powers pile (Power cards go to powers when played)
        let powers = battle.get_powers();
        assert_eq!(powers.len(), 1);
        assert_eq!(powers[0].get_name(), "Demon Form");

        // Demon Form should NOT be in exhaust pile
        let exhausted_cards = battle.cards.get_exhausted();
        let demon_form_exhausted = exhausted_cards.iter()
            .any(|card| card.get_name() == "Demon Form");
        assert!(!demon_form_exhausted, "Demon Form should not be exhausted");

        // Demon Form should NOT be in discard pile (Power cards go to powers, not discard)
        let discard = battle.cards.get_discard_pile();
        let demon_form_in_discard = discard.iter()
            .any(|card| card.get_name() == "Demon Form");
        assert!(!demon_form_in_discard, "Demon Form should not be in discard pile");
    }

    #[test]
    fn test_demon_form_upgraded_battle_integration() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create battle with Demon Form+ in hand
        let deck = Deck::new(vec![demon_form_upgraded()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        // Give player enough energy
        battle.get_player_mut().battle_info.energy = 2;

        // Play Demon Form+
        let initial_energy = battle.get_player().get_energy();
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());

        // Energy should be spent (Demon Form+ costs 2)
        assert_eq!(battle.get_player().get_energy(), initial_energy - 2);

        // Demon Form+ should be in the powers pile (Power cards go to powers when played)
        let powers = battle.get_powers();
        assert_eq!(powers.len(), 1);
        assert_eq!(powers[0].get_name(), "Demon Form+");
    }

    #[test]
    fn test_demon_form_turn_based_strength_gain() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create battle with Demon Form in hand
        let deck = Deck::new(vec![demon_form()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        // Give player enough energy to play Demon Form
        battle.get_player_mut().battle_info.energy = 3;

        // Check initial strength
        let initial_strength = battle.get_player().battle_info.get_strength();
        assert_eq!(initial_strength, 0);

        // Play Demon Form
        let demon_form_idx = 0;
        let result = battle.play_card(demon_form_idx, Entity::Player);
        assert!(result.is_ok());

        // Demon Form should not give immediate strength
        let strength_after_play = battle.get_player().battle_info.get_strength();
        assert_eq!(strength_after_play, 0);

        // Simulate turn start event to trigger Demon Form effect
        let turn_start_event = crate::battle::events::BattleEvent::TurnStart { entity: crate::battle::target::Entity::Player };
        battle.emit_event(turn_start_event);

        // Now player should have gained 2 Strength from Demon Form
        let strength_after_turn_start = battle.get_player().battle_info.get_strength();
        assert_eq!(strength_after_turn_start, 2, "Player should gain 2 Strength at turn start from Demon Form");
    }

    #[test]
    fn test_demon_form_upgraded_turn_based_strength_gain() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create battle with Demon Form+ in hand
        let deck = Deck::new(vec![demon_form_upgraded()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        // Give player enough energy
        battle.get_player_mut().battle_info.energy = 2;

        // Play Demon Form+
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());

        // Demon Form+ should not give immediate strength
        let strength_after_play = battle.get_player().battle_info.get_strength();
        assert_eq!(strength_after_play, 0);

        // Simulate turn start event to trigger Demon Form+ effect
        let turn_start_event = crate::battle::events::BattleEvent::TurnStart { entity: crate::battle::target::Entity::Player };
        battle.emit_event(turn_start_event);

        // Now player should have gained 3 Strength from Demon Form+
        let strength_after_turn_start = battle.get_player().battle_info.get_strength();
        assert_eq!(strength_after_turn_start, 3, "Player should gain 3 Strength at turn start from Demon Form+");
    }

    #[test]
    fn test_demon_form_multiple_turns_strength_accumulation() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create battle with Demon Form in hand
        let deck = Deck::new(vec![demon_form()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        // Give player enough energy to play Demon Form
        battle.get_player_mut().battle_info.energy = 3;

        // Play Demon Form
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());

        let turn_start_event = crate::battle::events::BattleEvent::TurnStart { entity: crate::battle::target::Entity::Player };

        // Simulate multiple turn starts
        for turn in 1..=3 {
            battle.emit_event(turn_start_event.clone());
            let expected_strength = turn * 2; // 2 Strength per turn from Demon Form
            let current_strength = battle.get_player().battle_info.get_strength();
            assert_eq!(current_strength, expected_strength,
                "After turn {}, player should have {} Strength", turn, expected_strength);
        }
    }

    #[test]
    fn test_demon_form_multiple_instances() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create battle with multiple Demon Form cards
        let deck = Deck::new(vec![demon_form(), demon_form_upgraded()]);
        let mut battle = Battle::new(deck, global_info, 50, 80, enemies, &mut rng);

        // Give player enough energy for both cards
        battle.get_player_mut().battle_info.energy = 5;

        // Play first Demon Form
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());
        assert_eq!(battle.get_player().get_energy(), 2); // Spent 3 energy

        // Play Demon Form+
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());
        assert_eq!(battle.get_player().get_energy(), 0); // Spent 2 more energy

        // Verify both cards are in the powers pile (Power cards go to powers when played)
        let powers = battle.get_powers();
        assert_eq!(powers.len(), 2);
        let power_names: Vec<String> = powers.iter().map(|card| card.get_name().to_string()).collect();
        assert!(power_names.contains(&"Demon Form".to_string()), "Demon Form should be in powers pile");
        assert!(power_names.contains(&"Demon Form+".to_string()), "Demon Form+ should be in powers pile");

        // Verify neither card is in exhaust pile (Power cards go to powers, not exhaust)
        let exhausted_cards = battle.cards.get_exhausted();
        let demon_form_exhausted = exhausted_cards.iter()
            .filter(|card| card.get_name() == "Demon Form" || card.get_name() == "Demon Form+")
            .count();
        assert_eq!(demon_form_exhausted, 0, "No Demon Form cards should be exhausted");

        // Verify neither card is in discard pile (Power cards go to powers, not discard)
        let discard = battle.cards.get_discard_pile();
        let demon_form_in_discard = discard.iter()
            .filter(|card| card.get_name() == "Demon Form" || card.get_name() == "Demon Form+")
            .count();
        assert_eq!(demon_form_in_discard, 0, "No Demon Form cards should be in discard pile");
    }
}

pub struct DemonFormListener {
    source: Entity,
    strength_per_turn: u32,
}

impl DemonFormListener {
    pub fn new(source: Entity, strength_per_turn: u32) -> Self {
        Self { source, strength_per_turn }
    }
}

impl EventListener for DemonFormListener {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<Effect> {
        match event {
            BattleEvent::TurnStart { entity } if *entity == self.source => {
                if self.source == Entity::Player {
                    // Return a GainStrength effect to be processed
                    vec![Effect::GainStrength { amount: self.strength_per_turn }]
                } else {
                    vec![]
                }
            }
            _ => vec![]
        }
    }

    fn is_active(&self) -> bool {
        true // Demon Form is always active once played
    }

    fn get_owner(&self) -> Entity {
        self.source
    }
}