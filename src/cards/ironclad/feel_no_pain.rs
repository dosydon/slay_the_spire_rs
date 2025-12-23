use crate::game::{card::{Card, Rarity, CardClass}, card_type::CardType, card_enum::CardEnum, effect::BattleEffect};
use crate::battle::battle_events::{BattleEvent, EventListener};
use crate::battle::target::Entity;

/// Event listener for Feel No Pain power
/// Whenever a card is exhausted, gain block
#[derive(Debug)]
pub struct FeelNoPainListener {
    owner: Entity,
    block_per_exhaust: u32,
}

impl FeelNoPainListener {
    pub fn new(owner: Entity, block_per_exhaust: u32) -> Self {
        Self {
            owner,
            block_per_exhaust,
        }
    }
}

impl EventListener for FeelNoPainListener {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<BattleEffect> {
        match event {
            BattleEvent::CardExhausted { source } if *source == self.owner => {
                // When the owner exhausts a card, gain block
                vec![BattleEffect::GainDefense {
                    amount: self.block_per_exhaust,
                }]
            }
            _ => vec![],
        }
    }

    fn is_active(&self) -> bool {
        true // Always active once played
    }

    fn get_owner(&self) -> Entity {
        self.owner
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Feel No Pain - Power Card
/// Cost: 1
/// Effect: Whenever you Exhaust a card, gain 3 Block.
pub fn feel_no_pain() -> Card {
    Card::new(CardEnum::FeelNoPain, 1, CardClass::IronClad(Rarity::Uncommon, CardType::Power), vec![BattleEffect::ActivateFeelNoPain { block_per_exhaust: 3 }])
        .set_playable(true)
}

/// Feel No Pain+ (Upgraded)
/// Cost: 1
/// Effect: Whenever you Exhaust a card, gain 4 Block.
pub fn feel_no_pain_upgraded() -> Card {
    Card::new(CardEnum::FeelNoPain, 1, CardClass::IronClad(Rarity::Uncommon, CardType::Power), vec![BattleEffect::ActivateFeelNoPain { block_per_exhaust: 4 }])
        .set_upgraded(true)
        .set_playable(true)
}

#[cfg(test)]
mod tests {
    use crate::game::PlayerRunState;

    use super::*;

    #[test]
    fn test_feel_no_pain_creation() {
        let card = feel_no_pain();

        assert_eq!(card.get_name(), "Feel No Pain");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), CardType::Power);
        assert_eq!(card.get_effects().len(), 1);
        match &card.get_effects()[0] {
            BattleEffect::ActivateFeelNoPain { block_per_exhaust } => {
                assert_eq!(*block_per_exhaust, 3);
            }
            _ => panic!("Expected ActivateFeelNoPain effect"),
        }
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_feel_no_pain_upgraded_creation() {
        let card = feel_no_pain_upgraded();

        assert_eq!(card.get_name(), "Feel No Pain+");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(card.get_card_type(), CardType::Power);
        assert_eq!(card.get_effects().len(), 1);
        match &card.get_effects()[0] {
            BattleEffect::ActivateFeelNoPain { block_per_exhaust } => {
                assert_eq!(*block_per_exhaust, 4);
            }
            _ => panic!("Expected ActivateFeelNoPain effect"),
        }
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_feel_no_pain_battle_integration() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::red_louse::RedLouse;
        use crate::enemies::enemy_enum::EnemyEnum;
        use crate::cards::status::slimed::slimed;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create battle with Feel No Pain and Slimed (an exhaust card)
        let deck = Deck::new(vec![feel_no_pain(), slimed()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        // Play Feel No Pain to activate the power
        let feel_no_pain_idx = battle.get_hand().iter().position(|c| c.get_name() == "Feel No Pain").unwrap();
        let result = battle.play_card(feel_no_pain_idx, Entity::Player);
        assert!(result.is_ok());

        // Verify Feel No Pain is now a power
        let powers = battle.get_powers();
        assert_eq!(powers.len(), 1);
        assert_eq!(powers[0].get_name(), "Feel No Pain");

        let initial_block = battle.get_player().get_block();

        // Play Slimed (which exhausts itself)
        let slimed_idx = battle.get_hand().iter().position(|c| c.get_name() == "Slimed").unwrap();
        let result = battle.play_card(slimed_idx, Entity::Player);
        assert!(result.is_ok());

        // Verify player gained 3 block from Feel No Pain when Slimed exhausted
        let final_block = battle.get_player().get_block();
        assert_eq!(final_block, initial_block + 3);

        // Verify Slimed was exhausted
        let exhausted_pile = battle.cards.get_exhausted();
        assert_eq!(exhausted_pile.len(), 1);
        assert_eq!(exhausted_pile[0].get_name(), "Slimed");
    }

    #[test]
    fn test_feel_no_pain_upgraded_battle_integration() {
        use crate::battle::Battle;
        use crate::battle::target::Entity;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::red_louse::RedLouse;
        use crate::enemies::enemy_enum::EnemyEnum;
        use crate::cards::status::slimed::slimed;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create battle with Feel No Pain+ and Slimed
        let deck = Deck::new(vec![feel_no_pain_upgraded(), slimed()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        // Play Feel No Pain+ to activate the power
        let feel_no_pain_idx = battle.get_hand().iter().position(|c| c.get_name() == "Feel No Pain+").unwrap();
        let result = battle.play_card(feel_no_pain_idx, Entity::Player);
        assert!(result.is_ok());

        // Verify Feel No Pain+ is now a power
        let powers = battle.get_powers();
        assert_eq!(powers.len(), 1);
        assert_eq!(powers[0].get_name(), "Feel No Pain+");

        let initial_block = battle.get_player().get_block();

        // Play Slimed (which exhausts itself)
        let slimed_idx = battle.get_hand().iter().position(|c| c.get_name() == "Slimed").unwrap();
        let result = battle.play_card(slimed_idx, Entity::Player);
        assert!(result.is_ok());

        // Verify player gained 4 block from Feel No Pain+ when Slimed exhausted
        let final_block = battle.get_player().get_block();
        assert_eq!(final_block, initial_block + 4);

        // Verify Slimed was exhausted
        let exhausted_pile = battle.cards.get_exhausted();
        assert_eq!(exhausted_pile.len(), 1);
        assert_eq!(exhausted_pile[0].get_name(), "Slimed");
    }

    #[test]
    fn test_feel_no_pain_costs_one_energy() {
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

        let deck = Deck::new(vec![feel_no_pain()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        let initial_energy = battle.get_player().get_energy();

        // Play Feel No Pain
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());

        // Verify energy was consumed
        assert_eq!(battle.get_player().get_energy(), initial_energy - 1);
    }
}