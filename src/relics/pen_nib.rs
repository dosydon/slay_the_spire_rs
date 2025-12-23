use crate::battle::battle_events::{BattleEvent, EventListener};
use crate::game::effect::BattleEffect;
use crate::battle::target::Entity;
use crate::game::card_type::CardType;

/// Pen Nib - Every 10th Attack you play deals double damage
pub struct PenNibRelic {
    attack_count: u32,
    owner: Entity,
}

impl PenNibRelic {
    pub fn new(owner: Entity) -> Self {
        Self {
            attack_count: 0,
            owner,
        }
    }
}

impl EventListener for PenNibRelic {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<BattleEffect> {
        match event {
            BattleEvent::CombatStart { player } if *player == self.owner => {
                self.attack_count = 0;
                vec![]
            }
            BattleEvent::CardPlayed { source, card_type } if *source == self.owner => {
                if *card_type == CardType::Attack {
                    self.attack_count += 1;
                    // Check if this is the 10th attack (10, 20, 30, etc.)
                    if self.attack_count % 10 == 0 {
                        // Return a marker effect that the damage system should interpret
                        // For now, we'll use a special effect to indicate double damage
                        vec![BattleEffect::AttackToTarget {
                            amount: 0,
                            num_attacks: 0,
                            strength_multiplier: 0,
                        }]
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                }
            }
            _ => vec![]
        }
    }

    fn is_active(&self) -> bool {
        true
    }

    fn get_owner(&self) -> Entity {
        self.owner
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl PenNibRelic {
    /// Check if the next attack should deal double damage
    pub fn should_double_damage(&self) -> bool {
        self.attack_count % 10 == 0 && self.attack_count > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pen_nib_creation() {
        let nib = PenNibRelic::new(Entity::Player);
        assert_eq!(nib.attack_count, 0);
        assert!(!nib.should_double_damage());
    }

    #[test]
    fn test_pen_nib_counts_attacks() {
        let mut nib = PenNibRelic::new(Entity::Player);

        // Play 10 attacks
        for i in 1..=10 {
            let effects = nib.on_event(&BattleEvent::CardPlayed {
                source: Entity::Player,
                card_type: CardType::Attack,
            });
            if i == 10 {
                // Should trigger on 10th attack
                assert_eq!(effects.len(), 1);
            } else {
                assert_eq!(effects.len(), 0);
            }
        }

        assert_eq!(nib.attack_count, 10);
        assert!(nib.should_double_damage());
    }

    #[test]
    fn test_pen_nib_only_counts_attacks() {
        let mut nib = PenNibRelic::new(Entity::Player);

        // Play a skill
        let effects1 = nib.on_event(&BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: CardType::Skill,
        });
        assert_eq!(effects1.len(), 0);
        assert_eq!(nib.attack_count, 0);

        // Play a power
        let effects2 = nib.on_event(&BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: CardType::Power,
        });
        assert_eq!(effects2.len(), 0);
        assert_eq!(nib.attack_count, 0);
    }

    #[test]
    fn test_pen_nib_resets_on_combat_start() {
        let mut nib = PenNibRelic::new(Entity::Player);

        // Play 5 attacks
        for _ in 0..5 {
            let _ = nib.on_event(&BattleEvent::CardPlayed {
                source: Entity::Player,
                card_type: CardType::Attack,
            });
        }
        assert_eq!(nib.attack_count, 5);

        // Combat start resets
        let _ = nib.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });
        assert_eq!(nib.attack_count, 0);
        assert!(!nib.should_double_damage());
    }

    #[test]
    fn test_pen_nib_triggers_multiple_times() {
        let mut nib = PenNibRelic::new(Entity::Player);

        // Play 20 attacks - should trigger twice
        let mut trigger_count = 0;
        for _ in 0..20 {
            let effects = nib.on_event(&BattleEvent::CardPlayed {
                source: Entity::Player,
                card_type: CardType::Attack,
            });
            if effects.len() > 0 {
                trigger_count += 1;
            }
        }

        assert_eq!(trigger_count, 2);
        assert_eq!(nib.attack_count, 20);
    }

    #[test]
    fn test_pen_nib_no_trigger_for_enemy_cards() {
        let mut nib = PenNibRelic::new(Entity::Player);

        // Enemy plays attack - shouldn't count
        let effects = nib.on_event(&BattleEvent::CardPlayed {
            source: Entity::Enemy(0),
            card_type: CardType::Attack,
        });
        assert_eq!(effects.len(), 0);
        assert_eq!(nib.attack_count, 0);
    }
}
