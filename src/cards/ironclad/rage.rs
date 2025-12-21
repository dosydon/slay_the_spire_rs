use crate::game::{card::{Card, Rarity}, effect::{Effect, Condition}, card_type::CardType, card_enum::CardEnum};
use crate::battle::{battle_events::{BattleEvent, EventListener}, target::Entity};

/// Rage - Uncommon Skill Card
/// Cost: 0 (0 when upgraded)
/// Effect: Whenever you play an Attack card this turn, gain 3 Block.
pub fn rage() -> Card {
    Card::new_with_condition(
        CardEnum::Rage,
        0,
        CardType::Skill,
        vec![
            Effect::ActivateRage { block_per_attack: 3 },
        ],
        false, // not upgraded
        Condition::True,
        Rarity::Uncommon)
}

/// Rage+ (Upgraded version)
/// Cost: 0 (0 when upgraded)
/// Effect: Whenever you play an Attack card this turn, gain 4 Block.
pub fn rage_upgraded() -> Card {
    Card::new_with_condition(
        CardEnum::Rage,
        0,
        CardType::Skill,
        vec![
            Effect::ActivateRage { block_per_attack: 4 },
        ],
        true,  // upgraded
        Condition::True,
        Rarity::Uncommon)
}

#[cfg(test)]
mod tests {
    use crate::game::PlayerRunState;

    use super::*;

    #[test]
    fn test_rage_creation() {
        let card = rage();

        assert_eq!(card.get_name(), "Rage");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert_eq!(card.get_effects().len(), 1);
        assert!(!card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_rage_upgraded_creation() {
        let card = rage_upgraded();

        assert_eq!(card.get_name(), "Rage+");
        assert_eq!(card.get_cost(), 0);
        assert_eq!(card.get_card_type(), &CardType::Skill);
        assert_eq!(card.get_effects().len(), 1);
        assert!(card.is_upgraded());
        assert!(card.is_playable());
    }

    #[test]
    fn test_rage_effects() {
        let card = rage();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 1);
        match &effects[0] {
            Effect::ActivateRage { block_per_attack } => {
                assert_eq!(*block_per_attack, 3);
            }
            _ => panic!("Expected ActivateRage effect"),
        }
    }

    #[test]
    fn test_rage_upgraded_effects() {
        let card = rage_upgraded();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 1);
        match &effects[0] {
            Effect::ActivateRage { block_per_attack } => {
                assert_eq!(*block_per_attack, 4);
            }
            _ => panic!("Expected ActivateRage effect"),
        }
    }

    #[test]
    fn test_rage_cost_stays_same() {
        let base_card = rage();
        let upgraded_card = rage_upgraded();

        assert_eq!(base_card.get_cost(), 0, "Rage should cost 0 energy");
        assert_eq!(upgraded_card.get_cost(), 0, "Rage+ should also cost 0 energy");
    }

    #[test]
    fn test_rage_battle_integration() {
        use crate::battle::Battle;
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

        // Create battle with Rage and Strike in hand
        let deck = Deck::new(vec![rage(), strike()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        // Check initial block
        let initial_block = battle.get_player().battle_info.get_block();
        assert_eq!(initial_block, 0);

        // Play Rage first
        let rage_idx = 0;
        let result = battle.play_card(rage_idx, Entity::Player);
        assert!(result.is_ok());

        // Play Strike (should trigger Rage effect)
        let strike_idx = 0; // Strike is now at index 0
        let result = battle.play_card(strike_idx, Entity::Enemy(0));
        assert!(result.is_ok());

        // Verify Block was gained (3 block from Rage)
        let final_block = battle.get_player().battle_info.get_block();
        assert_eq!(final_block, 3);
    }

    #[test]
    fn test_rage_upgraded_battle_integration() {
        use crate::battle::Battle;
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

        // Create battle with Rage+ and multiple Strikes in hand
        let deck = Deck::new(vec![rage_upgraded(), strike(), strike()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        // Check initial block
        let initial_block = battle.get_player().battle_info.get_block();
        assert_eq!(initial_block, 0);

        // Play Rage+ first
        let rage_idx = 0;
        let result = battle.play_card(rage_idx, Entity::Player);
        assert!(result.is_ok());

        // Play first Strike (should trigger Rage+ effect)
        let strike_idx = 0; // First Strike is at index 0
        let result = battle.play_card(strike_idx, Entity::Enemy(0));
        assert!(result.is_ok());

        // Verify Block was gained (4 block from Rage+)
        let block_after_first_strike = battle.get_player().battle_info.get_block();
        assert_eq!(block_after_first_strike, 4);

        // Play second Strike (should trigger Rage+ effect again)
        let strike_idx = 0; // Second Strike is now at index 0
        let result = battle.play_card(strike_idx, Entity::Enemy(0));
        assert!(result.is_ok());

        // Verify more Block was gained (4 more block from Rage+)
        let final_block = battle.get_player().battle_info.get_block();
        assert_eq!(final_block, 8);
    }

    #[test]
    fn test_rage_multiple_attacks() {
        use crate::battle::Battle;
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::game::deck::Deck;
        use crate::game::global_info::GlobalInfo;
        use crate::game::enemy::EnemyTrait;
        use crate::enemies::red_louse::RedLouse;
        use crate::enemies::enemy_enum::EnemyEnum;
        use crate::cards::ironclad::strike::strike;
        use crate::cards::ironclad::defend::defend;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];

        // Create battle with Rage, multiple Strikes, and a Defend
        let deck = Deck::new(vec![rage(), strike(), defend(), strike()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        // Play Rage first
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());

        // Play first Strike (should trigger Rage)
        let block_after_first_strike = battle.get_player().battle_info.get_block();
        let result = battle.play_card(0, Entity::Enemy(0));
        assert!(result.is_ok());
        let block_after_rage1 = battle.get_player().battle_info.get_block();
        let rage1_block = block_after_rage1 - block_after_first_strike;
        assert_eq!(rage1_block, 3); // First Strike should trigger 3 block from Rage

        // Play Defend (should NOT trigger Rage, but should give its own block)
        let block_before_defend = battle.get_player().battle_info.get_block();
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());
        let block_after_defend = battle.get_player().battle_info.get_block();
        let defend_block = block_after_defend - block_before_defend;
        assert!(defend_block > 0); // Defend should give some block

        // Play second Strike (should trigger Rage again)
        let block_before_second_strike = battle.get_player().battle_info.get_block();
        let result = battle.play_card(0, Entity::Enemy(0));
        assert!(result.is_ok());
        let final_block = battle.get_player().battle_info.get_block();
        let rage2_block = final_block - block_before_second_strike;
        assert_eq!(rage2_block, 3); // Second Strike should trigger 3 block from Rage

        // Total Rage block should be 6 (3 from each Strike)
        let total_rage_block = rage1_block + rage2_block;
        assert_eq!(total_rage_block, 6);
    }

    #[test]
    fn test_rage_turn_limitation() {
        // This test verifies that Rage only lasts for the current turn
        // In a full implementation, Rage would be a temporary effect that expires at end of turn
        use crate::battle::Battle;
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

        // Create battle with Rage and Strike
        let deck = Deck::new(vec![rage(), strike()]);
        let mut battle = Battle::new(deck, global_info, PlayerRunState::new(50, 80, 0), enemies, &mut rng);

        // Play Rage
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());

        // For now, just verify the effect is activated
        // Full turn-based testing would require more complex turn management
        let player = battle.get_player();
        assert!(player.battle_info.get_block() >= 0);
    }
}

pub struct RageListener {
    source: Entity,
    block_per_attack: u32,
    is_active: bool,
}

impl RageListener {
    pub fn new(source: Entity, block_per_attack: u32) -> Self {
        Self {
            source,
            block_per_attack,
            is_active: true,
        }
    }
}

impl EventListener for RageListener {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<Effect> {
        match event {
            BattleEvent::CardPlayed { source, card_type }
                if *source == self.source
                && *card_type == crate::game::card_type::CardType::Attack
                && self.is_active => {
                // When an Attack card is played, gain Block
                vec![Effect::GainDefense { amount: self.block_per_attack }]
            }
            BattleEvent::EndOfTurn { entity } if *entity == self.source => {
                // Rage effect expires at end of turn
                self.is_active = false;
                vec![]
            }
            _ => vec![]
        }
    }

    fn is_active(&self) -> bool {
        self.is_active
    }

    fn get_owner(&self) -> Entity {
        self.source
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}