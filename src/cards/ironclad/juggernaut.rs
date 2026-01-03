use crate::game::{card::Card, card_type::CardType, card_enum::CardEnum, effect::Effect};
use crate::battle::{events::{BattleEvent, EventListener}, target::Entity};

pub fn juggernaut() -> Card {
    Card::new(CardEnum::Juggernaut, 2, CardClass::IronClad(Rarity::Uncommon, CardType::Power), vec![
            Effect::ActivateJuggernaut { damage_per_block: 5 },
        ])
        .set_playable(true)
}

pub fn juggernaut_upgraded() -> Card {
    Card::new(CardEnum::Juggernaut, 2, CardClass::IronClad(Rarity::Uncommon, CardType::Power), vec![
            Effect::ActivateJuggernaut { damage_per_block: 7 },
        ])
        .set_upgraded(true)
        .set_playable(true)
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct JuggernautListener {
    source: Entity,
    damage_per_block: u32,
}

impl JuggernautListener {
    pub fn new(source: Entity, damage_per_block: u32) -> Self {
        Self {
            source,
            damage_per_block,
        }
    }
}

impl EventListener for JuggernautListener {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<Effect> {
        match event {
            BattleEvent::BlockGained { source, amount }
                if *source == self.source
                && *amount > 0 => {
                // When block is gained, deal damage to a random enemy
                let total_damage = amount * self.damage_per_block;
                vec![
                    Effect::AttackRandomEnemy {
                        amount: total_damage,
                        num_attacks: 1,
                        strength_multiplier: 1
                    }
                ]
            }
            _ => vec![]
        }
    }

    fn is_active(&self) -> bool {
        true // Juggernaut stays active indefinitely
    }

    fn get_owner(&self) -> Entity {
        self.source
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn hash_to(&self, state: &mut std::collections::hash_map::DefaultHasher) {
        use std::hash::Hash;
        self.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_juggernaut_creation() {
        let card = juggernaut();
        assert_eq!(card.get_name(), "Juggernaut");
        assert_eq!(card.get_cost(), 2);
        assert_eq!(card.get_card_type(), CardType::Power);
        assert!(!card.is_upgraded());
        assert_eq!(card.get_effects().len(), 1);
        assert!(card.is_playable());
    }

    #[test]
    fn test_juggernaut_upgraded_creation() {
        let card = juggernaut_upgraded();
        assert_eq!(card.get_name(), "Juggernaut+");
        assert_eq!(card.get_cost(), 2);
        assert_eq!(card.get_card_type(), CardType::Power);
        assert!(card.is_upgraded());
        assert_eq!(card.get_effects().len(), 1);
        assert!(card.is_playable());
    }

    #[test]
    fn test_juggernaut_effects() {
        let card = juggernaut();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 1);
        match &effects[0] {
            Effect::ActivateJuggernaut { damage_per_block } => {
                assert_eq!(*damage_per_block, 5);
            }
            _ => panic!("Expected ActivateJuggernaut effect"),
        }
    }

    #[test]
    fn test_juggernaut_upgraded_effects() {
        let card = juggernaut_upgraded();
        let effects = card.get_effects();

        assert_eq!(effects.len(), 1);
        match &effects[0] {
            Effect::ActivateJuggernaut { damage_per_block } => {
                assert_eq!(*damage_per_block, 7);
            }
            _ => panic!("Expected ActivateJuggernaut effect"),
        }
    }

    #[test]
    fn test_juggernaut_listener_creation() {
        let listener = JuggernautListener::new(Entity::Player, 5);
        assert_eq!(listener.source, Entity::Player);
        assert_eq!(listener.damage_per_block, 5);
        assert!(listener.is_active());
    }

    #[test]
    fn test_juggernaut_listener_on_block_gain() {
        let mut listener = JuggernautListener::new(Entity::Player, 5);

        // Simulate gaining 10 block
        let event = BattleEvent::BlockGained {
            source: Entity::Player,
            amount: 10,
        };

        let effects = listener.on_event(&event);
        assert_eq!(effects.len(), 1);

        match &effects[0] {
            Effect::AttackRandomEnemy { amount, .. } => {
                assert_eq!(*amount, 50); // 10 block * 5 damage per block
            }
            _ => panic!("Expected AttackRandomEnemy effect"),
        }
    }

    #[test]
    fn test_juggernaut_listener_no_block_gain() {
        let mut listener = JuggernautListener::new(Entity::Player, 5);

        // Simulate another entity gaining block
        let event = BattleEvent::BlockGained {
            source: Entity::Enemy(0),
            amount: 10,
        };

        let effects = listener.on_event(&event);
        assert_eq!(effects.len(), 0);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::battle::Battle;
    use crate::battle::target::Entity;
    use crate::battle::enemy_in_battle::EnemyInBattle;
    use crate::game::deck::Deck;
    use crate::game::global_info::GlobalInfo;
    use crate::game::enemy::EnemyTrait;
    use crate::enemies::jaw_worm::JawWorm;
    use crate::enemies::enemy_enum::EnemyEnum;
    use crate::cards::ironclad::defend::defend;

    #[test]
    fn test_juggernaut_is_power_card() {
        let card = juggernaut();
        assert_eq!(card.get_card_type(), CardType::Power);
        assert_eq!(card.get_cost(), 2);
        assert!(!card.is_upgraded());
    }

    #[test]
    fn test_juggernaut_upgraded_is_power_card() {
        let card = juggernaut_upgraded();
        assert_eq!(card.get_card_type(), CardType::Power);
        assert_eq!(card.get_cost(), 2);
        assert!(card.is_upgraded());
    }

    #[test]
    fn test_juggernaut_activates_listener() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm))];

        let deck = Deck::new(vec![juggernaut()]);
        let player_state = crate::game::player_run_state::PlayerRunState::new(50, 80, 0);
let mut battle = Battle::new(deck, global_info, player_state, enemies, &mut rng);

        // Play Juggernaut
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());

        // Verify the power card was played successfully
    }

    #[test]
    fn test_juggernaut_costs_two_energy() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm))];

        let deck = Deck::new(vec![juggernaut()]);
        let player_state = crate::game::player_run_state::PlayerRunState::new(50, 80, 0);
let mut battle = Battle::new(deck, global_info, player_state, enemies, &mut rng);

        let initial_energy = battle.get_player().get_energy();

        // Play Juggernaut
        let result = battle.play_card(0, Entity::Player);
        assert!(result.is_ok());

        // Verify energy was consumed (costs 2)
        assert_eq!(battle.get_player().get_energy(), initial_energy - 2);
    }

    #[test]
    fn test_juggernaut_triggers_on_block_gain() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm))];

        // Create battle with Juggernaut and Defend
        let deck = Deck::new(vec![juggernaut(), defend()]);
        let player_state = crate::game::player_run_state::PlayerRunState::new(50, 80, 0);
let mut battle = Battle::new(deck, global_info, player_state, enemies, &mut rng);

        // Play Juggernaut first
        let juggernaut_idx = battle.get_hand().iter().position(|c| c.get_name() == "Juggernaut").unwrap();
        let result = battle.play_card(juggernaut_idx, Entity::Player);
        assert!(result.is_ok());

        let initial_enemy_hp = battle.get_enemies()[0].get_current_hp();

        // Play Defend to gain block (which should trigger Juggernaut)
        let defend_idx = battle.get_hand().iter().position(|c| c.get_name() == "Defend").unwrap();
        let result = battle.play_card(defend_idx, Entity::Player);
        assert!(result.is_ok());

        // Verify player gained block
        assert!(battle.get_player().get_block() > 0);

        // Verify enemy took damage from Juggernaut trigger
        // Defend gives 5 block, Juggernaut deals 5 damage per block = 25 damage
        let final_enemy_hp = battle.get_enemies()[0].get_current_hp();
        assert_eq!(final_enemy_hp, initial_enemy_hp - 25);
    }

    #[test]
    fn test_juggernaut_upgraded_higher_damage() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let jaw_worm = JawWorm::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::JawWorm(jaw_worm))];

        // Create battle with upgraded Juggernaut and Defend
        let deck = Deck::new(vec![juggernaut_upgraded(), defend()]);
        let player_state = crate::game::player_run_state::PlayerRunState::new(50, 80, 0);
let mut battle = Battle::new(deck, global_info, player_state, enemies, &mut rng);

        // Play Juggernaut+ first
        let juggernaut_idx = battle.get_hand().iter().position(|c| c.get_name() == "Juggernaut+").unwrap();
        let result = battle.play_card(juggernaut_idx, Entity::Player);
        assert!(result.is_ok());

        let initial_enemy_hp = battle.get_enemies()[0].get_current_hp();

        // Play Defend to gain block (which should trigger Juggernaut+)
        let defend_idx = battle.get_hand().iter().position(|c| c.get_name() == "Defend").unwrap();
        let result = battle.play_card(defend_idx, Entity::Player);
        assert!(result.is_ok());

        // Verify enemy took higher damage from upgraded Juggernaut
        // Defend gives 5 block, Juggernaut+ deals 7 damage per block = 35 damage
        let final_enemy_hp = battle.get_enemies()[0].get_current_hp();
        assert_eq!(final_enemy_hp, initial_enemy_hp - 35);
    }

    #[test]
    fn test_juggernaut_listener_calculation() {
        let listener = JuggernautListener::new(Entity::Player, 5);

        // Verify listener is created with correct parameters
        assert_eq!(listener.source, Entity::Player);
        assert_eq!(listener.damage_per_block, 5);
        assert!(listener.is_active());
    }

    #[test]
    fn test_juggernaut_upgraded_listener_calculation() {
        let listener = JuggernautListener::new(Entity::Player, 7);

        // Verify upgraded listener has higher damage multiplier
        assert_eq!(listener.damage_per_block, 7);
    }
}