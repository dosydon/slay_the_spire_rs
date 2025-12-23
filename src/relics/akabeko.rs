use crate::battle::battle_events::{BattleEvent, EventListener};
use crate::game::effect::BattleEffect;
use crate::battle::target::Entity;

/// Akabeko - Your first Attack each combat deals 8 additional damage
pub struct AkabekoRelic {
    triggered: bool,
    owner: Entity,
}

impl AkabekoRelic {
    pub fn new(owner: Entity) -> Self {
        Self {
            triggered: false,
            owner,
        }
    }
}

impl EventListener for AkabekoRelic {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<BattleEffect> {
        match event {
            BattleEvent::CombatStart { player } if *player == self.owner => {
                self.triggered = false;
                vec![]
            }
            BattleEvent::CardPlayed { source, card_type } if *source == self.owner && !self.triggered => {
                use crate::game::card_type::CardType;
                if *card_type == CardType::Attack {
                    self.triggered = true;
                    // Add a temporary effect that increases damage
                    // Since we can't directly modify the card being played, we add a damage bonus effect
                    vec![BattleEffect::AttackToTarget {
                        amount: 8,
                        num_attacks: 1,
                        strength_multiplier: 0,
                    }]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::card_type::CardType;

    #[test]
    fn test_akabeko_creation() {
        let akabeko = AkabekoRelic::new(Entity::Player);
        assert!(!akabeko.triggered);
    }

    #[test]
    fn test_akabeko_deals_bonus_on_first_attack() {
        let mut akabeko = AkabekoRelic::new(Entity::Player);

        // Reset on combat start
        let _ = akabeko.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });

        // Play first attack
        let effects = akabeko.on_event(&BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: CardType::Attack,
        });

        assert_eq!(effects.len(), 1);
        assert!(matches!(effects[0], BattleEffect::AttackToTarget { amount: 8, num_attacks: 1, strength_multiplier: 0 }));
        assert!(akabeko.triggered);
    }

    #[test]
    fn test_akabeko_only_triggers_once_per_combat() {
        let mut akabeko = AkabekoRelic::new(Entity::Player);

        let _ = akabeko.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });

        // First attack - should trigger
        let effects1 = akabeko.on_event(&BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: CardType::Attack,
        });
        assert_eq!(effects1.len(), 1);
        assert!(akabeko.triggered);

        // Second attack - should not trigger
        let effects2 = akabeko.on_event(&BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: CardType::Attack,
        });
        assert_eq!(effects2.len(), 0);
    }

    #[test]
    fn test_akabeko_resets_on_new_combat() {
        let mut akabeko = AkabekoRelic::new(Entity::Player);

        // First combat
        let _ = akabeko.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });
        let _ = akabeko.on_event(&BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: CardType::Attack,
        });
        assert!(akabeko.triggered);

        // New combat
        let _ = akabeko.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });
        assert!(!akabeko.triggered);

        // Should trigger again
        let effects = akabeko.on_event(&BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: CardType::Attack,
        });
        assert_eq!(effects.len(), 1);
    }

    #[test]
    fn test_akabeko_no_trigger_on_non_attack_cards() {
        let mut akabeko = AkabekoRelic::new(Entity::Player);

        let _ = akabeko.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });

        let effects = akabeko.on_event(&BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: CardType::Skill,
        });

        assert_eq!(effects.len(), 0);
        assert!(!akabeko.triggered);
    }
}
