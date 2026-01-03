use serde::{Serialize, Deserialize};
use crate::battle::battle_events::{BattleEvent, EventListener};
use crate::game::effect::BattleEffect;
use crate::battle::target::Entity;
use crate::game::card_type::CardType;

/// Ornamental Fan - Every time you play 3 Attacks in a single turn, gain 4 Block
#[derive(Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrnamentalFanRelic {
    attacks_this_turn: u32,
    owner: Entity,
}

impl OrnamentalFanRelic {
    pub fn new(owner: Entity) -> Self {
        Self {
            attacks_this_turn: 0,
            owner,
        }
    }
}

impl EventListener for OrnamentalFanRelic {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<BattleEffect> {
        match event {
            BattleEvent::CombatStart { player } if *player == self.owner => {
                self.attacks_this_turn = 0;
                vec![]
            }
            BattleEvent::StartOfPlayerTurn => {
                // Reset attack counter at start of turn
                self.attacks_this_turn = 0;
                vec![]
            }
            BattleEvent::CardPlayed { source, card_type } if *source == self.owner => {
                if *card_type == CardType::Attack {
                    self.attacks_this_turn += 1;
                    // Check if we've played 3, 6, 9, etc. attacks this turn
                    if self.attacks_this_turn % 3 == 0 && self.attacks_this_turn > 0 {
                        vec![BattleEffect::GainDefense { amount: 4 }]
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

    fn hash_to(&self, state: &mut std::collections::hash_map::DefaultHasher) {
        use std::hash::Hash;
        self.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ornamental_fan_creation() {
        let fan = OrnamentalFanRelic::new(Entity::Player);
        assert_eq!(fan.attacks_this_turn, 0);
    }

    #[test]
    fn test_ornamental_fan_gains_block_every_3_attacks() {
        let mut fan = OrnamentalFanRelic::new(Entity::Player);

        // Combat start
        let _ = fan.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });

        // Turn start
        let _ = fan.on_event(&BattleEvent::StartOfPlayerTurn);

        // Play 3 attacks
        let effects1 = fan.on_event(&BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: CardType::Attack,
        });
        assert_eq!(effects1.len(), 0);

        let effects2 = fan.on_event(&BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: CardType::Attack,
        });
        assert_eq!(effects2.len(), 0);

        let effects3 = fan.on_event(&BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: CardType::Attack,
        });
        assert_eq!(effects3.len(), 1);
        assert!(matches!(effects3[0], BattleEffect::GainDefense { amount: 4 }));
    }

    #[test]
    fn test_ornamental_fan_resets_on_turn_start() {
        let mut fan = OrnamentalFanRelic::new(Entity::Player);

        // Combat start and turn 1
        let _ = fan.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });
        let _ = fan.on_event(&BattleEvent::StartOfPlayerTurn);

        // Play 2 attacks
        let _ = fan.on_event(&BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: CardType::Attack,
        });
        let _ = fan.on_event(&BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: CardType::Attack,
        });
        assert_eq!(fan.attacks_this_turn, 2);

        // Turn 2 starts - should reset
        let _ = fan.on_event(&BattleEvent::StartOfPlayerTurn);
        assert_eq!(fan.attacks_this_turn, 0);
    }

    #[test]
    fn test_ornamental_fan_only_counts_attacks() {
        let mut fan = OrnamentalFanRelic::new(Entity::Player);

        let _ = fan.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });
        let _ = fan.on_event(&BattleEvent::StartOfPlayerTurn);

        // Play skills and powers - shouldn't count
        let effects1 = fan.on_event(&BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: CardType::Skill,
        });
        assert_eq!(effects1.len(), 0);
        assert_eq!(fan.attacks_this_turn, 0);

        let effects2 = fan.on_event(&BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: CardType::Power,
        });
        assert_eq!(effects2.len(), 0);
        assert_eq!(fan.attacks_this_turn, 0);
    }

    #[test]
    fn test_ornamental_fan_triggers_multiple_times_per_turn() {
        let mut fan = OrnamentalFanRelic::new(Entity::Player);

        let _ = fan.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });
        let _ = fan.on_event(&BattleEvent::StartOfPlayerTurn);

        // Play 6 attacks - should trigger twice
        let mut trigger_count = 0;
        for _ in 0..6 {
            let effects = fan.on_event(&BattleEvent::CardPlayed {
                source: Entity::Player,
                card_type: CardType::Attack,
            });
            if effects.len() > 0 {
                trigger_count += 1;
            }
        }

        assert_eq!(trigger_count, 2);
    }

    #[test]
    fn test_ornamental_fan_no_trigger_for_enemy_cards() {
        let mut fan = OrnamentalFanRelic::new(Entity::Player);

        let _ = fan.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });
        let _ = fan.on_event(&BattleEvent::StartOfPlayerTurn);

        // Enemy plays attack - shouldn't count
        let effects = fan.on_event(&BattleEvent::CardPlayed {
            source: Entity::Enemy(0),
            card_type: CardType::Attack,
        });

        assert_eq!(effects.len(), 0);
        assert_eq!(fan.attacks_this_turn, 0);
    }

    #[test]
    fn test_ornamental_fan_resets_on_combat_start() {
        let mut fan = OrnamentalFanRelic::new(Entity::Player);

        let _ = fan.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });
        let _ = fan.on_event(&BattleEvent::StartOfPlayerTurn);

        // Play some attacks
        let _ = fan.on_event(&BattleEvent::CardPlayed {
            source: Entity::Player,
            card_type: CardType::Attack,
        });
        assert_eq!(fan.attacks_this_turn, 1);

        // New combat resets
        let _ = fan.on_event(&BattleEvent::CombatStart {
            player: Entity::Player,
        });
        assert_eq!(fan.attacks_this_turn, 0);
    }
}
