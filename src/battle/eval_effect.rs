use super::Battle;
use crate::game::effect::BaseEffect;
use crate::battle::{target::Entity, events::BattleEvent};

impl Battle {
    /// Apply a specific effect with its target
    pub(crate) fn eval_base_effect(&mut self, effect: &BaseEffect) {
        match effect {
            BaseEffect::AttackToTarget { source, target, amount, num_attacks } => {
                for _ in 0..*num_attacks {
                    let damage = match source {
                        Entity::Player => self.player.battle_info.calculate_damage(*amount),
                        Entity::Enemy(idx) => {
                            if *idx < self.enemies.len() {
                                self.enemies[*idx].battle_info.calculate_damage(*amount)
                            } else {
                                *amount // Fallback to base damage if enemy not found
                            }
                        },
                        Entity::None => *amount, // Use base damage
                    };
                    self.apply_damage(*target, damage);
                }
            },
            BaseEffect::GainDefense { source, amount } => {
                // Defense effects apply to the source entity
                self.apply_block(*source, *amount);
            },
            BaseEffect::ApplyVulnerable { target, duration } => {
                match target {
                    Entity::Player => self.player.battle_info.apply_vulnerable(*duration),
                    Entity::Enemy(idx) => {
                        if *idx < self.enemies.len() {
                            self.enemies[*idx].battle_info.apply_vulnerable(*duration);
                        }
                    },
                    Entity::None => {} // No target
                }
            },
            BaseEffect::ApplyWeak { target, duration } => {
                match target {
                    Entity::Player => self.player.battle_info.apply_weak(*duration),
                    Entity::Enemy(idx) => {
                        if *idx < self.enemies.len() {
                            self.enemies[*idx].battle_info.apply_weak(*duration);
                        }
                    },
                    Entity::None => {} // No target
                }
            },
            BaseEffect::GainStrength { source, amount } => {
                match source {
                    Entity::Player => self.player.battle_info.gain_strength(*amount),
                    Entity::Enemy(idx) => {
                        if *idx < self.enemies.len() {
                            self.enemies[*idx].battle_info.gain_strength(*amount);
                        }
                    },
                    Entity::None => {} // No source
                }
            },
            BaseEffect::GainRitual { source, amount } => {
                match source {
                    Entity::Player => self.player.battle_info.gain_ritual(*amount),
                    Entity::Enemy(idx) => {
                        if *idx < self.enemies.len() {
                            self.enemies[*idx].battle_info.gain_ritual(*amount);
                        }
                    },
                    Entity::None => {} // No source
                }
            },
            BaseEffect::ApplyFrail { target, duration } => {
                match target {
                    Entity::Player => {
                        self.player.battle_info.apply_frail(*duration);
                    },
                    Entity::Enemy(idx) => {
                        if *idx < self.enemies.len() {
                            self.enemies[*idx].battle_info.apply_frail(*duration);
                        }
                    },
                    Entity::None => {} // No target
                }
            },
            BaseEffect::AddSlimed { target, count } => {
                match target {
                    Entity::Player => {
                        for _ in 0..*count {
                            let slimed_card = crate::cards::status::slimed::slimed();
                            self.cards.add_card_to_discard(slimed_card);
                        }
                    },
                    Entity::Enemy(_) => {
                        // Enemies don't receive slimed cards
                    },
                    Entity::None => {} // No target
                }
            },
            BaseEffect::Exhaust { source: _ } => {
                // Exhaust effect is handled during card playing, not as a post-effect
                // This is here for completeness but should not be reached in normal gameplay
            },
            BaseEffect::ActivateEnrage { source, amount } => {
                // Add EnrageListener for the specified enemy
                if let Entity::Enemy(enemy_idx) = source {
                    let enrage_listener = crate::battle::listeners::EnrageListener::new(*source, *amount);
                    self.add_listener(Box::new(enrage_listener));
                }
            },
        }
    }

    /// Apply damage to an entity (player or enemy)
    pub(in crate::battle) fn apply_damage(&mut self, target: Entity, damage: u32) -> u32 {
        let actual_damage = match target {
            Entity::Player => self.player.battle_info.take_damage(damage),
            Entity::Enemy(idx) => {
                if idx < self.enemies.len() {
                    self.enemies[idx].battle_info.take_damage(damage)
                } else {
                    0 // Invalid enemy index, no damage dealt
                }
            }
            Entity::None => 0, // No target, no damage dealt
        };
        
        // Emit damage taken event if actual damage was dealt
        if actual_damage > 0 {
            let damage_event = BattleEvent::DamageTaken {
                target,
                amount: actual_damage,
                source: Entity::None, // TODO: Track damage source
            };
            self.emit_event(damage_event);
        }
        
        actual_damage
    }

    /// Apply block to an entity (player or enemy) 
    pub(in crate::battle) fn apply_block(&mut self, target: Entity, amount: u32) {
        match target {
            Entity::Player => self.player.battle_info.gain_block(amount),
            Entity::Enemy(idx) => {
                if idx < self.enemies.len() {
                    self.enemies[idx].battle_info.gain_block(amount);
                }
            }
            Entity::None => {} // No target, no block gained
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::ironclad::starter_deck::starter_deck;
    use crate::battle::enemy_in_battle::EnemyInBattle;
    use crate::enemies::{red_louse::RedLouse, enemy_enum::EnemyEnum};
    use crate::game::{global_info::GlobalInfo, card_type::CardType, enemy::EnemyTrait};

    #[test]
    fn test_eval_base_effect() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        let initial_enemy_hp = battle.enemies[0].battle_info.get_hp();
        let damage_effect = BaseEffect::AttackToTarget {
            source: Entity::Player,
            target: Entity::Enemy(0),
            amount: 10,
            num_attacks: 1,
        };
        
        battle.eval_base_effect(&damage_effect);
        
        assert_eq!(battle.enemies[0].battle_info.get_hp(), initial_enemy_hp - 10);
    }

    #[test]
    fn test_vulnerable_effect_integration() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        // Apply vulnerable to enemy
        let vulnerable_effect = BaseEffect::ApplyVulnerable { target: Entity::Enemy(0), duration: 2 };
        battle.eval_base_effect(&vulnerable_effect);
        
        // Check that enemy is vulnerable
        assert!(battle.enemies[0].battle_info.is_vulnerable());
        assert_eq!(battle.enemies[0].battle_info.get_vulnerable_turns(), 2);
        
        // Apply damage - should be increased by 50%
        let initial_hp = battle.enemies[0].battle_info.get_hp();
        
        let damage_effect = BaseEffect::AttackToTarget {
            source: Entity::Player,
            target: Entity::Enemy(0),
            amount: 10,
            num_attacks: 1,
        };
        battle.eval_base_effect(&damage_effect);
        
        // 10 damage * 1.5 = 15 damage should be dealt (but capped by enemy's HP)
        let expected_damage = 15u32.min(initial_hp);
        assert_eq!(battle.enemies[0].battle_info.get_hp(), initial_hp - expected_damage);
    }

    #[test]
    fn test_character_block_integration() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        // Give enemy some block
        battle.enemies[0].battle_info.gain_block(5);
        assert_eq!(battle.enemies[0].battle_info.get_block(), 5);
        
        let initial_hp = battle.enemies[0].battle_info.get_hp();
        let damage_effect = BaseEffect::AttackToTarget {
            source: Entity::Player,
            target: Entity::Enemy(0),
            amount: 8,
            num_attacks: 1,
        };
        battle.eval_base_effect(&damage_effect);
        
        // 8 damage - 5 block = 3 actual damage
        // But taking damage triggers Curl Up, giving enemy 3-7 more block (ascension 0)
        assert_eq!(battle.enemies[0].battle_info.get_hp(), initial_hp - 3);
        let curl_up_block = battle.enemies[0].battle_info.get_block();
        assert!(curl_up_block >= 3 && curl_up_block <= 7); // Curl Up activated with random amount
    }

    #[test]
    fn test_damage_to_player() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        let initial_hp = battle.player.battle_info.get_hp();
        
        // Create an effect that damages the player
        let damage_effect = BaseEffect::AttackToTarget {
            source: Entity::Enemy(0),
            target: Entity::Player,
            amount: 10,
            num_attacks: 1,
        };
        battle.eval_base_effect(&damage_effect);
        
        // Player should take 10 damage
        assert_eq!(battle.player.battle_info.get_hp(), initial_hp - 10);
    }

    #[test]
    fn test_attack_with_strength() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        // Give player some strength
        battle.player.battle_info.gain_strength(3);
        assert_eq!(battle.player.battle_info.get_strength(), 3);
        
        let initial_enemy_hp = battle.enemies[0].battle_info.get_hp();
        let attack_effect = BaseEffect::AttackToTarget {
            source: Entity::Player,
            target: Entity::Enemy(0),
            amount: 6,
            num_attacks: 1,
        };
        battle.eval_base_effect(&attack_effect);
        
        // 6 base damage + 3 strength = 9 total damage
        let expected_damage = 9u32.min(initial_enemy_hp);
        assert_eq!(battle.enemies[0].battle_info.get_hp(), initial_enemy_hp - expected_damage);
    }

    #[test]
    fn test_add_slimed_effect() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        let initial_discard_size = battle.cards.discard_pile_size();
        let initial_total_cards = battle.cards.total_cards();
        
        // Apply AddSlimed effect to add 2 Slimed cards
        let add_slimed_effect = BaseEffect::AddSlimed { 
            target: Entity::Player, 
            count: 2 
        };
        battle.eval_base_effect(&add_slimed_effect);
        
        // Should have 2 more cards in discard pile
        assert_eq!(battle.cards.discard_pile_size(), initial_discard_size + 2);
        assert_eq!(battle.cards.total_cards(), initial_total_cards + 2);
        
        // Check that the added cards are Slimed
        let discard_pile = battle.cards.get_discard_pile();
        let last_two_cards = &discard_pile[discard_pile.len()-2..];
        for card in last_two_cards {
            assert_eq!(card.get_name(), "Slimed");
            assert_eq!(card.get_cost(), 1);
            assert_eq!(card.get_card_type(), &CardType::Status);
        }
    }
}