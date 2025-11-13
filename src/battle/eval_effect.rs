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